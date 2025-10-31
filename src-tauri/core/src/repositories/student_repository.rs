use crate::models::{Student, CreateStudentRequest, UpdateStudentRequest};
use crate::error::{AppError, AppResult};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct StudentRepository;

impl StudentRepository {
    pub async fn create(pool: &SqlitePool, req: CreateStudentRequest) -> AppResult<Student> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let status = "active".to_string();
        let risk = "YOK".to_string();
        let gender = req.gender.unwrap_or_else(|| "Belirtilmemiş".to_string());

        let student = sqlx::query_as::<_, Student>(
            r#"
            INSERT INTO students (
                id, name, surname, email, phone, birthDate, address, class,
                enrollmentDate, status, gender, risk, parentContact, notes, created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#
        )
        .bind(&id)
        .bind(&req.name)
        .bind(&req.surname)
        .bind(&req.email)
        .bind(&req.phone)
        .bind(&req.birthDate)
        .bind(&req.address)
        .bind(&req.class)
        .bind(&req.enrollmentDate)
        .bind(&status)
        .bind(&gender)
        .bind(&risk)
        .bind(&req.parentContact)
        .bind(&req.notes)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(student)
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> AppResult<Student> {
        let student = sqlx::query_as::<_, Student>("SELECT * FROM students WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::StudentNotFound(id.to_string()))?;

        Ok(student)
    }

    pub async fn get_all(pool: &SqlitePool) -> AppResult<Vec<Student>> {
        let students = sqlx::query_as::<_, Student>(
            "SELECT * FROM students ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(students)
    }

    pub async fn update(pool: &SqlitePool, id: &str, req: UpdateStudentRequest) -> AppResult<Student> {
        let now = Utc::now().to_rfc3339();

        let mut query = String::from("UPDATE students SET updated_at = ?");
        let mut params: Vec<String> = vec![now.clone()];

        if let Some(name) = &req.name {
            query.push_str(", name = ?");
            params.push(name.clone());
        }
        if let Some(surname) = &req.surname {
            query.push_str(", surname = ?");
            params.push(surname.clone());
        }
        if let Some(email) = &req.email {
            query.push_str(", email = ?");
            params.push(email.clone());
        }
        if let Some(phone) = &req.phone {
            query.push_str(", phone = ?");
            params.push(phone.clone());
        }
        if let Some(birth_date) = &req.birthDate {
            query.push_str(", birthDate = ?");
            params.push(birth_date.clone());
        }
        if let Some(address) = &req.address {
            query.push_str(", address = ?");
            params.push(address.clone());
        }
        if let Some(class) = &req.class {
            query.push_str(", class = ?");
            params.push(class.clone());
        }
        if let Some(parent_contact) = &req.parentContact {
            query.push_str(", parentContact = ?");
            params.push(parent_contact.clone());
        }
        if let Some(notes) = &req.notes {
            query.push_str(", notes = ?");
            params.push(notes.clone());
        }
        if let Some(gender) = &req.gender {
            query.push_str(", gender = ?");
            params.push(gender.clone());
        }
        if let Some(risk) = &req.risk {
            query.push_str(", risk = ?");
            params.push(risk.clone());
        }

        query.push_str(" WHERE id = ?");

        let mut q = sqlx::query(&query).bind(&now);
        for param in params {
            q = q.bind(param);
        }
        q = q.bind(id);

        q.execute(pool).await?;

        Self::get_by_id(pool, id).await
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM students WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::StudentNotFound(id.to_string()));
        }

        Ok(())
    }

    pub async fn search(pool: &SqlitePool, query: &str) -> AppResult<Vec<Student>> {
        let search_pattern = format!("%{}%", query);
        
        let students = sqlx::query_as::<_, Student>(
            r#"
            SELECT * FROM students 
            WHERE name LIKE ? OR surname LIKE ? OR email LIKE ? OR class LIKE ?
            ORDER BY created_at DESC
            "#
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;

        Ok(students)
    }

    pub async fn get_by_class(pool: &SqlitePool, class: &str) -> AppResult<Vec<Student>> {
        let students = sqlx::query_as::<_, Student>(
            "SELECT * FROM students WHERE class = ? ORDER BY surname, name"
        )
        .bind(class)
        .fetch_all(pool)
        .await?;

        Ok(students)
    }

    pub async fn get_by_risk_level(pool: &SqlitePool, risk: &str) -> AppResult<Vec<Student>> {
        let students = sqlx::query_as::<_, Student>(
            "SELECT * FROM students WHERE risk = ? ORDER BY surname, name"
        )
        .bind(risk)
        .fetch_all(pool)
        .await?;

        Ok(students)
    }

    pub async fn count(pool: &SqlitePool) -> AppResult<i64> {
        let result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM students")
            .fetch_one(pool)
            .await?;

        Ok(result)
    }
}
