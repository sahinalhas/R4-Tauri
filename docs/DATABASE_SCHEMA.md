# Rehber360 Database Schema

## 📋 İçindekiler

1. [Genel Bakış](#genel-bakış)
2. [Schema Diagram](#schema-diagram)
3. [Tablolar](#tablolar)
4. [Migrations](#migrations)
5. [Indexes](#indexes)
6. [Relations](#relations)

---

## Genel Bakış

Rehber360, **SQLite** veritabanı kullanır. Schema management **SQLx migrations** ile yapılır.

### Temel Bilgiler

- **Database:** SQLite 3
- **Location:** `{app_data_dir}/rehber360.db`
- **Driver:** SQLx 0.8
- **Migrations:** 8 adet migration dosyası
- **Tables:** 12+ tablo

### Migration Sistemi

```bash
# Migrations dizini
src-tauri/core/migrations/
├── 001_create_users.sql
├── 002_create_students.sql
├── 003_create_academic.sql
├── 004_create_ai_suggestions.sql
├── 005_create_surveys.sql
├── 006_create_counseling.sql
├── 007_create_notifications.sql
└── 008_create_settings.sql
```

---

## Schema Diagram

```
┌─────────────┐
│    users    │
└─────┬───────┘
      │
      │ (counselorId)
      │
      ▼
┌─────────────────┐        ┌──────────────────┐
│   students      │◄──────│ student_documents│
└────┬────────────┘        └──────────────────┘
     │
     ├─────────►┌─────────────────────┐
     │          │ counseling_sessions │
     │          └─────────────────────┘
     │
     ├─────────►┌─────────────────┐
     │          │ academic_records│
     │          └─────────────────┘
     │
     ├─────────►┌──────────────────┐
     │          │ attendance_records│
     │          └──────────────────┘
     │
     ├─────────►┌─────────────────┐
     │          │ behavior_records│
     │          └─────────────────┘
     │
     └─────────►┌─────────────────┐
                │ ai_suggestions  │
                └─────────────────┘

┌─────────────┐        ┌──────────────────┐
│  surveys    │◄──────│ survey_responses │
└─────────────┘        └──────────────────┘

┌──────────────┐
│notifications │
└──────────────┘

┌──────────────┐
│  settings    │
└──────────────┘
```

---

## Tablolar

### 1. `users`

Kullanıcı hesapları (rehber öğretmenler, adminler).

```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    passwordHash TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('admin', 'counselor', 'teacher', 'observer')),
    institution TEXT NOT NULL,
    isActive BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**Indexes:**
- `idx_users_email` ON `email`
- `idx_users_role` ON `role`
- `idx_users_isActive` ON `isActive`

**Örnek:**
```sql
INSERT INTO users (id, name, email, passwordHash, role, institution)
VALUES ('uuid-1', 'Ahmet Yılmaz', 'ahmet@okul.edu.tr', '$2b$...', 'counselor', 'XYZ Lisesi');
```

---

### 2. `user_sessions`

Kullanıcı oturum yönetimi.

```sql
CREATE TABLE user_sessions (
    id TEXT PRIMARY KEY,
    userId TEXT NOT NULL,
    token TEXT NOT NULL UNIQUE,
    expiresAt TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (userId) REFERENCES users (id) ON DELETE CASCADE
);
```

**Indexes:**
- `idx_user_sessions_token` ON `token`
- `idx_user_sessions_userId` ON `userId`

---

### 3. `students`

Öğrenci kayıtları.

```sql
CREATE TABLE students (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    surname TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    birthDate TEXT,
    address TEXT,
    class TEXT,
    enrollmentDate TEXT NOT NULL,
    status TEXT DEFAULT 'active',
    avatar TEXT,
    parentContact TEXT,
    notes TEXT,
    gender TEXT CHECK (gender IN ('K', 'E')) DEFAULT 'K',
    risk TEXT CHECK (risk IN ('Düşük', 'Orta', 'Yüksek')) DEFAULT 'Düşük',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**Indexes:**
- `idx_students_class` ON `class`
- `idx_students_status` ON `status`
- `idx_students_risk` ON `risk`
- `idx_students_name_surname` ON `name, surname`

**Örnek:**
```sql
INSERT INTO students (id, name, surname, enrollmentDate, gender, class, risk)
VALUES ('uuid-1', 'Ahmet', 'Yılmaz', '2024-09-01', 'E', '9-A', 'Düşük');
```

---

### 4. `student_documents`

Öğrenci belgeleri (PDF, Word vb.).

```sql
CREATE TABLE student_documents (
    id TEXT PRIMARY KEY,
    studentId TEXT NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    dataUrl TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (studentId) REFERENCES students (id) ON DELETE CASCADE
);
```

**Indexes:**
- `idx_student_documents_studentId` ON `studentId`

---

### 5. `counseling_sessions`

Rehberlik görüşme kayıtları.

```sql
CREATE TABLE counseling_sessions (
    id TEXT PRIMARY KEY,
    studentId TEXT NOT NULL,
    counselorId TEXT NOT NULL,
    date TEXT NOT NULL,
    duration INTEGER,
    type TEXT NOT NULL,
    topic TEXT,
    notes TEXT,
    nextFollowUp TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (studentId) REFERENCES students (id) ON DELETE CASCADE,
    FOREIGN KEY (counselorId) REFERENCES users (id) ON DELETE CASCADE
);
```

**Indexes:**
- `idx_counseling_sessions_studentId` ON `studentId`
- `idx_counseling_sessions_counselorId` ON `counselorId`
- `idx_counseling_sessions_date` ON `date`

**Type values:**
- `individual` - Bireysel görüşme
- `group` - Grup görüşmesi
- `family` - Aile görüşmesi
- `crisis` - Kriz müdahalesi

---

### 6. `academic_records`

Akademik başarı kayıtları (sınavlar, notlar).

```sql
CREATE TABLE academic_records (
    id TEXT PRIMARY KEY,
    studentId TEXT NOT NULL,
    term TEXT NOT NULL,
    subject TEXT NOT NULL,
    examType TEXT NOT NULL,
    grade REAL NOT NULL,
    maxGrade REAL DEFAULT 100,
    date TEXT NOT NULL,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (studentId) REFERENCES students (id) ON DELETE CASCADE
);
```

**Indexes:**
- `idx_academic_records_studentId` ON `studentId`
- `idx_academic_records_term` ON `term`
- `idx_academic_records_subject` ON `subject`

**ExamType values:**
- `midterm` - Ara sınav
- `final` - Final
- `quiz` - Kısa sınav
- `project` - Proje

---

### 7. `attendance_records`

Devamsızlık kayıtları.

```sql
CREATE TABLE attendance_records (
    id TEXT PRIMARY KEY,
    studentId TEXT NOT NULL,
    date TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('present', 'absent', 'late', 'excused')),
    reason TEXT,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (studentId) REFERENCES students (id) ON DELETE CASCADE
);
```

**Indexes:**
- `idx_attendance_records_studentId` ON `studentId`
- `idx_attendance_records_date` ON `date`
- `idx_attendance_records_status` ON `status`

---

### 8. `behavior_records`

Davranış kayıtları (olumlu/olumsuz).

```sql
CREATE TABLE behavior_records (
    id TEXT PRIMARY KEY,
    studentId TEXT NOT NULL,
    date TEXT NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('positive', 'negative')),
    category TEXT NOT NULL,
    description TEXT NOT NULL,
    severity TEXT CHECK(severity IN ('low', 'medium', 'high')),
    actionTaken TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (studentId) REFERENCES students (id) ON DELETE CASCADE
);
```

**Indexes:**
- `idx_behavior_records_studentId` ON `studentId`
- `idx_behavior_records_type` ON `type`
- `idx_behavior_records_date` ON `date`

**Category values:**
- `academic` - Akademik
- `social` - Sosyal
- `emotional` - Duygusal
- `behavioral` - Davranışsal

---

### 9. `ai_suggestions`

AI tarafından üretilen öneriler.

```sql
CREATE TABLE ai_suggestions (
    id TEXT PRIMARY KEY,
    studentId TEXT NOT NULL,
    type TEXT NOT NULL,
    content TEXT NOT NULL,
    confidence REAL NOT NULL,
    status TEXT DEFAULT 'pending' CHECK(status IN ('pending', 'approved', 'rejected')),
    reviewedBy TEXT,
    reviewedAt TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (studentId) REFERENCES students (id) ON DELETE CASCADE,
    FOREIGN KEY (reviewedBy) REFERENCES users (id)
);
```

**Indexes:**
- `idx_ai_suggestions_studentId` ON `studentId`
- `idx_ai_suggestions_status` ON `status`
- `idx_ai_suggestions_type` ON `type`

**Type values:**
- `profile_update` - Profil güncelleme önerisi
- `intervention` - Müdahale önerisi
- `goal` - Hedef önerisi
- `risk_alert` - Risk uyarısı

---

### 10. `surveys`

Anket şablonları.

```sql
CREATE TABLE surveys (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    type TEXT NOT NULL,
    questions TEXT NOT NULL, -- JSON
    targetAudience TEXT NOT NULL,
    status TEXT DEFAULT 'draft' CHECK(status IN ('draft', 'active', 'closed')),
    createdBy TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (createdBy) REFERENCES users (id)
);
```

---

### 11. `survey_responses`

Anket cevapları.

```sql
CREATE TABLE survey_responses (
    id TEXT PRIMARY KEY,
    surveyId TEXT NOT NULL,
    studentId TEXT NOT NULL,
    responses TEXT NOT NULL, -- JSON
    completedAt TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (surveyId) REFERENCES surveys (id) ON DELETE CASCADE,
    FOREIGN KEY (studentId) REFERENCES students (id) ON DELETE CASCADE,
    UNIQUE(surveyId, studentId)
);
```

**Indexes:**
- `idx_survey_responses_surveyId` ON `surveyId`
- `idx_survey_responses_studentId` ON `studentId`

---

### 12. `notifications`

Sistem bildirimleri.

```sql
CREATE TABLE notifications (
    id TEXT PRIMARY KEY,
    userId TEXT NOT NULL,
    type TEXT NOT NULL,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    priority TEXT DEFAULT 'normal' CHECK(priority IN ('low', 'normal', 'high', 'urgent')),
    isRead BOOLEAN DEFAULT FALSE,
    actionUrl TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (userId) REFERENCES users (id) ON DELETE CASCADE
);
```

**Indexes:**
- `idx_notifications_userId` ON `userId`
- `idx_notifications_isRead` ON `isRead`
- `idx_notifications_priority` ON `priority`

---

### 13. `settings`

Uygulama ayarları (key-value store).

```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('string', 'number', 'boolean', 'json')),
    description TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**Örnek:**
```sql
INSERT INTO settings (key, value, type, description)
VALUES 
    ('ai_provider', 'openai', 'string', 'Active AI provider'),
    ('theme', 'light', 'string', 'Application theme'),
    ('notificationsEnabled', 'true', 'boolean', 'Enable notifications');
```

---

## Migrations

### Migration Dosyaları

Migrations SQLx CLI ile otomatik uygulanır:

```bash
# Migration çalıştır
cd src-tauri/core
sqlx migrate run --database-url sqlite:../../database.db

# Yeni migration oluştur
sqlx migrate add create_new_table
```

### Migration History

SQLx otomatik olarak `_sqlx_migrations` tablosu oluşturur:

```sql
CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
```

---

## Indexes

### Performance Indexes

Tüm foreign key'ler üzerinde index var:

```sql
-- Users
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);

-- Students
CREATE INDEX idx_students_class ON students(class);
CREATE INDEX idx_students_risk ON students(risk);

-- Counseling
CREATE INDEX idx_counseling_sessions_studentId ON counseling_sessions(studentId);
CREATE INDEX idx_counseling_sessions_date ON counseling_sessions(date);

-- Academic
CREATE INDEX idx_academic_records_studentId ON academic_records(studentId);
CREATE INDEX idx_academic_records_term ON academic_records(term);
```

---

## Relations

### Foreign Key Constraints

Tüm ilişkiler `ON DELETE CASCADE` ile tanımlanmış:

```sql
-- Öğrenci silindiğinde tüm related data silinir
FOREIGN KEY (studentId) REFERENCES students (id) ON DELETE CASCADE
```

### Relationship Types

1. **One-to-Many:**
   - `users` → `counseling_sessions`
   - `students` → `academic_records`
   - `students` → `attendance_records`
   - `students` → `behavior_records`
   - `students` → `ai_suggestions`

2. **Many-to-Many:**
   - `students` ↔ `surveys` (via `survey_responses`)

---

## Queries Örnekleri

### 1. Öğrenci ile tüm görüşmelerini getir

```sql
SELECT 
    s.*,
    COUNT(cs.id) as session_count
FROM students s
LEFT JOIN counseling_sessions cs ON s.id = cs.studentId
WHERE s.id = ?
GROUP BY s.id;
```

### 2. Yüksek riskli öğrencileri listele

```sql
SELECT * FROM students
WHERE risk = 'Yüksek'
ORDER BY updated_at DESC;
```

### 3. Son 30 günde devamsızlık sayısı

```sql
SELECT 
    studentId,
    COUNT(*) as absence_count
FROM attendance_records
WHERE status = 'absent'
    AND date >= date('now', '-30 days')
GROUP BY studentId
HAVING absence_count > 5;
```

### 4. AI önerilerini onay bekleyenler

```sql
SELECT 
    a.id,
    a.content,
    s.name,
    s.surname
FROM ai_suggestions a
JOIN students s ON a.studentId = s.id
WHERE a.status = 'pending'
ORDER BY a.confidence DESC;
```

---

## Backup ve Restore

### Database Backup

```rust
// Rust code
use std::fs;
use std::path::PathBuf;

pub fn backup_database(db_path: PathBuf) -> Result<PathBuf> {
    let backup_dir = db_path.parent().unwrap().join("backups");
    fs::create_dir_all(&backup_dir)?;
    
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_path = backup_dir.join(format!("rehber360_{}.db", timestamp));
    
    fs::copy(&db_path, &backup_path)?;
    Ok(backup_path)
}
```

---

## Optimizations

### 1. WAL Mode

```sql
PRAGMA journal_mode=WAL;
```

**Avantajları:**
- Concurrent reads
- Better performance
- Crash resistance

### 2. Foreign Keys

```sql
PRAGMA foreign_keys=ON;
```

### 3. Vacuum

```sql
-- Periyodik olarak çalıştır
VACUUM;
ANALYZE;
```

---

## Gelecek İyileştirmeler

1. **Full-Text Search:**
```sql
CREATE VIRTUAL TABLE students_fts USING fts5(name, surname, notes);
```

2. **Triggers:**
```sql
CREATE TRIGGER update_student_timestamp
AFTER UPDATE ON students
BEGIN
    UPDATE students SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
```

3. **Views:**
```sql
CREATE VIEW student_summary AS
SELECT 
    s.id,
    s.name,
    s.surname,
    COUNT(DISTINCT cs.id) as sessions,
    COUNT(DISTINCT ar.id) as absences,
    AVG(acr.grade) as avg_grade
FROM students s
LEFT JOIN counseling_sessions cs ON s.id = cs.studentId
LEFT JOIN attendance_records ar ON s.id = ar.studentId AND ar.status = 'absent'
LEFT JOIN academic_records acr ON s.id = acr.studentId
GROUP BY s.id;
```

---

**Son Güncelleme:** 31 Ekim 2025  
**Schema Version:** 2.0.0  
**Migrations:** 8 dosya
