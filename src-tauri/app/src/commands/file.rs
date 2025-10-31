use tauri::AppHandle;
use std::path::{Path, PathBuf};
use tokio::fs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: String,
    pub name: String,
    pub file_type: String,
    pub size: u64,
    pub student_id: Option<String>,
}

/// Validate filename to prevent path traversal attacks
fn validate_and_sanitize_filename(filename: &str) -> Result<String, String> {
    // Reject absolute paths
    if filename.starts_with('/') || filename.starts_with('\\') {
        return Err("Absolute paths are not allowed".to_string());
    }
    
    // Reject path traversal attempts
    if filename.contains("..") || filename.contains("./") || filename.contains(".\\") {
        return Err("Path traversal is not allowed".to_string());
    }
    
    // Reject Windows drive letters
    if filename.len() >= 2 && filename.chars().nth(1) == Some(':') {
        return Err("Drive letters are not allowed".to_string());
    }
    
    // Only allow alphanumeric, dash, underscore, and dot
    if !filename.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
        return Err("Invalid characters in filename".to_string());
    }
    
    Ok(filename.to_string())
}

/// Generate a safe unique filename
fn generate_safe_filename(extension: Option<&str>) -> String {
    let uuid = Uuid::new_v4();
    match extension {
        Some(ext) => format!("{}.{}", uuid, ext),
        None => uuid.to_string(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub created_at: String,
}

#[tauri::command]
pub async fn upload_file(
    app: AppHandle,
    file_data: Vec<u8>,
    metadata: FileMetadata,
) -> Result<String, String> {
    // Validate and sanitize filename
    let safe_filename = validate_and_sanitize_filename(&metadata.name)?;
    
    // Generate unique safe filename (ignore user-provided ID for security)
    let extension = safe_filename.split('.').last();
    let unique_filename = generate_safe_filename(extension);
    
    let uploads_dir = get_uploads_dir(&app)?;
    fs::create_dir_all(&uploads_dir)
        .await
        .map_err(|e| e.to_string())?;

    // Double-check path is within uploads directory
    let file_path = uploads_dir.join(&unique_filename);
    if !file_path.starts_with(&uploads_dir) {
        return Err("Invalid file path: path traversal detected".to_string());
    }

    fs::write(&file_path, file_data)
        .await
        .map_err(|e| e.to_string())?;

    Ok(unique_filename)
}

#[tauri::command]
pub async fn download_file(
    app: AppHandle,
    file_id: String,
) -> Result<Vec<u8>, String> {
    // Validate filename
    validate_and_sanitize_filename(&file_id)?;
    
    let uploads_dir = get_uploads_dir(&app)?;
    let file_path = uploads_dir.join(&file_id);

    // Security check: ensure path is within uploads directory
    if !file_path.starts_with(&uploads_dir) {
        return Err("Invalid file path: path traversal detected".to_string());
    }

    if !file_path.exists() {
        return Err(format!("File not found: {}", file_id));
    }

    fs::read(&file_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_file(
    app: AppHandle,
    file_id: String,
) -> Result<(), String> {
    // Validate filename
    validate_and_sanitize_filename(&file_id)?;
    
    let uploads_dir = get_uploads_dir(&app)?;
    let file_path = uploads_dir.join(&file_id);

    // Security check: ensure path is within uploads directory
    if !file_path.starts_with(&uploads_dir) {
        return Err("Invalid file path: path traversal detected".to_string());
    }

    if !file_path.exists() {
        return Err(format!("File not found: {}", file_id));
    }

    fs::remove_file(&file_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_file_list(
    app: AppHandle,
    student_id: Option<String>,
) -> Result<Vec<FileInfo>, String> {
    let uploads_dir = get_uploads_dir(&app)?;
    
    if !uploads_dir.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    let mut entries = fs::read_dir(&uploads_dir)
        .await
        .map_err(|e| e.to_string())?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
        let metadata = entry.metadata().await.map_err(|e| e.to_string())?;
        
        if metadata.is_file() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_path = entry.path().to_string_lossy().to_string();
            
            files.push(FileInfo {
                id: file_name.clone(),
                name: file_name,
                path: file_path,
                size: metadata.len(),
                created_at: chrono::Utc::now().to_rfc3339(),
            });
        }
    }

    Ok(files)
}

#[tauri::command]
pub async fn open_file_in_explorer(
    app: AppHandle,
    file_id: String,
) -> Result<(), String> {
    // Validate filename
    validate_and_sanitize_filename(&file_id)?;
    
    let uploads_dir = get_uploads_dir(&app)?;
    let file_path = uploads_dir.join(&file_id);

    // Security check: ensure path is within uploads directory
    if !file_path.starts_with(&uploads_dir) {
        return Err("Invalid file path: path traversal detected".to_string());
    }

    if !file_path.exists() {
        return Err(format!("File not found: {}", file_id));
    }
    
    let path_str = file_path.to_string_lossy().to_string();

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path_str])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path_str])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path_str)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

// Helper function to get uploads directory
fn get_uploads_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    Ok(app_data_dir.join("uploads"))
}
