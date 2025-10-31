// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use log::{info, error};
use commands::{auth, student};
use rehber360_core::database;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    info!("Starting Rehber360 v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");
            let db_path = app_data_dir.join("rehber360.db");
            
            // Initialize database synchronously (blocking to ensure it's ready before commands run)
            let pool = tauri::async_runtime::block_on(async {
                database::initialize_database(db_path).await
            }).map_err(|e| {
                error!("Failed to initialize database: {}", e);
                e
            })?;
            
            info!("Database initialized successfully");
            
            // Manage the pool so commands can access it
            app.manage(pool);
            
            info!("Tauri application setup completed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Auth commands
            auth::login,
            auth::logout,
            auth::get_current_user,
            // Student commands
            student::get_all_students,
            student::get_student,
            student::create_student,
            student::update_student,
            student::delete_student,
            student::search_students,
            // Counseling commands
            counseling::get_all_counseling_sessions,
            counseling::get_counseling_session,
            counseling::get_student_counseling_sessions,
            counseling::create_counseling_session,
            counseling::update_counseling_session,
            counseling::delete_counseling_session,
            counseling::add_student_to_session,
            counseling::create_meeting_note,
            counseling::get_student_meeting_notes,
            counseling::create_follow_up,
            counseling::get_pending_follow_ups,
            // Academic commands
            academic::create_exam_result,
            academic::get_exam_result,
            academic::get_student_exam_results,
            academic::get_student_exam_results_by_type,
            academic::update_exam_result,
            academic::delete_exam_result,
            academic::create_behavior_incident,
            academic::get_behavior_incident,
            academic::get_student_behavior_incidents,
            academic::create_academic_goal,
            academic::get_student_academic_goals,
            // AI commands
            ai::create_ai_suggestion,
            ai::get_ai_suggestion,
            ai::get_student_ai_suggestions,
            ai::get_pending_ai_suggestions,
            ai::get_ai_suggestions_by_priority,
            ai::review_ai_suggestion,
            ai::delete_ai_suggestion,
            ai::clean_expired_suggestions,
            ai::get_ai_suggestion_statistics,
            ai::analyze_student_profile,
            ai::generate_counseling_recommendations,
            // Survey commands
            survey::create_survey_template,
            survey::get_survey_template,
            survey::get_all_survey_templates,
            survey::create_survey_distribution,
            survey::create_survey_response,
            survey::get_distribution_responses,
            survey::get_student_survey_responses,
            survey::create_legacy_survey,
            survey::update_survey_responses,
            survey::get_student_surveys,
            survey::delete_survey_template,
            // Notification commands
            notification::create_notification,
            notification::get_notification,
            notification::get_user_notifications,
            notification::get_student_notifications,
            notification::get_pending_notifications,
            notification::update_notification_status,
            notification::mark_notification_read,
            notification::get_notification_preferences,
            notification::update_notification_preferences,
            notification::create_scheduled_task,
            notification::get_due_tasks,
            notification::update_task_next_run,
            notification::delete_notification,
            notification::send_native_notification,
            // Settings commands
            settings::get_settings,
            settings::save_settings,
            settings::update_ai_provider,
            // AI service commands
            ai::chat_with_ai,
            ai::test_ai_connection,
            // File commands
            file::upload_file,
            file::download_file,
            file::delete_file,
            file::get_file_list,
            file::open_file_in_explorer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
