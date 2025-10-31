use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MeetingNote {
    pub id: String,
    pub studentId: String,
    pub date: String,
    #[serde(rename = "type")]
    pub meeting_type: String,
    pub note: String,
    pub plan: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CounselingSession {
    pub id: String,
    pub sessionType: String,
    pub groupName: Option<String>,
    pub counselorId: String,
    pub sessionDate: String,
    pub entryTime: String,
    pub entryClassHourId: Option<i32>,
    pub exitTime: Option<String>,
    pub exitClassHourId: Option<i32>,
    pub topic: String,
    pub participantType: String,
    pub relationshipType: Option<String>,
    pub otherParticipants: Option<String>,
    pub parentName: Option<String>,
    pub parentRelationship: Option<String>,
    pub teacherName: Option<String>,
    pub teacherBranch: Option<String>,
    pub otherParticipantDescription: Option<String>,
    pub sessionMode: String,
    pub sessionLocation: String,
    pub disciplineStatus: Option<String>,
    pub institutionalCooperation: Option<String>,
    pub sessionDetails: Option<String>,
    pub detailedNotes: Option<String>,
    pub sessionFlow: Option<String>,
    pub studentParticipationLevel: Option<String>,
    pub cooperationLevel: Option<i32>,
    pub emotionalState: Option<String>,
    pub physicalState: Option<String>,
    pub communicationQuality: Option<String>,
    pub sessionTags: Option<String>,
    pub achievedOutcomes: Option<String>,
    pub followUpNeeded: bool,
    pub followUpPlan: Option<String>,
    pub actionItems: Option<String>,
    pub autoCompleted: bool,
    pub extensionGranted: bool,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CounselingSessionStudent {
    pub id: i32,
    pub sessionId: String,
    pub studentId: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ParentMeeting {
    pub id: String,
    pub studentId: String,
    pub meetingDate: String,
    pub attendees: String,
    pub topics: String,
    pub outcomes: Option<String>,
    pub followUpActions: Option<String>,
    pub nextMeetingDate: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct HomeVisit {
    pub id: String,
    pub studentId: String,
    pub date: String,
    pub time: Option<String>,
    pub visitDuration: Option<i32>,
    pub visitors: Option<String>,
    pub familyPresent: Option<String>,
    pub homeEnvironment: Option<String>,
    pub familyInteraction: Option<String>,
    pub observations: Option<String>,
    pub recommendations: Option<String>,
    pub concerns: Option<String>,
    pub resources: Option<String>,
    pub followUpActions: Option<String>,
    pub nextVisitPlanned: Option<String>,
    pub notes: Option<String>,
    pub createdBy: Option<String>,
    pub createdAt: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FamilyParticipation {
    pub id: String,
    pub studentId: String,
    pub eventType: String,
    pub eventName: Option<String>,
    pub eventDate: String,
    pub participationStatus: Option<String>,
    pub participants: Option<String>,
    pub engagementLevel: Option<String>,
    pub communicationFrequency: Option<String>,
    pub preferredContactMethod: Option<String>,
    pub parentAvailability: Option<String>,
    pub notes: Option<String>,
    pub recordedBy: Option<String>,
    pub recordedAt: Option<String>,
    pub description: Option<String>,
    pub participantNames: Option<String>,
    pub outcomes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CounselingReminder {
    pub id: String,
    pub sessionId: Option<String>,
    pub reminderType: String,
    pub reminderDate: String,
    pub reminderTime: String,
    pub title: String,
    pub description: Option<String>,
    pub studentIds: Option<String>,
    pub status: String,
    pub notificationSent: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CounselingFollowUp {
    pub id: String,
    pub sessionId: Option<String>,
    pub followUpDate: String,
    pub assignedTo: String,
    pub priority: String,
    pub status: String,
    pub actionItems: String,
    pub notes: Option<String>,
    pub completedDate: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CounselingOutcome {
    pub id: String,
    pub sessionId: String,
    pub effectivenessRating: Option<i32>,
    pub progressNotes: Option<String>,
    pub goalsAchieved: Option<String>,
    pub nextSteps: Option<String>,
    pub recommendations: Option<String>,
    pub followUpRequired: bool,
    pub followUpDate: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
