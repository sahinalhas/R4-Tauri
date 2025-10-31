/**
 * Transport Layer - Tauri Desktop API
 * 
 * Maps REST-style API endpoints to Tauri Rust commands via invoke().
 * Desktop-only application - no web/HTTP mode.
 */

import { isDesktopMode } from '@/lib/utils/platform';
import { ApiError, ApiErrorCode } from '@/lib/types/api-types';

/**
 * Transport request configuration
 */
export interface TransportConfig {
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
  body?: unknown;
  headers?: Record<string, string>;
  timeout?: number;
}

/**
 * Transport interface - implemented by both Tauri and HTTP transports
 */
export interface Transport {
  request<T>(endpoint: string, config?: TransportConfig): Promise<T>;
}

/**
 * Tauri Transport - uses invoke() to call Rust backend commands
 */
class TauriTransport implements Transport {
  async request<T>(endpoint: string, config: TransportConfig = {}): Promise<T> {
    try {
      // Dynamically import Tauri API
      const { invoke } = await import('@tauri-apps/api/core');
      
      // Map HTTP endpoint to Tauri command
      const command = this.endpointToCommand(endpoint, config.method || 'GET');
      const args = this.buildCommandArgs(endpoint, config);
      
      const result = await invoke<T>(command, args);
      return result;
    } catch (error) {
      throw this.handleTauriError(error);
    }
  }
  
