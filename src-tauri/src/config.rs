use crate::nfs::{NfsConfig, NfsError};
use std::fs;
use std::path::PathBuf;

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, NfsError> {
        let home = std::env::var("HOME")
            .map_err(|_| NfsError::ConfigError("HOME not set".to_string()))?;
        let config_path = PathBuf::from(home).join(".nfs-manager.conf");

        if !config_path.exists() {
            fs::write(&config_path, "")?;
        }

        Ok(ConfigManager { config_path })
    }

    pub fn load_configs(&self) -> Result<Vec<NfsConfig>, NfsError> {
        let content = fs::read_to_string(&self.config_path)?;
        let mut configs = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            configs.push(NfsConfig::from_line(line)?);
        }

        Ok(configs)
    }

    pub fn save_configs(&self, configs: &[NfsConfig]) -> Result<(), NfsError> {
        let content = configs
            .iter()
            .map(|c| c.to_line())
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&self.config_path, content)?;
        Ok(())
    }

    pub fn add_config(&self, config: NfsConfig) -> Result<(), NfsError> {
        let mut configs = self.load_configs()?;

        if configs.iter().any(|c| c.name == config.name) {
            return Err(NfsError::ConfigError(format!(
                "Config with name '{}' already exists",
                config.name
            )));
        }

        configs.push(config);
        self.save_configs(&configs)
    }

    pub fn remove_config(&self, name: &str) -> Result<(), NfsError> {
        let mut configs = self.load_configs()?;
        configs.retain(|c| c.name != name);
        self.save_configs(&configs)
    }

    pub fn get_config(&self, name: &str) -> Result<NfsConfig, NfsError> {
        self.load_configs()?
            .into_iter()
            .find(|c| c.name == name)
            .ok_or_else(|| NfsError::ConfigError(format!("Config '{}' not found", name)))
    }
}
