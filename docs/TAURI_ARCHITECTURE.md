# Rehber360 Tauri Architecture

## 📋 İçindekiler

1. [Genel Bakış](#genel-bakış)
2. [Proje Yapısı](#proje-yapısı)
3. [Mimari Katmanlar](#mimari-katmanlar)
4. [Veri Akışı](#veri-akışı)
5. [Güvenlik Mimarisi](#güvenlik-mimarisi)
6. [Native Desktop Özellikleri](#native-desktop-özellikleri)

---

## Genel Bakış

Rehber360, **Tauri 2.x** framework'ü kullanılarak geliştirilmiş modern bir masaüstü uygulamasıdır. Uygulama, **Rust backend** ve **React frontend** kombinasyonunu kullanarak hafif, hızlı ve güvenli bir deneyim sunar.

### Teknoloji Stack'i

**Backend (Rust):**
- Tauri 2.1 - Desktop application framework
- SQLx 0.8 - Type-safe SQL database driver
- Tokio - Async runtime
- Serde - Serialization/deserialization
- Bcrypt - Password hashing
- Reqwest - HTTP client (AI integrations)

**Frontend (React):**
- React 18 + TypeScript
- Vite - Build tool
- Radix UI - Component library
- Tailwind CSS - Styling
- TanStack React Query - Server state management
- React Router DOM - Routing

**Database:**
- SQLite (via SQLx)
- 8 migration files
- 9+ tables

---

## Proje Yapısı

```
rehber360/
├── src-tauri/                  # Rust backend
│   ├── app/                    # Tauri application crate
│   │   ├── src/
│   │   │   ├── main.rs         # Entry point, tray, menu
│   │   │   └── commands/       # Tauri commands (API endpoints)
│   │   │       ├── auth.rs     # Authentication
│   │   │       ├── student.rs  # Student management
│   │   │       ├── counseling.rs
│   │   │       ├── academic.rs
│   │   │       ├── ai.rs       # AI integrations
│   │   │       ├── survey.rs
│   │   │       ├── notification.rs
│   │   │       ├── settings.rs
│   │   │       └── file.rs     # File operations
│   │   ├── Cargo.toml
│   │   └── tauri.conf.json     # Tauri configuration
│   │
│   ├── core/                   # Core business logic crate
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── database/       # Database layer
│   │   │   │   ├── connection.rs
│   │   │   │   └── repositories/
│   │   │   ├── models/         # Data models
│   │   │   │   ├── user.rs
│   │   │   │   ├── student.rs
│   │   │   │   ├── academic.rs
│   │   │   │   ├── counseling.rs
│   │   │   │   ├── ai_suggestion.rs
│   │   │   │   ├── survey.rs
│   │   │   │   ├── notification.rs
│   │   │   │   └── settings.rs
│   │   │   ├── services/       # Business logic
│   │   │   │   └── ai/         # AI service integrations
│   │   │   └── security.rs     # Input validation, sanitization
│   │   ├── migrations/         # SQLx migrations
│   │   │   ├── 001_create_users.sql
│   │   │   ├── 002_create_students.sql
│   │   │   ├── 003_create_academic.sql
│   │   │   ├── 004_create_ai_suggestions.sql
│   │   │   ├── 005_create_surveys.sql
│   │   │   ├── 006_create_counseling.sql
│   │   │   ├── 007_create_notifications.sql
│   │   │   └── 008_create_settings.sql
│   │   └── Cargo.toml
│   │
│   └── Cargo.toml              # Workspace configuration
│
├── client/                     # React frontend
│   ├── src/
│   │   ├── components/
│   │   ├── pages/
│   │   ├── services/
│   │   │   ├── tauri-api.ts    # Tauri IPC client
│   │   │   ├── desktop-integration.ts
│   │   │   ├── window-manager.ts
│   │   │   ├── native-notifications.ts
│   │   │   └── auto-updater.ts
│   │   ├── hooks/
│   │   └── App.tsx
│   └── vite.config.ts
│
├── .github/
│   └── workflows/
│       └── release.yml         # CI/CD for releases
│
└── docs/                       # Documentation
```

---

## Mimari Katmanlar

Rehber360, **clean architecture** ve **separation of concerns** prensiplerini takip eder:

### 1. Presentation Layer (Frontend - React)

**Sorumluluklar:**
- Kullanıcı arayüzü render etme
- Kullanıcı etkileşimlerini yönetme
- Tauri IPC ile backend'e istek gönderme
- State management (React Query + Context API)

**Önemli Servisler:**
```typescript
// client/src/services/tauri-api.ts
import { invoke } from '@tauri-apps/api/core';

export const apiClient = {
  async getStudents() {
    return await invoke<Student[]>('get_all_students');
  },
  async createStudent(data: CreateStudentRequest) {
    return await invoke<Student>('create_student', { data });
  }
};
```

### 2. API Layer (Tauri Commands)

**Sorumluluklar:**
- Frontend isteklerini alır
- Input validation
- Business logic katmanını çağırır
- Response serialization

**Örnek Command:**
```rust
// src-tauri/app/src/commands/student.rs
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
```

### 3. Business Logic Layer (Core Services)

**Sorumluluklar:**
- İş mantığı kuralları
- AI entegrasyonları
- Complex data processing
- Cross-cutting concerns (logging, validation)

**Yapı:**
```rust
// src-tauri/core/src/services/ai/
├── mod.rs
├── openai.rs    # OpenAI client
├── gemini.rs    # Google Gemini client
└── ollama.rs    # Ollama local LLM client
```

### 4. Data Access Layer (Repositories)

**Sorumluluklar:**
- Database operations (CRUD)
- Query building
- Transaction management
- Data mapping

**Repository Pattern:**
```rust
// Repository trait example
pub trait StudentRepository {
    async fn find_all(&self) -> Result<Vec<Student>>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Student>>;
    async fn create(&self, student: CreateStudentRequest) -> Result<Student>;
    async fn update(&self, id: &str, data: UpdateStudentRequest) -> Result<Student>;
    async fn delete(&self, id: &str) -> Result<()>;
}
```

### 5. Database Layer (SQLite + SQLx)

**Sorumluluklar:**
- Database connection management
- Schema migrations
- Type-safe queries

**Migration System:**
```rust
// src-tauri/core/src/database/connection.rs
pub async fn initialize_database(db_path: PathBuf) -> Result<SqlitePool> {
    let db_url = format!("sqlite:{}", db_path.display());
    
    if !Sqlite::database_exists(&db_url).await? {
        Sqlite::create_database(&db_url).await?;
    }
    
    let pool = SqlitePool::connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}
```

---

## Veri Akışı

### Frontend → Backend Flow

```
[User Action]
    ↓
[React Component]
    ↓
[Tauri API Client (invoke)]
    ↓ IPC
[Tauri Command Handler]
    ↓
[Business Logic / Repository]
    ↓
[SQLx Query]
    ↓
[SQLite Database]
    ↓ (Result)
[Serialize (Serde)]
    ↓ IPC
[Frontend (React Query)]
    ↓
[UI Update]
```

### Örnek: Öğrenci Oluşturma

1. **Frontend (React):**
```typescript
const { mutate } = useMutation({
  mutationFn: (data: CreateStudentRequest) => 
    apiClient.createStudent(data),
  onSuccess: () => {
    queryClient.invalidateQueries(['students']);
  }
});
```

2. **Tauri Command:**
```rust
#[tauri::command]
pub async fn create_student(
    pool: State<'_, SqlitePool>,
    data: CreateStudentRequest,
) -> Result<Student, String> {
    // Validation
    validate_student_data(&data)?;
    
    // Insert to database
    let student_id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO students (...) VALUES (...)")
        .bind(...)
        .execute(pool.inner())
        .await
        .map_err(|e| format!("Failed to create student: {}", e))?;
    
    // Return created student
    Ok(student)
}
```

---

## Güvenlik Mimarisi

### 1. Input Validation

**Katman:** `src-tauri/core/src/security.rs`

```rust
pub fn validate_email(email: &str) -> Result<(), String> { ... }
pub fn validate_student_number(number: &str) -> Result<(), String> { ... }
pub fn validate_turkish_phone(phone: &str) -> Result<(), String> { ... }
pub fn sanitize_html(input: &str) -> String { ... }
```

### 2. SQL Injection Protection

- **SQLx compile-time checked queries**
- Type-safe query parameters
- No raw SQL string concatenation

### 3. XSS Protection

- HTML sanitization
- Pattern detection
- Content Security Policy (CSP)

### 4. Path Traversal Protection

- Filename validation
- Restricted file operations
- UUID-based file naming

### 5. Authentication & Authorization

- Password hashing (bcrypt)
- Session-based authentication
- Token expiration (24 hours)
- Role-based access control (RBAC)

---

## Native Desktop Özellikleri

### System Tray

```rust
// main.rs
let tray = TrayIconBuilder::new()
    .menu(&tray_menu)
    .tooltip("Rehber360")
    .on_menu_event(|app, event| { ... })
    .build()?;
```

**Özellikler:**
- Minimize-to-tray
- Quick actions menu
- Click/double-click handling

### Native Notifications

```rust
use tauri_plugin_notification::NotificationExt;

app.notification()
    .builder()
    .title("Bildirim Başlığı")
    .body("Bildirim içeriği")
    .show()?;
```

### Window Management

```rust
// Window state persistence
use tauri_plugin_store::StoreExt;

let store = app.store("settings.json")?;
store.set("window_bounds", bounds)?;
```

### Application Menu

```rust
let menu = MenuBuilder::new(app)
    .item(&file_menu)
    .item(&view_menu)
    .item(&help_menu)
    .build()?;
```

### Auto-Updater

```json
// tauri.conf.json
{
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": ["https://releases.rehber360.com/latest.json"],
      "dialog": true,
      "pubkey": "..."
    }
  }
}
```

---

## Performance Optimizations

### 1. Cargo.toml Release Profile

```toml
[profile.release]
opt-level = "z"          # Size optimization
lto = true               # Link Time Optimization
codegen-units = 1        # Better optimization
strip = true             # Strip debug symbols
panic = "abort"          # Smaller binary
```

**Sonuç:** ~5-10MB installer size

### 2. Frontend Code Splitting

```typescript
// Lazy loading
const StudentPage = lazy(() => import('./pages/StudentPage'));
const DashboardPage = lazy(() => import('./pages/DashboardPage'));
```

### 3. React Query Caching

```typescript
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 60000,  // 1 minute
      gcTime: 300000,    // 5 minutes
    },
  },
});
```

---

## Build ve Deployment

### Development

```bash
# Core crate test (GUI-free)
cd src-tauri/core
cargo test

# Full development (local only)
npm run tauri:dev
```

### Production Build

```bash
# Local build
npm run tauri:build

# CI/CD (GitHub Actions)
# Otomatik olarak Windows, macOS, Linux için build
```

### Build Artifacts

- **Windows:** `.exe`, `.msi`, NSIS installer
- **macOS:** `.dmg`, `.app`
- **Linux:** `.AppImage`, `.deb`

---

## Gelecek İyileştirmeler

1. **Auto-Updater Production Setup**
   - Ed25519 key pair generation
   - Release signing
   - Update manifest hosting

2. **Testing Coverage**
   - Unit tests (>80% coverage)
   - Integration tests
   - E2E tests

3. **Monitoring & Analytics**
   - Crash reporting
   - Usage analytics
   - Performance monitoring

4. **Multi-language Support**
   - i18n infrastructure
   - Translation files
   - Language selector

---

## Kaynaklar

- [Tauri Documentation](https://v2.tauri.app/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [React Documentation](https://react.dev/)

---

**Son Güncelleme:** 31 Ekim 2025  
**Versiyon:** 2.0.0  
**Mimari Durum:** Stable & Production-Ready
