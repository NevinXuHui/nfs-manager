use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Platform {
    MacOS,
    Linux,
    Windows,
}

impl Platform {
    pub fn current() -> Self {
        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(target_os = "linux")]
        return Platform::Linux;

        #[cfg(target_os = "windows")]
        return Platform::Windows;
    }
}

#[derive(Error, Debug)]
pub enum NfsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Mount failed: {0}")]
    MountFailed(String),
    #[error("Umount failed: {0}")]
    UmountFailed(String),
    #[error("Config error: {0}")]
    ConfigError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfsConfig {
    pub name: String,
    pub server: String,
    pub mount_point: String,
    pub options: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountStatus {
    pub name: String,
    pub mounted: bool,
    pub mount_point: String,
    pub actual_path: String,
}

impl NfsConfig {
    pub fn from_line(line: &str) -> Result<Self, NfsError> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 4 {
            return Err(NfsError::ConfigError(format!("Invalid config line: {}", line)));
        }

        Ok(NfsConfig {
            name: parts[0].trim().to_string(),
            server: parts[1].trim().to_string(),
            mount_point: parts[2].trim().to_string(),
            options: parts[3].trim().to_string(),
        })
    }

    pub fn to_line(&self) -> String {
        format!("{}|{}|{}|{}", self.name, self.server, self.mount_point, self.options)
    }

    pub fn mount(&self) -> Result<(), NfsError> {
        let platform = Platform::current();
        let mount_path = self.get_mount_path();

        std::fs::create_dir_all(&mount_path)?;

        match platform {
            Platform::MacOS => {
                // macOS requires sudo and resvport option for NFS mount
                let output = Command::new("osascript")
                    .arg("-e")
                    .arg(format!(
                        "do shell script \"mount -t nfs -o resvport,{} {} {}\" with administrator privileges",
                        &self.options,
                        &self.server,
                        mount_path.to_string_lossy()
                    ))
                    .output()?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(NfsError::MountFailed(err.to_string()));
                }
            }
            Platform::Linux => {
                // Linux may need sudo, try with pkexec or sudo
                let output = Command::new("pkexec")
                    .arg("mount")
                    .arg("-t")
                    .arg("nfs")
                    .arg("-o")
                    .arg(&self.options)
                    .arg(&self.server)
                    .arg(&mount_path)
                    .output();

                let result = if output.is_err() {
                    // Fallback to sudo if pkexec not available
                    Command::new("sudo")
                        .arg("mount")
                        .arg("-t")
                        .arg("nfs")
                        .arg("-o")
                        .arg(&self.options)
                        .arg(&self.server)
                        .arg(&mount_path)
                        .output()?
                } else {
                    output?
                };

                if !result.status.success() {
                    let err = String::from_utf8_lossy(&result.stderr);
                    return Err(NfsError::MountFailed(err.to_string()));
                }
            }
            Platform::Windows => {
                // Windows uses net use command
                let drive_letter = self.mount_point.chars().next()
                    .ok_or_else(|| NfsError::ConfigError("Invalid mount point".to_string()))?;

                let server_path = self.server.replace(':', "");
                let unc_path = format!("\\\\{}", server_path.replace('/', "\\"));

                let output = Command::new("net")
                    .args(&["use", &format!("{}:", drive_letter), &unc_path])
                    .output()?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(NfsError::MountFailed(err.to_string()));
                }
            }
        }

        Ok(())
    }

    pub fn umount(&self, force: bool) -> Result<(), NfsError> {
        let platform = Platform::current();
        let mount_path = self.get_mount_path();

        match platform {
            Platform::MacOS => {
                // macOS: try diskutil first, fallback to umount -f
                let diskutil_cmd = if force {
                    format!("diskutil unmount force {}", mount_path.to_string_lossy())
                } else {
                    format!("diskutil unmount {}", mount_path.to_string_lossy())
                };

                let output = Command::new("osascript")
                    .arg("-e")
                    .arg(format!(
                        "do shell script \"{}\" with administrator privileges",
                        diskutil_cmd
                    ))
                    .output()?;

                if !output.status.success() {
                    // Fallback to umount -f
                    let umount_output = Command::new("osascript")
                        .arg("-e")
                        .arg(format!(
                            "do shell script \"umount -f {}\" with administrator privileges",
                            mount_path.to_string_lossy()
                        ))
                        .output()?;

                    if !umount_output.status.success() {
                        let err = String::from_utf8_lossy(&umount_output.stderr);
                        return Err(NfsError::UmountFailed(err.to_string()));
                    }
                }
            }
            Platform::Linux => {
                let mut args = vec!["umount"];
                if force {
                    args.push("-f");
                }
                args.push(mount_path.to_str().unwrap());

                let output = Command::new("pkexec")
                    .args(&args)
                    .output();

                let result = if output.is_err() {
                    // Fallback to sudo
                    Command::new("sudo")
                        .args(&args)
                        .output()?
                } else {
                    output?
                };

                if !result.status.success() {
                    let err = String::from_utf8_lossy(&result.stderr);
                    return Err(NfsError::UmountFailed(err.to_string()));
                }
            }
            Platform::Windows => {
                let drive_letter = self.mount_point.chars().next()
                    .ok_or_else(|| NfsError::ConfigError("Invalid mount point".to_string()))?;

                let output = Command::new("net")
                    .args(&["use", &format!("{}:", drive_letter), "/delete"])
                    .output()?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    return Err(NfsError::UmountFailed(err.to_string()));
                }
            }
        }

        Ok(())
    }

    pub fn is_mounted(&self) -> bool {
        let platform = Platform::current();
        let mount_path = self.get_mount_path();

        match platform {
            Platform::MacOS | Platform::Linux => {
                Command::new("mount")
                    .output()
                    .ok()
                    .and_then(|output| {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        Some(stdout.contains(&mount_path.to_string_lossy().to_string()))
                    })
                    .unwrap_or(false)
            }
            Platform::Windows => {
                let drive_letter = match self.mount_point.chars().next() {
                    Some(c) => c,
                    None => return false,
                };

                Command::new("net")
                    .args(&["use"])
                    .output()
                    .ok()
                    .and_then(|output| {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        Some(stdout.contains(&format!("{}:", drive_letter)))
                    })
                    .unwrap_or(false)
            }
        }
    }

    pub fn open_in_file_manager(&self) -> Result<(), NfsError> {
        let platform = Platform::current();
        let mount_path = self.get_mount_path();

        if !mount_path.exists() {
            return Err(NfsError::ConfigError(format!(
                "挂载点不存在: {}",
                mount_path.display()
            )));
        }

        match platform {
            Platform::MacOS => {
                Command::new("open")
                    .arg(&mount_path)
                    .spawn()
                    .map_err(|e| NfsError::ConfigError(format!("打开文件管理器失败: {}", e)))?;
            }
            Platform::Linux => {
                // Try xdg-open first, fallback to common file managers
                let result = Command::new("xdg-open")
                    .arg(&mount_path)
                    .spawn();

                if result.is_err() {
                    // Try common file managers
                    let file_managers = ["nautilus", "dolphin", "thunar", "nemo", "caja"];
                    let mut opened = false;

                    for fm in &file_managers {
                        if Command::new(fm).arg(&mount_path).spawn().is_ok() {
                            opened = true;
                            break;
                        }
                    }

                    if !opened {
                        return Err(NfsError::ConfigError(
                            "未找到可用的文件管理器".to_string(),
                        ));
                    }
                }
            }
            Platform::Windows => {
                Command::new("explorer")
                    .arg(&mount_path)
                    .spawn()
                    .map_err(|e| NfsError::ConfigError(format!("打开文件管理器失败: {}", e)))?;
            }
        }

        Ok(())
    }

    pub fn get_actual_mount_path(&self) -> String {
        self.get_mount_path().to_string_lossy().to_string()
    }

    fn get_mount_path(&self) -> PathBuf {
        let platform = Platform::current();

        match platform {
            Platform::MacOS | Platform::Linux => {
                // If mount_point is an absolute path, use it directly
                if self.mount_point.starts_with('/') {
                    PathBuf::from(&self.mount_point)
                } else {
                    // Otherwise, use ~/nfs-mounts/ prefix
                    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                    PathBuf::from(&home).join("nfs-mounts").join(&self.mount_point)
                }
            }
            Platform::Windows => {
                // Windows uses drive letters
                PathBuf::from(&self.mount_point)
            }
        }
    }
}
