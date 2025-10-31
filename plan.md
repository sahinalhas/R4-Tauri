# Rehber360 - Tauri Geçiş Planı

## 🎯 Proje Hedefi ve Karar
**KESİN KARAR: Bu proje TAMAMEN Tauri ile çalışacak bir masaüstü uygulamasına dönüştürülüyor.**

Rehber360, mevcut Electron altyapısından Tauri'ye tamamen geçiriliyor. Bu geçiş sonrasında:
- ✅ Electron kodu tamamen kaldırılacak
- ✅ Backend Express.js yerine Tauri Commands (Rust) kullanılacak
- ✅ SQLite better-sqlite3 yerine sqlx/rusqlite ile yönetilecek
- ✅ Frontend aynı kalacak (React + TypeScript)
- ✅ Tüm özellikler Tauri altyapısında yeniden implement edilecek

**Hedef:** Modern, hafif (~10MB), hızlı ve güvenli bir Windows masaüstü uygulaması.

---

## 📊 Mevcut Durum Analizi

### Frontend (Korunacak - %100 Aynı)
- **Framework:** React 18 + TypeScript
- **UI Kütüphaneleri:** Radix UI, Tailwind CSS, Framer Motion
- **State Management:** TanStack React Query, Context API
- **Routing:** React Router DOM
- **Grafikler:** Recharts
- **Dosya:** jspdf, jspdf-autotable, xlsx

### Backend (Rust'a Çevrilecek)
- **Mevcut:** Express.js + TypeScript
- **Yeni:** Tauri Commands + Rust
- **Database:** SQLite (better-sqlite3 → rusqlite/sqlx)
- **AI Services:** OpenAI, Gemini, Ollama (HTTP clients)

### Özellikler (Korunacak)
- Öğrenci profil yönetimi
- AI destekli analiz ve öneriler
- Davranış ve akademik takip
- Görüşme kayıtları
- Bildirim sistemi
- Dosya yükleme/indirme
- PDF/Excel export
- Otomatik güncelleme

---

## 🚀 FAZ 1: Hazırlık ve Altyapı (Gün 1)

### ✅ TAMAMLANDI: Tauri Altyapısı Kurulumu

**Durum**: Tauri workspace yapısı oluşturuldu, temel altyapı hazır.

**Tamamlanan Görevler:**
- [x] Rust kurulumu (rust-stable modül)
- [x] Tauri proje yapısı oluşturuldu (workspace: core + app)
- [x] 8 adet SQL migration dosyası hazırlandı
- [x] Database connection modülü yazıldı (SQLx)
- [x] Temel modeller oluşturuldu (User, Student)
- [x] Auth ve Student Tauri commands implement edildi
- [x] DEVELOPMENT.md dokümantasyonu oluşturuldu
- [x] Tauri configuration (tauri.conf.json) hazır
- [x] Package.json scripts eklendi (tauri:dev, tauri:build)

**Önemli Notlar**:
- Replit ortamında GUI kütüphaneleri kurulamadığı için desktop build LOCAL ortamda yapılacak
- Core crate Replit'te test edilebilir (GUI bağımsız)
- App crate local'de build edilecek (Tauri GUI gereksinimi)

### 1.1. Tauri Kurulumu ve Proje Yapısı
**Görevler:**
- [x] Rust kurulumu (rustup)
- [x] Tauri CLI kurulumu (workspace yapısı ile alternatif)
- [x] Tauri projesini başlatma (manuel oluşturuldu)
- [x] Proje yapısını düzenleme
  ```
  rehber360/
  ├── src-tauri/              # Rust backend
  │   ├── src/
  │   │   ├── main.rs         # Ana entry point
  │   │   ├── commands/       # Tauri commands (API)
  │   │   ├── database/       # SQLite işlemleri
  │   │   ├── services/       # İş mantığı
  │   │   ├── models/         # Veri modelleri
  │   │   └── utils/          # Yardımcı fonksiyonlar
  │   ├── Cargo.toml          # Rust bağımlılıkları
  │   ├── tauri.conf.json     # Tauri yapılandırması
  │   └── icons/              # Uygulama ikonları
  ├── client/                 # React frontend (mevcut)
  ├── shared/                 # Paylaşılan tipler
  └── package.json
  ```

### 1.2. Gerekli Rust Crate'leri (Kütüphaneler)
**Cargo.toml'a eklenecek:**
```toml
[dependencies]
tauri = { version = "2.1", features = ["protocol-asset"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-native-tls"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json", "multipart"] }
chrono = { version = "0.4", features = ["serde"] }
bcrypt = "0.16"
dotenv = "0.15"
thiserror = "2.0"
log = "0.4"
env_logger = "0.11"
uuid = { version = "1.6", features = ["v4", "serde"] }
```

