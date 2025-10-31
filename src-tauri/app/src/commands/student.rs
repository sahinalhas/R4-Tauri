use rehber360_core::models::{Student, CreateStudentRequest, UpdateStudentRequest};
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn get_all_students(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Student>, String> {
    let students = sqlx::query_as::<_, Student>(
        "SELECT * FROM students ORDER BY created_at DESC"
    )
        .fetch_all(pool.inner())
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(students)
}

#[tauri::command]
pub async fn get_student(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<Student, String> {
    let student = sqlx::query_as::<_, Student>("SELECT * FROM students WHERE id = ?")
        .bind(&id)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Student not found".to_string())?;
    
    Ok(student)
}

#[tauri::command]
pub async fn create_student(
    pool: State<'_, SqlitePool>,
    request: CreateStudentRequest,
) -> Result<Student, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let gender = request.gender.unwrap_or_else(|| "K".to_string());
    
    sqlx::query(
        "INSERT INTO students (id, name, surname, email, phone, birthDate, address, class, enrollmentDate, parentContact, notes, gender, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
        .bind(&id)
        .bind(&request.name)
        .bind(&request.surname)
        .bind(&request.email)
        .bind(&request.phone)
        .bind(&request.birthDate)
        .bind(&request.address)
        .bind(&request.class)
        .bind(&request.enrollmentDate)
        .bind(&request.parentContact)
        .bind(&request.notes)
        .bind(&gender)
        .bind(&now)
        .bind(&now)
        .execute(pool.inner())
        .await
        .map_err(|e| format!("Failed to create student: {}", e))?;
    
    get_student(pool, id).await
}

#[tauri::command]
pub async fn update_student(
    pool: State<'_, SqlitePool>,
    id: String,
    request: UpdateStudentRequest,
) -> Result<Student, String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    // Build dynamic update query
    let mut query_parts = vec!["updated_at = ?"];
    let mut has_updates = false;
    
    if request.name.is_some() { query_parts.push("name = ?"); has_updates = true; }
    if request.surname.is_some() { query_parts.push("surname = ?"); has_updates = true; }
    if request.email.is_some() { query_parts.push("email = ?"); has_updates = true; }
    if request.phone.is_some() { query_parts.push("phone = ?"); has_updates = true; }
    if request.birthDate.is_some() { query_parts.push("birthDate = ?"); has_updates = true; }
    if request.address.is_some() { query_parts.push("address = ?"); has_updates = true; }
    if request.class.is_some() { query_parts.push("class = ?"); has_updates = true; }
    if request.parentContact.is_some() { query_parts.push("parentContact = ?"); has_updates = true; }
    if request.notes.is_some() { query_parts.push("notes = ?"); has_updates = true; }
    if request.gender.is_some() { query_parts.push("gender = ?"); has_updates = true; }
    if request.risk.is_some() { query_parts.push("risk = ?"); has_updates = true; }
    
    if !has_updates {
        return get_student(pool, id).await;
    }
    
    let query_str = format!("UPDATE students SET {} WHERE id = ?", query_parts.join(", "));
    let mut query = sqlx::query(&query_str).bind(&now);
    
    if let Some(v) = &request.name { query = query.bind(v); }
    if let Some(v) = &request.surname { query = query.bind(v); }
    if let Some(v) = &request.email { query = query.bind(v); }
    if let Some(v) = &request.phone { query = query.bind(v); }
    if let Some(v) = &request.birthDate { query = query.bind(v); }
    if let Some(v) = &request.address { query = query.bind(v); }
    if let Some(v) = &request.class { query = query.bind(v); }
    if let Some(v) = &request.parentContact { query = query.bind(v); }
    if let Some(v) = &request.notes { query = query.bind(v); }
    if let Some(v) = &request.gender { query = query.bind(v); }
    if let Some(v) = &request.risk { query = query.bind(v); }
    
    query = query.bind(&id);
    
    query.execute(pool.inner())
        .await
        .map_err(|e| format!("Failed to update student: {}", e))?;
    
    get_student(pool, id).await
}

#[tauri::command]
pub async fn delete_student(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM students WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| format!("Failed to delete student: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn search_students(
    pool: State<'_, SqlitePool>,
    query: String,
) -> Result<Vec<Student>, String> {
    let search_pattern = format!("%{}%", query);
    
    let students = sqlx::query_as::<_, Student>(
        "SELECT * FROM students WHERE name LIKE ? OR surname LIKE ? OR email LIKE ? ORDER BY created_at DESC"
    )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(pool.inner())
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(students)
}
