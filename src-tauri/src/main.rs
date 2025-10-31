// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod models;
mod commands;

use log::{info, error};
use commands::{auth, student};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    info!("Starting Rehber360 v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            
            // Initialize database
            tauri::async_runtime::spawn(async move {
                match database::initialize_database(&handle).await {
                    Ok(pool) => {
                        info!("Database initialized successfully");
                        handle.manage(pool);
                    }
                    Err(e) => {
                        error!("Failed to initialize database: {}", e);
                        std::process::exit(1);
                    }
                }
            });
            
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
