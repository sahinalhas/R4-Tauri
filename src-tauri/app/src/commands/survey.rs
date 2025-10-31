use rehber360_core::{
    models::{Survey, SurveyTemplate, SurveyDistribution, SurveyResponse},
    repositories::SurveyRepository,
};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_survey_template(
    pool: State<'_, SqlitePool>,
    template: SurveyTemplate,
) -> Result<SurveyTemplate, String> {
    SurveyRepository::create_template(pool.inner(), template)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_survey_template(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<SurveyTemplate, String> {
    SurveyRepository::get_template_by_id(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_survey_templates(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<SurveyTemplate>, String> {
    SurveyRepository::get_all_templates(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_survey_distribution(
    pool: State<'_, SqlitePool>,
    distribution: SurveyDistribution,
) -> Result<SurveyDistribution, String> {
    SurveyRepository::create_distribution(pool.inner(), distribution)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_survey_response(
    pool: State<'_, SqlitePool>,
    response: SurveyResponse,
) -> Result<SurveyResponse, String> {
    SurveyRepository::create_response(pool.inner(), response)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_distribution_responses(
    pool: State<'_, SqlitePool>,
    distribution_id: String,
) -> Result<Vec<SurveyResponse>, String> {
    SurveyRepository::get_responses_by_distribution(pool.inner(), &distribution_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_survey_responses(
    pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<Vec<SurveyResponse>, String> {
    SurveyRepository::get_responses_by_student(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_legacy_survey(
    pool: State<'_, SqlitePool>,
    student_id: String,
    survey_type: String,
    questions: String,
) -> Result<Survey, String> {
    SurveyRepository::create_legacy_survey(pool.inner(), &student_id, &survey_type, &questions)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_survey_responses(
    pool: State<'_, SqlitePool>,
    id: String,
    responses: String,
) -> Result<(), String> {
    SurveyRepository::update_legacy_survey_responses(pool.inner(), &id, &responses)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_surveys(
    pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<Vec<Survey>, String> {
    SurveyRepository::get_surveys_by_student(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_survey_template(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    SurveyRepository::delete_template(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}