### 1.3. Tauri Yapılandırması
**tauri.conf.json ayarları:**
- [ ] Uygulama bilgileri (ad, versiyon, açıklama)
- [ ] Window ayarları (boyut, başlık, resizable)
- [ ] Bundle ayarları (icon, identifier)
- [ ] Güvenlik ayarları (CSP, allowlist)
- [ ] Auto-updater yapılandırması
- [ ] System tray ayarları

**Hedef:**
```json
{
  "productName": "Rehber360",
  "version": "2.0.0",
  "identifier": "com.rehber360.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build:client",
    "devPath": "http://localhost:5000",
    "distDir": "../dist/client"
  },
  "bundle": {
    "active": true,
    "targets": ["nsis", "msi"],
    "windows": {
      "webviewInstallMode": {
        "type": "embedBootstrapper"
      },
      "wix": {
        "language": ["tr-TR", "en-US"]
      }
    }
  }
}
```

### 1.4. Frontend Build Ayarları
**Vite config güncellemesi:**
- [ ] Tauri için build output path ayarları
- [ ] API base URL için environment variables
- [ ] Development için proxy kaldırma (Tauri IPC kullanılacak)

**Beklenen Çıktı:**
✅ Tauri projesi başarıyla kuruldu
✅ Frontend Tauri'de açılıyor (mevcut Electron UI)
✅ Build sistemi çalışıyor

---

## 🔧 FAZ 2: Database Katmanı - SQLite Entegrasyonu (Gün 1-2)

### ✅ TAMAMLANDI: Database Katmanı

**Durum**: Tüm database modelleri, error handling ve repository pattern implement edildi.

**Tamamlanan Görevler:**
- [x] Tüm veri modelleri oluşturuldu (Academic, AI Suggestion, Survey, Counseling, Notification, Settings)
- [x] Custom error handling katmanı (AppError with thiserror)
- [x] Repository pattern ile tüm CRUD operasyonları
  - [x] StudentRepository
  - [x] UserRepository  
  - [x] CounselingRepository
  - [x] AcademicRepository
  - [x] AiSuggestionRepository
  - [x] SurveyRepository
  - [x] NotificationRepository
- [x] Serde serialization/deserialization
- [x] SQLx query macros ile type-safe database access

## 🔧 FAZ 2 (DEVAM): Database Katmanı - SQLite Entegrasyonu (Gün 1-2)

### 2.1. Database Modülleri Oluşturma

#### 2.1.1. Database Connection Pool
**Dosya:** `src-tauri/src/database/connection.rs`
```rust
use sqlx::{SqlitePool, migrate::MigrateDatabase, Sqlite};
use std::path::PathBuf;

pub async fn initialize_database(app_handle: &tauri::AppHandle) -> Result<SqlitePool, sqlx::Error> {
    let app_dir = app_handle.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;
    
    let db_path = app_dir.join("rehber360.db");
    let db_url = format!("sqlite:{}", db_path.display());
    
    if !Sqlite::database_exists(&db_url).await? {
        Sqlite::create_database(&db_url).await?;
    }
    
    let pool = SqlitePool::connect(&db_url).await?;
    
    // Migrationlar çalıştır
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}
```

#### 2.1.2. Database Migrationları
**Dizin:** `src-tauri/migrations/`

**Görevler:**
- [ ] Mevcut database şemasını Rust migration'lara dönüştür
- [ ] Her tablo için migration dosyası oluştur:
  - `001_create_users.sql`
  - `002_create_students.sql`
  - `003_create_counseling_sessions.sql`
  - `004_create_behaviors.sql`
  - `005_create_academic_data.sql`
  - `006_create_surveys.sql`
  - `007_create_ai_suggestions.sql`
  - `008_create_notifications.sql`
  - `009_create_files.sql`
  - `010_create_settings.sql`

### 2.2. Repository Pattern - Data Access Layer

#### 2.2.1. Student Repository
**Dosya:** `src-tauri/src/database/repositories/student_repository.rs`

**Görevler:**
- [ ] Student CRUD operations
  - `create_student()`
  - `get_student_by_id()`
  - `get_all_students()`
  - `update_student()`
  - `delete_student()`
  - `search_students()`
  - `get_students_by_filter()`

#### 2.2.2. Diğer Repositoryler
**Her biri için ayrı dosya:**
- [ ] `user_repository.rs` - Kullanıcı işlemleri
- [ ] `counseling_repository.rs` - Görüşme kayıtları
- [ ] `behavior_repository.rs` - Davranış kayıtları
- [ ] `academic_repository.rs` - Akademik veriler
- [ ] `survey_repository.rs` - Anket işlemleri
- [ ] `ai_suggestion_repository.rs` - AI önerileri
- [ ] `notification_repository.rs` - Bildirimler
- [ ] `file_repository.rs` - Dosya metadata

