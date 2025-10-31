use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::path::PathBuf;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationReport {
    pub success: bool,
    pub students_migrated: usize,
    pub counseling_sessions_migrated: usize,
    pub academic_records_migrated: usize,
    pub behavior_records_migrated: usize,
    pub documents_migrated: usize,
    pub settings_migrated: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationProgress {
    pub stage: String,
    pub current: usize,
    pub total: usize,
    pub percentage: f32,
}

#[tauri::command]
pub async fn detect_electron_database() -> Result<Option<String>, String> {
    let possible_paths = vec![
        dirs::data_dir().map(|p| p.join("rehber360-electron").join("database.db")),
        dirs::home_dir().map(|p| p.join(".rehber360").join("database.db")),
        Some(PathBuf::from("./electron-db/database.db")),
    ];

    for path_option in possible_paths {
        if let Some(path) = path_option {
            if path.exists() {
                return Ok(Some(path.to_string_lossy().to_string()));
            }
        }
    }

    Ok(None)
}

#[tauri::command]
pub async fn migrate_from_electron(
    old_db_path: String,
    pool: State<'_, SqlitePool>,
) -> Result<MigrationReport, String> {
    let mut report = MigrationReport {
        success: false,
        students_migrated: 0,
        counseling_sessions_migrated: 0,
        academic_records_migrated: 0,
        behavior_records_migrated: 0,
        documents_migrated: 0,
        settings_migrated: 0,
        errors: Vec::new(),
        warnings: Vec::new(),
    };

    let old_db_url = format!("sqlite:{}", old_db_path);
    let old_pool = SqlitePool::connect(&old_db_url)
        .await
        .map_err(|e| format!("Eski veritabanına bağlanılamadı: {}", e))?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Transaction başlatılamadı: {}", e))?;

    match migrate_students(&old_pool, &mut tx, &mut report).await {
        Ok(_) => {},
        Err(e) => report.errors.push(format!("Öğrenci migrasyonu: {}", e)),
    }

    match migrate_counseling_sessions(&old_pool, &mut tx, &mut report).await {
        Ok(_) => {},
        Err(e) => report.errors.push(format!("Görüşme migrasyonu: {}", e)),
    }

    match migrate_academic_records(&old_pool, &mut tx, &mut report).await {
        Ok(_) => {},
        Err(e) => report.errors.push(format!("Akademik kayıt migrasyonu: {}", e)),
    }

    match migrate_behavior_records(&old_pool, &mut tx, &mut report).await {
        Ok(_) => {},
        Err(e) => report.errors.push(format!("Davranış kaydı migrasyonu: {}", e)),
    }

    match migrate_documents(&old_pool, &mut tx, &mut report).await {
        Ok(_) => {},
        Err(e) => report.errors.push(format!("Belge migrasyonu: {}", e)),
    }

    match migrate_settings(&old_pool, &mut tx, &mut report).await {
        Ok(_) => {},
        Err(e) => report.errors.push(format!("Ayar migrasyonu: {}", e)),
    }

    if report.errors.is_empty() {
        tx.commit()
            .await
            .map_err(|e| format!("Transaction commit hatası: {}", e))?;
        report.success = true;
    } else {
        tx.rollback()
            .await
            .map_err(|e| format!("Transaction rollback hatası: {}", e))?;
        report.warnings.push("Hatalar nedeniyle hiçbir değişiklik yapılmadı.".to_string());
    }

    old_pool.close().await;

    Ok(report)
}

async fn migrate_students(
    old_pool: &SqlitePool,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    report: &mut MigrationReport,
) -> Result<(), String> {
    let students = sqlx::query("SELECT * FROM students")
        .fetch_all(old_pool)
        .await
        .map_err(|e| format!("Öğrenci verileri okunamadı: {}", e))?;

    for student in students {
        let id: String = student.try_get("id").unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());
        let name: String = student.try_get("name").unwrap_or_default();
        let surname: String = student.try_get("surname").unwrap_or_default();
        let student_number: Option<String> = student.try_get("studentNumber").ok();
        let class: Option<String> = student.try_get("class").ok();
        let enrollment_date: String = student.try_get("enrollmentDate").unwrap_or_else(|_| "2025-01-01".to_string());
        let gender: String = student.try_get("gender").unwrap_or_else(|_| "Belirtilmemiş".to_string());
        let birth_date: Option<String> = student.try_get("birthDate").ok();
        let contact_phone: Option<String> = student.try_get("contactPhone").ok();
        let contact_email: Option<String> = student.try_get("contactEmail").ok();
        let parent_name: Option<String> = student.try_get("parentName").ok();
        let parent_phone: Option<String> = student.try_get("parentPhone").ok();
        let risk: String = student.try_get("risk").unwrap_or_else(|_| "Düşük".to_string());
        let status: String = student.try_get("status").unwrap_or_else(|_| "active".to_string());

        sqlx::query(
            "INSERT OR IGNORE INTO students 
            (id, name, surname, studentNumber, class, enrollmentDate, gender, birthDate, 
             contactPhone, contactEmail, parentName, parentPhone, risk, status)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(name)
        .bind(surname)
        .bind(student_number)
        .bind(class)
        .bind(enrollment_date)
        .bind(gender)
        .bind(birth_date)
        .bind(contact_phone)
        .bind(contact_email)
        .bind(parent_name)
        .bind(parent_phone)
        .bind(risk)
        .bind(status)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Öğrenci eklenemedi: {}", e))?;

        report.students_migrated += 1;
    }

    Ok(())
}

async fn migrate_counseling_sessions(
    old_pool: &SqlitePool,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    report: &mut MigrationReport,
) -> Result<(), String> {
    let sessions = sqlx::query("SELECT * FROM counseling_sessions")
        .fetch_all(old_pool)
        .await
        .map_err(|e| format!("Görüşme verileri okunamadı: {}", e))?;

    for session in sessions {
        let id: String = session.try_get("id").unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());
        let student_id: String = session.try_get("studentId").unwrap_or_default();
        let date: String = session.try_get("date").unwrap_or_default();
        let session_type: String = session.try_get("type").unwrap_or_default();
        let notes: Option<String> = session.try_get("notes").ok();
        let duration: Option<i64> = session.try_get("duration").ok();
        let follow_up_required: bool = session.try_get("followUpRequired").unwrap_or(false);
        let status: String = session.try_get("status").unwrap_or_else(|_| "completed".to_string());

        sqlx::query(
            "INSERT OR IGNORE INTO counseling_sessions 
            (id, studentId, date, type, notes, duration, followUpRequired, status)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(student_id)
        .bind(date)
        .bind(session_type)
        .bind(notes)
        .bind(duration)
        .bind(follow_up_required)
        .bind(status)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Görüşme eklenemedi: {}", e))?;

        report.counseling_sessions_migrated += 1;
    }

    Ok(())
}

async fn migrate_academic_records(
    old_pool: &SqlitePool,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    report: &mut MigrationReport,
) -> Result<(), String> {
    let records = sqlx::query("SELECT * FROM academic_records")
        .fetch_all(old_pool)
        .await
        .map_err(|e| format!("Akademik kayıt verileri okunamadı: {}", e))?;

    for record in records {
        let id: String = record.try_get("id").unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());
        let student_id: String = record.try_get("studentId").unwrap_or_default();
        let term: String = record.try_get("term").unwrap_or_default();
        let gpa: Option<f64> = record.try_get("gpa").ok();
        let attendance: Option<f64> = record.try_get("attendance").ok();
        let absences: Option<i64> = record.try_get("absences").ok();

        sqlx::query(
            "INSERT OR IGNORE INTO academic_records 
            (id, studentId, term, gpa, attendance, absences)
            VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(student_id)
        .bind(term)
        .bind(gpa)
        .bind(attendance)
        .bind(absences)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Akademik kayıt eklenemedi: {}", e))?;

        report.academic_records_migrated += 1;
    }

    Ok(())
}

async fn migrate_behavior_records(
    old_pool: &SqlitePool,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    report: &mut MigrationReport,
) -> Result<(), String> {
    let records = sqlx::query("SELECT * FROM behavior_records")
        .fetch_all(old_pool)
        .await
        .map_err(|e| format!("Davranış kaydı verileri okunamadı: {}", e))?;

    for record in records {
        let id: String = record.try_get("id").unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());
        let student_id: String = record.try_get("studentId").unwrap_or_default();
        let date: String = record.try_get("date").unwrap_or_default();
        let behavior_type: String = record.try_get("type").unwrap_or_default();
        let description: String = record.try_get("description").unwrap_or_default();
        let severity: Option<String> = record.try_get("severity").ok();

        sqlx::query(
            "INSERT OR IGNORE INTO behavior_records 
            (id, studentId, date, type, description, severity)
            VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(student_id)
        .bind(date)
        .bind(behavior_type)
        .bind(description)
        .bind(severity)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Davranış kaydı eklenemedi: {}", e))?;

        report.behavior_records_migrated += 1;
    }

    Ok(())
}

async fn migrate_documents(
    old_pool: &SqlitePool,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    report: &mut MigrationReport,
) -> Result<(), String> {
    let documents = sqlx::query("SELECT * FROM student_documents")
        .fetch_all(old_pool)
        .await
        .map_err(|e| format!("Belge verileri okunamadı: {}", e))?;

    for document in documents {
        let id: String = document.try_get("id").unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());
        let student_id: String = document.try_get("studentId").unwrap_or_default();
        let name: String = document.try_get("name").unwrap_or_default();
        let doc_type: String = document.try_get("type").unwrap_or_default();
        let data_url: String = document.try_get("dataUrl").unwrap_or_default();
        let uploaded_at: String = document.try_get("uploadedAt").unwrap_or_else(|_| chrono::Utc::now().to_rfc3339());

        sqlx::query(
            "INSERT OR IGNORE INTO student_documents 
            (id, studentId, name, type, dataUrl, uploadedAt)
            VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(student_id)
        .bind(name)
        .bind(doc_type)
        .bind(data_url)
        .bind(uploaded_at)
        .execute(&mut **tx)
        .await
        .map_err(|e| format!("Belge eklenemedi: {}", e))?;

        report.documents_migrated += 1;
    }

    Ok(())
}

async fn migrate_settings(
    old_pool: &SqlitePool,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    report: &mut MigrationReport,
) -> Result<(), String> {
    let settings = sqlx::query("SELECT * FROM settings")
        .fetch_all(old_pool)
        .await;

    if let Ok(settings) = settings {
        for setting in settings {
            let key: String = setting.try_get("key").unwrap_or_default();
            let value: String = setting.try_get("value").unwrap_or_default();
            let setting_type: String = setting.try_get("type").unwrap_or_else(|_| "string".to_string());
            let description: Option<String> = setting.try_get("description").ok();

            sqlx::query(
                "INSERT OR REPLACE INTO settings 
                (key, value, type, description)
                VALUES (?, ?, ?, ?)"
            )
            .bind(key)
            .bind(value)
            .bind(setting_type)
            .bind(description)
            .execute(&mut **tx)
            .await
            .map_err(|e| format!("Ayar eklenemedi: {}", e))?;

            report.settings_migrated += 1;
        }
    } else {
        report.warnings.push("Eski veritabanında settings tablosu bulunamadı.".to_string());
    }

    Ok(())
}

#[tauri::command]
pub async fn validate_electron_database(db_path: String) -> Result<bool, String> {
    let db_url = format!("sqlite:{}", db_path);
    let pool = SqlitePool::connect(&db_url)
        .await
        .map_err(|e| format!("Veritabanına bağlanılamadı: {}", e))?;

    let table_check = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='students'"
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| format!("Tablo kontrolü başarısız: {}", e))?;

    pool.close().await;

    Ok(table_check.is_some())
}
