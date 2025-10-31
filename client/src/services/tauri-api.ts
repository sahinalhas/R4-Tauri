/**
 * Tauri API Client
 * 
 * Type-safe wrapper around Tauri's invoke() API for communicating with the Rust backend.
 * This client provides all the necessary methods to interact with the Tauri commands.
 * 
 * Usage:
 * ```typescript
 * import { tauriApi } from '@/services/tauri-api';
 * 
 * const students = await tauriApi.students.getAll();
 * const user = await tauriApi.auth.login(email, password);
 * ```
 */

// Conditional Tauri invoke - lazily imports the real Tauri API when available
const invoke = async <T>(command: string, args?: Record<string, unknown>): Promise<T> => {
  if (typeof window !== 'undefined' && '__TAURI__' in window) {
    // Running in Tauri desktop app - dynamically import the actual invoke
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return tauriInvoke<T>(command, args);
  } else {
    // Running in web browser (development) - provide helpful error
    throw new Error(`Tauri command '${command}' is only available in desktop mode. Use the web API instead.`);
  }
};

// ==================== Type Definitions ====================

export interface User {
  id: string;
  name: string;
  email: string;
  role: string;
  institution: string;
  isActive: boolean;
  created_at: string;
  updated_at: string;
}

export interface UserSession {
  user: User;
  token: string;
}

export interface Student {
  id: string;
  name: string;
  surname: string;
  email?: string;
  phone?: string;
  birthDate?: string;
  address?: string;
  class?: string;
  enrollmentDate: string;
  status: string;
  avatar?: string;
  parentContact?: string;
  notes?: string;
  gender: string;
  risk: string;
  created_at: string;
  updated_at: string;
}

export interface CounselingSession {
  id: string;
  sessionType: string;
  groupName?: string;
  counselorId: string;
  sessionDate: string;
  entryTime: string;
  topic: string;
  participantType: string;
  sessionMode: string;
  sessionLocation: string;
  detailedNotes?: string;
  sessionFlow?: string;
  followUpNeeded: boolean;
  completed: boolean;
  created_at: string;
  updated_at: string;
}

export interface ExamResult {
  id: string;
  studentId: string;
  examType: string;
  examName: string;
  examDate: string;
  totalScore?: number;
  percentileRank?: number;
  created_at: string;
  updated_at: string;
}

export interface AiSuggestion {
  id: string;
  studentId: string;
  suggestionType: string;
  priority: string;
  title: string;
  description: string;
  status: string;
  createdAt: string;
  updatedAt: string;
}

export interface NotificationLog {
  id: string;
  recipientType: string;
  notificationType: string;
  channel: string;
  message: string;
  status: string;
  priority: string;
  created_at: string;
}

// ==================== API Client ====================

