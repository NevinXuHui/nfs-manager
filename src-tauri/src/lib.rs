mod config;
mod nfs;

use config::ConfigManager;
use nfs::{MountStatus, NfsConfig};
use tauri::State;
use std::sync::Mutex;

struct AppState {
    config_manager: Mutex<ConfigManager>,
}

#[tauri::command]
fn get_configs(state: State<AppState>) -> Result<Vec<NfsConfig>, String> {
    state
        .config_manager
        .lock()
        .unwrap()
        .load_configs()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn add_config(state: State<AppState>, config: NfsConfig) -> Result<(), String> {
    state
        .config_manager
        .lock()
        .unwrap()
        .add_config(config)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn remove_config(state: State<AppState>, name: String) -> Result<(), String> {
    state
        .config_manager
        .lock()
        .unwrap()
        .remove_config(&name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn mount_nfs(state: State<AppState>, name: String) -> Result<(), String> {
    let config = state
        .config_manager
        .lock()
        .unwrap()
        .get_config(&name)
        .map_err(|e| e.to_string())?;

    config.mount().map_err(|e| e.to_string())
}

#[tauri::command]
fn mount_all(state: State<AppState>) -> Result<Vec<String>, String> {
    let configs = state
        .config_manager
        .lock()
        .unwrap()
        .load_configs()
        .map_err(|e| e.to_string())?;

    let mut errors = Vec::new();

    for config in configs {
        if let Err(e) = config.mount() {
            errors.push(format!("{}: {}", config.name, e));
        }
    }

    if errors.is_empty() {
        Ok(vec![])
    } else {
        Ok(errors)
    }
}

#[tauri::command]
fn umount_nfs(state: State<AppState>, name: String, force: bool) -> Result<(), String> {
    let config = state
        .config_manager
        .lock()
        .unwrap()
        .get_config(&name)
        .map_err(|e| e.to_string())?;

    config.umount(force).map_err(|e| e.to_string())
}

#[tauri::command]
fn umount_all(state: State<AppState>, force: bool) -> Result<Vec<String>, String> {
    let configs = state
        .config_manager
        .lock()
        .unwrap()
        .load_configs()
        .map_err(|e| e.to_string())?;

    let mut errors = Vec::new();

    for config in configs {
        if let Err(e) = config.umount(force) {
            errors.push(format!("{}: {}", config.name, e));
        }
    }

    if errors.is_empty() {
        Ok(vec![])
    } else {
        Ok(errors)
    }
}

#[tauri::command]
fn get_status(state: State<AppState>) -> Result<Vec<MountStatus>, String> {
    let configs = state
        .config_manager
        .lock()
        .unwrap()
        .load_configs()
        .map_err(|e| e.to_string())?;

    let statuses = configs
        .iter()
        .map(|config| MountStatus {
            name: config.name.clone(),
            mounted: config.is_mounted(),
            mount_point: config.mount_point.clone(),
        })
        .collect();

    Ok(statuses)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config_manager = ConfigManager::new().expect("Failed to initialize config manager");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            config_manager: Mutex::new(config_manager),
        })
        .invoke_handler(tauri::generate_handler![
            get_configs,
            add_config,
            remove_config,
            mount_nfs,
            mount_all,
            umount_nfs,
            umount_all,
            get_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
