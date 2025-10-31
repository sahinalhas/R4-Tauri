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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