  /**
   * Map HTTP endpoint to Tauri command name
   * Example: GET /api/students -> get_all_students
   */
  private endpointToCommand(endpoint: string, method: string): string {
    // Remove /api prefix and leading/trailing slashes
    const path = endpoint.replace(/^\/api\//, '').replace(/^\//, '').replace(/\/$/, '');
    
    // Special case mappings for Tauri commands
    // NOTE: This is a hybrid approach - explicit mappings for common patterns,
    // automatic fallback (snake_case conversion) for others
    const commandMap: Record<string, string> = {
      // Auth
      'POST:auth/login': 'login',
      'POST:auth/logout': 'logout',
      'GET:auth/me': 'get_current_user',
      
      // Students
      'GET:students': 'get_all_students',
      'POST:students': 'create_student',
      'GET:students/:id': 'get_student',
      'PUT:students/:id': 'update_student',
      'DELETE:students/:id': 'delete_student',
      'POST:students/bulk': 'bulk_create_students',
      
      // Counseling Sessions
      'GET:counseling-sessions': 'get_all_counseling_sessions',
      'POST:counseling-sessions': 'create_counseling_session',
      'GET:counseling-sessions/:id': 'get_counseling_session',
      'PUT:counseling-sessions/:id': 'update_counseling_session',
      'DELETE:counseling-sessions/:id': 'delete_counseling_session',
      
      // Academic / Exams
      'GET:exams': 'get_all_exam_results',
      'POST:exams': 'create_exam_result',
      'GET:exams/:id': 'get_exam_result',
      'PUT:exams/:id': 'update_exam_result',
      'DELETE:exams/:id': 'delete_exam_result',
      
      // Behavior
      'GET:behavior/:studentId': 'get_student_behavior_incidents',
      'POST:behavior': 'create_behavior_incident',
      
      // Academic Goals
      'GET:academic/goals/:studentId': 'get_student_academic_goals',
      'POST:academic/goals': 'create_academic_goal',
      
      // Meeting Notes
      'GET:meeting-notes/:studentId': 'get_student_meeting_notes',
      'POST:meeting-notes': 'create_meeting_note',
      
      // Follow-ups
      'GET:follow-ups/pending': 'get_pending_follow_ups',
      
      // AI Suggestions
      'POST:ai-suggestions': 'create_ai_suggestion',
      'GET:ai-suggestions/:id': 'get_ai_suggestion',
      'GET:ai-suggestions/student/:studentId': 'get_student_ai_suggestions',
      'GET:ai-suggestions/pending': 'get_pending_ai_suggestions',
      'POST:ai-suggestions/:id/approve': 'approve_ai_suggestion',
      'POST:ai-suggestions/:id/reject': 'reject_ai_suggestion',
      'DELETE:ai-suggestions/:id': 'delete_ai_suggestion',
      
      // AI Analysis
      'POST:ai/analyze/:studentId': 'analyze_student_profile',
      'POST:ai/recommendations/:studentId': 'generate_counseling_recommendations',
      
      // Notifications
      'POST:notifications': 'create_notification',
      'GET:notifications/user/:userId': 'get_user_notifications',
      'GET:notifications/student/:studentId': 'get_student_notifications',
      'PUT:notifications/:id/read': 'mark_notification_read',
      'POST:notifications/native': 'send_native_notification',
      
      // Surveys
      'POST:surveys/templates': 'create_survey_template',
      'GET:surveys/templates': 'get_all_survey_templates',
      'POST:surveys/distributions': 'create_survey_distribution',
      'POST:surveys/responses': 'create_survey_response',
      'GET:surveys/student/:studentId': 'get_student_surveys',
      
      // Files
      'POST:files/upload': 'upload_file',
      'GET:files/:id': 'download_file',
      'DELETE:files/:id': 'delete_file',
      'GET:files/student/:studentId': 'get_student_files',
      'POST:files/open': 'open_file_in_explorer',
    };
    
    // Try exact match first
    const key = `${method}:${path}`;
    if (commandMap[key]) {
      return commandMap[key];
    }
    
    // Try pattern matching for :id params
    for (const [pattern, command] of Object.entries(commandMap)) {
      const [patternMethod, patternPath] = pattern.split(':');
      if (patternMethod === method && this.matchPattern(path, patternPath)) {
        return command;
      }
    }
    
    // Fallback: convert to snake_case command name
    console.warn(`[TauriTransport] No explicit mapping for ${method} ${endpoint}, using fallback`);
    return path.replace(/\//g, '_').replace(/-/g, '_').toLowerCase();
  }
  
  /**
   * Check if path matches pattern (supports :param)
   */
  private matchPattern(path: string, pattern: string): boolean {
    const pathParts = path.split('/');
    const patternParts = pattern.split('/');
    
    if (pathParts.length !== patternParts.length) {
      return false;
    }
    
    return patternParts.every((part, i) => {
      return part.startsWith(':') || part === pathParts[i];
    });
  }
  
  /**
   * Build Tauri command arguments from endpoint and config
   */
  private buildCommandArgs(endpoint: string, config: TransportConfig): Record<string, unknown> {
    const args: Record<string, unknown> = {};
    
    // Remove /api prefix and query string
    const cleanPath = endpoint.replace(/^\/api\//, '').split('?')[0];
    const pathParts = cleanPath.split('/').filter(Boolean);
    
    // Extract path parameters intelligently
    // Patterns:
    // - /students/123 → { id: "123" }
    // - /behavior/123 → { student_id: "123" }
    // - /notifications/user/456 → { user_id: "456" } or { recipient_id: "456" }
    // - /ai-suggestions/student/789 → { student_id: "789" }
    
    if (pathParts.length >= 2) {
      const lastPart = pathParts[pathParts.length - 1];
      const secondToLast = pathParts[pathParts.length - 2];
      
      // Check if last part is an ID (numeric or UUID-like)
      if (/^\d+$/.test(lastPart) || /^[a-f0-9-]{36}$/i.test(lastPart)) {
        // Determine parameter name based on context
        if (secondToLast === 'student' || endpoint.includes('/student/')) {
          args.student_id = lastPart;
        } else if (secondToLast === 'user' || endpoint.includes('/user/')) {
          args.user_id = lastPart;
        } else if (secondToLast === 'session' || endpoint.includes('/session/')) {
          args.session_id = lastPart;
        } else {
          // Default to 'id'
          args.id = lastPart;
        }
      }
    }
    
    // Extract query parameters from endpoint if present
    const queryMatch = endpoint.match(/\?(.+)$/);
    if (queryMatch) {
      const queryString = queryMatch[1];
      const queryParams = new URLSearchParams(queryString);
      queryParams.forEach((value, key) => {
        args[key] = value;
      });
    }
    
    // Add body as request/payload
    if (config.body) {
      // If body is object, spread it into args for simpler command signatures
      if (typeof config.body === 'object' && config.body !== null && !Array.isArray(config.body)) {
        Object.assign(args, config.body);
      } else {
        args.request = config.body;
      }
    }
    
    return args;
  }
  
  /**
   * Convert Tauri error to ApiError
   */
  private handleTauriError(error: unknown): ApiError {
    const message = error instanceof Error ? error.message : String(error);
    const apiError = new Error(message) as ApiError;
    
    apiError.name = 'TauriCommandError';
    apiError.code = ApiErrorCode.TAURI_COMMAND_ERROR;
    apiError.statusCode = 500;
    
    return apiError;
  }
}


/**
 * Create Tauri transport (desktop-only application)
 */
export function createTransport(): Transport {
  if (!isDesktopMode()) {
    console.error('[Transport] ERROR: Tauri not detected!');
    console.error('[Transport] This is a desktop-only application.');
    console.error('[Transport] Please run: npm run tauri:dev');
  }
  
  return new TauriTransport();
}

/**
 * Global transport instance
 */
export const transport = createTransport();
