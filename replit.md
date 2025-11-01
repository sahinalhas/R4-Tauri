# Rehber360 - Student Guidance System

## Overview
Rehber360 is a comprehensive Turkish-language student guidance and management system for educational institutions. It offers tools for student tracking, counseling, risk assessment, behavioral monitoring, and academic performance analysis. A core feature is its AI-powered profile analysis, which generates standardized student profiles from diverse data. The system includes an AI Assistant for local, AI-powered student counseling, supporting OpenAI and Ollama (Llama 3.1) models. Built as a Tauri desktop application with React frontend and Rust backend, Rehber360 aims to drive data standardization and evidence-based interventions for student success.

## Recent Changes
**Date: November 01, 2025 (Latest)**
- **🚀 GITHUB ACTIONS CI/CD PIPELINE PRODUCTION READY:**
  - ✅ **Multi-Platform Release Workflow** (`release.yml`):
    - Windows (.exe + .msi), macOS (.dmg Universal), Linux (.AppImage + .deb)
    - Tag-based otomatik release (örn: `v2.0.1`)
    - Code signing desteği hazır (secrets gerekli)
    - Rust cache optimizasyonu (`./src-tauri -> target`)
    - Platform-specific dependencies otomatik kurulum
  - ✅ **Windows Fast Build** (`build-windows.yml`):
    - Sadece Windows için hızlı build (~10-15 dakika)
    - Artifact upload (7 gün saklanır)
    - Push veya manuel tetikleme
    - Cache-optimized
  - ✅ **CI Pipeline** (`ci.yml`):
    - TypeScript type checking, ESLint, Prettier
    - Frontend + Backend testleri
    - Rust clippy + cargo check
    - Coverage reports (Codecov)
  - ✅ **Test Suite** (`test.yml`):
    - Frontend unit + integration testleri
    - Backend Rust testleri (cargo test)
    - Code quality kontrolü (fmt, clippy)
    - Detaylı coverage raporları
  - ✅ **Dokümantasyon:**
    - `.github/RELEASE_GUIDE.md` (detaylı kullanım kılavuzu)
    - `.github/QUICK_START.md` (3 adımda release)
  - 🎯 **Kullanım:** `git tag v2.0.1 && git push origin v2.0.1`
  - ⚡ **Sonuç:** Tek tag ile 3 platform için otomatik build ve release

**Date: October 31, 2025**
- **🎉 TAURI-NATIVE CODEBASE - SON TEMİZLİK TAMAMLANDI:**
  - ✅ **Kök Dizin Temizliği:**
    - `mebbis.md` (Chrome extension planı) kaldırıldı
    - `backups/` (boş klasör) silindi
    - `scripts/` (web build artifacts) silindi
  - ✅ **Config Dosyaları Tauri-Native:**
    - `vite.config.ts`: Replit HMR ayarları kaldırıldı, localhost strict
    - `vitest.config.ts`: server/ referansları → src-tauri/ ile değiştirildi
    - `package.json`: Scripts sadeleştirildi (dev, build = Tauri komutları)
  - ✅ **Documentation 100% Tauri:**
    - README.md: Tüm Electron/Express/web/PWA referansları temizlendi
    - Tauri desktop app vurgusu eklendi (Windows/macOS/Linux)
    - Build/deployment instructions Tauri-native
  - ✅ **Sonuç:** %100 Tauri desktop application, sıfır web/Electron kalıntısı
  
**Date: October 31, 2025**
- **🎉 FRONTEND → TAURI ENTEGRASYONU TAMAMLANDI:**
  - ✅ Frontend %100 Tauri kullanıyor (Express plugin kaldırıldı)
  - ✅ HTTP transport kaldırıldı (~100 satır temizlendi)
  - ✅ Desktop-only yaklaşım, web mode yok
  - ✅ Tauri transport layer (40+ endpoint mapping)
  - ✅ Intelligent parameter extraction
  - ✅ Mimari: Frontend → Tauri Transport → Rust Commands → SQLite

**Date: October 31, 2025**
- **🎉 TAURI MIGRATION COMPLETE - FAZ 1-10 TAMAMLANDI:**
  - ✅ **Final Cleanup (FAZ 10):**
    - All Electron code removed
    - All Express backend removed (server/ folder 2.6MB)
    - Frontend Electron references cleaned
    - Package.json optimized (18 unused dependencies removed)
    - **Result:** Clean, Tauri-only codebase
  - ✅ **TAURI IS NOW THE ONLY BACKEND**
  - ✅ **Project is now 100% Tauri Desktop Application**
  
