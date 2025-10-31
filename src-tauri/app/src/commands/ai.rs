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
    pool: State<'_, SqlitePool>,
    student_id: String,
    ai_config: rehber360_core::services::config_service::AiProviderConfig,
) -> Result<rehber360_core::services::ai_service::AiAnalysisResponse, String> {
    use rehber360_core::{repositories::StudentRepository, services::ai_service::AiService};

    // Get student data
    let student = StudentRepository::get_by_id(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())?;

    // Create student data summary
    let student_summary = serde_json::to_string_pretty(&student)
        .map_err(|e| e.to_string())?;

    // Call AI service
    let ai_service = AiService::new(ai_config);
    let analysis = ai_service
        .analyze_student_profile(&student_summary)
        .await
        .map_err(|e| e.to_string())?;

    Ok(analysis)
}

#[tauri::command]
pub async fn generate_counseling_recommendations(
    pool: State<'_, SqlitePool>,
    student_id: String,
    ai_config: rehber360_core::services::config_service::AiProviderConfig,
) -> Result<Vec<String>, String> {
    use rehber360_core::{repositories::StudentRepository, services::ai_service::AiService};

    // Get student data
    let student = StudentRepository::get_by_id(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())?;

    // Create student data summary
    let student_summary = serde_json::to_string_pretty(&student)
        .map_err(|e| e.to_string())?;

    // Call AI service
    let ai_service = AiService::new(ai_config);
    let recommendations = ai_service
        .generate_recommendations(&student_summary)
        .await
        .map_err(|e| e.to_string())?;

    Ok(recommendations)
}

#[tauri::command]
pub async fn chat_with_ai(
    messages: Vec<rehber360_core::services::ai_service::ChatMessage>,
    ai_config: rehber360_core::services::config_service::AiProviderConfig,
) -> Result<String, String> {
    use rehber360_core::services::ai_service::AiService;

    let ai_service = AiService::new(ai_config);
    let response = ai_service
        .chat(messages)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

#[tauri::command]
pub async fn test_ai_connection(
    ai_config: rehber360_core::services::config_service::AiProviderConfig,
) -> Result<bool, String> {
    rehber360_core::services::ai_service::test_ai_connection(ai_config)
        .await
        .map_err(|e| e.to_string())
}