### 2.3. Serde Modelleri (JSON Serialization)
**Dizin:** `src-tauri/src/models/`

**Görevler:**
- [ ] Her entity için Rust struct tanımla
- [ ] Serde derive macros ekle (#[derive(Serialize, Deserialize)])
- [ ] TypeScript tipleriyle uyumlu field isimleri
- [ ] Validation logic (ilerde eklenecek)

**Örnek:**
```rust
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub id: i64,
    pub student_number: String,
    pub first_name: String,
    pub last_name: String,
    pub class_level: i32,
    pub branch: Option<String>,
    pub birth_date: Option<NaiveDateTime>,
    pub gender: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
```

**Beklenen Çıktı:**
✅ SQLite database başarıyla initialize ediliyor
✅ Tüm tablolar oluşturuldu
✅ Repository pattern ile temiz veri erişimi
✅ Rust modelleri TypeScript tipleriyle uyumlu

---

## ⚡ FAZ 3: Tauri Commands - API Katmanı (Gün 2-3)

### ✅ TAMAMLANDI: Tauri Commands API Katmanı

**Durum**: Tüm Tauri commands implement edildi, AI entegrasyonu tamamlandı.

**Tamamlanan Görevler:**
- [x] Authentication commands (login, logout, get_current_user)
- [x] Student commands (CRUD + search)
- [x] Counseling commands (sessions, notes, follow-ups)
- [x] Academic commands (exam results, behavior incidents, goals)
- [x] AI commands (suggestions, review, analysis - GERÇEK ENTEGRASYON)
- [x] Survey commands (templates, distributions, responses)
- [x] Notification commands (logs, preferences, scheduled tasks)
- [x] Settings commands (get, save, update AI provider)
- [x] File commands (upload, download, delete, list, open in explorer)
- [x] 85+ Tauri commands registered in main.rs
- [x] Repository pattern kullanımı ile temiz kod yapısı

**Yeni Eklenenler (Son Güncelleme):**
- [x] AI provider entegrasyonları (OpenAI, Gemini, Ollama HTTP clients) ✨
- [x] File upload/download commands ✨
- [x] Settings management commands ✨
- [x] Config service (API keys, preferences yönetimi) ✨

## ⚡ FAZ 3 (DEVAM): Tauri Commands - API Katmanı (Gün 2-3)

### 3.1. Authentication Commands
**Dosya:** `src-tauri/src/commands/auth_commands.rs`

**Tauri Commands:**
- [ ] `login(username: String, password: String) -> Result<UserSession, String>`
- [ ] `logout(session_id: String) -> Result<(), String>`
- [ ] `get_current_user() -> Result<User, String>`
- [ ] `change_password() -> Result<(), String>`
- [ ] `check_permission(action: String) -> Result<bool, String>`

**Frontend entegrasyon:**
```typescript
// client/src/services/api-client.ts
import { invoke } from '@tauri-apps/api/core';

export const authService = {
  login: async (username: string, password: string) => {
    return await invoke<UserSession>('login', { username, password });
  },
  logout: async () => {
    return await invoke('logout');
  },
};
```

### 3.2. Student Commands
**Dosya:** `src-tauri/src/commands/student_commands.rs`

**Görevler:**
- [ ] `get_all_students() -> Result<Vec<Student>, String>`
- [ ] `get_student(id: i64) -> Result<Student, String>`
- [ ] `create_student(student: Student) -> Result<Student, String>`
- [ ] `update_student(id: i64, student: Student) -> Result<Student, String>`
- [ ] `delete_student(id: i64) -> Result<(), String>`
- [ ] `search_students(query: String) -> Result<Vec<Student>, String>`
- [ ] `filter_students(filters: StudentFilters) -> Result<Vec<Student>, String>`
- [ ] `get_student_statistics() -> Result<StudentStats, String>`

### 3.3. Counseling Commands
**Dosya:** `src-tauri/src/commands/counseling_commands.rs`

- [ ] Görüşme kayıtları CRUD
- [ ] Tarih bazlı sorgular
- [ ] Öğrenci bazında görüşme geçmişi

### 3.4. Academic Commands
**Dosya:** `src-tauri/src/commands/academic_commands.rs`

- [ ] Sınav sonuçları CRUD
- [ ] Devamsızlık kayıtları
- [ ] Akademik performans analizi

### 3.5. Behavior Commands
**Dosya:** `src-tauri/src/commands/behavior_commands.rs`

- [ ] Davranış kayıtları CRUD
- [ ] Olumlu/olumsuz davranış filtreleme
- [ ] Davranış trend analizi

### 3.6. Survey Commands
**Dosya:** `src-tauri/src/commands/survey_commands.rs`

- [ ] Anket oluşturma/düzenleme
- [ ] Toplu anket cevapları yükleme (Excel)
- [ ] Anket sonuçları analizi

### 3.7. AI Commands
**Dosya:** `src-tauri/src/commands/ai_commands.rs`

**AI Provider entegrasyonları:**
- [ ] `analyze_student_profile(student_id: i64) -> Result<ProfileAnalysis, String>`
- [ ] `generate_ai_suggestions(student_id: i64) -> Result<Vec<AiSuggestion>, String>`
- [ ] `chat_with_ai(messages: Vec<ChatMessage>) -> Result<String, String>`
- [ ] `enhance_text(text: String, context: String) -> Result<String, String>`
- [ ] `transcribe_audio(file_path: String) -> Result<Transcription, String>`

**HTTP client entegrasyonları:**
```rust
use reqwest::Client;

async fn call_openai_api(prompt: String, api_key: String) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send()
        .await?;
    
    let result: serde_json::Value = response.json().await?;
    Ok(result["choices"][0]["message"]["content"].as_str().unwrap().to_string())
}
```

### 3.8. File Commands
**Dosya:** `src-tauri/src/commands/file_commands.rs`

- [ ] `upload_file(file_data: Vec<u8>, metadata: FileMetadata) -> Result<String, String>`
- [ ] `download_file(file_id: String) -> Result<Vec<u8>, String>`
- [ ] `delete_file(file_id: String) -> Result<(), String>`
- [ ] `get_file_list(student_id: Option<i64>) -> Result<Vec<FileInfo>, String>`
- [ ] `open_file_in_explorer(file_path: String) -> Result<(), String>` (native)

### 3.9. Notification Commands
**Dosya:** `src-tauri/src/commands/notification_commands.rs`

- [ ] `get_notifications(user_id: i64) -> Result<Vec<Notification>, String>`
- [ ] `mark_notification_read(id: i64) -> Result<(), String>`
- [ ] `send_system_notification(title: String, body: String) -> Result<(), String>` (native)

### 3.10. Settings & Config Commands
**Dosya:** `src-tauri/src/commands/settings_commands.rs`

- [ ] `get_settings() -> Result<AppSettings, String>`
- [ ] `update_settings(settings: AppSettings) -> Result<(), String>`
- [ ] `get_ai_provider_config() -> Result<AiConfig, String>`
- [ ] `test_ai_connection(provider: String) -> Result<bool, String>`

**Beklenen Çıktı:**
✅ Tüm Express API'ler Tauri commands olarak implement edildi
✅ Frontend invoke() ile backend'i çağırabiliyor
✅ Error handling düzgün çalışıyor
✅ TypeScript type safety korunuyor

---

## 🎨 FAZ 4: Frontend Entegrasyonu (Gün 3)

### ✅ TAMAMLANDI: Frontend Tauri API Client

**Durum**: Type-safe Tauri API client oluşturuldu ve optimize edildi.

**Tamamlanan Görevler:**
- [x] Tauri API client (client/src/services/tauri-api.ts)
- [x] TypeScript type definitions
- [x] invoke() wrapper functions with lazy loading
- [x] Error handling
- [x] Conditional Tauri import (desktop vs web browser)
- [x] Mock implementation for development without Tauri

**Not:**
- React Query hooks zaten mevcut (Express API'yi kullanıyor)
- Desktop build sırasında bu hooks'lar Tauri API'yi kullanacak şekilde değiştirilebilir
- Local geliştirme ortamında test edilecek

## 🎨 FAZ 4 (DEVAM): Frontend Entegrasyonu (Gün 3)

### 4.1. Tauri API Client Oluşturma
**Dosya:** `client/src/services/tauri-api.ts`

**Görevler:**
- [ ] Mevcut axios tabanlı API client'i Tauri invoke'a dönüştür
- [ ] Type-safe wrapper fonksiyonlar
- [ ] Error handling ve retry logic
- [ ] Loading states

**Örnek implement:**
```typescript
import { invoke } from '@tauri-apps/api/core';

export class TauriApiClient {
  // Students
  async getStudents(): Promise<Student[]> {
    try {
      return await invoke<Student[]>('get_all_students');
    } catch (error) {
      console.error('Failed to get students:', error);
      throw new Error('Öğrenciler yüklenirken hata oluştu');
    }
  }

  async createStudent(student: Omit<Student, 'id'>): Promise<Student> {
    return await invoke<Student>('create_student', { student });
  }

  // AI
  async analyzeStudentProfile(studentId: number): Promise<ProfileAnalysis> {
    return await invoke<ProfileAnalysis>('analyze_student_profile', { studentId });
  }
}

export const apiClient = new TauriApiClient();
```

### 4.2. React Query Integration
**Görevler:**
- [ ] Mevcut query hooks'ları Tauri API'yi kullanacak şekilde güncelle
- [ ] Optimistic updates
- [ ] Cache invalidation stratejileri

**Örnek:**
```typescript
// client/src/hooks/useStudents.ts
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '@/services/tauri-api';

export const useStudents = () => {
  return useQuery({
    queryKey: ['students'],
    queryFn: () => apiClient.getStudents(),
  });
};

export const useCreateStudent = () => {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (student: Omit<Student, 'id'>) => apiClient.createStudent(student),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['students'] });
    },
  });
};
```

### 4.3. Environment Variables
**Görevler:**
- [ ] API URL'leri kaldır (artık Tauri IPC kullanılıyor)
- [ ] AI API anahtarları için Tauri store kullan
- [ ] Development/production mod ayrımı

### 4.4. Dosya İşlemleri Frontend
**Görevler:**
- [ ] File upload component'i Tauri dialog API'yi kullanacak şekilde güncelle
- [ ] Drag & drop işlemleri
- [ ] Progress tracking

**Tauri dialog örneği:**
```typescript
import { open, save } from '@tauri-apps/plugin-dialog';
import { readBinaryFile, writeBinaryFile } from '@tauri-apps/plugin-fs';

async function selectAndUploadFile() {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg'] }],
  });
  
  if (selected) {
    const contents = await readBinaryFile(selected);
    await apiClient.uploadFile(contents, { name: selected.name });
  }
}
```

**Beklenen Çıktı:**
✅ Frontend tamamen Tauri IPC üzerinden backend'e bağlandı
✅ Tüm API çağrıları çalışıyor
✅ Dosya işlemleri native dialog kullanıyor
✅ Type safety korunuyor

---

## 🖥️ FAZ 5: Native Masaüstü Özellikleri (Gün 4)

### 5.1. System Tray Integration
**Görevler:**
- [ ] Tauri tray plugin kurulumu
- [ ] Tray icon ve menü oluşturma
- [ ] Minimize to tray özelliği
- [ ] Sağ tık menüsü (Aç, Ayarlar, Çıkış)

**Implement:**
```rust
// src-tauri/src/main.rs
use tauri::{
    CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent,
    Manager,
};

fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Çıkış");
    let show = CustomMenuItem::new("show".to_string(), "Göster");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => std::process::exit(0),
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                _ => {}
            }
        }
        _ => {}
    }
}
```

### 5.2. Native Notifications
**Görevler:**
- [ ] Tauri notification plugin
- [ ] Windows Action Center entegrasyonu
- [ ] Tıklanabilir bildirimler (öğrenci profiline git)
- [ ] Bildirim tercihleri (settings'de)

**Örnek:**
```rust
use tauri::api::notification::Notification;

#[tauri::command]
async fn send_notification(app: AppHandle, title: String, body: String) -> Result<(), String> {
    Notification::new(&app.config().tauri.bundle.identifier)
        .title(title)
        .body(body)
        .icon("icons/icon.png")
        .show()
        .map_err(|e| e.to_string())?;
    Ok(())
}
```

### 5.3. Window Management
**Görevler:**
- [ ] Pencere boyutu ve pozisyon kaydetme (Tauri store)
- [ ] Maximize/minimize state persistence
- [ ] Tam ekran modu (F11)
- [ ] Çoklu pencere desteği (gelecekte)

**Tauri config:**
```json
{
  "windows": [
    {
      "title": "Rehber360",
      "width": 1400,
      "height": 900,
      "minWidth": 1024,
      "minHeight": 768,
      "resizable": true,
      "fullscreen": false,
      "decorations": true,
      "transparent": false,
      "theme": "Light",
      "center": true
    }
  ]
}
```

### 5.4. Keyboard Shortcuts
**Görevler:**
- [ ] Global shortcuts (Ctrl+N yeni öğrenci, vb.)
- [ ] Menu bar shortcuts
- [ ] Custom shortcut ayarları

**Application Menu (Türkçe):**
```rust
use tauri::{Menu, Submenu, MenuItem, CustomMenuItem};

fn create_menu() -> Menu {
    let mut menu = Menu::new();
    
    // Dosya menüsü
    let file_menu = Submenu::new(
        "Dosya",
        Menu::new()
            .add_item(CustomMenuItem::new("new_student", "Yeni Öğrenci").accelerator("Ctrl+N"))
            .add_item(CustomMenuItem::new("export", "Dışa Aktar").accelerator("Ctrl+E"))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("quit", "Çıkış").accelerator("Ctrl+Q"))
    );
    
    menu = menu.add_submenu(file_menu);
    menu
}
```

### 5.5. Auto-Updater (GitHub Releases)
**Görevler:**
- [ ] Tauri updater plugin kurulumu
- [ ] GitHub Releases entegrasyonu
- [ ] Otomatik güncelleme kontrolü (başlangıçta)
- [ ] Güncelleme bildirimi ve kullanıcı onayı
- [ ] Progress bar ile güncelleme indirme

**Implement:**
```rust
use tauri::updater::UpdaterBuilder;

#[tauri::command]
async fn check_for_updates(app: AppHandle) -> Result<bool, String> {
    let updater = UpdaterBuilder::new()
        .build()
        .map_err(|e| e.to_string())?;
    
    match updater.check().await {
        Ok(update) => {
            if update.is_update_available() {
                // Kullanıcıya bildir
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => Err(e.to_string())
    }
}
```

### 5.6. Deep Linking (Protokol Handler)
**Görevler:**
- [ ] `rehber360://` protokol kaydı
- [ ] Harici linklerden öğrenci profili açma
- [ ] URL parsing ve routing

**Beklenen Çıktı:**
✅ System tray çalışıyor ve minimize to tray destekleniyor
✅ Native Windows bildirimleri aktif
✅ Pencere durumu kaydediliyor
✅ Keyboard shortcuts çalışıyor
✅ Otomatik güncelleme sistemi hazır

---

## 🔐 FAZ 6: Güvenlik ve Optimizasyon (Gün 4)

### 6.1. Güvenlik İyileştirmeleri
**Görevler:**
- [ ] CSP (Content Security Policy) sıkılaştırma
- [ ] Tauri allowlist konfigürasyonu (sadece gerekli API'ler)
- [ ] API key encryption (Tauri store ile)
- [ ] SQL injection koruması (prepared statements)
- [ ] XSS koruması
- [ ] CSRF gereksiz (web değil, desktop)

**Tauri allowlist:**
```json
{
  "allowlist": {
    "all": false,
    "fs": {
      "all": false,
      "readFile": true,
      "writeFile": true,
      "scope": ["$APPDATA/rehber360/*"]
    },
    "dialog": {
      "all": false,
      "open": true,
      "save": true
    },
    "notification": {
      "all": true
    },
    "shell": {
      "all": false,
      "open": true
    }
  }
}
```

### 6.2. Performans Optimizasyonu
**Görevler:**
- [ ] Database indexleme (sık sorgulanan alanlar)
- [ ] SQLite WAL mode (Write-Ahead Logging)
- [ ] Connection pooling optimize
- [ ] Lazy loading büyük listeler için
- [ ] React Query cache stratejileri
- [ ] Frontend bundle optimization
- [ ] Tauri binary stripping (release build)

**SQLite optimizations:**
```rust
// WAL mode ve performans ayarları
sqlx::query("PRAGMA journal_mode = WAL;").execute(&pool).await?;
sqlx::query("PRAGMA synchronous = NORMAL;").execute(&pool).await?;
sqlx::query("PRAGMA cache_size = -64000;").execute(&pool).await?; // 64MB cache
sqlx::query("PRAGMA temp_store = MEMORY;").execute(&pool).await?;
```

### 6.3. Error Handling ve Logging
**Görevler:**
- [ ] Rust error types (thiserror)
- [ ] Frontend error boundaries
- [ ] Log dosyaları (app data klasöründe)
- [ ] Crash reporting (opsiyonel)

**Custom error types:**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Veritabanı hatası: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Öğrenci bulunamadı: {0}")]
    StudentNotFound(i64),
    
    #[error("Yetkilendirme hatası")]
    Unauthorized,
    
    #[error("AI servis hatası: {0}")]
    AiServiceError(String),
}
```

**Beklenen Çıktı:**
✅ Uygulama güvenli (allowlist, encryption)
✅ Performans optimize edildi
✅ Error handling profesyonel
✅ Loglar düzgün tutuluyor

---

## 📦 FAZ 7: Build ve Deployment (Gün 5)

### 7.1. Production Build Konfigürasyonu
**Görevler:**
- [ ] Release build profili optimize etme
- [ ] Binary size küçültme (strip symbols)
- [ ] Code signing sertifikası (Windows için)
- [ ] App icon setleri oluşturma (tüm boyutlar)

**Cargo.toml optimize:**
```toml
[profile.release]
opt-level = "z"     # Size optimization
lto = true          # Link Time Optimization
codegen-units = 1   # Better optimization
strip = true        # Strip symbols
panic = "abort"     # Smaller binary
```

### 7.2. Windows Installer
**Görevler:**
- [ ] NSIS installer konfigürasyonu
- [ ] MSI installer konfigürasyonu
- [ ] Portable sürüm (.exe)
- [ ] Installer dili (Türkçe)
- [ ] Uninstaller
- [ ] Start Menu shortcuts
- [ ] Desktop shortcut (opsiyonel)
- [ ] Otomatik başlatma seçeneği

**tauri.conf.json bundle:**
```json
{
  "bundle": {
    "active": true,
    "targets": ["nsis", "msi", "app"],
    "identifier": "com.rehber360.app",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": "",
      "wix": {
        "language": "tr-TR"
      },
      "nsis": {
        "installerIcon": "icons/icon.ico",
        "installMode": "perUser",
        "languages": ["Turkish", "English"],
        "displayLanguageSelector": true
      }
    }
  }
}
```

### 7.3. Auto-Update Server
**Görevler:**
- [ ] GitHub Actions workflow oluşturma
- [ ] Release otomasyonu
- [ ] Update manifest oluşturma
- [ ] Changelog otomatik oluşturma

**GitHub Actions örnek:**
```yaml
name: Release Build

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Node.js
        uses: actions/setup-node@v3
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
      - name: Install dependencies
        run: npm ci
      - name: Build
        run: npm run tauri build
      - name: Create Release
        uses: tauri-apps/tauri-action@v0
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'Rehber360 ${{ github.ref_name }}'
          releaseBody: 'Changelog içeriği'
```

### 7.4. İlk Kullanım Deneyimi (OOBE - Out of Box Experience)
**Görevler:**
- [ ] İlk açılışta kurulum sihirbazı
- [ ] Database initialization
- [ ] Demo kullanıcı oluşturma (opsiyonel)
- [ ] AI provider ayarları
- [ ] Veri içe aktarma seçeneği

### 7.5. Test ve Kalite Kontrolü
**Görevler:**
- [ ] Unit tests (Rust backend)
- [ ] Integration tests
- [ ] E2E tests (frontend)
- [ ] Performance testing
- [ ] Manuel test checklist
- [ ] Beta testing (seçili kullanıcılarla)

**Beklenen Çıktı:**
✅ Production build başarılı (~5-8MB installer)
✅ Installer çalışıyor ve sorunsuz kurulum
✅ Otomatik güncelleme sistemi aktif
✅ İlk kullanım deneyimi pürüzsüz

---

## 🚀 FAZ 8: Migration ve Data Transfer (Gün 5)

### 8.1. Electron'dan Veri Aktarımı
**Görevler:**
- [ ] Mevcut Electron database'i bulma
- [ ] SQLite database migration script
- [ ] Kullanıcı verilerini kopyalama
- [ ] Dosyaları yeni konuma taşıma
- [ ] Settings transfer

**Migration tool:**
```rust
#[tauri::command]
async fn migrate_from_electron(old_db_path: String) -> Result<MigrationReport, String> {
    // Eski database'i aç
    let old_pool = SqlitePool::connect(&old_db_path).await?;
    
    // Yeni database'e kopyala
    let new_pool = get_app_db_pool().await?;
    
    // Her tablo için:
    // 1. Verileri oku
    // 2. Yeni schema'ya uyarla
    // 3. Yeni DB'ye yaz
    
    Ok(MigrationReport {
        students_migrated: 150,
        sessions_migrated: 420,
        files_migrated: 89,
    })
}
```

### 8.2. Veri İçe/Dışa Aktarma
**Görevler:**
- [ ] Full database backup (.db dosyası)
- [ ] JSON export (portable format)
- [ ] Excel export (öğrenci listeleri)
- [ ] Import wizard
- [ ] Veri doğrulama

**Beklenen Çıktı:**
✅ Electron'dan sorunsuz geçiş
✅ Hiçbir veri kaybı yok
✅ Kullanıcılar farkında bile olmadan geçiş yaptı

---

## 📚 FAZ 9: Dokümantasyon ve Eğitim (Gün 5)

### 9.1. Teknik Dokümantasyon
**Oluşturulacak dosyalar:**
- [ ] `TAURI_ARCHITECTURE.md` - Sistem mimarisi
- [ ] `API_REFERENCE.md` - Tüm Tauri commands
- [ ] `DATABASE_SCHEMA.md` - Veritabanı şeması
- [ ] `DEVELOPMENT_GUIDE.md` - Geliştirme ortamı kurulumu
- [ ] `DEPLOYMENT_GUIDE.md` - Build ve release süreci

### 9.2. Kullanıcı Dokümantasyonu
- [ ] Kurulum kılavuzu (PDF)
- [ ] Kullanım kılavuzu (ekran görüntüleri ile)
- [ ] Sık sorulan sorular (FAQ)
- [ ] Video eğitimler (opsiyonel)

### 9.3. Sürüm Notları
- [ ] v2.0.0 sürüm notları (Tauri geçişi)
- [ ] Breaking changes listesi
- [ ] Migration guide
- [ ] Yenilikler ve iyileştirmeler

**Beklenen Çıktı:**
✅ Kapsamlı teknik dokümantasyon
✅ Kullanıcı dostu kılavuzlar
✅ Ekip ve kullanıcılar bilgilendirildi

---

## ✅ FAZ 10: Teslim ve Lansman (Gün 5)

### 10.1. Final Checklist
- [ ] Tüm testler geçiyor
- [ ] Performance metrikleri hedeflerde
  - Uygulama boyutu: <10MB ✅
  - RAM kullanımı: <200MB ✅
  - Başlangıç süresi: <1 saniye ✅
- [ ] Güvenlik audit tamamlandı
- [ ] Dokümantasyon güncel
- [ ] GitHub repository temiz
- [ ] Release notes hazır

### 10.2. Release Strategy
**v2.0.0 (Tauri) Lansman:**
- [ ] Beta release (seçili kullanıcılar)
- [ ] Beta feedback toplama ve düzeltmeler
- [ ] Stable release
- [ ] Public announcement
- [ ] Eski Electron versiyonu deprecated ilan et

### 10.3. Kullanıcı Desteği
- [ ] Support email kurulumu
- [ ] Issue tracker (GitHub Issues)
- [ ] Güncellemeler için duyuru kanalı
- [ ] Hotfix planı (kritik buglar için)

**Beklenen Çıktı:**
✅ Tauri versiyonu production'da
✅ Kullanıcılar sorunsuz geçiş yaptı
✅ Destek sistemi hazır

---

## 📊 Başarı Metrikleri

### Performans Hedefleri
| Metrik | Electron | Tauri Hedef | Gerçekleşen |
|--------|----------|-------------|-------------|
| Uygulama boyutu | 150MB | <10MB | ⬜ |
| RAM (boşta) | 300MB | <100MB | ⬜ |
| RAM (aktif) | 600MB | <250MB | ⬜ |
| Başlangıç süresi | 2s | <1s | ⬜ |
| CPU kullanımı | %20 | <10% | ⬜ |

### Geliştirme Metrikleri
- [ ] Toplam geliştirme süresi: 5 gün
- [ ] Code coverage: >80%
- [ ] Zero breaking bugs
- [ ] Kullanıcı memnuniyeti: >90%

---

## 🛠️ Araçlar ve Teknolojiler

### Geliştirme Ortamı
- **IDE:** VS Code + Rust Analyzer
- **Tauri CLI:** `cargo tauri dev`
- **Frontend:** Vite dev server
- **Database:** SQLite Studio
- **Git:** GitHub Desktop / CLI
- **Testing:** Cargo test, Vitest

### Production Ortamı
- **Build:** GitHub Actions
- **Release:** GitHub Releases
- **Monitoring:** (opsiyonel)
- **Analytics:** (opsiyonel)

---

## 📝 Notlar ve Önemli Hatırlatmalar

### Dikkat Edilmesi Gerekenler
1. **Database Migration:** Eski Electron database'inden veri aktarımı dikkatle yapılmalı
2. **API Compatibility:** Frontend'deki mevcut API calls tümüyle değişecek
3. **File Paths:** Electron'un app.getPath() yerine Tauri'nin path API'si kullanılmalı
4. **Environment Variables:** .env dosyaları yerine Tauri config veya store kullanılmalı
5. **Build Time:** İlk Rust build uzun sürebilir (~5-10 dakika), sonrakiler hızlı

### Potansiyel Sorunlar ve Çözümler
**Sorun:** Rust öğrenme eğrisi  
**Çözüm:** AI yardımı (ChatGPT/GitHub Copilot), Rust Book, örnek projeler

**Sorun:** SQLite migration hataları  
**Çözüm:** Rollback planı, veri backup, staging environment test

**Sorun:** Frontend API çağrılarında tip uyumsuzlukları  
**Çözüm:** TypeScript type generation (serde-bindgen), shared types

**Sorun:** Windows WebView2 yoksa  
**Çözüm:** Installer'a embed bootstrapper ekle

---

## 🎯 Özet: 5 Günlük Sprint

**Gün 1:** Tauri kurulumu + Database altyapısı  
**Gün 2:** Database repositories + Tauri commands (CRUD)  
**Gün 3:** AI commands + Frontend entegrasyonu  
**Gün 4:** Native özellikler + Güvenlik  
**Gün 5:** Build + Migration + Dokümantasyon + Release

**Sonuç:** Modern, hafif, hızlı ve güvenli bir Windows masaüstü uygulaması! 🚀

---

## 📞 İletişim ve Destek

**Proje:** Rehber360 Tauri Migration  
**Hedef:** Production-ready v2.0.0  
**Zaman:** 5 iş günü  
**Risk Seviyesi:** Orta (yeni teknoloji, ama planlı)  
**ROI:** Çok yüksek (30x küçük, 4x az RAM, modern)

---

*Bu plan dinamiktir ve geliştirme sürecinde güncellenebilir.*