**Previous Migration Phases:**
- **Tauri Migration - FAZ 1-7 (Complete):**
  - ✅ Workspace infrastructure (core + app crates)
  - ✅ Database layer (SQLx, 8 migrations, 9 models, 8 repositories)
  - ✅ 85+ Tauri commands (auth, students, counseling, academic, AI, surveys, notifications, settings, files)
  - ✅ AI Services integration (OpenAI, Gemini, Ollama HTTP clients)
  - ✅ Settings/Config management system
  - ✅ File operations with security hardening
  - ✅ Type-safe frontend API client (lazy Tauri loading)
  - ✅ **Native Desktop Features (FAZ 5):**
    - System Tray (minimize-to-tray, Türkçe menu)
    - Native Notifications (OS-native, templated)
    - Window Management (controls, state persistence)
    - Application Menu (Türkçe, keyboard shortcuts)
    - 6 Tauri plugins configured (notification, store, dialog, positioner, process, updater)
  - ✅ **Security Hardening (FAZ 6):**
    - Input validation module (email, phone, student ID, filename)
    - XSS/SQL injection detection
    - HTML sanitization
    - Turkish character support
    - Comprehensive test suite (14 tests)
  - ✅ **Documentation (FAZ 7):**
    - TAURI_NATIVE_FEATURES.md (detailed feature guide)
    - BUILD_GUIDE.md (local build/test guide)
    - AUTO_UPDATER_SETUP.md (future enhancement guide)
  - 🔒 **Security:** Multi-layer protection (SQLx, input validation, path traversal, CSP, sandbox)
  - ⚠️ **Security Note:** API keys in plaintext for development - production requires Tauri Secure Storage
  - 🚀 **Ready for Local Build:** All features production-ready except auto-updater (requires local signing key setup)
  - 📦 **Auto-Updater:** Scoped out (requires local setup: signing keys, manifest hosting, release signing) - template and documentation ready
  - 📖 **Documentation:** Complete technical documentation for all features


**Date: October 29, 2025**
- **Page Navigation Performance Optimizations:**
  - Implemented smart route prefetching that automatically loads likely next pages based on current location
  - Added hover prefetching to navigation menu for instant page transitions
  - Optimized React Query cache settings (1 min staleTime, 5 min gcTime) for balanced freshness and performance
  - Page transitions now feel instant due to prefetched code and cached data
  - Note: High-volatility views (notifications, AI status) may need per-query cache overrides in the future


## User Preferences
Preferred communication style: Simple, everyday language.

## System Architecture

### Frontend
- **Technology Stack:** React 18, TypeScript, Vite, Radix UI, Tailwind CSS, TanStack React Query, React Hook Form + Zod, React Router DOM, Framer Motion, Recharts.
- **Key Decisions:** Feature-based organization with lazy loading, global error boundaries, mobile-first and accessible design (WCAG AAA), React Query for server state, Context API for authentication, and performance optimizations.
- **Performance Optimizations:**
  - Smart route prefetching: Automatically prefetches likely next pages based on user's current location
  - Hover prefetching: Prefetches routes when user hovers over navigation links
  - Optimized React Query cache: 1-minute staleTime with 5-minute garbage collection for fast page transitions while maintaining data freshness
  - Lazy-loaded route components for optimal bundle splitting
- **UI/UX Decisions:** Modern SIS (Student Information System) standards, enhanced visual hierarchy with text-3xl headers, gradient backgrounds, modernized badges, hover effects, and animations. Responsive design with breakpoint-optimized font sizes and flexible layouts. Semantic color system for navigation.
- **Technical Implementations:** Component architecture follows modern best practices with separation of concerns. Proper props flow (studentId, studentName, onUpdate) across components. Grid systems configured for responsive layouts. Loading states and error handling implemented.

### Backend
- **Technology Stack:** Tauri 2.0, Rust, SQLite with `rusqlite/sqlx`, Serde (JSON serialization), Tauri Commands.
- **Key Decisions:** Tauri Commands architecture, Repository Pattern for data access, Service Layer (Rust) for business logic, shared type safety (TypeScript ↔ Rust), robust security (input validation, prepared statements, path traversal protection, CSP, sandbox), file operations with security hardening, and centralized error handling with transaction support.
- **Migration Complete:** All Express.js code removed, all features migrated to Tauri Commands.
- **Core Features:** Students, Surveys (with Excel bulk upload), Academic Data, Student Support, Administrative Functions, and AI features (holistic-profile, standardized-profile, student-profile-ai, ai-assistant, profile-sync).
- **Feature Specifications:**
    - **Excel Bulk Upload for Survey Responses:** Drag-and-drop, validation preview, server-side parsing, transaction support for atomic batch inserts, detailed row-level error reporting.
    - **Student Management:** StatsCards, AdvancedFilters, BulkActions, EnhancedStudentTable, export functionality (PDF, Excel, CSV), sortable columns, column visibility, compact view, sticky header, responsive mobile card view, student quick preview drawer.
    - **Exam Management:** UnifiedAnalysisTab (consolidates Statistics, Comparison, Trend Analysis), QuickActionsPanel on Dashboard, AdvancedAnalyticsTab with accordion groups.

