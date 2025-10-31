use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Student not found: {0}")]
    StudentNotFound(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Authorization error: insufficient permissions")]
    Unauthorized,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("AI service error: {0}")]
    AiServiceError(String),

    #[error("File operation error: {0}")]
    FileError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("BCrypt error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("UUID error: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
