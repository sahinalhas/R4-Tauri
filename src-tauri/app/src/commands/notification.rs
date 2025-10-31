use rehber360_core::{
    models::{NotificationLog, NotificationPreference, ScheduledTask},
    repositories::NotificationRepository,
};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_notification(
    pool: State<'_, SqlitePool>,
    recipient_type: String,
    notification_type: String,
    channel: String,
    message: String,
    student_id: Option<String>,
) -> Result<NotificationLog, String> {
    use chrono::Utc;
    use uuid::Uuid;

    let log = NotificationLog {
        id: Uuid::new_v4().to_string(),
        recipientType: recipient_type,
        recipientId: None,
        recipientName: None,
        recipientContact: None,
        notificationType: notification_type,
        channel,
        subject: None,
        message,
        studentId: student_id,
        alertId: None,
        interventionId: None,
        status: "PENDING".to_string(),
        priority: "NORMAL".to_string(),
        metadata: None,
        templateId: None,
        sentAt: None,
        deliveredAt: None,
        readAt: None,
        failureReason: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    NotificationRepository::create_log(pool.inner(), log)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_notification(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<NotificationLog, String> {
    NotificationRepository::get_log_by_id(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_user_notifications(
    pool: State<'_, SqlitePool>,
    recipient_id: String,
) -> Result<Vec<NotificationLog>, String> {
    NotificationRepository::get_logs_by_recipient(pool.inner(), &recipient_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_notifications(
    pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<Vec<NotificationLog>, String> {
    NotificationRepository::get_logs_by_student(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_pending_notifications(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<NotificationLog>, String> {
    NotificationRepository::get_pending_notifications(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_notification_status(
    pool: State<'_, SqlitePool>,
    id: String,
    status: String,
    failure_reason: Option<String>,
) -> Result<(), String> {
    NotificationRepository::update_status(pool.inner(), &id, &status, failure_reason)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mark_notification_read(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    NotificationRepository::mark_as_read(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_notification_preferences(
    pool: State<'_, SqlitePool>,
    user_id: String,
    user_type: String,
) -> Result<NotificationPreference, String> {
    NotificationRepository::get_preferences_by_user(pool.inner(), &user_id, &user_type)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_notification_preferences(
    pool: State<'_, SqlitePool>,
    preferences: NotificationPreference,
) -> Result<NotificationPreference, String> {
    NotificationRepository::upsert_preferences(pool.inner(), preferences)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_scheduled_task(
    pool: State<'_, SqlitePool>,
    task: ScheduledTask,
) -> Result<ScheduledTask, String> {
    NotificationRepository::create_scheduled_task(pool.inner(), task)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_due_tasks(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<ScheduledTask>, String> {
    NotificationRepository::get_due_tasks(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_task_next_run(
    pool: State<'_, SqlitePool>,
    id: String,
    next_run: String,
) -> Result<(), String> {
    NotificationRepository::update_task_next_run(pool.inner(), &id, &next_run)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_notification(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    NotificationRepository::delete_log(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_native_notification(
    _app: tauri::AppHandle,
    title: String,
    body: String,
) -> Result<(), String> {
    // Native notification using Tauri's notification API
    // This would be implemented when running in actual Tauri environment
    Ok(())
}
