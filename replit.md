# Rehber360 - Student Guidance System

## Overview
Rehber360 is a comprehensive Turkish-language student guidance and management system for educational institutions. It offers tools for student tracking, counseling, risk assessment, behavioral monitoring, and academic performance analysis. A core feature is its AI-powered profile analysis, which generates standardized student profiles from diverse data. The system includes an AI Assistant for local, AI-powered student counseling, supporting OpenAI and Ollama (Llama 3.1) models. Built as a full-stack TypeScript application with React, Express.js, and SQLite, Rehber360 aims to drive data standardization and evidence-based interventions for student success.

## Recent Changes
**Date: October 31, 2025**
- **Tauri Migration - FAZ 1-4 Tamamlandı:**
  - ✅ Workspace infrastructure (core + app crates)
  - ✅ Database layer (SQLx, 8 migrations, 9 models, 8 repositories)
  - ✅ 85+ Tauri commands (auth, students, counseling, academic, AI, surveys, notifications, settings, files)
  - ✅ AI Services integration (OpenAI, Gemini, Ollama HTTP clients)
  - ✅ Settings/Config management system
  - ✅ File operations with security hardening
  - ✅ Type-safe frontend API client (lazy Tauri loading)
  - 🔒 **Security:** Path traversal protection implemented (UUID filenames, strict validation, sandbox)
  - ⚠️ **Security Note:** API keys in plaintext for development - production requires Tauri Secure Storage (documented in SECURITY_NOTES.md)
  - 📖 **Documentation:** DEVELOPMENT.md and SECURITY_NOTES.md created
  - 🚧 **Next:** Native desktop features (system tray, notifications), production security hardening

**Date: October 30, 2025**
- **Güncelleme ve Yayınlama Stratejisi Dokümante Edildi:**
  - Oluşturulan doküman: docs/GUNCELLEME_STRATEJISI.md
  - Geliştirme akışı: Replit (web) ve lokal (desktop) paralel geliştirme
  - Güncelleme süreci: Semantic versioning, GitHub Releases, otomatik güncelleme
  - Auto-updater zaten kurulu ve çalışır (electron-updater, 2 saatte bir kontrol)
  - Kod tabanı birleşik: Web ve desktop aynı kodu kullanır, değişiklikler otomatik yansır
- **Electron Desktop Application - Core Infrastructure Completed (Task 2):**
  - Created modular Electron architecture with 9 core files and 4 IPC handler modules
  - Implemented logger system (electron-log) with file/console transports
  - Built type-safe settings store (electron-store) for window state, theme, language, notifications
  - Developed IPC handlers: database backup/restore, file operations, window controls, native notifications
  - Created Turkish application menu with keyboard shortcuts (Ctrl+N, Ctrl+I, Ctrl+E, F11, etc.)
  - Implemented system tray with minimize-to-tray support and quick access menu
  - Integrated auto-updater (electron-updater) with automatic checks every 2 hours
  - Enhanced main.ts: Express backend child process, port management (3000-9000), CSP security headers
  - Expanded preload.ts: 50+ secure IPC methods exposed via contextBridge
  - Fixed critical notification bug: replaced ipcMain.emit with shared createAndShowNotification helper
  - All security best practices: nodeIntegration=false, contextIsolation=true, preload-only API exposure
  - LSP: No errors, Build: Successful, Architect: Approved ✓

**Date: October 29, 2025**
- **Page Navigation Performance Optimizations:**
  - Implemented smart route prefetching that automatically loads likely next pages based on current location
  - Added hover prefetching to navigation menu for instant page transitions
  - Optimized React Query cache settings (1 min staleTime, 5 min gcTime) for balanced freshness and performance
  - Page transitions now feel instant due to prefetched code and cached data
  - Note: High-volatility views (notifications, AI status) may need per-query cache overrides in the future

**Date: October 28, 2025**
- Imported GitHub repository into Replit environment
- Configured development workflow with Vite dev server on port 5000
- Database automatically initialized with demo user (rehber@okul.edu.tr / demo)
- All 71 career profiles seeded successfully
- Background schedulers configured (analytics, auto-complete, daily action plans)
- Deployment configuration set up for production (VM target with build and start scripts)
- AI Provider initialized with Ollama (llama3 model)
- **Modernized CSRF Protection (Modern Best Practice):**
  - Migrated from complex csrf-csrf token-based system to modern SameSite cookie approach
  - Removed CSRF token endpoint and manual token management
  - Simplified client-side API client (no token interceptors needed)
  - Added credentials: 'include' to fetch for proper cookie handling
  - Eliminated "invalid csrf token" errors completely
  - Security maintained through SameSite=Lax cookies + CORS (97%+ browser support)
  - Note: Authentication uses JSON responses (no auth cookies), so CSRF risk is minimal

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
- **Technology Stack:** Express.js v5, SQLite with `better-sqlite3`, TypeScript, Zod, Multer.
- **Key Decisions:** Modular architecture, Repository Pattern for data access, Service Layer for business logic, shared type safety, robust security (input sanitization, prepared statements, CORS, rate limiting), file upload validation, and centralized error handling with transaction support.
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
- **Build Process:** Two-stage build (client and server) using Vite.
- **Deployment Target:** Replit VM, running `dist/server/production.mjs` on port 3000.
- **Database:** File-based SQLite (`database.db`) with automatic backups and schema migrations.

### Desktop Application (Electron)
- **Technology Stack:** Electron, electron-log, electron-store, electron-updater, TypeScript.
- **Architecture:** Modular design with separated IPC handlers (database, file, window, notifications).
- **Security:** nodeIntegration=false, contextIsolation=true, CSP headers, preload-only API exposure.
- **Features:**
  - Express backend integration via child process with dynamic port allocation (3000-9000)
  - Native notification system with user preferences and click handlers
  - Database backup/restore with timestamp-based file management
  - File operations: select, save, read, shell integration (open path, trash)
  - Window controls: minimize, maximize, close, bounds persistence
  - System tray with minimize-to-tray and quick navigation menu
  - Turkish application menu with full keyboard shortcuts
  - Auto-update system with progress tracking and user notifications
  - Centralized logging (userData/logs/main.log)
  - Type-safe settings store (window state, theme, language, notifications, backups)
- **IPC Communication:** 50+ typed methods exposed via contextBridge for secure renderer-main interaction.
- **Development:** `npm run electron:dev` builds TypeScript and launches Electron with dev server.
- **Production:** `npm run electron:build` packages Windows installer (NSIS + Portable).

## External Dependencies

### Core Runtime
- **Frontend:** `react`, `react-router-dom`, `@tanstack/react-query`, `@tanstack/react-virtual`, Radix UI.
- **Backend:** `express`, `better-sqlite3`, `bcryptjs`, `cors`, `dotenv`.
- **Shared:** `zod`, `xlsx`, `jspdf`.

### Third-Party Services
- **Gemini API:** Primary AI provider.
- **OpenAI API:** Optional integration for AI features.
- **Ollama:** Recommended for local, privacy-focused AI.

### Database
- **SQLite Database:** `database.db` (root directory) using `better-sqlite3` driver.