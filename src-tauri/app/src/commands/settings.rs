use rehber360_core::services::config_service::{AppSettings, AiProviderConfig, ConfigService};
use tauri::State;
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    let config_path = get_config_path(&app)?;
    ConfigService::load_settings(config_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(
    app: AppHandle,
    settings: AppSettings,
) -> Result<(), String> {
    let config_path = get_config_path(&app)?;
    ConfigService::save_settings(config_path, &settings)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_ai_provider(
    app: AppHandle,
    ai_config: AiProviderConfig,
) -> Result<AppSettings, String> {
    let config_path = get_config_path(&app)?;
    ConfigService::update_ai_provider(config_path, ai_config)
        .await
        .map_err(|e| e.to_string())
}

// Helper function to get config file path
fn get_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    Ok(app_data_dir.join("settings.json"))
}
