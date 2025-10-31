// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use log::{info, error};
use commands::{auth, student, counseling, academic, ai, survey, notification, settings, file, export, migration};
use rehber360_core::database;
use tauri::{
    Manager, RunEvent, WindowEvent,
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    info!("Starting Rehber360 v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        // Register plugins
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            info!("Setting up Tauri application...");
            
            // Get app data directory and initialize database
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");
            let db_path = app_data_dir.join("rehber360.db");
            
            // Initialize database synchronously
            let pool = tauri::async_runtime::block_on(async {
                database::initialize_database(db_path).await
            }).map_err(|e| {
                error!("Failed to initialize database: {}", e);
                e
            })?;
            
            info!("Database initialized successfully");
            app.manage(pool);
            
            // Create system tray menu (Turkish)
            let show_item = MenuItemBuilder::with_id("show", "Göster").build(app)?;
            let hide_item = MenuItemBuilder::with_id("hide", "Gizle").build(app)?;
            let separator = tauri::menu::PredefinedMenuItem::separator(app)?;
            let settings_item = MenuItemBuilder::with_id("settings", "Ayarlar").build(app)?;
            let about_item = MenuItemBuilder::with_id("about", "Hakkında").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "Çıkış").build(app)?;
            
            let tray_menu = MenuBuilder::new(app)
                .item(&show_item)
                .item(&hide_item)
                .item(&separator)
                .item(&settings_item)
                .item(&about_item)
                .item(&separator)
                .item(&quit_item)
                .build()?;
            
            // Create system tray icon
            let _tray = TrayIconBuilder::new()
                .menu(&tray_menu)
                .tooltip("Rehber360")
                .on_menu_event(|app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                info!("Main window shown from tray");
                            }
                        }
                        "hide" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.hide();
                                info!("Main window hidden to tray");
                            }
                        }
                        "settings" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                // Navigate to settings (frontend will handle)
                                let _ = window.eval("window.location.hash = '#/settings'");
                                info!("Navigated to settings");
                            }
                        }
                        "about" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                info!("About menu clicked");
                            }
                        }
                        "quit" => {
                            info!("Quit requested from tray menu");
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    // Handle tray icon events
                    match event {
                        TrayIconEvent::Click { 
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } => {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                        TrayIconEvent::DoubleClick { 
                            button: MouseButton::Left,
                            ..
                        } => {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                let _ = window.unminimize();
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            
            // Create application menu (Turkish)
            let file_menu = SubmenuBuilder::new(app, "Dosya")
                .text("new_student", "Yeni Öğrenci")
                .text("import", "İçe Aktar")
                .text("export", "Dışa Aktar")
                .separator()
                .text("preferences", "Tercihler")
                .separator()
                .text("quit_menu", "Çıkış")
                .build()?;
            
            let view_menu = SubmenuBuilder::new(app, "Görünüm")
                .text("students", "Öğrenciler")
                .text("counseling", "Görüşmeler")
                .text("analytics", "Analitik")
                .separator()
                .text("fullscreen", "Tam Ekran")
                .build()?;
            
            let help_menu = SubmenuBuilder::new(app, "Yardım")
                .text("documentation", "Dokümantasyon")
                .text("check_updates", "Güncellemeleri Kontrol Et")
                .separator()
                .text("about_menu", "Hakkında")
                .build()?;
            
            let app_menu = MenuBuilder::new(app)
                .item(&file_menu)
                .item(&view_menu)
                .item(&help_menu)
                .build()?;
            
            app.set_menu(app_menu)?;
            
            // Handle menu events
            app.on_menu_event(|app, event| {
                match event.id().as_ref() {
                    "new_student" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.eval("window.location.hash = '#/students/new'");
                        }
                    }
                    "quit_menu" => {
                        app.exit(0);
                    }
                    "fullscreen" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let is_fullscreen = window.is_fullscreen().unwrap_or(false);
                            let _ = window.set_fullscreen(!is_fullscreen);
                        }
                    }
                    "check_updates" => {
                        info!("Checking for updates...");
                        // Updater will be triggered automatically
                    }
                    _ => {
                        info!("Menu event: {}", event.id());
                    }
                }
            });
            
            // Window event handler for minimize-to-tray
            if let Some(window) = app.get_webview_window("main") {
                window.on_window_event(|event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        // Hide window instead of closing (minimize to tray)
                        event.window().hide().unwrap();
                        api.prevent_close();
                        info!("Window close prevented, minimized to tray");
                    }
                });
            }
            
            info!("Tauri application setup completed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Auth commands
            auth::login,
            auth::logout,
            auth::get_current_user,
            auth::create_initial_admin,
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
            // Export/Import commands
            export::export_database_json,
            export::import_database_json,
            export::export_students_csv,
            export::get_export_statistics,
        ])
        .build(tauri::generate_context!())
        .expect("error building tauri application")
        .run(|app, event| {
            // Handle app-level run events
            match event {
                RunEvent::ExitRequested { api, .. } => {
                    // Optional: Prevent exit and hide windows instead
                    // api.prevent_exit();
                    info!("Application exit requested");
                }
                RunEvent::Updater(updater_event) => {
                    info!("Updater event: {:?}", updater_event);
                }
                _ => {}
            }
        });
}
