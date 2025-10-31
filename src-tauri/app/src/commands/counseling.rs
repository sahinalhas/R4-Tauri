use rehber360_core::{
    models::{CounselingSession, MeetingNote, CounselingFollowUp},
    repositories::CounselingRepository,
};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn get_all_counseling_sessions(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<CounselingSession>, String> {
    CounselingRepository::get_all_sessions(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_counseling_session(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<CounselingSession, String> {
    CounselingRepository::get_session_by_id(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_counseling_sessions(
    pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<Vec<CounselingSession>, String> {
    CounselingRepository::get_sessions_by_student(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_counseling_session(
    pool: State<'_, SqlitePool>,
    session_type: String,
    counselor_id: String,
    session_date: String,
    entry_time: String,
    topic: String,
    participant_type: String,
    session_mode: String,
    session_location: String,
) -> Result<CounselingSession, String> {
    use chrono::Utc;
    use uuid::Uuid;

    let session = rehber360_core::models::CounselingSession {
        id: Uuid::new_v4().to_string(),
        sessionType: session_type,
        groupName: None,
        counselorId: counselor_id,
        sessionDate: session_date,
        entryTime: entry_time,
        entryClassHourId: None,
        exitTime: None,
        exitClassHourId: None,
        topic,
        participantType: participant_type,
        relationshipType: None,
        otherParticipants: None,
        parentName: None,
        parentRelationship: None,
        teacherName: None,
        teacherBranch: None,
        otherParticipantDescription: None,
        sessionMode: session_mode,
        sessionLocation: session_location,
        disciplineStatus: None,
        institutionalCooperation: None,
        sessionDetails: None,
        detailedNotes: None,
        sessionFlow: None,
        studentParticipationLevel: None,
        cooperationLevel: None,
        emotionalState: None,
        physicalState: None,
        communicationQuality: None,
        sessionTags: None,
        achievedOutcomes: None,
        followUpNeeded: false,
        followUpPlan: None,
        actionItems: None,
        autoCompleted: false,
        extensionGranted: false,
        completed: false,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    CounselingRepository::create_session(pool.inner(), session)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_counseling_session(
    pool: State<'_, SqlitePool>,
    id: String,
    session_type: Option<String>,
    group_name: Option<String>,
    topic: Option<String>,
    detailed_notes: Option<String>,
    follow_up_needed: Option<bool>,
    completed: Option<bool>,
) -> Result<(), String> {
    use chrono::Utc;

    // Fetch existing session
    let mut existing = CounselingRepository::get_session_by_id(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())?;

    // Apply updates
    if let Some(val) = session_type {
        existing.sessionType = val;
    }
    if let Some(val) = group_name {
        existing.groupName = Some(val);
    }
    if let Some(val) = topic {
        existing.topic = val;
    }
    if let Some(val) = detailed_notes {
        existing.detailedNotes = Some(val);
    }
    if let Some(val) = follow_up_needed {
        existing.followUpNeeded = val;
    }
    if let Some(val) = completed {
        existing.completed = val;
    }

    existing.updated_at = Utc::now().to_rfc3339();

    CounselingRepository::update_session(pool.inner(), &id, existing)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_counseling_session(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    CounselingRepository::delete_session(pool.inner(), &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_student_to_session(
    pool: State<'_, SqlitePool>,
    session_id: String,
    student_id: String,
) -> Result<(), String> {
    CounselingRepository::add_student_to_session(pool.inner(), &session_id, &student_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_meeting_note(
    pool: State<'_, SqlitePool>,
    student_id: String,
    date: String,
    note_type: String,
    note: String,
    plan: Option<String>,
) -> Result<MeetingNote, String> {
    CounselingRepository::create_meeting_note(pool.inner(), &student_id, &date, &note_type, &note, plan)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_student_meeting_notes(
    pool: State<'_, SqlitePool>,
    student_id: String,
) -> Result<Vec<MeetingNote>, String> {
    CounselingRepository::get_meeting_notes_by_student(pool.inner(), &student_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_follow_up(
    pool: State<'_, SqlitePool>,
    follow_up: CounselingFollowUp,
) -> Result<CounselingFollowUp, String> {
    CounselingRepository::create_follow_up(pool.inner(), follow_up)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_pending_follow_ups(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<CounselingFollowUp>, String> {
    CounselingRepository::get_pending_follow_ups(pool.inner())
        .await
        .map_err(|e| e.to_string())
}
