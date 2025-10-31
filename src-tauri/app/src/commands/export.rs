use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;
use rehber360_core::models::{Student, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseExport {
    pub version: String,
    pub exported_at: String,
    pub students: Vec<Student>,
    pub users: Vec<UserExport>,
    pub counseling_sessions: Vec<CounselingSessionExport>,
    pub academic_records: Vec<AcademicRecordExport>,
    pub surveys: Vec<SurveyExport>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserExport {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub institution: String,
    pub isActive: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CounselingSessionExport {
    pub id: String,
    pub studentId: String,
    pub counselorId: String,
    pub date: String,
    pub duration: Option<i32>,
    #[sqlx(rename = "type")]
    pub session_type: String,
    pub topic: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AcademicRecordExport {
    pub id: String,
    pub studentId: String,
    pub term: String,
    pub subject: String,
    pub examType: String,
    pub grade: f64,
    pub date: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SurveyExport {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    #[sqlx(rename = "type")]
    pub survey_type: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub success: bool,
    pub message: String,
    pub students_imported: usize,
    pub users_imported: usize,
    pub sessions_imported: usize,
    pub records_imported: usize,
}

#[tauri::command]
pub async fn export_database_json(
    pool: State<'_, SqlitePool>,
) -> Result<String, String> {
    let students = sqlx::query_as::<_, Student>("SELECT * FROM students")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| format!("Failed to fetch students: {}", e))?;

    let users = sqlx::query_as::<_, UserExport>(
        "SELECT id, name, email, role, institution, isActive, created_at FROM users"
    )
        .fetch_all(pool.inner())
        .await
        .map_err(|e| format!("Failed to fetch users: {}", e))?;

    let counseling_sessions = sqlx::query_as::<_, CounselingSessionExport>(
        "SELECT id, studentId, counselorId, date, duration, type, topic, notes, created_at 
         FROM counseling_sessions"
    )
        .fetch_all(pool.inner())
        .await
        .map_err(|e| format!("Failed to fetch counseling sessions: {}", e))?;

    let academic_records = sqlx::query_as::<_, AcademicRecordExport>(
        "SELECT id, studentId, term, subject, examType, grade, date, created_at 
         FROM academic_records"
    )
        .fetch_all(pool.inner())
        .await
        .map_err(|e| format!("Failed to fetch academic records: {}", e))?;

    let surveys = sqlx::query_as::<_, SurveyExport>(
        "SELECT id, title, description, type, status, created_at FROM surveys"
    )
        .fetch_all(pool.inner())
        .await
        .map_err(|e| format!("Failed to fetch surveys: {}", e))?;

    let export = DatabaseExport {
        version: "2.0.0".to_string(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        students,
        users,
        counseling_sessions,
        academic_records,
        surveys,
    };

    serde_json::to_string_pretty(&export)
        .map_err(|e| format!("Failed to serialize data: {}", e))
}

#[tauri::command]
pub async fn import_database_json(
    pool: State<'_, SqlitePool>,
    json_data: String,
) -> Result<ImportResult, String> {
    let import_data: DatabaseExport = serde_json::from_str(&json_data)
        .map_err(|e| format!("Invalid JSON format: {}", e))?;

    if import_data.version != "2.0.0" {
        return Err(format!(
            "Unsupported export version: {}. Expected 2.0.0",
            import_data.version
        ));
    }

    let mut tx = pool.begin()
        .await
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    let mut students_imported = 0;
    let mut users_imported = 0;
    let mut sessions_imported = 0;
    let mut records_imported = 0;

    for student in import_data.students {
        let result = sqlx::query(
            "INSERT OR REPLACE INTO students 
             (id, name, surname, email, phone, birthDate, address, class, enrollmentDate, 
              status, avatar, parentContact, notes, gender, risk, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&student.id)
        .bind(&student.name)
        .bind(&student.surname)
        .bind(&student.email)
        .bind(&student.phone)
        .bind(&student.birthDate)
        .bind(&student.address)
        .bind(&student.class)
        .bind(&student.enrollmentDate)
        .bind(&student.status)
        .bind(&student.avatar)
        .bind(&student.parentContact)
        .bind(&student.notes)
        .bind(&student.gender)
        .bind(&student.risk)
        .bind(&student.created_at)
        .bind(&student.updated_at)
        .execute(&mut *tx)
        .await;

        if result.is_ok() {
            students_imported += 1;
        }
    }

    for user in import_data.users {
        let result = sqlx::query(
            "INSERT OR IGNORE INTO users 
             (id, name, email, passwordHash, role, institution, isActive, created_at, updated_at)
             VALUES (?, ?, ?, '', ?, ?, ?, ?, ?)"
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.role)
        .bind(&user.institution)
        .bind(user.isActive)
        .bind(&user.created_at)
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&mut *tx)
        .await;

        if result.is_ok() {
            users_imported += 1;
        }
    }

    for session in import_data.counseling_sessions {
        let result = sqlx::query(
            "INSERT OR REPLACE INTO counseling_sessions 
             (id, studentId, counselorId, date, duration, type, topic, notes, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&session.id)
        .bind(&session.studentId)
        .bind(&session.counselorId)
        .bind(&session.date)
        .bind(session.duration)
        .bind(&session.session_type)
        .bind(&session.topic)
        .bind(&session.notes)
        .bind(&session.created_at)
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&mut *tx)
        .await;

        if result.is_ok() {
            sessions_imported += 1;
        }
    }

    for record in import_data.academic_records {
        let result = sqlx::query(
            "INSERT OR REPLACE INTO academic_records 
             (id, studentId, term, subject, examType, grade, date, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&record.id)
        .bind(&record.studentId)
        .bind(&record.term)
        .bind(&record.subject)
        .bind(&record.examType)
        .bind(record.grade)
        .bind(&record.date)
        .bind(&record.created_at)
        .execute(&mut *tx)
        .await;

        if result.is_ok() {
            records_imported += 1;
        }
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(ImportResult {
        success: true,
        message: format!(
            "Import completed successfully. {} students, {} users, {} sessions, {} academic records imported",
            students_imported, users_imported, sessions_imported, records_imported
        ),
        students_imported,
        users_imported,
        sessions_imported,
        records_imported,
    })
}

#[derive(Debug, Serialize)]
pub struct StudentExportRow {
    pub id: String,
    pub ad: String,
    pub soyad: String,
    pub email: String,
    pub telefon: String,
    pub dogum_tarihi: String,
    pub sinif: String,
    pub kayit_tarihi: String,
    pub durum: String,
    pub risk_seviyesi: String,
    pub cinsiyet: String,
}

#[tauri::command]
pub async fn export_students_csv(
    pool: State<'_, SqlitePool>,
) -> Result<String, String> {
    let students = sqlx::query_as::<_, Student>("SELECT * FROM students ORDER BY class, surname, name")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| format!("Failed to fetch students: {}", e))?;

    let mut csv = String::from("ID,Ad,Soyad,E-posta,Telefon,Doğum Tarihi,Sınıf,Kayıt Tarihi,Durum,Risk Seviyesi,Cinsiyet\n");

    for student in students {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{}\n",
            student.id,
            student.name,
            student.surname,
            student.email.unwrap_or_default(),
            student.phone.unwrap_or_default(),
            student.birthDate.unwrap_or_default(),
            student.class.unwrap_or_default(),
            student.enrollmentDate,
            student.status,
            student.risk,
            student.gender
        ));
    }

    Ok(csv)
}

#[tauri::command]
pub async fn get_export_statistics(
    pool: State<'_, SqlitePool>,
) -> Result<ExportStatistics, String> {
    let student_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM students")
        .fetch_one(pool.inner())
        .await
        .map_err(|e| format!("Failed to count students: {}", e))?;

    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool.inner())
        .await
        .map_err(|e| format!("Failed to count users: {}", e))?;

    let session_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM counseling_sessions")
        .fetch_one(pool.inner())
        .await
        .map_err(|e| format!("Failed to count sessions: {}", e))?;

    let record_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM academic_records")
        .fetch_one(pool.inner())
        .await
        .map_err(|e| format!("Failed to count records: {}", e))?;

    Ok(ExportStatistics {
        total_students: student_count.0 as usize,
        total_users: user_count.0 as usize,
        total_sessions: session_count.0 as usize,
        total_academic_records: record_count.0 as usize,
    })
}

#[derive(Debug, Serialize)]
pub struct ExportStatistics {
    pub total_students: usize,
    pub total_users: usize,
    pub total_sessions: usize,
    pub total_academic_records: usize,
}
