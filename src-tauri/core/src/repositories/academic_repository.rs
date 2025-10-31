use crate::models::{ExamResult, BehaviorIncident, AcademicGoal, Progress};
use crate::error::{AppError, AppResult};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct AcademicRepository;

impl AcademicRepository {
    pub async fn create_exam_result(pool: &SqlitePool, result: ExamResult) -> AppResult<ExamResult> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO exam_results (
                id, studentId, examType, examName, examDate, examProvider, totalScore,
                percentileRank, turkishScore, mathScore, scienceScore, socialScore,
                foreignLanguageScore, turkishNet, mathNet, scienceNet, socialNet,
                foreignLanguageNet, totalNet, correctAnswers, wrongAnswers, emptyAnswers,
                totalQuestions, subjectBreakdown, topicAnalysis, strengthAreas, weaknessAreas,
                improvementSuggestions, comparedToGoal, comparedToPrevious, comparedToClassAverage,
                schoolRank, classRank, isOfficial, certificateUrl, answerKeyUrl,
                detailedReportUrl, goalsMet, parentNotified, counselorNotes, actionPlan,
                notes, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&result.studentId)
        .bind(&result.examType)
        .bind(&result.examName)
        .bind(&result.examDate)
        .bind(&result.examProvider)
        .bind(&result.totalScore)
        .bind(&result.percentileRank)
        .bind(&result.turkishScore)
        .bind(&result.mathScore)
        .bind(&result.scienceScore)
        .bind(&result.socialScore)
        .bind(&result.foreignLanguageScore)
        .bind(&result.turkishNet)
        .bind(&result.mathNet)
        .bind(&result.scienceNet)
        .bind(&result.socialNet)
        .bind(&result.foreignLanguageNet)
        .bind(&result.totalNet)
        .bind(&result.correctAnswers)
        .bind(&result.wrongAnswers)
        .bind(&result.emptyAnswers)
        .bind(&result.totalQuestions)
        .bind(&result.subjectBreakdown)
        .bind(&result.topicAnalysis)
        .bind(&result.strengthAreas)
        .bind(&result.weaknessAreas)
        .bind(&result.improvementSuggestions)
        .bind(&result.comparedToGoal)
        .bind(&result.comparedToPrevious)
        .bind(&result.comparedToClassAverage)
        .bind(&result.schoolRank)
        .bind(&result.classRank)
        .bind(&result.isOfficial)
        .bind(&result.certificateUrl)
        .bind(&result.answerKeyUrl)
        .bind(&result.detailedReportUrl)
        .bind(&result.goalsMet)
        .bind(&result.parentNotified)
        .bind(&result.counselorNotes)
        .bind(&result.actionPlan)
        .bind(&result.notes)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        Self::get_exam_result_by_id(pool, &id).await
    }

    pub async fn get_exam_result_by_id(pool: &SqlitePool, id: &str) -> AppResult<ExamResult> {
        let result = sqlx::query_as::<_, ExamResult>("SELECT * FROM exam_results WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Exam result {}", id)))?;

        Ok(result)
    }

    pub async fn get_exam_results_by_student(pool: &SqlitePool, student_id: &str) -> AppResult<Vec<ExamResult>> {
        let results = sqlx::query_as::<_, ExamResult>(
            "SELECT * FROM exam_results WHERE studentId = ? ORDER BY examDate DESC"
        )
        .bind(student_id)
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn get_exam_results_by_type(pool: &SqlitePool, student_id: &str, exam_type: &str) -> AppResult<Vec<ExamResult>> {
        let results = sqlx::query_as::<_, ExamResult>(
            "SELECT * FROM exam_results WHERE studentId = ? AND examType = ? ORDER BY examDate DESC"
        )
        .bind(student_id)
        .bind(exam_type)
        .fetch_all(pool)
        .await?;

        Ok(results)
    }

    pub async fn create_behavior_incident(pool: &SqlitePool, incident: BehaviorIncident) -> AppResult<BehaviorIncident> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO behavior_incidents (
                id, studentId, incidentDate, incidentTime, location, behaviorType,
                behaviorCategory, description, antecedent, consequence, duration,
                intensity, frequency, witnessedBy, othersInvolved, interventionUsed,
                interventionEffectiveness, parentNotified, parentNotificationMethod,
                parentResponse, followUpRequired, followUpDate, followUpNotes,
                adminNotified, consequenceGiven, supportProvided, triggerAnalysis,
                patternNotes, positiveAlternative, status, recordedBy, notes,
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&incident.studentId)
        .bind(&incident.incidentDate)
        .bind(&incident.incidentTime)
        .bind(&incident.location)
        .bind(&incident.behaviorType)
        .bind(&incident.behaviorCategory)
        .bind(&incident.description)
        .bind(&incident.antecedent)
        .bind(&incident.consequence)
        .bind(&incident.duration)
        .bind(&incident.intensity)
        .bind(&incident.frequency)
        .bind(&incident.witnessedBy)
        .bind(&incident.othersInvolved)
        .bind(&incident.interventionUsed)
        .bind(&incident.interventionEffectiveness)
        .bind(&incident.parentNotified)
        .bind(&incident.parentNotificationMethod)
        .bind(&incident.parentResponse)
        .bind(&incident.followUpRequired)
        .bind(&incident.followUpDate)
        .bind(&incident.followUpNotes)
        .bind(&incident.adminNotified)
        .bind(&incident.consequenceGiven)
        .bind(&incident.supportProvided)
        .bind(&incident.triggerAnalysis)
        .bind(&incident.patternNotes)
        .bind(&incident.positiveAlternative)
        .bind(&incident.status)
        .bind(&incident.recordedBy)
        .bind(&incident.notes)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        Self::get_behavior_incident_by_id(pool, &id).await
    }

    pub async fn get_behavior_incident_by_id(pool: &SqlitePool, id: &str) -> AppResult<BehaviorIncident> {
        let incident = sqlx::query_as::<_, BehaviorIncident>(
            "SELECT * FROM behavior_incidents WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Behavior incident {}", id)))?;

        Ok(incident)
    }

    pub async fn get_behavior_incidents_by_student(pool: &SqlitePool, student_id: &str) -> AppResult<Vec<BehaviorIncident>> {
        let incidents = sqlx::query_as::<_, BehaviorIncident>(
            "SELECT * FROM behavior_incidents WHERE studentId = ? ORDER BY incidentDate DESC, incidentTime DESC"
        )
        .bind(student_id)
        .fetch_all(pool)
        .await?;

        Ok(incidents)
    }

    pub async fn create_academic_goal(pool: &SqlitePool, goal: AcademicGoal) -> AppResult<AcademicGoal> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let result = sqlx::query_as::<_, AcademicGoal>(
            r#"
            INSERT INTO academic_goals (
                id, studentId, title, description, targetScore, currentScore,
                examType, deadline, status, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(&goal.studentId)
        .bind(&goal.title)
        .bind(&goal.description)
        .bind(&goal.targetScore)
        .bind(&goal.currentScore)
        .bind(&goal.examType)
        .bind(&goal.deadline)
        .bind(&goal.status)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_academic_goals_by_student(pool: &SqlitePool, student_id: &str) -> AppResult<Vec<AcademicGoal>> {
        let goals = sqlx::query_as::<_, AcademicGoal>(
            "SELECT * FROM academic_goals WHERE studentId = ? AND status = 'active' ORDER BY deadline ASC"
        )
        .bind(student_id)
        .fetch_all(pool)
        .await?;

        Ok(goals)
    }

    pub async fn update_exam_result(pool: &SqlitePool, id: &str, result: ExamResult) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE exam_results SET
                counselorNotes = ?, actionPlan = ?, parentNotified = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&result.counselorNotes)
        .bind(&result.actionPlan)
        .bind(&result.parentNotified)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_exam_result(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM exam_results WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Exam result {}", id)));
        }

        Ok(())
    }
}
