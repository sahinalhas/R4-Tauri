use crate::models::{User, UserSession};
use crate::error::{AppError, AppResult};
use sqlx::SqlitePool;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use chrono::{Utc, Duration};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        pool: &SqlitePool,
        name: &str,
        email: &str,
        password: &str,
        role: &str,
        institution: &str,
    ) -> AppResult<User> {
        let id = Uuid::new_v4().to_string();
        let password_hash = hash(password, DEFAULT_COST)?;
        let now = Utc::now().to_rfc3339();
        let is_active = true;

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, name, email, passwordHash, role, institution, isActive, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(name)
        .bind(email)
        .bind(&password_hash)
        .bind(role)
        .bind(institution)
        .bind(is_active)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::UserNotFound(id.to_string()))?;

        Ok(user)
    }

    pub async fn get_by_email(pool: &SqlitePool, email: &str) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::AuthError("User not found".to_string()))?;

        Ok(user)
    }

    pub async fn verify_password(stored_hash: &str, password: &str) -> AppResult<bool> {
        Ok(verify(password, stored_hash)?)
    }

    pub async fn create_session(
        pool: &SqlitePool,
        user_id: &str,
    ) -> AppResult<UserSession> {
        let session_id = Uuid::new_v4().to_string();
        let token = Uuid::new_v4().to_string();
        let expires_at = (Utc::now() + Duration::days(7)).to_rfc3339();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO user_sessions (id, userId, token, expiresAt, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(&session_id)
        .bind(user_id)
        .bind(&token)
        .bind(&expires_at)
        .bind(&now)
        .execute(pool)
        .await?;

        let user = Self::get_by_id(pool, user_id).await?;

        Ok(UserSession {
            user,
            token,
        })
    }

    pub async fn get_session_by_token(pool: &SqlitePool, token: &str) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT u.* FROM users u
            INNER JOIN user_sessions s ON u.id = s.userId
            WHERE s.token = ? AND s.expiresAt > datetime('now')
            "#
        )
        .bind(token)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::AuthError("Invalid or expired session".to_string()))?;

        Ok(user)
    }

    pub async fn delete_session(pool: &SqlitePool, token: &str) -> AppResult<()> {
        sqlx::query("DELETE FROM user_sessions WHERE token = ?")
            .bind(token)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update_password(pool: &SqlitePool, user_id: &str, new_password: &str) -> AppResult<()> {
        let password_hash = hash(new_password, DEFAULT_COST)?;
        let now = Utc::now().to_rfc3339();

        sqlx::query("UPDATE users SET passwordHash = ?, updated_at = ? WHERE id = ?")
            .bind(&password_hash)
            .bind(&now)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_all(pool: &SqlitePool) -> AppResult<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE isActive = 1 ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    pub async fn deactivate(pool: &SqlitePool, user_id: &str) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query("UPDATE users SET isActive = 0, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
