use crate::models::{AiSuggestion, CreateAiSuggestionRequest, ReviewAiSuggestionRequest};
use crate::error::{AppError, AppResult};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct AiSuggestionRepository;

impl AiSuggestionRepository {
    pub async fn create(pool: &SqlitePool, req: CreateAiSuggestionRequest) -> AppResult<AiSuggestion> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let status = "PENDING".to_string();

        let suggestion = sqlx::query_as::<_, AiSuggestion>(
            r#"
            INSERT INTO ai_suggestion_queue (
                id, studentId, suggestionType, source, sourceId, priority, title,
                description, reasoning, confidence, proposedChanges, currentValues,
                aiModel, aiVersion, analysisData, status, expiresAt, createdAt, updatedAt
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(&req.studentId)
        .bind(&req.suggestionType)
        .bind(&req.source)
        .bind(&req.sourceId)
        .bind(&req.priority)
        .bind(&req.title)
        .bind(&req.description)
        .bind(&req.reasoning)
        .bind(&req.confidence)
        .bind(&req.proposedChanges)
        .bind(&req.currentValues)
        .bind(&req.aiModel)
        .bind(&req.aiVersion)
        .bind(&req.analysisData)
        .bind(&status)
        .bind(&req.expiresAt)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(suggestion)
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> AppResult<AiSuggestion> {
        let suggestion = sqlx::query_as::<_, AiSuggestion>(
            "SELECT * FROM ai_suggestion_queue WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("AI suggestion {}", id)))?;

        Ok(suggestion)
    }

    pub async fn get_by_student(pool: &SqlitePool, student_id: &str) -> AppResult<Vec<AiSuggestion>> {
        let suggestions = sqlx::query_as::<_, AiSuggestion>(
            r#"
            SELECT * FROM ai_suggestion_queue 
            WHERE studentId = ? AND status = 'PENDING' AND expiresAt > datetime('now')
            ORDER BY priority DESC, createdAt DESC
            "#
        )
        .bind(student_id)
        .fetch_all(pool)
        .await?;

        Ok(suggestions)
    }

    pub async fn get_pending(pool: &SqlitePool) -> AppResult<Vec<AiSuggestion>> {
        let suggestions = sqlx::query_as::<_, AiSuggestion>(
            r#"
            SELECT * FROM ai_suggestion_queue 
            WHERE status = 'PENDING' AND expiresAt > datetime('now')
            ORDER BY priority DESC, createdAt DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(suggestions)
    }

    pub async fn get_by_priority(pool: &SqlitePool, priority: &str) -> AppResult<Vec<AiSuggestion>> {
        let suggestions = sqlx::query_as::<_, AiSuggestion>(
            r#"
            SELECT * FROM ai_suggestion_queue 
            WHERE priority = ? AND status = 'PENDING' AND expiresAt > datetime('now')
            ORDER BY createdAt DESC
            "#
        )
        .bind(priority)
        .fetch_all(pool)
        .await?;

        Ok(suggestions)
    }

    pub async fn review(pool: &SqlitePool, id: &str, review: ReviewAiSuggestionRequest) -> AppResult<AiSuggestion> {
        let now = Utc::now().to_rfc3339();
        let applied_at = if review.status == "APPROVED" {
            Some(now.clone())
        } else {
            None
        };

        sqlx::query(
            r#"
            UPDATE ai_suggestion_queue SET
                status = ?,
                reviewedBy = ?,
                reviewedAt = ?,
                reviewNotes = ?,
                feedbackRating = ?,
                feedbackNotes = ?,
                appliedAt = ?,
                updatedAt = ?
            WHERE id = ?
            "#
        )
        .bind(&review.status)
        .bind(&review.reviewedBy)
        .bind(&now)
        .bind(&review.reviewNotes)
        .bind(&review.feedbackRating)
        .bind(&review.feedbackNotes)
        .bind(&applied_at)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

        Self::get_by_id(pool, id).await
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM ai_suggestion_queue WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("AI suggestion {}", id)));
        }

        Ok(())
    }

    pub async fn clean_expired(pool: &SqlitePool) -> AppResult<i64> {
        let result = sqlx::query(
            "DELETE FROM ai_suggestion_queue WHERE expiresAt < datetime('now') AND status = 'PENDING'"
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() as i64)
    }

    pub async fn get_statistics(pool: &SqlitePool) -> AppResult<SuggestionStats> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ai_suggestion_queue")
            .fetch_one(pool)
            .await?;

        let pending: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ai_suggestion_queue WHERE status = 'PENDING'"
        )
        .fetch_one(pool)
        .await?;

        let approved: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ai_suggestion_queue WHERE status = 'APPROVED'"
        )
        .fetch_one(pool)
        .await?;

        let rejected: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ai_suggestion_queue WHERE status = 'REJECTED'"
        )
        .fetch_one(pool)
        .await?;

        Ok(SuggestionStats {
            total,
            pending,
            approved,
            rejected,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct SuggestionStats {
    pub total: i64,
    pub pending: i64,
    pub approved: i64,
    pub rejected: i64,
}
