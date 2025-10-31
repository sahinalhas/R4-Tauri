#[cfg(test)]
mod integration_tests {
    use sqlx::SqlitePool;
    use rehber360_core::database;
    use std::path::PathBuf;

    async fn setup_test_db() -> SqlitePool {
        let temp_dir = std::env::temp_dir();
        let test_db = temp_dir.join(format!("test_rehber360_{}.db", uuid::Uuid::new_v4()));
        
        if test_db.exists() {
            std::fs::remove_file(&test_db).ok();
        }
        
        database::initialize_database(test_db)
            .await
            .expect("Failed to initialize test database")
    }

    #[tokio::test]
    async fn test_database_initialization() {
        let pool = setup_test_db().await;
        
        let result: Result<(i64,), sqlx::Error> = 
            sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table'")
                .fetch_one(&pool)
                .await;
        
        assert!(result.is_ok());
        let count = result.unwrap().0;
        assert!(count > 0, "Database should have tables after initialization");
    }

    #[tokio::test]
    async fn test_create_and_fetch_student() {
        let pool = setup_test_db().await;
        
        let student_id = uuid::Uuid::new_v4().to_string();
        
        let insert_result = sqlx::query(
            "INSERT INTO students (id, name, surname, enrollmentDate, gender, risk, status) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&student_id)
        .bind("Test")
        .bind("Student")
        .bind("2025-01-01")
        .bind("E")
        .bind("Düşük")
        .bind("active")
        .execute(&pool)
        .await;
        
        assert!(insert_result.is_ok());
        
        let fetch_result: Result<(String, String), sqlx::Error> = 
            sqlx::query_as("SELECT name, surname FROM students WHERE id = ?")
                .bind(&student_id)
                .fetch_one(&pool)
                .await;
        
        assert!(fetch_result.is_ok());
        let (name, surname) = fetch_result.unwrap();
        assert_eq!(name, "Test");
        assert_eq!(surname, "Student");
    }

    #[tokio::test]
    async fn test_foreign_key_cascade_delete() {
        let pool = setup_test_db().await;
        
        let student_id = uuid::Uuid::new_v4().to_string();
        let document_id = uuid::Uuid::new_v4().to_string();
        
        sqlx::query(
            "INSERT INTO students (id, name, surname, enrollmentDate, gender, risk, status) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&student_id)
        .bind("Test")
        .bind("Student")
        .bind("2025-01-01")
        .bind("E")
        .bind("Düşük")
        .bind("active")
        .execute(&pool)
        .await
        .expect("Failed to insert student");
        
        sqlx::query(
            "INSERT INTO student_documents (id, studentId, name, type, dataUrl) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&document_id)
        .bind(&student_id)
        .bind("Test Document")
        .bind("pdf")
        .bind("data:...")
        .execute(&pool)
        .await
        .expect("Failed to insert document");
        
        sqlx::query("DELETE FROM students WHERE id = ?")
            .bind(&student_id)
            .execute(&pool)
            .await
            .expect("Failed to delete student");
        
        let doc_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM student_documents WHERE studentId = ?"
        )
        .bind(&student_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to count documents");
        
        assert_eq!(doc_count.0, 0, "Documents should be cascade deleted");
    }

    #[tokio::test]
    async fn test_settings_crud() {
        let pool = setup_test_db().await;
        
        sqlx::query(
            "INSERT INTO settings (key, value, type, description) 
             VALUES (?, ?, ?, ?)"
        )
        .bind("test_setting")
        .bind("test_value")
        .bind("string")
        .bind("Test setting description")
        .execute(&pool)
        .await
        .expect("Failed to insert setting");
        
        let (value,): (String,) = sqlx::query_as(
            "SELECT value FROM settings WHERE key = ?"
        )
        .bind("test_setting")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch setting");
        
        assert_eq!(value, "test_value");
        
        sqlx::query("UPDATE settings SET value = ? WHERE key = ?")
            .bind("updated_value")
            .bind("test_setting")
            .execute(&pool)
            .await
            .expect("Failed to update setting");
        
        let (updated_value,): (String,) = sqlx::query_as(
            "SELECT value FROM settings WHERE key = ?"
        )
        .bind("test_setting")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch updated setting");
        
        assert_eq!(updated_value, "updated_value");
    }
}
