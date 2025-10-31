# Rehber360 Tauri - Development Guide

## 📋 İçindekiler
1. [Proje Yapısı](#proje-yapısı)
2. [Geliştirme Ortamı Kurulumu](#geliştirme-ortamı-kurulumu)
3. [Mimari Overview](#mimari-overview)
4. [Tauri Commands](#tauri-commands)
5. [Database Schema](#database-schema)
6. [AI Services](#ai-services)
7. [Build & Deploy](#build--deploy)
8. [Testing](#testing)

---

## 🏗️ Proje Yapısı

```
rehber360/
├── src-tauri/                  # Rust backend (Tauri)
│   ├── core/                   # Core business logic library
│   │   ├── src/
│   │   │   ├── database/       # Database connection & migrations
│   │   │   ├── models/         # Data models (9 models)
│   │   │   ├── repositories/   # Repository pattern (8 repositories)
│   │   │   ├── services/       # Business logic services
│   │   │   │   ├── ai_service.rs    # AI providers (OpenAI, Gemini, Ollama)
│   │   │   │   └── config_service.rs # Settings & config management
│   │   │   ├── error.rs        # Custom error types
│   │   │   └── lib.rs
│   │   ├── migrations/         # SQL migrations (8 files)
│   │   └── Cargo.toml
│   ├── app/                    # Tauri desktop application
│   │   ├── src/
│   │   │   ├── commands/       # Tauri commands (9 modules, 85+ commands)
│   │   │   │   ├── auth.rs
│   │   │   │   ├── student.rs
│   │   │   │   ├── counseling.rs
│   │   │   │   ├── academic.rs
│   │   │   │   ├── ai.rs
│   │   │   │   ├── survey.rs
│   │   │   │   ├── notification.rs
│   │   │   │   ├── settings.rs
│   │   │   │   └── file.rs
│   │   │   └── main.rs
│   │   ├── tauri.conf.json     # Tauri configuration
│   │   └── Cargo.toml
│   └── Cargo.toml              # Workspace root
├── client/                     # React frontend
│   └── src/
│       └── services/
│           └── tauri-api.ts    # Type-safe Tauri API client
├── package.json
└── plan.md
```

---

## 🚀 Geliştirme Ortamı Kurulumu

### Gereksinimler

**Sistem:**
- Rust 1.70+ (`rustup`)
- Node.js 20+
- SQLite 3
- (Windows için) Visual Studio Build Tools

**Optional:**
- OpenAI API Key
- Google Gemini API Key
- Ollama (local AI)

### Kurulum Adımları

#### 1. Rust Kurulumu
```bash
# Rustup installer (Windows/Mac/Linux)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Rust version check
rustc --version
cargo --version
```

#### 2. Node Dependencies
```bash
npm install
```

#### 3. Tauri CLI (Optional, workspace kullanıyoruz)
```bash
cargo install tauri-cli
```

#### 4. Environment Setup
```bash
# .env dosyası oluştur (optional, API keys için)
cp .env.example .env

# API keys ekle:
# OPENAI_API_KEY=sk-...
# GEMINI_API_KEY=...
# OLLAMA_URL=http://localhost:11434
```

---

## 🏛️ Mimari Overview

### Workspace Yapısı

Proje **workspace** olarak yapılandırılmış:
- `core`: GUI-independent business logic (library crate)
- `app`: Tauri desktop application (binary crate)

**Avantajlar:**
- Core logic Replit'te test edilebilir (GUI bağımsız)
- App crate local'de build edilir (Tauri GUI gereksinimi)
- Modüler ve test edilebilir kod

### Katmanlar

```
┌─────────────────────────────────┐
│     Frontend (React + TS)       │
│  - Tauri API Client (invoke)    │
└────────────┬────────────────────┘
             │ IPC (invoke)
┌────────────▼────────────────────┐
│   Tauri Commands Layer (app)    │
│  - 85+ commands                  │
│  - Request/Response DTOs         │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│   Services Layer (core)          │
│  - AI Service (OpenAI/Gemini)    │
│  - Config Service                │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│   Repository Layer (core)        │
│  - 8 repositories                │
│  - CRUD operations               │
└────────────┬────────────────────┘
             │
┌────────────▼────────────────────┐
│   Database (SQLite + SQLx)       │
│  - 8 migrations                  │
│  - Type-safe queries             │
└─────────────────────────────────┘
```

---

## 📡 Tauri Commands

### Authentication
- `login(credentials)` - User login
- `logout(token)` - User logout
- `get_current_user(token)` - Get current user

### Students (CRUD + Search)
- `get_all_students()` - List all students
- `get_student(id)` - Get student by ID
- `create_student(request)` - Create new student
- `update_student(id, request)` - Update student
- `delete_student(id)` - Delete student
- `search_students(query)` - Search students

### Counseling Sessions
- `get_all_counseling_sessions()`
- `get_counseling_session(id)`
- `get_student_counseling_sessions(student_id)`
- `create_counseling_session(...)`
- `update_counseling_session(id, ...)`
- `delete_counseling_session(id)`
- `add_student_to_session(session_id, student_id)`
- `create_meeting_note(...)`
- `get_student_meeting_notes(student_id)`
- `create_follow_up(...)`
- `get_pending_follow_ups()`

### Academic Data
- `create_exam_result(...)`
- `get_exam_result(id)`
- `get_student_exam_results(student_id)`
- `get_student_exam_results_by_type(student_id, exam_type)`
- `update_exam_result(id, ...)`
- `delete_exam_result(id)`
- `create_behavior_incident(...)`
- `get_student_behavior_incidents(student_id)`
- `create_academic_goal(...)`
- `get_student_academic_goals(student_id)`

### AI Services
- `create_ai_suggestion(request)`
- `get_ai_suggestion(id)`
- `get_student_ai_suggestions(student_id)`
- `get_pending_ai_suggestions()`
- `review_ai_suggestion(id, review)`
- `delete_ai_suggestion(id)`
- `clean_expired_suggestions()`
- `get_ai_suggestion_statistics()`
- `analyze_student_profile(student_id, ai_config)` ⭐
- `generate_counseling_recommendations(student_id, ai_config)` ⭐
- `chat_with_ai(messages, ai_config)` ⭐
- `test_ai_connection(ai_config)` ⭐

### Surveys
- `create_survey_template(template)`
- `get_survey_template(id)`
- `get_all_survey_templates()`
- `create_survey_distribution(distribution)`
- `create_survey_response(response)`
- `get_distribution_responses(distribution_id)`
- `get_student_survey_responses(student_id)`
- `create_legacy_survey(...)`
- `update_survey_responses(...)`
- `get_student_surveys(student_id)`
- `delete_survey_template(id)`

### Notifications
- `create_notification(...)`
- `get_notification(id)`
- `get_user_notifications(recipient_id)`
- `get_student_notifications(student_id)`
- `get_pending_notifications()`
- `update_notification_status(id, status)`
- `mark_notification_read(id)`
- `get_notification_preferences(user_id)`
- `update_notification_preferences(user_id, preferences)`
- `create_scheduled_task(...)`
- `get_due_tasks()`
- `update_task_next_run(task_id, next_run)`
- `delete_notification(id)`
- `send_native_notification(title, body)` 🔔

### Settings & Config
- `get_settings()` - Get app settings
- `save_settings(settings)` - Save app settings
- `update_ai_provider(ai_config)` - Update AI provider settings

### File Operations
- `upload_file(file_data, metadata)` - Upload file
- `download_file(file_id)` - Download file
- `delete_file(file_id)` - Delete file
- `get_file_list(student_id?)` - List files
- `open_file_in_explorer(file_path)` - Open in system file explorer

**Total: 85+ commands**

---

## 🗄️ Database Schema

### Migrations

Location: `src-tauri/core/migrations/`

1. `001_create_users.sql` - Users table
2. `002_create_students.sql` - Students table
3. `003_create_academic.sql` - Exam results, behavior incidents, goals
4. `004_create_ai_suggestions.sql` - AI suggestions queue
5. `005_create_surveys.sql` - Survey templates, distributions, responses
6. `006_create_counseling.sql` - Counseling sessions, meeting notes, follow-ups
7. `007_create_notifications.sql` - Notification logs, preferences, scheduled tasks
8. `008_create_settings.sql` - App settings

### Key Tables

**Students**
- Core student profile information
- Fields: id, name, surname, email, class, enrollment_date, status, gender, risk, etc.

**Counseling Sessions**
- Individual/group counseling sessions
- Meeting notes
- Follow-up tasks

**Academic Data**
- Exam results (multiple types: TYT, AYT, LGS, etc.)
- Behavior incidents (positive/negative)
- Academic goals

**AI Suggestions**
- User-approval-required AI recommendations
- Types: profile_update, insight, intervention
- Status: pending, approved, rejected, expired
- Priority: low, medium, high

**Surveys**
- Flexible survey system
- Templates, distributions, responses
- Legacy support for existing data

**Notifications**
- Notification logs
- User preferences
- Scheduled tasks (cron-like)

---

## 🤖 AI Services

### Supported Providers

1. **OpenAI** (GPT-4, GPT-3.5)
2. **Google Gemini** (gemini-pro)
3. **Ollama** (llama3, local)

### Configuration

AI provider configured via `AppSettings`:

```rust
pub struct AiProviderConfig {
    pub provider: String, // "openai", "gemini", "ollama"
    pub api_key: Option<String>,
    pub api_url: Option<String>, // For Ollama
    pub model: String,
}
```

Stored in: `{app_data_dir}/settings.json`

### Usage

```rust
use rehber360_core::services::ai_service::AiService;

let ai_service = AiService::new(config);

// Analyze student profile
let analysis = ai_service.analyze_student_profile(student_data).await?;

// Generate recommendations
let recommendations = ai_service.generate_recommendations(student_data).await?;

// Chat
let response = ai_service.chat(messages).await?;
```

### HTTP Clients

- **OpenAI**: `https://api.openai.com/v1/chat/completions`
- **Gemini**: `https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent`
- **Ollama**: `http://localhost:11434/api/chat` (default)

---

## 🛠️ Build & Deploy

### Development

```bash
# Option 1: Tauri CLI (workspace-aware)
npm run tauri:dev

# Option 2: Cargo direct
cd src-tauri
cargo tauri dev
```

This will:
1. Build the core crate
2. Build the app crate
3. Start Vite dev server (port 5000)
4. Launch Tauri desktop app

### Production Build

```bash
npm run tauri:build
```

Output: `src-tauri/target/release/bundle/`
- Windows: `.exe` installer (NSIS)
- macOS: `.dmg`, `.app`
- Linux: `.deb`, `.AppImage`

### Build Options

**Cargo.toml features:**
```toml
[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

**tauri.conf.json:**
```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build:client",
    "devPath": "http://localhost:5000",
    "distDir": "../dist/client"
  }
}
```

---

## 🧪 Testing

### Unit Tests (Rust)

```bash
# Test core crate
cd src-tauri/core
cargo test

# Test with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Integration Tests

```bash
# Test workspace
cd src-tauri
cargo test --workspace
```

### Frontend Tests

```bash
npm test
```

---

## 📝 Coding Guidelines

### Rust

1. **Error Handling**: Use `AppError` custom type
2. **Repository Pattern**: All database access through repositories
3. **DTOs**: Separate request/response types
4. **Async/Await**: All I/O operations are async
5. **Type Safety**: Leverage SQLx compile-time query checking

### TypeScript (Frontend)

1. **Type-Safe API**: Use `tauriApi` client
2. **React Query**: For state management
3. **Error Boundaries**: Catch runtime errors
4. **Lazy Loading**: For routes

---

## 🐛 Debugging

### Rust Backend

```bash
# Enable debug logging
RUST_LOG=debug cargo tauri dev

# VSCode launch.json
{
  "type": "lldb",
  "request": "launch",
  "name": "Tauri Development Debug",
  "cargo": {
    "args": ["build", "--manifest-path=./src-tauri/Cargo.toml"]
  }
}
```

### Frontend

- Chrome DevTools (F12) inside Tauri window
- React DevTools extension

### Database

```bash
# Inspect database
sqlite3 ~/AppData/Local/com.rehber360.app/rehber360.db
```

---

## 📦 Dependencies

### Core Rust Dependencies

- `tauri` 2.1 - Desktop framework
- `sqlx` 0.8 - Database (SQLite)
- `tokio` 1.0 - Async runtime
- `reqwest` 0.12 - HTTP client (AI services)
- `serde` 1.0 - Serialization
- `chrono` 0.4 - Date/time
- `bcrypt` 0.16 - Password hashing
- `thiserror` 2.0 - Error handling
- `uuid` 1.6 - UUIDs

### Frontend Dependencies

- React 18
- TypeScript 5
- Vite 5
- TanStack React Query 5
- Radix UI
- Tailwind CSS

---

## 🔄 Migration from Electron

**Status**: In Progress (FAZ 1-4 completed)

**Next Steps**:
1. Native masaüstü özellikleri (system tray, native notifications)
2. Frontend React Query hooks'larını Tauri API'ye bağla
3. Production build test
4. Data migration (Electron DB → Tauri DB)

---

## 📞 Support

**GitHub**: https://github.com/rehber360/tauri-migration
**Issues**: Report bugs via GitHub Issues
**Docs**: See `plan.md` for detailed migration plan

---

*Last updated: October 31, 2025*
