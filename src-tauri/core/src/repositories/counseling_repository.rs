use crate::models::{
    CounselingSession, MeetingNote, ParentMeeting, HomeVisit,
    CounselingFollowUp, CounselingOutcome, CounselingReminder,
};
use crate::error::{AppError, AppResult};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct CounselingRepository;

impl CounselingRepository {
    pub async fn create_session(pool: &SqlitePool, session: CounselingSession) -> AppResult<CounselingSession> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO counseling_sessions (
                id, sessionType, groupName, counselorId, sessionDate, entryTime, entryClassHourId,
                exitTime, exitClassHourId, topic, participantType, relationshipType, otherParticipants,
                parentName, parentRelationship, teacherName, teacherBranch, otherParticipantDescription,
                sessionMode, sessionLocation, disciplineStatus, institutionalCooperation, sessionDetails,
                detailedNotes, sessionFlow, studentParticipationLevel, cooperationLevel, emotionalState,
                physicalState, communicationQuality, sessionTags, achievedOutcomes, followUpNeeded,
                followUpPlan, actionItems, autoCompleted, extensionGranted, completed, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&session.sessionType)
        .bind(&session.groupName)
        .bind(&session.counselorId)
        .bind(&session.sessionDate)
        .bind(&session.entryTime)
        .bind(&session.entryClassHourId)
        .bind(&session.exitTime)
        .bind(&session.exitClassHourId)
        .bind(&session.topic)
        .bind(&session.participantType)
        .bind(&session.relationshipType)
        .bind(&session.otherParticipants)
        .bind(&session.parentName)
        .bind(&session.parentRelationship)
        .bind(&session.teacherName)
        .bind(&session.teacherBranch)
        .bind(&session.otherParticipantDescription)
        .bind(&session.sessionMode)
        .bind(&session.sessionLocation)
        .bind(&session.disciplineStatus)
        .bind(&session.institutionalCooperation)
        .bind(&session.sessionDetails)
        .bind(&session.detailedNotes)
        .bind(&session.sessionFlow)
        .bind(&session.studentParticipationLevel)
        .bind(&session.cooperationLevel)
        .bind(&session.emotionalState)
        .bind(&session.physicalState)
        .bind(&session.communicationQuality)
        .bind(&session.sessionTags)
        .bind(&session.achievedOutcomes)
        .bind(&session.followUpNeeded)
        .bind(&session.followUpPlan)
        .bind(&session.actionItems)
        .bind(&session.autoCompleted)
        .bind(&session.extensionGranted)
        .bind(&session.completed)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        Self::get_session_by_id(pool, &id).await
    }

    pub async fn get_session_by_id(pool: &SqlitePool, id: &str) -> AppResult<CounselingSession> {
        let session = sqlx::query_as::<_, CounselingSession>(
            "SELECT * FROM counseling_sessions WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Counseling session {}", id)))?;

        Ok(session)
    }

    pub async fn get_sessions_by_student(pool: &SqlitePool, student_id: &str) -> AppResult<Vec<CounselingSession>> {
        let sessions = sqlx::query_as::<_, CounselingSession>(
            r#"
            SELECT DISTINCT cs.* FROM counseling_sessions cs
            INNER JOIN counseling_session_students css ON cs.id = css.sessionId
            WHERE css.studentId = ?
            ORDER BY cs.sessionDate DESC, cs.entryTime DESC
            "#
        )
        .bind(student_id)
        .fetch_all(pool)
        .await?;

        Ok(sessions)
    }

    pub async fn get_all_sessions(pool: &SqlitePool) -> AppResult<Vec<CounselingSession>> {
        let sessions = sqlx::query_as::<_, CounselingSession>(
            "SELECT * FROM counseling_sessions ORDER BY sessionDate DESC, entryTime DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(sessions)
    }

    pub async fn add_student_to_session(pool: &SqlitePool, session_id: &str, student_id: &str) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT OR IGNORE INTO counseling_session_students (sessionId, studentId, created_at) VALUES (?, ?, ?)"
        )
        .bind(session_id)
        .bind(student_id)
        .bind(&now)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn create_meeting_note(pool: &SqlitePool, student_id: &str, date: &str, note_type: &str, note: &str, plan: Option<String>) -> AppResult<MeetingNote> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let meeting_note = sqlx::query_as::<_, MeetingNote>(
            r#"
            INSERT INTO meeting_notes (id, studentId, date, type, note, plan, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(student_id)
        .bind(date)
        .bind(note_type)
        .bind(note)
        .bind(&plan)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(meeting_note)
    }

    pub async fn get_meeting_notes_by_student(pool: &SqlitePool, student_id: &str) -> AppResult<Vec<MeetingNote>> {
        let notes = sqlx::query_as::<_, MeetingNote>(
            "SELECT * FROM meeting_notes WHERE studentId = ? ORDER BY date DESC"
        )
        .bind(student_id)
        .fetch_all(pool)
        .await?;

        Ok(notes)
    }

    pub async fn create_follow_up(pool: &SqlitePool, follow_up: CounselingFollowUp) -> AppResult<CounselingFollowUp> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let result = sqlx::query_as::<_, CounselingFollowUp>(
            r#"
            INSERT INTO counseling_follow_ups (
                id, sessionId, followUpDate, assignedTo, priority, status, actionItems, notes, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(&follow_up.sessionId)
        .bind(&follow_up.followUpDate)
        .bind(&follow_up.assignedTo)
        .bind(&follow_up.priority)
        .bind(&follow_up.status)
        .bind(&follow_up.actionItems)
        .bind(&follow_up.notes)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_pending_follow_ups(pool: &SqlitePool) -> AppResult<Vec<CounselingFollowUp>> {
        let follow_ups = sqlx::query_as::<_, CounselingFollowUp>(
            "SELECT * FROM counseling_follow_ups WHERE status = 'pending' ORDER BY followUpDate ASC"
        )
        .fetch_all(pool)
        .await?;

        Ok(follow_ups)
    }

    pub async fn update_session(pool: &SqlitePool, id: &str, session: CounselingSession) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE counseling_sessions SET
                detailedNotes = ?, sessionFlow = ?, studentParticipationLevel = ?,
                cooperationLevel = ?, emotionalState = ?, followUpNeeded = ?,
                followUpPlan = ?, completed = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&session.detailedNotes)
        .bind(&session.sessionFlow)
        .bind(&session.studentParticipationLevel)
        .bind(&session.cooperationLevel)
        .bind(&session.emotionalState)
        .bind(&session.followUpNeeded)
        .bind(&session.followUpPlan)
        .bind(&session.completed)
        .bind(&now)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete_session(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM counseling_sessions WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Counseling session {}", id)));
        }

        Ok(())
    }
}
