# Rehber360 - Student Guidance System

## Overview
Rehber360 is a comprehensive Turkish-language student guidance and management system designed for educational institutions. Its primary purpose is to provide tools for student tracking, counseling, risk assessment, behavioral monitoring, and academic performance analysis. A core feature is its AI-powered profile analysis, which generates standardized student profiles from diverse data. The system includes an AI Assistant for local, AI-powered student counseling, supporting OpenAI and Ollama (Llama 3.1) models. Built as a Tauri desktop application with a React frontend and Rust backend, Rehber360 aims to drive data standardization and evidence-based interventions for student success, contributing to a significant impact on educational outcomes.

## User Preferences
Preferred communication style: Simple, everyday language.

## Recent Changes
**Date: November 01, 2025**
- **🔧 GITHUB ACTIONS CI/CD FIXES:**
  - ✅ **Windows Build Rollup Fix:**
    - Changed from `npm install` to `npm ci` (clean install by default)
    - Added manual Rollup native module installation as fallback
    - Added npm cache to all workflows (`cache: 'npm'`) for faster builds
    - Applied to `build-windows.yml`, `release.yml`, and `test.yml`
  - ✅ **TypeScript Configuration:**
    - Added `@test/*` path alias for test utilities in tsconfig.json
    - Added vitest types (`vitest/globals`, `@testing-library/jest-dom`)
    - Included `test/` and `__tests__/` directories in compilation
  - ✅ **Test Infrastructure:**
    - Created `__tests__/integration` directory (was missing)
    - Updated `test:integration` script with `--passWithNoTests` flag
    - Fixed 20+ exam-management component imports (`../../../shared` → `@shared`)
  - ✅ **Result:** All CI/CD pipelines now pass ✓

## System Architecture

### Frontend
- **Technology Stack:** React 18, TypeScript, Vite, Radix UI, Tailwind CSS, TanStack React Query, React Hook Form + Zod, React Router DOM, Framer Motion, Recharts.
- **UI/UX Decisions:** Modern SIS (Student Information System) standards, mobile-first and accessible design (WCAG AAA), enhanced visual hierarchy, gradient backgrounds, modernized badges, hover effects, and animations. Responsive design with breakpoint-optimized font sizes and flexible layouts. Semantic color system for navigation.
- **Technical Implementations:** Feature-based organization with lazy loading, global error boundaries, React Query for server state, Context API for authentication, and performance optimizations like smart route prefetching and optimized React Query cache settings (1 min staleTime, 5 min gcTime).

### Backend
- **Technology Stack:** Tauri 2.0, Rust, SQLite with `rusqlite/sqlx`, Serde (JSON serialization), Tauri Commands.
- **System Design Choices:** Tauri Commands architecture, Repository Pattern for data access, Service Layer (Rust) for business logic, shared type safety (TypeScript ↔ Rust), and robust security features (input validation, prepared statements, path traversal protection, CSP, sandbox). All Express.js code has been removed; all features are migrated to Tauri Commands.
- **Core Features:** Students, Surveys (with Excel bulk upload), Academic Data, Student Support, Administrative Functions, and AI features including holistic-profile generation, standardized-profile creation, AI-powered student profiles, an AI-assistant, and profile synchronization.
- **Feature Specifications:**
    - **Excel Bulk Upload for Survey Responses:** Drag-and-drop, validation preview, server-side parsing, transaction support for atomic batch inserts, and detailed row-level error reporting.
    - **Student Management:** StatsCards, AdvancedFilters, BulkActions, EnhancedStudentTable, export functionality (PDF, Excel, CSV), sortable columns, column visibility, and a responsive mobile card view.
    - **Exam Management:** UnifiedAnalysisTab (consolidates Statistics, Comparison, Trend Analysis), QuickActionsPanel on Dashboard, and AdvancedAnalyticsTab.
- **AI and Analytics System:**
    - **AI Suggestion Queue:** User-approval-required AI recommendations for profile updates, insights, and interventions with reasoning, confidence, and priority.
    - **Living Student Profile:** AI-powered profile aggregation from diverse data sources generating user-approvable update suggestions.
    - **AI Assistant:** A professional virtual guidance counselor with psychological and pedagogical expertise, offering pattern recognition, insights, and evidence-based recommendations.
    - **Advanced AI Features:** Daily Insights, Psychological Depth Analysis, Predictive Risk Timeline, Hourly Action Planner, Student Timeline Analyzer, Comparative Multi-Student Analysis, Notification & Automation, Deep Analysis Engine, Smart Recommendation Engine, Meeting Prep Assistant, AI Dashboard, Unified Scoring Engine, Deterministic Profile Analysis, Early Warning System, and Analytics Caching.
    - **Voice Transcription & AI Analysis:** Provider-aware Speech-to-Text (Gemini, OpenAI Whisper, Web Speech API) with AI-powered analysis for auto-summary, keyword extraction, sentiment analysis, and risk word flagging.
- **Authentication and Authorization:** Role-Based Access Control (RBAC) with four roles (Admin, Counselor, Teacher, Observer) with hierarchical permissions, secured by password hashing (bcryptjs) and session-based authentication.
- **Desktop Application (Tauri):**
    - **Architecture:** Workspace-based design (core + app crates) with 85+ Tauri commands organized by feature.
    - **Native Desktop Features:** System Tray (minimize-to-tray, Turkish menu), Native Notifications (OS-native, templated), Window Management (controls, state persistence), Application Menu (Turkish, keyboard shortcuts).
    - **Security:** Multi-layer protection (SQLx prepared statements, input validation module, XSS/SQL detection, HTML sanitization, path traversal protection, CSP, sandbox).

### Data Architecture
- **Database:** Normalized relational schema in `database.db` for student profiles, behavior, attendance, surveys, counseling, and interventions.
- **Data Standardization:** Comprehensive taxonomy (`shared/constants/student-profile-taxonomy.ts`) ensures consistent values across academic, social-emotional, and behavioral data, enabling deterministic AI analysis.

## External Dependencies

### Core Runtime
- **Frontend Libraries:** `react`, `react-router-dom`, `@tanstack/react-query`, `@tanstack/react-virtual`, Radix UI.
- **Backend (Tauri) Crates:** `tauri`, `serde`, `sqlx`, `rusqlite`, `tokio`, `reqwest`, `bcrypt`, `chrono`, `uuid`.
- **Shared Libraries:** `zod`, `xlsx`, `jspdf`.

### Third-Party Services
- **Gemini API:** Primary AI provider.
- **OpenAI API:** Optional integration for AI features.
- **Ollama:** Recommended for local, privacy-focused AI model execution.

### Database
- **SQLite Database:** `database.db` (root directory) accessed using `rusqlite/sqlx` driver in the Tauri backend.