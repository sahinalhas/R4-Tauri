use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationLog {
    pub id: String,
    pub recipientType: String,
    pub recipientId: Option<String>,
    pub recipientName: Option<String>,
    pub recipientContact: Option<String>,
    pub notificationType: String,
    pub channel: String,
    pub subject: Option<String>,
    pub message: String,
    pub studentId: Option<String>,
    pub alertId: Option<String>,
    pub interventionId: Option<String>,
    pub status: String,
    pub priority: String,
    pub metadata: Option<String>,
    pub templateId: Option<String>,
    pub sentAt: Option<String>,
    pub deliveredAt: Option<String>,
    pub readAt: Option<String>,
    pub failureReason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationPreference {
    pub id: String,
    pub userId: Option<String>,
    pub parentId: Option<String>,
    pub studentId: Option<String>,
    pub userType: String,
    pub emailEnabled: bool,
    pub smsEnabled: bool,
    pub pushEnabled: bool,
    pub inAppEnabled: bool,
    pub emailAddress: Option<String>,
    pub phoneNumber: Option<String>,
    pub alertTypes: Option<String>,
    pub riskLevels: Option<String>,
    pub quietHoursStart: Option<String>,
    pub quietHoursEnd: Option<String>,
    pub weeklyDigest: bool,
    pub monthlyReport: bool,
    pub language: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationTemplate {
    pub id: String,
    pub templateName: String,
    pub category: String,
    pub channel: String,
    pub subject: Option<String>,
    pub bodyTemplate: String,
    pub variables: Option<String>,
    pub isActive: bool,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ParentAccessToken {
    pub id: String,
    pub studentId: String,
    pub parentName: String,
    pub parentContact: String,
    pub accessToken: String,
    pub accessLevel: String,
    pub expiresAt: Option<String>,
    pub isActive: bool,
    pub createdBy: Option<String>,
    pub lastAccessedAt: Option<String>,
    pub accessCount: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ScheduledTask {
    pub id: String,
    pub taskType: String,
    pub targetType: String,
    pub targetId: Option<String>,
    pub scheduleType: String,
    pub scheduledTime: String,
    pub nextRun: String,
    pub lastRun: Option<String>,
    pub status: String,
    pub taskData: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
