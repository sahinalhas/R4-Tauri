# Rehber360 - Development Guide

## 🏗️ Architecture Overview

Rehber360 is being migrated from Electron to Tauri for a modern, lightweight desktop application. The project uses a Rust workspace structure:

```
src-tauri/
├── core/          # Backend logic, database, models (Tauri-independent)
├── app/           # Tauri desktop application shell
└── migrations/    # SQLx database migrations
```

### Why This Structure?

- **`rehber360-core`**: Pure Rust library with business logic, can be tested/developed in Replit without GUI dependencies
- **`rehber360-app`**: Tauri desktop wrapper, requires local development environment with GUI libraries

## 🚀 Development Environments

### Replit (Cloud Development)

✅ **What Works:**
- Backend logic development (`rehber360-core`)
- Database migrations and testing
- SQLx queries and models
- Unit tests
- Code refactoring

❌ **What Doesn't Work:**
- Tauri GUI builds (missing WebKitGTK, GDK libraries)
- Desktop application testing
- Full integration tests with UI

**Commands:**
```bash
# Test core logic
cd src-tauri/core
cargo test

# Check compilation
cargo check
```

### Local Development (Windows/Mac/Linux)

✅ **Required for:**
- Building desktop application
- Testing Tauri features (system tray, native notifications, etc.)
- Creating production builds
- Full integration testing

## 📋 Local Setup Requirements

### Prerequisites

#### Windows
1. **Visual Studio C++ Build Tools**: [Download](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. **WebView2**: Already installed on Windows 11; for Windows 10, [download here](https://developer.microsoft.com/microsoft-edge/webview2/)
3. **Rust**: Install via [rustup.rs](https://rustup.rs/)
4. **Node.js**: v18+ [Download](https://nodejs.org/)

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (via Homebrew)
brew install node
```

#### Linux (Debian/Ubuntu)
```bash
# Install system dependencies
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs
```

#### Linux (Fedora/RHEL)
```bash
# Install system dependencies
sudo dnf install webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    libappindicator-gtk3 \
    librsvg2-devel

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
sudo dnf install nodejs
```

### Project Setup

```bash
# Clone and install dependencies
npm install

# Build frontend
npm run build:client

# Run Tauri in development mode
npm run tauri:dev

# Build production desktop app
npm run tauri:build
```

## 🗄️ Database Management

### Migrations

Migrations are located in `src-tauri/core/migrations/` and managed by SQLx:

**Create new migration:**
```bash
cd src-tauri/core
sqlx migrate add migration_name
```

**Run migrations:**
Migrations automatically run when the application starts. For manual execution:
```bash
sqlx migrate run --database-url sqlite:path/to/database.db
```

### Database Location

- **Development**: `~/.local/share/com.rehber360.app/rehber360.db` (Linux/Mac)  
  `%APPDATA%\com.rehber360.app\rehber360.db` (Windows)
- **Testing**: In-memory or test-specific path

## 🧪 Testing

### Core Logic Tests (Replit-compatible)
```bash
cd src-tauri/core
cargo test
```

### Integration Tests (Local only)
```bash
# Run all tests including Tauri integration
cargo test --workspace
```

## 📦 Building for Production

### Development Build
```bash
npm run tauri:dev
```

### Production Build
```bash
# Build optimized desktop app
npm run tauri:build
```

Output locations:
- **Windows**: `src-tauri/target/release/bundle/nsis/` and `.../msi/`
- **macOS**: `src-tauri/target/release/bundle/dmg/` and `.../macos/`
- **Linux**: `src-tauri/target/release/bundle/deb/` and `.../appimage/`

## 🔧 Troubleshooting

### "Failed to run build script for gdk-sys"
**Cause**: Missing GTK/WebKit system libraries.  
**Solution**: Install prerequisites listed above for your OS.

### "Database locked" error
**Cause**: Multiple processes accessing SQLite database.  
**Solution**: Ensure only one app instance is running. Check for zombie processes.

### Frontend not loading
**Cause**: Frontend not built or dev server not configured.  
**Solution**: Run `npm run build:client` before `npm run tauri:dev`.

## 📖 Recommended Workflow

### For Feature Development (Replit)
1. Develop backend logic in `src-tauri/core`
2. Write and run unit tests
3. Test SQLx queries and migrations
4. Commit and push changes

### For Desktop Features (Local)
1. Pull latest changes from Replit
2. Develop Tauri-specific features in `src-tauri/app`
3. Test with `npm run tauri:dev`
4. Build and validate with `npm run tauri:build`
5. Push changes

## 🔐 Security Notes

- Never commit API keys or secrets
- Use environment variables for sensitive data
- Database is local-only (no network exposure by default)
- Tauri security policies enforced via `tauri.conf.json`

## 📞 Support

- **Tauri Documentation**: https://tauri.app/
- **SQLx Documentation**: https://github.com/launchbadge/sqlx
- **Rust Book**: https://doc.rust-lang.org/book/

---

**Status**: 🚧 Migration in Progress  
**Version**: 2.0.0 (Tauri)  
**Previous**: 1.0.0 (Electron)
