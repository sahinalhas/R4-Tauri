use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use crate::error::AppError;

// SECURITY WARNING:
// API keys are currently stored in plaintext JSON.
// For production deployment, use Tauri Secure Storage or OS keychain.
// See SECURITY_NOTES.md for details.

/// AI Provider configuration
/// 
/// WARNING: API keys stored in plaintext for development.
/// Production: Use Tauri Secure Storage or OS keychain (see SECURITY_NOTES.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProviderConfig {
    pub provider: String, // "openai", "gemini", "ollama"
    pub api_key: Option<String>, // TODO: Move to secure storage in production
    pub api_url: Option<String>, // For Ollama
    pub model: String,
}

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub ai_provider: AiProviderConfig,
    pub notification_enabled: bool,
    pub auto_backup: bool,
    pub backup_interval_hours: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            language: "tr".to_string(),
            ai_provider: AiProviderConfig {
                provider: "ollama".to_string(),
                api_key: None,
                api_url: Some("http://localhost:11434".to_string()),
                model: "llama3".to_string(),
            },
            notification_enabled: true,
            auto_backup: false,
            backup_interval_hours: 24,
        }
    }
}

pub struct ConfigService;

impl ConfigService {
    /// Load settings from file
    pub async fn load_settings(config_path: PathBuf) -> Result<AppSettings, AppError> {
        if !config_path.exists() {
            // Return default settings if file doesn't exist
            return Ok(AppSettings::default());
        }

        let content = fs::read_to_string(&config_path).await?;
        let settings: AppSettings = serde_json::from_str(&content)?;
        Ok(settings)
    }

    /// Save settings to file
    pub async fn save_settings(
        config_path: PathBuf,
        settings: &AppSettings,
    ) -> Result<(), AppError> {
        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = serde_json::to_string_pretty(settings)?;
        fs::write(&config_path, content).await?;
        Ok(())
    }

    /// Update AI provider settings
    pub async fn update_ai_provider(
        config_path: PathBuf,
        provider_config: AiProviderConfig,
    ) -> Result<AppSettings, AppError> {
        let mut settings = Self::load_settings(config_path.clone()).await?;
        settings.ai_provider = provider_config;
        Self::save_settings(config_path, &settings).await?;
        Ok(settings)
    }
}
