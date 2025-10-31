# Rehber360 API Reference

## 📋 İçindekiler

1. [Authentication Commands](#authentication-commands)
2. [Student Commands](#student-commands)
3. [Counseling Commands](#counseling-commands)
4. [Academic Commands](#academic-commands)
5. [AI Commands](#ai-commands)
6. [Survey Commands](#survey-commands)
7. [Notification Commands](#notification-commands)
8. [Settings Commands](#settings-commands)
9. [File Commands](#file-commands)
10. [Error Handling](#error-handling)

---

## Genel Bilgiler

Rehber360, **Tauri IPC (Inter-Process Communication)** kullanarak frontend ve backend arasında iletişim sağlar. Tüm API çağrıları `invoke()` fonksiyonu ile yapılır.

### Kullanım

```typescript
import { invoke } from '@tauri-apps/api/core';

// Type-safe API call
const students = await invoke<Student[]>('get_all_students');

// With parameters
const student = await invoke<Student>('get_student_by_id', { 
  id: 'student-uuid' 
});
```

### Return Types

- **Success:** `Result<T, String>` → Frontend'de `T` tipinde veri
- **Error:** `String` error message

---

## Authentication Commands

### `login`

Kullanıcı girişi yapar ve session oluşturur.

**Request:**
```typescript
{
  email: string;
  password: string;
}
```

**Response:**
```typescript
{
  user: User;
  token: string;  // 24 saat geçerli
}
```

**Example:**
```typescript
const session = await invoke<UserSession>('login', {
  credentials: {
    email: 'rehber@okul.edu.tr',
    password: 'demo'
  }
});
```

**Errors:**
- `"Invalid credentials"` - Yanlış email/şifre
- `"User account is inactive"` - Pasif kullanıcı
- `"Database error: ..."` - Veritabanı hatası

---

### `logout`

Kullanıcı oturumunu kapatır.

**Request:**
```typescript
{
  token: string;
}
```

**Response:**
```typescript
void
```

**Example:**
```typescript
await invoke('logout', { token: sessionToken });
```

---

### `get_current_user`

Mevcut oturum bilgilerini getirir.

**Request:**
```typescript
{
  token: string;
}
```

**Response:**
```typescript
User {
  id: string;
  name: string;
  email: string;
  role: 'admin' | 'counselor' | 'teacher' | 'observer';
  institution: string;
  isActive: boolean;
  created_at: string;
  updated_at: string;
}
```

**Example:**
```typescript
const user = await invoke<User>('get_current_user', { 
  token: sessionToken 
});
```

**Errors:**
- `"Invalid session"` - Token bulunamadı
- `"Session expired"` - Token süresi dolmuş

---

## Student Commands

### `get_all_students`

Tüm öğrencileri listeler.

**Request:** None

**Response:**
```typescript
Student[] {
  id: string;
  name: string;
  surname: string;
  email?: string;
  phone?: string;
  birthDate?: string;
  address?: string;
  class?: string;
  enrollmentDate: string;
  status: string;
  avatar?: string;
  parentContact?: string;
  notes?: string;
  gender: 'K' | 'E';
  risk: 'Düşük' | 'Orta' | 'Yüksek';
  created_at: string;
  updated_at: string;
}
```

**Example:**
```typescript
const students = await invoke<Student[]>('get_all_students');
```

---

### `get_student_by_id`

Belirli bir öğrenciyi getirir.

**Request:**
```typescript
{
  id: string;
}
```

**Response:**
```typescript
Student
```

**Example:**
```typescript
const student = await invoke<Student>('get_student_by_id', {
  id: 'student-uuid'
});
```

**Errors:**
- `"Student not found"` - Öğrenci bulunamadı

---

### `create_student`

Yeni öğrenci oluşturur.

**Request:**
```typescript
{
  name: string;
  surname: string;
  email?: string;
  phone?: string;
  birthDate?: string;
  address?: string;
  class?: string;
  enrollmentDate: string;
  parentContact?: string;
  notes?: string;
  gender?: 'K' | 'E';
}
```

**Response:**
```typescript
Student
```

**Example:**
```typescript
const newStudent = await invoke<Student>('create_student', {
  data: {
    name: 'Ahmet',
    surname: 'Yılmaz',
    enrollmentDate: '2025-09-01',
    gender: 'E',
    class: '9-A'
  }
});
```

**Errors:**
- `"Invalid email format"` - Email formatı hatalı
- `"Invalid phone number"` - Telefon numarası hatalı
- `"Database error: ..."` - Veritabanı hatası

---

### `update_student`

Öğrenci bilgilerini günceller.

**Request:**
```typescript
{
  id: string;
  data: {
    name?: string;
    surname?: string;
    email?: string;
    phone?: string;
    birthDate?: string;
    address?: string;
    class?: string;
    parentContact?: string;
    notes?: string;
    gender?: 'K' | 'E';
    risk?: 'Düşük' | 'Orta' | 'Yüksek';
  }
}
```

**Response:**
```typescript
Student
```

---

### `delete_student`

Öğrenciyi siler (soft delete).

**Request:**
```typescript
{
  id: string;
}
```

**Response:**
```typescript
void
```

---

### `search_students`

Öğrencilerde arama yapar.

**Request:**
```typescript
{
  query: string;  // Ad, soyad, sınıf, email'de arama
}
```

**Response:**
```typescript
Student[]
```

**Example:**
```typescript
const results = await invoke<Student[]>('search_students', {
  query: '9-A'
});
```

---

## Counseling Commands

### `get_counseling_sessions`

Tüm görüşme kayıtlarını getirir.

**Request:**
```typescript
{
  studentId?: string;  // Optional filter
}
```

**Response:**
```typescript
CounselingSession[] {
  id: string;
  studentId: string;
  date: string;
  duration: number;
  type: string;
  notes: string;
  counselorId: string;
  created_at: string;
  updated_at: string;
}
```

---

### `create_counseling_session`

Yeni görüşme kaydı oluşturur.

**Request:**
```typescript
{
  studentId: string;
  date: string;
  duration: number;
  type: string;
  notes: string;
  counselorId: string;
}
```

---

### `update_counseling_session`

Görüşme kaydını günceller.

---

### `delete_counseling_session`

Görüşme kaydını siler.

---

## Academic Commands

### `get_academic_records`

Akademik kayıtları getirir.

**Request:**
```typescript
{
  studentId: string;
}
```

**Response:**
```typescript
AcademicRecord[] {
  id: string;
  studentId: string;
  term: string;
  subject: string;
  grade: number;
  examType: string;
  date: string;
  notes?: string;
  created_at: string;
}
```

---

### `create_academic_record`

Yeni akademik kayıt oluşturur.

---

### `get_attendance_records`

Devamsızlık kayıtlarını getirir.

---

### `create_attendance_record`

Devamsızlık kaydı oluşturur.

---

## AI Commands

### `analyze_student_profile`

Öğrenci profilini AI ile analiz eder.

**Request:**
```typescript
{
  studentId: string;
}
```

**Response:**
```typescript
{
  summary: string;
  strengths: string[];
  concerns: string[];
  recommendations: string[];
  riskLevel: 'Düşük' | 'Orta' | 'Yüksek';
}
```

**Example:**
```typescript
const analysis = await invoke('analyze_student_profile', {
  studentId: 'student-uuid'
});
```

---

### `generate_ai_suggestions`

AI önerileri oluşturur.

**Request:**
```typescript
{
  studentId: string;
  context: string;
}
```

**Response:**
```typescript
AiSuggestion[] {
  id: string;
  studentId: string;
  type: string;
  content: string;
  confidence: number;
  created_at: string;
}
```

---

### `chat_with_ai`

AI asistanı ile sohbet eder.

**Request:**
```typescript
{
  messages: Array<{
    role: 'user' | 'assistant';
    content: string;
  }>;
  provider: 'openai' | 'gemini' | 'ollama';
}
```

**Response:**
```typescript
{
  message: string;
  usage: {
    prompt_tokens: number;
    completion_tokens: number;
  };
}
```

---

## Survey Commands

### `get_surveys`

Tüm anketleri listeler.

---

### `create_survey`

Yeni anket oluşturur.

---

### `get_survey_responses`

Anket cevaplarını getirir.

---

### `submit_survey_response`

Anket cevabı gönderir.

---

## Notification Commands

### `get_notifications`

Bildirimleri getirir.

**Request:**
```typescript
{
  userId: string;
  unreadOnly?: boolean;
}
```

---

### `mark_notification_read`

Bildirimi okundu olarak işaretler.

---

### `send_notification`

Sistem bildirimi gönderir (native).

**Request:**
```typescript
{
  title: string;
  body: string;
  icon?: string;
}
```

---

## Settings Commands

### `get_settings`

Uygulama ayarlarını getirir.

**Response:**
```typescript
Settings {
  aiProvider: 'openai' | 'gemini' | 'ollama';
  openaiApiKey?: string;
  geminiApiKey?: string;
  ollamaUrl?: string;
  theme: 'light' | 'dark' | 'system';
  language: 'tr' | 'en';
  notificationsEnabled: boolean;
}
```

---

### `save_settings`

Ayarları kaydeder.

**Request:**
```typescript
Settings
```

---

### `test_ai_connection`

AI bağlantısını test eder.

**Request:**
```typescript
{
  provider: 'openai' | 'gemini' | 'ollama';
}
```

**Response:**
```typescript
{
  success: boolean;
  message: string;
}
```

---

## File Commands

### `upload_file`

Dosya yükler.

**Request:**
```typescript
{
  filename: string;
  data: Uint8Array;
  studentId?: string;
}
```

**Response:**
```typescript
{
  id: string;
  path: string;
  url: string;
}
```

---

### `download_file`

Dosya indirir.

**Request:**
```typescript
{
  fileId: string;
}
```

**Response:**
```typescript
Uint8Array
```

---

### `delete_file`

Dosya siler.

---

### `list_files`

Dosyaları listeler.

**Request:**
```typescript
{
  studentId?: string;
}
```

**Response:**
```typescript
FileInfo[] {
  id: string;
  name: string;
  type: string;
  size: number;
  studentId?: string;
  created_at: string;
}
```

---

## Error Handling

### Error Response Format

Tüm hatalar `String` olarak dönülür:

```typescript
try {
  const student = await invoke('get_student_by_id', { id: 'invalid' });
} catch (error) {
  console.error('Error:', error); // String error message
}
```

### Common Error Messages

- `"Database error: ..."` - Veritabanı hatası
- `"Invalid credentials"` - Kimlik doğrulama hatası
- `"Not found"` - Kayıt bulunamadı
- `"Invalid input: ..."` - Geçersiz girdi
- `"Permission denied"` - Yetki hatası
- `"Session expired"` - Oturum süresi dolmuş

### Frontend Error Handling

```typescript
// React Query ile error handling
const { data, error, isError } = useQuery({
  queryKey: ['students'],
  queryFn: () => invoke<Student[]>('get_all_students'),
  onError: (error) => {
    toast.error(`Hata: ${error}`);
  }
});
```

---

## Rate Limiting

Backend'de rate limiting YOK çünkü desktop uygulaması.  
Frontend tarafında throttle/debounce gerekirse kullanılmalı.

---

## Type Definitions

Tüm TypeScript type definitions:

```typescript
// client/src/types/api.ts
export interface Student { ... }
export interface User { ... }
export interface CounselingSession { ... }
// ... diğer tipler
```

---

**Son Güncelleme:** 31 Ekim 2025  
**API Versiyon:** 2.0.0  
**Toplam Commands:** 85+
