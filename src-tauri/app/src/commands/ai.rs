use rehber360_core::{
    models::{AiSuggestion, CreateAiSuggestionRequest, ReviewAiSuggestionRequest},
    repositories::{AiSuggestionRepository, SuggestionStats},
};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_ai_suggestion(
    pool: State<'_, SqlitePool>,
    request: CreateAiSuggestionRequest,
) -> Result<AiSuggestion, String> {
    AiSuggestionRepository::create(pool.inner(), request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_suggestion(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<AiSuggestion, String> {
    AiSuggestionRepository::get_by_id(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_ai_suggestions(
    pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<Vec<AiSuggestion>, String> {
    AiSuggestionRepository::get_by_student(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_pending_ai_suggestions(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<AiSuggestion>, String> {
    AiSuggestionRepository::get_pending(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_suggestions_by_priority(
    pool: State<'_, SqlitePool>,
    priority: String,
) -> Result<Vec<AiSuggestion>, String> {
    AiSuggestionRepository::get_by_priority(pool.inner(), &priority)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn review_ai_suggestion(
    pool: State<'_, SqlitePool>,
    id: String,
    review: ReviewAiSuggestionRequest,
) -> Result<AiSuggestion, String> {
    AiSuggestionRepository::review(pool.inner(), &id, review)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_ai_suggestion(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    AiSuggestionRepository::delete(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clean_expired_suggestions(
    pool: State<'_, SqlitePool>,
) -> Result<i64, String> {
    AiSuggestionRepository::clean_expired(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_suggestion_statistics(
    pool: State<'_, SqlitePool>,
) -> Result<SuggestionStats, String> {
    AiSuggestionRepository::get_statistics(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn analyze_student_profile(
    _pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<String, String> {
    // Placeholder for AI analysis - would integrate with OpenAI/Gemini/Ollama
    Ok(format!("AI analysis for student {} would be performed here", student_id))
}

#[tauri::command]
pub async fn generate_counseling_recommendations(
    _pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<Vec<String>, String> {
    // Placeholder for AI recommendations
    Ok(vec![
        format!("Recommendation 1 for student {}", student_id),
        "Recommendation 2".to_string(),
    ])
}
