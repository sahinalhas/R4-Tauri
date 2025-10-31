use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthDate: Option<String>,
    pub address: Option<String>,
    pub class: Option<String>,
    pub enrollmentDate: String,
    pub status: String,
    pub avatar: Option<String>,
    pub parentContact: Option<String>,
    pub notes: Option<String>,
    pub gender: String,
    pub risk: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateStudentRequest {
    pub name: String,
    pub surname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthDate: Option<String>,
    pub address: Option<String>,
    pub class: Option<String>,
    pub enrollmentDate: String,
    pub parentContact: Option<String>,
    pub notes: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateStudentRequest {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthDate: Option<String>,
    pub address: Option<String>,
    pub class: Option<String>,
    pub parentContact: Option<String>,
    pub notes: Option<String>,
    pub gender: Option<String>,
    pub risk: Option<String>,
}
