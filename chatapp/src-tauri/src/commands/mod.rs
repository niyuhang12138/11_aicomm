use std::sync::Arc;

use tauri::{AppHandle, Manager};

use crate::{config::AppConfig, utils, AppState};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn get_app_dir() -> String {
    utils::app_dir().to_string_lossy().to_string()
}

#[tauri::command]
pub fn get_config(handle: AppHandle) -> Arc<AppConfig> {
    handle.state::<AppState>().config.load().clone()
}
