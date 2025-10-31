use rehber360_core::models::{User, UserSession, LoginRequest};
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn login(
    pool: State<'_, SqlitePool>,
    credentials: LoginRequest,
) -> Result<UserSession, String> {
    // Find user by email
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&credentials.email)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Invalid credentials".to_string())?;
    
    // Verify password
    let is_valid = bcrypt::verify(&credentials.password, &user.passwordHash)
        .map_err(|e| format!("Password verification error: {}", e))?;
    
    if !is_valid {
        return Err("Invalid credentials".to_string());
    }
    
    if !user.isActive {
        return Err("User account is inactive".to_string());
    }
    
    // Generate session token
    let token = Uuid::new_v4().to_string();
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
    
    // Save session
    sqlx::query("INSERT INTO user_sessions (id, userId, token, expiresAt) VALUES (?, ?, ?, ?)")
        .bind(Uuid::new_v4().to_string())
        .bind(&user.id)
        .bind(&token)
        .bind(expires_at.to_rfc3339())
        .execute(pool.inner())
        .await
        .map_err(|e| format!("Failed to create session: {}", e))?;
    
    Ok(UserSession { user, token })
}

#[tauri::command]
pub async fn logout(
    pool: State<'_, SqlitePool>,
    token: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM user_sessions WHERE token = ?")
        .bind(&token)
        .execute(pool.inner())
        .await
        .map_err(|e| format!("Failed to logout: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_current_user(
    pool: State<'_, SqlitePool>,
    token: String,
) -> Result<User, String> {
    let session = sqlx::query_as::<_, (String, String)>(
        "SELECT userId, expiresAt FROM user_sessions WHERE token = ?"
    )
        .bind(&token)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Invalid session".to_string())?;
    
    // Check if session is expired
    let expires_at = chrono::DateTime::parse_from_rfc3339(&session.1)
        .map_err(|e| format!("Invalid date format: {}", e))?;
    
    if expires_at < chrono::Utc::now() {
        return Err("Session expired".to_string());
    }
    
    // Get user
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(&session.0)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "User not found".to_string())?;
    
    Ok(user)
}
