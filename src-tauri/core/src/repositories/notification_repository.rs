use crate::models::{NotificationLog, NotificationPreference, ScheduledTask};
use crate::error::{AppError, AppResult};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct NotificationRepository;

impl NotificationRepository {
    pub async fn create_log(pool: &SqlitePool, log: NotificationLog) -> AppResult<NotificationLog> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let result = sqlx::query_as::<_, NotificationLog>(
            r#"
            INSERT INTO notification_logs (
                id, recipientType, recipientId, recipientName, recipientContact,
                notificationType, channel, subject, message, studentId, alertId,
                interventionId, status, priority, metadata, templateId, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(&log.recipientType)
        .bind(&log.recipientId)
        .bind(&log.recipientName)
        .bind(&log.recipientContact)
        .bind(&log.notificationType)
        .bind(&log.channel)
        .bind(&log.subject)
        .bind(&log.message)
        .bind(&log.studentId)
        .bind(&log.alertId)
        .bind(&log.interventionId)
        .bind(&log.status)
        .bind(&log.priority)
        .bind(&log.metadata)
        .bind(&log.templateId)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_log_by_id(pool: &SqlitePool, id: &str) -> AppResult<NotificationLog> {
        let log = sqlx::query_as::<_, NotificationLog>(
            "SELECT * FROM notification_logs WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Notification log {}", id)))?;

        Ok(log)
    }

    pub async fn get_logs_by_recipient(pool: &SqlitePool, recipient_id: &str) -> AppResult<Vec<NotificationLog>> {
        let logs = sqlx::query_as::<_, NotificationLog>(
            "SELECT * FROM notification_logs WHERE recipientId = ? ORDER BY created_at DESC LIMIT 100"
        )
        .bind(recipient_id)
        .fetch_all(pool)
        .await?;

        Ok(logs)
    }

    pub async fn get_logs_by_student(pool: &SqlitePool, student_id: &str) -> AppResult<Vec<NotificationLog>> {
        let logs = sqlx::query_as::<_, NotificationLog>(
            "SELECT * FROM notification_logs WHERE studentId = ? ORDER BY created_at DESC"
        )
        .bind(student_id)
        .fetch_all(pool)
        .await?;

        Ok(logs)
    }

    pub async fn get_pending_notifications(pool: &SqlitePool) -> AppResult<Vec<NotificationLog>> {
        let logs = sqlx::query_as::<_, NotificationLog>(
            "SELECT * FROM notification_logs WHERE status = 'PENDING' ORDER BY priority DESC, created_at ASC LIMIT 50"
        )
        .fetch_all(pool)
        .await?;

        Ok(logs)
    }

    pub async fn update_status(pool: &SqlitePool, id: &str, status: &str, failure_reason: Option<String>) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        let sent_at = if status == "SENT" || status == "DELIVERED" { Some(now.clone()) } else { None };
        let delivered_at = if status == "DELIVERED" { Some(now.clone()) } else { None };

        sqlx::query(
            r#"
            UPDATE notification_logs SET
                status = ?,
                sentAt = COALESCE(sentAt, ?),
                deliveredAt = COALESCE(deliveredAt, ?),
                failureReason = ?,
                updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(status)
        .bind(&sent_at)
        .bind(&delivered_at)
        .bind(&failure_reason)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn mark_as_read(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE notification_logs SET status = 'READ', readAt = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&now)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_preferences_by_user(pool: &SqlitePool, user_id: &str, user_type: &str) -> AppResult<NotificationPreference> {
        let pref = sqlx::query_as::<_, NotificationPreference>(
            "SELECT * FROM notification_preferences WHERE userId = ? AND userType = ?"
        )
        .bind(user_id)
        .bind(user_type)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Notification preferences for user {}", user_id)))?;

        Ok(pref)
    }

    pub async fn upsert_preferences(pool: &SqlitePool, pref: NotificationPreference) -> AppResult<NotificationPreference> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO notification_preferences (
                id, userId, parentId, studentId, userType, emailEnabled, smsEnabled,
                pushEnabled, inAppEnabled, emailAddress, phoneNumber, alertTypes,
                riskLevels, quietHoursStart, quietHoursEnd, weeklyDigest, monthlyReport,
                language, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(userId, userType) DO UPDATE SET
                emailEnabled = excluded.emailEnabled,
                smsEnabled = excluded.smsEnabled,
                pushEnabled = excluded.pushEnabled,
                inAppEnabled = excluded.inAppEnabled,
                emailAddress = excluded.emailAddress,
                phoneNumber = excluded.phoneNumber,
                alertTypes = excluded.alertTypes,
                riskLevels = excluded.riskLevels,
                quietHoursStart = excluded.quietHoursStart,
                quietHoursEnd = excluded.quietHoursEnd,
                weeklyDigest = excluded.weeklyDigest,
                monthlyReport = excluded.monthlyReport,
                language = excluded.language,
                updated_at = excluded.updated_at
            "#
        )
        .bind(&id)
        .bind(&pref.userId)
        .bind(&pref.parentId)
        .bind(&pref.studentId)
        .bind(&pref.userType)
        .bind(&pref.emailEnabled)
        .bind(&pref.smsEnabled)
        .bind(&pref.pushEnabled)
        .bind(&pref.inAppEnabled)
        .bind(&pref.emailAddress)
        .bind(&pref.phoneNumber)
        .bind(&pref.alertTypes)
        .bind(&pref.riskLevels)
        .bind(&pref.quietHoursStart)
        .bind(&pref.quietHoursEnd)
        .bind(&pref.weeklyDigest)
        .bind(&pref.monthlyReport)
        .bind(&pref.language)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        Self::get_preferences_by_user(pool, &pref.userId.unwrap_or_default(), &pref.userType).await
    }

    pub async fn create_scheduled_task(pool: &SqlitePool, task: ScheduledTask) -> AppResult<ScheduledTask> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let result = sqlx::query_as::<_, ScheduledTask>(
            r#"
            INSERT INTO scheduled_tasks (
                id, taskType, targetType, targetId, scheduleType, scheduledTime,
                nextRun, status, taskData, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(&task.taskType)
        .bind(&task.targetType)
        .bind(&task.targetId)
        .bind(&task.scheduleType)
        .bind(&task.scheduledTime)
        .bind(&task.nextRun)
        .bind(&task.status)
        .bind(&task.taskData)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_due_tasks(pool: &SqlitePool) -> AppResult<Vec<ScheduledTask>> {
        let tasks = sqlx::query_as::<_, ScheduledTask>(
            r#"
            SELECT * FROM scheduled_tasks 
            WHERE status = 'ACTIVE' AND nextRun <= datetime('now')
            ORDER BY nextRun ASC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    pub async fn update_task_next_run(pool: &SqlitePool, id: &str, next_run: &str) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE scheduled_tasks SET lastRun = ?, nextRun = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&now)
        .bind(next_run)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_log(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM notification_logs WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Notification log {}", id)));
        }

        Ok(())
    }
}
