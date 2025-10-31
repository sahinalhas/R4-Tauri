use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AiSuggestion {
    pub id: String,
    pub studentId: String,
    pub suggestionType: String,
    pub source: String,
    pub sourceId: Option<String>,
    pub priority: String,
    pub title: String,
    pub description: String,
    pub reasoning: Option<String>,
    pub confidence: Option<f64>,
    pub proposedChanges: Option<String>,
    pub currentValues: Option<String>,
    pub aiModel: Option<String>,
    pub aiVersion: Option<String>,
    pub analysisData: Option<String>,
    pub status: String,
    pub reviewedBy: Option<String>,
    pub reviewedAt: Option<String>,
    pub reviewNotes: Option<String>,
    pub feedbackRating: Option<i32>,
    pub feedbackNotes: Option<String>,
    pub expiresAt: String,
    pub appliedAt: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAiSuggestionRequest {
    pub studentId: String,
    pub suggestionType: String,
    pub source: String,
    pub sourceId: Option<String>,
    pub priority: String,
    pub title: String,
    pub description: String,
    pub reasoning: Option<String>,
    pub confidence: Option<f64>,
    pub proposedChanges: Option<String>,
    pub currentValues: Option<String>,
    pub aiModel: Option<String>,
    pub aiVersion: Option<String>,
    pub analysisData: Option<String>,
    pub expiresAt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReviewAiSuggestionRequest {
    pub status: String,
    pub reviewedBy: String,
    pub reviewNotes: Option<String>,
    pub feedbackRating: Option<i32>,
    pub feedbackNotes: Option<String>,
}