export const tauriApi = {
  // ==================== Authentication ====================
  auth: {
    login: async (email: string, password: string): Promise<UserSession> => {
      return await invoke<UserSession>('login', { credentials: { email, password } });
    },

    logout: async (token: string): Promise<void> => {
      return await invoke<void>('logout', { token });
    },

    getCurrentUser: async (token: string): Promise<User> => {
      return await invoke<User>('get_current_user', { token });
    },
  },

  // ==================== Students ====================
  students: {
    getAll: async (): Promise<Student[]> => {
      return await invoke<Student[]>('get_all_students');
    },

    getById: async (id: string): Promise<Student> => {
      return await invoke<Student>('get_student', { id });
    },

    create: async (student: Partial<Student>): Promise<Student> => {
      return await invoke<Student>('create_student', { request: student });
    },

    update: async (id: string, student: Partial<Student>): Promise<Student> => {
      return await invoke<Student>('update_student', { id, request: student });
    },

    delete: async (id: string): Promise<void> => {
      return await invoke<void>('delete_student', { id });
    },

    search: async (query: string): Promise<Student[]> => {
      return await invoke<Student[]>('search_students', { query });
    },
  },

  // ==================== Counseling ====================
  counseling: {
    getAllSessions: async (): Promise<CounselingSession[]> => {
      return await invoke<CounselingSession[]>('get_all_counseling_sessions');
    },

    getSession: async (id: string): Promise<CounselingSession> => {
      return await invoke<CounselingSession>('get_counseling_session', { id });
    },

    getStudentSessions: async (studentId: string): Promise<CounselingSession[]> => {
      return await invoke<CounselingSession[]>('get_student_counseling_sessions', { student_id: studentId });
    },

    createSession: async (
      sessionType: string,
      counselorId: string,
      sessionDate: string,
      entryTime: string,
      topic: string,
      participantType: string,
      sessionMode: string,
      sessionLocation: string
    ): Promise<CounselingSession> => {
      return await invoke<CounselingSession>('create_counseling_session', {
        session_type: sessionType,
        counselor_id: counselorId,
        session_date: sessionDate,
        entry_time: entryTime,
        topic,
        participant_type: participantType,
        session_mode: sessionMode,
        session_location: sessionLocation,
      });
    },

    updateSession: async (id: string, session: Partial<CounselingSession>): Promise<void> => {
      return await invoke<void>('update_counseling_session', { id, session });
    },

    deleteSession: async (id: string): Promise<void> => {
      return await invoke<void>('delete_counseling_session', { id });
    },

    addStudentToSession: async (sessionId: string, studentId: string): Promise<void> => {
      return await invoke<void>('add_student_to_session', { session_id: sessionId, student_id: studentId });
    },

    createMeetingNote: async (studentId: string, date: string, noteType: string, note: string, plan?: string): Promise<unknown> => {
      return await invoke('create_meeting_note', { student_id: studentId, date, note_type: noteType, note, plan });
    },

    getStudentMeetingNotes: async (studentId: string): Promise<unknown[]> => {
      return await invoke<unknown[]>('get_student_meeting_notes', { student_id: studentId });
    },

    getPendingFollowUps: async (): Promise<unknown[]> => {
      return await invoke<unknown[]>('get_pending_follow_ups');
    },
  },

  // ==================== Academic ====================
  academic: {
    createExamResult: async (
      studentId: string,
      examType: string,
      examName: string,
      examDate: string,
      totalScore?: number
    ): Promise<ExamResult> => {
      return await invoke<ExamResult>('create_exam_result', {
        student_id: studentId,
        exam_type: examType,
        exam_name: examName,
        exam_date: examDate,
        total_score: totalScore,
      });
    },

    getExamResult: async (id: string): Promise<ExamResult> => {
      return await invoke<ExamResult>('get_exam_result', { id });
    },

    getStudentExamResults: async (studentId: string): Promise<ExamResult[]> => {
      return await invoke<ExamResult[]>('get_student_exam_results', { student_id: studentId });
    },

    getStudentExamResultsByType: async (studentId: string, examType: string): Promise<ExamResult[]> => {
      return await invoke<ExamResult[]>('get_student_exam_results_by_type', { student_id: studentId, exam_type: examType });
    },

    updateExamResult: async (id: string, result: Partial<ExamResult>): Promise<void> => {
      return await invoke<void>('update_exam_result', { id, result });
    },

    deleteExamResult: async (id: string): Promise<void> => {
      return await invoke<void>('delete_exam_result', { id });
    },

    createBehaviorIncident: async (
      studentId: string,
      incidentDate: string,
      incidentTime: string,
      location: string,
      behaviorType: string,
      behaviorCategory: string,
      description: string,
      recordedBy: string
    ): Promise<unknown> => {
      return await invoke('create_behavior_incident', {
        student_id: studentId,
        incident_date: incidentDate,
        incident_time: incidentTime,
        location,
        behavior_type: behaviorType,
        behavior_category: behaviorCategory,
        description,
        recorded_by: recordedBy,
      });
    },

    getStudentBehaviorIncidents: async (studentId: string): Promise<unknown[]> => {
      return await invoke<unknown[]>('get_student_behavior_incidents', { student_id: studentId });
    },

    createAcademicGoal: async (
      studentId: string,
      title: string,
      targetScore?: number,
      examType?: string,
      deadline?: string
    ): Promise<unknown> => {
      return await invoke('create_academic_goal', {
        student_id: studentId,
        title,
        target_score: targetScore,
        exam_type: examType,
        deadline,
      });
    },

    getStudentAcademicGoals: async (studentId: string): Promise<unknown[]> => {
      return await invoke<unknown[]>('get_student_academic_goals', { student_id: studentId });
    },
  },

  // ==================== AI ====================
  ai: {
    createSuggestion: async (request: Partial<AiSuggestion>): Promise<AiSuggestion> => {
      return await invoke<AiSuggestion>('create_ai_suggestion', { request });
    },

    getSuggestion: async (id: string): Promise<AiSuggestion> => {
      return await invoke<AiSuggestion>('get_ai_suggestion', { id });
    },

    getStudentSuggestions: async (studentId: string): Promise<AiSuggestion[]> => {
      return await invoke<AiSuggestion[]>('get_student_ai_suggestions', { student_id: studentId });
    },

    getPendingSuggestions: async (): Promise<AiSuggestion[]> => {
      return await invoke<AiSuggestion[]>('get_pending_ai_suggestions');
    },

    reviewSuggestion: async (id: string, review: { status: string; reviewedBy: string }): Promise<AiSuggestion> => {
      return await invoke<AiSuggestion>('review_ai_suggestion', { id, review });
    },

    deleteSuggestion: async (id: string): Promise<void> => {
      return await invoke<void>('delete_ai_suggestion', { id });
    },

    analyzeStudentProfile: async (studentId: string): Promise<string> => {
      return await invoke<string>('analyze_student_profile', { student_id: studentId });
    },

    generateRecommendations: async (studentId: string): Promise<string[]> => {
      return await invoke<string[]>('generate_counseling_recommendations', { student_id: studentId });
    },
  },

  // ==================== Surveys ====================
  surveys: {
    createTemplate: async (template: unknown): Promise<unknown> => {
      return await invoke('create_survey_template', { template });
    },

    getAllTemplates: async (): Promise<unknown[]> => {
      return await invoke<unknown[]>('get_all_survey_templates');
    },

    createDistribution: async (distribution: unknown): Promise<unknown> => {
      return await invoke('create_survey_distribution', { distribution });
    },

    createResponse: async (response: unknown): Promise<unknown> => {
      return await invoke('create_survey_response', { response });
    },

    getStudentSurveys: async (studentId: string): Promise<unknown[]> => {
      return await invoke<unknown[]>('get_student_surveys', { student_id: studentId });
    },
  },

  // ==================== Notifications ====================
  notifications: {
    create: async (
      recipientType: string,
      notificationType: string,
      channel: string,
      message: string,
      studentId?: string
    ): Promise<NotificationLog> => {
      return await invoke<NotificationLog>('create_notification', {
        recipient_type: recipientType,
        notification_type: notificationType,
        channel,
        message,
        student_id: studentId,
      });
    },

    getUserNotifications: async (recipientId: string): Promise<NotificationLog[]> => {
      return await invoke<NotificationLog[]>('get_user_notifications', { recipient_id: recipientId });
    },

    getStudentNotifications: async (studentId: string): Promise<NotificationLog[]> => {
      return await invoke<NotificationLog[]>('get_student_notifications', { student_id: studentId });
    },

    markAsRead: async (id: string): Promise<void> => {
      return await invoke<void>('mark_notification_read', { id });
    },

    sendNative: async (title: string, body: string): Promise<void> => {
      return await invoke<void>('send_native_notification', { title, body });
    },
  },
};

export default tauriApi;
