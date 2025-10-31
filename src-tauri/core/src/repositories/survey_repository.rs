use crate::models::{Survey, SurveyTemplate, SurveyDistribution, SurveyResponse};
use crate::error::{AppError, AppResult};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct SurveyRepository;

impl SurveyRepository {
    pub async fn create_template(pool: &SqlitePool, template: SurveyTemplate) -> AppResult<SurveyTemplate> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let result = sqlx::query_as::<_, SurveyTemplate>(
            r#"
            INSERT INTO survey_templates (
                id, title, description, type, mebCompliant, isActive, createdBy,
                tags, estimatedDuration, targetGrades, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(&template.title)
        .bind(&template.description)
        .bind(&template.survey_type)
        .bind(&template.mebCompliant)
        .bind(&template.isActive)
        .bind(&template.createdBy)
        .bind(&template.tags)
        .bind(&template.estimatedDuration)
        .bind(&template.targetGrades)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_template_by_id(pool: &SqlitePool, id: &str) -> AppResult<SurveyTemplate> {
        let template = sqlx::query_as::<_, SurveyTemplate>(
            "SELECT * FROM survey_templates WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Survey template {}", id)))?;

        Ok(template)
    }

    pub async fn get_all_templates(pool: &SqlitePool) -> AppResult<Vec<SurveyTemplate>> {
        let templates = sqlx::query_as::<_, SurveyTemplate>(
            "SELECT * FROM survey_templates WHERE isActive = 1 ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(templates)
    }

    pub async fn create_distribution(pool: &SqlitePool, distribution: SurveyDistribution) -> AppResult<SurveyDistribution> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let result = sqlx::query_as::<_, SurveyDistribution>(
            r#"
            INSERT INTO survey_distributions (
                id, templateId, title, description, targetClasses, targetStudents,
                distributionType, excelTemplate, publicLink, startDate, endDate,
                allowAnonymous, maxResponses, status, createdBy, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(&distribution.templateId)
        .bind(&distribution.title)
        .bind(&distribution.description)
        .bind(&distribution.targetClasses)
        .bind(&distribution.targetStudents)
        .bind(&distribution.distributionType)
        .bind(&distribution.excelTemplate)
        .bind(&distribution.publicLink)
        .bind(&distribution.startDate)
        .bind(&distribution.endDate)
        .bind(&distribution.allowAnonymous)
        .bind(&distribution.maxResponses)
        .bind(&distribution.status)
        .bind(&distribution.createdBy)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn create_response(pool: &SqlitePool, response: SurveyResponse) -> AppResult<SurveyResponse> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let result = sqlx::query_as::<_, SurveyResponse>(
            r#"
            INSERT INTO survey_responses (
                id, distributionId, studentId, studentInfo, responseData,
                submissionType, isComplete, submittedAt, ipAddress, userAgent,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(&response.distributionId)
        .bind(&response.studentId)
        .bind(&response.studentInfo)
        .bind(&response.responseData)
        .bind(&response.submissionType)
        .bind(&response.isComplete)
        .bind(&response.submittedAt)
        .bind(&response.ipAddress)
        .bind(&response.userAgent)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_responses_by_distribution(pool: &SqlitePool, distribution_id: &str) -> AppResult<Vec<SurveyResponse>> {
        let responses = sqlx::query_as::<_, SurveyResponse>(
            "SELECT * FROM survey_responses WHERE distributionId = ? ORDER BY created_at DESC"
        )
        .bind(distribution_id)
        .fetch_all(pool)
        .await?;

        Ok(responses)
    }

    pub async fn get_responses_by_student(pool: &SqlitePool, student_id: &str) -> AppResult<Vec<SurveyResponse>> {
        let responses = sqlx::query_as::<_, SurveyResponse>(
            "SELECT * FROM survey_responses WHERE studentId = ? ORDER BY created_at DESC"
        )
        .bind(student_id)
        .fetch_all(pool)
        .await?;

        Ok(responses)
    }

    pub async fn create_legacy_survey(pool: &SqlitePool, student_id: &str, survey_type: &str, questions: &str) -> AppResult<Survey> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let survey = sqlx::query_as::<_, Survey>(
            r#"
            INSERT INTO surveys (id, studentId, type, questions, completed, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(student_id)
        .bind(survey_type)
        .bind(questions)
        .bind(false)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(survey)
    }

    pub async fn update_legacy_survey_responses(pool: &SqlitePool, id: &str, responses: &str) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE surveys SET responses = ?, completed = 1, updated_at = ? WHERE id = ?"
        )
        .bind(responses)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_surveys_by_student(pool: &SqlitePool, student_id: &str) -> AppResult<Vec<Survey>> {
        let surveys = sqlx::query_as::<_, Survey>(
            "SELECT * FROM surveys WHERE studentId = ? ORDER BY created_at DESC"
        )
        .bind(student_id)
        .fetch_all(pool)
        .await?;

        Ok(surveys)
    }

    pub async fn delete_template(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM survey_templates WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Survey template {}", id)));
        }

        Ok(())
    }
}