### Data Architecture
- **Database:** Normalized relational schema in `database.db` for student profiles, behavior, attendance, surveys, counseling, and interventions.
- **Data Standardization:** Comprehensive taxonomy (`shared/constants/student-profile-taxonomy.ts`) for consistent values across academic, social-emotional, and behavioral data, enabling deterministic AI analysis.

### AI and Analytics System
- **AI Suggestion Queue System:** User-approval-required AI recommendation system for profile updates, insights, and interventions with reasoning, confidence, and priority.
- **Living Student Profile:** AI-powered profile aggregation from diverse data sources to generate user-approvable profile update suggestions.
- **AI Assistant:** Professional virtual guidance counselor with psychological and pedagogical expertise, offering pattern recognition, insights, and evidence-based recommendations. Supports OpenAI, Ollama, and Gemini.
- **Advanced AI Features:** Daily Insights, Psychological Depth Analysis, Predictive Risk Timeline, Hourly Action Planner, Student Timeline Analyzer, Comparative Multi-Student Analysis, Notification & Automation, Deep Analysis Engine, Smart Recommendation Engine, Meeting Prep Assistant, AI Dashboard, Unified Scoring Engine, Deterministic Profile Analysis, Early Warning System, and Analytics Caching.
- **Voice Transcription & AI Analysis:** Provider-aware STT (Gemini, OpenAI Whisper, Web Speech API) with AI-powered analysis for auto-summary, keyword extraction, sentiment analysis, and risk word flagging.
- **Enhanced Text Input:** `EnhancedTextarea` with integrated voice input and Gemini-powered text enhancement.

### Authentication and Authorization
- **Role-Based Access Control (RBAC):** Four roles (Admin, Counselor, Teacher, Observer) with hierarchical permissions.
- **Security:** Password hashing (bcryptjs), session-based authentication, and permission guards.

### Build and Deployment
- **Build Process:** Tauri desktop application build with Rust backend.
- **Deployment:** Platform-specific installers (MSI/DMG/DEB) for Windows/macOS/Linux.
- **Database:** File-based SQLite (`database.db`) with SQLx and automatic migrations.

### Desktop Application (Tauri)
- **Technology Stack:** Tauri 2.0, Rust, SQLite (rusqlite/sqlx), Serde, 6 Tauri plugins.
- **Architecture:** Workspace-based design (core + app crates) with 85+ Tauri commands organized by feature.
- **Security:** Multi-layer protection (SQLx prepared statements, input validation module, XSS/SQL detection, HTML sanitization, path traversal protection, CSP, sandbox).
- **Features:**
  - 🗃️ **Database:** SQLite with SQLx, 8 migrations, 9 models, 8 repositories
  - 🤖 **AI Services:** Gemini, OpenAI, Ollama HTTP clients with secure API key management
  - ⚙️ **Settings:** Type-safe configuration management (Tauri Store)
  - 📁 **File Operations:** Secure file handling with path validation and sanitization
  - 🔔 **Native Notifications:** OS-native notifications with templated messages
  - 🪟 **Window Management:** Window controls, state persistence, minimize-to-tray
  - 🎯 **System Tray:** Quick access menu with Turkish language support
  - 📋 **Application Menu:** Turkish menu with keyboard shortcuts
  - 🔄 **Auto-Update:** Template ready (requires local signing key setup)
  - 🔐 **Security:** Comprehensive input validation (email, phone, student ID, filename, Turkish characters)
- **Commands:** 85+ Tauri commands covering auth, students, counseling, academic, AI, surveys, notifications, settings, files.
- **Development:** `npm run tauri:dev` builds frontend and launches Tauri dev mode.
- **Production:** `npm run tauri:build` packages platform-specific installers (MSI/DMG/DEB).
- **Note:** Desktop build requires local environment (Tauri GUI dependencies not available in Replit).

## External Dependencies

### Core Runtime
- **Frontend:** `react`, `react-router-dom`, `@tanstack/react-query`, `@tanstack/react-virtual`, Radix UI.
- **Backend (Tauri):** Rust crates: `tauri`, `serde`, `sqlx`, `rusqlite`, `tokio`, `reqwest`, `bcrypt`, `chrono`, `uuid`.
- **Shared:** `zod`, `xlsx`, `jspdf`.
- **Removed:** `express`, `better-sqlite3`, `bcryptjs`, `cors`, `dotenv`, `openai`, `@google/genai`, `multer`, `cookie-parser`, `csrf-csrf`, `express-rate-limit`, `serverless-http`, `sharp`, `form-data`.

### Third-Party Services
- **Gemini API:** Primary AI provider.
- **OpenAI API:** Optional integration for AI features.
- **Ollama:** Recommended for local, privacy-focused AI.

### Database
- **SQLite Database:** `database.db` (root directory) using `rusqlite/sqlx` driver in Tauri backend.