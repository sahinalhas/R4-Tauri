use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SurveyTemplate {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub survey_type: String,
    pub mebCompliant: bool,
    pub isActive: bool,
    pub createdBy: Option<String>,
    pub tags: Option<String>,
    pub estimatedDuration: Option<i32>,
    pub targetGrades: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SurveyQuestion {
    pub id: String,
    pub templateId: String,
    pub questionText: String,
    pub questionType: String,
    pub required: bool,
    pub orderIndex: i32,
    pub options: Option<String>,
    pub validation: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SurveyDistribution {
    pub id: String,
    pub templateId: String,
    pub title: String,
    pub description: Option<String>,
    pub targetClasses: Option<String>,
    pub targetStudents: Option<String>,
    pub distributionType: String,
    pub excelTemplate: Option<String>,
    pub publicLink: Option<String>,
    pub startDate: Option<String>,
    pub endDate: Option<String>,
    pub allowAnonymous: bool,
    pub maxResponses: Option<i32>,
    pub status: String,
    pub createdBy: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SurveyResponse {
    pub id: String,
    pub distributionId: String,
    pub studentId: Option<String>,
    pub studentInfo: Option<String>,
    pub responseData: String,
    pub submissionType: String,
    pub isComplete: bool,
    pub submittedAt: Option<String>,
    pub ipAddress: Option<String>,
    pub userAgent: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Survey {
    pub id: String,
    pub studentId: String,
    #[serde(rename = "type")]
    pub survey_type: String,
    pub questions: String,
    pub responses: Option<String>,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}
