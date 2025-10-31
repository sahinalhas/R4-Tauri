use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppSettings {
    pub id: i32,
    pub settings: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSession {
    pub id: String,
    pub userId: String,
    pub token: String,
    pub expiresAt: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub aiProvider: Option<String>,
    pub openaiApiKey: Option<String>,
    pub geminiApiKey: Option<String>,
    pub ollamaEndpoint: Option<String>,
    pub ollamaModel: Option<String>,
    pub theme: Option<String>,
    pub language: Option<String>,
    pub autoBackup: Option<bool>,
    pub backupFrequency: Option<String>,
}
