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
            Platform::MacOS | Platform::Linux => {
                let output = Command::new("mount")
                    .arg("-t")
                    .arg("nfs")
                    .arg("-o")
                    .arg(&self.options)
                    .arg(&self.server)
                    .arg(&mount_path)
                    .output()?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
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
            Platform::MacOS | Platform::Linux => {
                let mut cmd = Command::new("umount");
                if force {
                    cmd.arg("-f");
                }
                cmd.arg(&mount_path);

                let output = cmd.output()?;

                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
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

    fn get_mount_path(&self) -> PathBuf {
        let platform = Platform::current();

        match platform {
            Platform::MacOS | Platform::Linux => {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                PathBuf::from(&home).join("nfs-mounts").join(&self.mount_point)
            }
            Platform::Windows => {
                // Windows uses drive letters
                PathBuf::from(&self.mount_point)
            }
        }
    }
}
