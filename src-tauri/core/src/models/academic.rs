use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AcademicRecord {
    pub id: i64,
    pub studentId: String,
    pub semester: String,
    pub gpa: Option<f64>,
    pub year: Option<i32>,
    pub exams: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Intervention {
    pub id: String,
    pub studentId: String,
    pub date: String,
    pub title: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub code: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Topic {
    pub id: String,
    pub subjectId: String,
    pub name: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub estimatedHours: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Progress {
    pub id: String,
    pub studentId: String,
    pub topicId: String,
    pub completed: i32,
    pub remaining: i32,
    pub lastStudied: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AcademicGoal {
    pub id: String,
    pub studentId: String,
    pub title: String,
    pub description: Option<String>,
    pub targetScore: Option<f64>,
    pub currentScore: Option<f64>,
    pub examType: Option<String>,
    pub deadline: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StudySession {
    pub id: String,
    pub studentId: String,
    pub topicId: String,
    pub startTime: String,
    pub endTime: Option<String>,
    pub duration: Option<i32>,
    pub notes: Option<String>,
    pub efficiency: Option<f64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Note {
    pub id: String,
    pub studentId: String,
    pub title: String,
    pub content: Option<String>,
    pub category: Option<String>,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StudyAssignment {
    pub id: String,
    pub studentId: String,
    pub topicId: String,
    pub dueDate: String,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExamResult {
    pub id: String,
    pub studentId: String,
    pub examType: String,
    pub examName: String,
    pub examDate: String,
    pub examProvider: Option<String>,
    pub totalScore: Option<f64>,
    pub percentileRank: Option<f64>,
    pub turkishScore: Option<f64>,
    pub mathScore: Option<f64>,
    pub scienceScore: Option<f64>,
    pub socialScore: Option<f64>,
    pub foreignLanguageScore: Option<f64>,
    pub turkishNet: Option<f64>,
    pub mathNet: Option<f64>,
    pub scienceNet: Option<f64>,
    pub socialNet: Option<f64>,
    pub foreignLanguageNet: Option<f64>,
    pub totalNet: Option<f64>,
    pub correctAnswers: Option<i32>,
    pub wrongAnswers: Option<i32>,
    pub emptyAnswers: Option<i32>,
    pub totalQuestions: Option<i32>,
    pub subjectBreakdown: Option<String>,
    pub topicAnalysis: Option<String>,
    pub strengthAreas: Option<String>,
    pub weaknessAreas: Option<String>,
    pub improvementSuggestions: Option<String>,
    pub comparedToGoal: Option<String>,
    pub comparedToPrevious: Option<String>,
    pub comparedToClassAverage: Option<f64>,
    pub schoolRank: Option<i32>,
    pub classRank: Option<i32>,
    pub isOfficial: bool,
    pub certificateUrl: Option<String>,
    pub answerKeyUrl: Option<String>,
    pub detailedReportUrl: Option<String>,
    pub goalsMet: bool,
    pub parentNotified: bool,
    pub counselorNotes: Option<String>,
    pub actionPlan: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BehaviorIncident {
    pub id: String,
    pub studentId: String,
    pub incidentDate: String,
    pub incidentTime: String,
    pub location: String,
    pub behaviorType: String,
    pub behaviorCategory: String,
    pub description: String,
    pub antecedent: Option<String>,
    pub consequence: Option<String>,
    pub duration: Option<i32>,
    pub intensity: Option<String>,
    pub frequency: Option<String>,
    pub witnessedBy: Option<String>,
    pub othersInvolved: Option<String>,
    pub interventionUsed: Option<String>,
    pub interventionEffectiveness: Option<String>,
    pub parentNotified: bool,
    pub parentNotificationMethod: Option<String>,
    pub parentResponse: Option<String>,
    pub followUpRequired: bool,
    pub followUpDate: Option<String>,
    pub followUpNotes: Option<String>,
    pub adminNotified: bool,
    pub consequenceGiven: Option<String>,
    pub supportProvided: Option<String>,
    pub triggerAnalysis: Option<String>,
    pub patternNotes: Option<String>,
    pub positiveAlternative: Option<String>,
    pub status: String,
    pub recordedBy: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
