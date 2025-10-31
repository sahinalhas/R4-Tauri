use rehber360_core::{
    models::{ExamResult, BehaviorIncident, AcademicGoal},
    repositories::AcademicRepository,
};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_exam_result(
    pool: State<'_, SqlitePool>,
    student_id: String,
    exam_type: String,
    exam_name: String,
    exam_date: String,
    total_score: Option<f64>,
) -> Result<ExamResult, String> {
    use chrono::Utc;
    use uuid::Uuid;

    let result = ExamResult {
        id: Uuid::new_v4().to_string(),
        studentId: student_id,
        examType: exam_type,
        examName: exam_name,
        examDate: exam_date,
        examProvider: None,
        totalScore: total_score,
        percentileRank: None,
        turkishScore: None,
        mathScore: None,
        scienceScore: None,
        socialScore: None,
        foreignLanguageScore: None,
        turkishNet: None,
        mathNet: None,
        scienceNet: None,
        socialNet: None,
        foreignLanguageNet: None,
        totalNet: None,
        correctAnswers: None,
        wrongAnswers: None,
        emptyAnswers: None,
        totalQuestions: None,
        subjectBreakdown: None,
        topicAnalysis: None,
        strengthAreas: None,
        weaknessAreas: None,
        improvementSuggestions: None,
        comparedToGoal: None,
        comparedToPrevious: None,
        comparedToClassAverage: None,
        schoolRank: None,
        classRank: None,
        isOfficial: false,
        certificateUrl: None,
        answerKeyUrl: None,
        detailedReportUrl: None,
        goalsMet: false,
        parentNotified: false,
        counselorNotes: None,
        actionPlan: None,
        notes: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    AcademicRepository::create_exam_result(pool.inner(), result)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_exam_result(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ExamResult, String> {
    AcademicRepository::get_exam_result_by_id(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_exam_results(
    pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<Vec<ExamResult>, String> {
    AcademicRepository::get_exam_results_by_student(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_exam_results_by_type(
    pool: State<'_, SqlitePool>,
    student_id: String,
    exam_type: String,
) -> Result<Vec<ExamResult>, String> {
    AcademicRepository::get_exam_results_by_type(pool.inner(), &student_id, &exam_type)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_exam_result(
    pool: State<'_, SqlitePool>,
    id: String,
    result: ExamResult,
) -> Result<(), String> {
    AcademicRepository::update_exam_result(pool.inner(), &id, result)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_exam_result(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    AcademicRepository::delete_exam_result(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_behavior_incident(
    pool: State<'_, SqlitePool>,
    student_id: String,
    incident_date: String,
    incident_time: String,
    location: String,
    behavior_type: String,
    behavior_category: String,
    description: String,
    recorded_by: String,
) -> Result<BehaviorIncident, String> {
    use chrono::Utc;
    use uuid::Uuid;

    let incident = BehaviorIncident {
        id: Uuid::new_v4().to_string(),
        studentId: student_id,
        incidentDate: incident_date,
        incidentTime: incident_time,
        location,
        behaviorType: behavior_type,
        behaviorCategory: behavior_category,
        description,
        antecedent: None,
        consequence: None,
        duration: None,
        intensity: None,
        frequency: None,
        witnessedBy: None,
        othersInvolved: None,
        interventionUsed: None,
        interventionEffectiveness: None,
        parentNotified: false,
        parentNotificationMethod: None,
        parentResponse: None,
        followUpRequired: false,
        followUpDate: None,
        followUpNotes: None,
        adminNotified: false,
        consequenceGiven: None,
        supportProvided: None,
        triggerAnalysis: None,
        patternNotes: None,
        positiveAlternative: None,
        status: "Açık".to_string(),
        recordedBy: recorded_by,
        notes: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    AcademicRepository::create_behavior_incident(pool.inner(), incident)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_behavior_incident(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<BehaviorIncident, String> {
    AcademicRepository::get_behavior_incident_by_id(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_behavior_incidents(
    pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<Vec<BehaviorIncident>, String> {
    AcademicRepository::get_behavior_incidents_by_student(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_academic_goal(
    pool: State<'_, SqlitePool>,
    student_id: String,
    title: String,
    target_score: Option<f64>,
    exam_type: Option<String>,
    deadline: Option<String>,
) -> Result<AcademicGoal, String> {
    use chrono::Utc;
    use uuid::Uuid;

    let goal = AcademicGoal {
        id: Uuid::new_v4().to_string(),
        studentId: student_id,
        title,
        description: None,
        targetScore: target_score,
        currentScore: None,
        examType: exam_type,
        deadline,
        status: "active".to_string(),
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    AcademicRepository::create_academic_goal(pool.inner(), goal)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_academic_goals(
    pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<Vec<AcademicGoal>, String> {
    AcademicRepository::get_academic_goals_by_student(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())
}
