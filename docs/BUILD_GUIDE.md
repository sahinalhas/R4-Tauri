# Rehber360 - Local Build & Test Guide

## Prerequisites

### System Requirements

**Windows:**
- Windows 10/11 (64-bit)
- Visual Studio 2022 Build Tools or Visual Studio 2022
- WebView2 Runtime (usually pre-installed on Windows 11)

**macOS:**
- macOS 10.13 or later
- Xcode Command Line Tools

**Linux:**
- Modern Linux distribution
- WebKit2GTK, GTK3, and various system libraries

### Required Software

1. **Node.js** (v18 or later)
   ```bash
   node --version  # Should be v18+
   npm --version
   ```

2. **Rust** (stable)
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Verify installation
   rustc --version  # Should be 1.70+
   cargo --version
   ```

3. **Git**
   ```bash
   git --version
   ```

---

## Initial Setup

### 1. Clone Repository

```bash
git clone https://github.com/rehber360/rehber360.git
cd rehber360
```

### 2. Install Dependencies

```bash
# Install Node.js dependencies
npm install

# This automatically installs:
# - Frontend dependencies (React, Vite, etc.)
# - Tauri CLI
# - Frontend Tauri plugins
```

### 3. Install Rust Dependencies

```bash
cd src-tauri
cargo fetch  # Pre-download Rust dependencies
cd ..
```

**Note**: First-time Rust build can take 5-10 minutes.

---

## Development Workflow

### Web Development (No Tauri)

Fastest iteration for frontend-only changes:

```bash
npm run dev
```

- Opens: `http://localhost:5000`
- Hot reload enabled
- Backend: Express.js server
- Database: SQLite (local)
- No desktop features (tray, notifications, etc.)

### Tauri Development (Desktop App)

Full desktop app with native features:

```bash
npm run tauri:dev
```

- Opens: Tauri window (desktop app)
- Hot reload enabled (frontend only)
- Backend: Tauri Rust backend
- Database: SQLite (Tauri app data directory)
- Full desktop features enabled

**Important**:
- Rust changes require restart
- Frontend changes hot reload
- First run is slow (Rust compilation)

---

## Build for Production

### Production Build (Desktop)

```bash
npm run tauri:build
```

**Build Process:**
1. Builds optimized frontend (Vite)
2. Compiles Rust backend (release mode)
3. Bundles application
4. Creates installers

**Output Location:**
```
src-tauri/target/release/
├── rehber360.exe           (Windows executable)
└── bundle/
    ├── nsis/
    │   └── Rehber360_2.0.0_x64-setup.exe  (NSIS installer)
    └── msi/
        └── Rehber360_2.0.0_x64_en-US.msi  (MSI installer)
```

### Build Options

**NSIS Installer** (Recommended for distribution):
- Single-file installer
- Auto-updates supported
- Custom install location
- Start menu shortcuts
- Languages: Turkish, English

**MSI Installer** (Enterprise):
- Windows Installer format
- Group Policy deployable
- Silent install support

---

## Testing

### Frontend Testing

```bash
# Run tests once
npm test

# Watch mode
npm run test:watch

# With coverage
npm run test:coverage

# UI mode
npm run test:ui
```

### Rust Testing

```bash
cd src-tauri/core
cargo test

# Specific module
cargo test security

# With output
cargo test -- --nocapture
```

### Type Checking

```bash
# TypeScript type check
npm run typecheck

# Rust type check
cd src-tauri
cargo check
```

---

## Debugging

### Frontend Debugging

**Web Mode**:
- Chrome DevTools: `F12` / `Ctrl+Shift+I`
- React DevTools extension

**Tauri Mode**:
- Right-click → Inspect Element
- Or: `Ctrl+Shift+I` (Windows/Linux), `Cmd+Option+I` (macOS)

### Rust Debugging

**Logs**:
```bash
# Enable debug logs
RUST_LOG=debug npm run tauri:dev

# Specific module
RUST_LOG=rehber360_core=trace npm run tauri:dev
```

**Breakpoints** (VS Code):
1. Install "rust-analyzer" extension
2. Install "CodeLLDB" extension
3. Use `launch.json` configuration:
   ```json
   {
     "type": "lldb",
     "request": "launch",
     "name": "Tauri Development Debug",
     "cargo": {
       "args": ["build", "--manifest-path=src-tauri/app/Cargo.toml"]
     }
   }
   ```

---

## Common Issues & Solutions

### Issue: GUI Build Fails on Replit/Linux Server

**Symptom**: `gobject-2.0` or `gtk-3` not found

**Cause**: Tauri requires GUI libraries for desktop build

**Solution**: Build on local Windows/macOS machine, not on Replit

---

### Issue: WebView2 Not Found (Windows)

**Symptom**: App won't start, WebView2 error

**Solution**:
```bash
# Installer auto-downloads WebView2, or:
# Download manually: https://developer.microsoft.com/microsoft-edge/webview2/
```

---

### Issue: First Build Takes Forever

**Symptom**: `cargo build` stuck at "Compiling..."

**Solution**:
- Normal for first build (5-10 min)
- Compiling 300+ Rust crates
- Subsequent builds are fast (incremental)

---

### Issue: Hot Reload Not Working

**Frontend (Vite)**:
- Check `vite.config.ts` → `server.port: 5000`
- Check firewall/antivirus

**Tauri**:
- Rust changes need full restart
- Only frontend hot reloads

---

### Issue: Database Not Found

**Web Mode**:
- Location: `./database.db` (project root)
- Auto-created on first run

**Tauri Mode**:
- Location: `%APPDATA%/com.rehber360.app/rehber360.db`
- Auto-created on first run
- Each mode has separate database

---

### Issue: Port 5000 Already in Use

**Solution**:
```bash
# Find and kill process
# Windows
netstat -ano | findstr :5000
taskkill /PID <PID> /F

# Linux/macOS
lsof -i :5000
kill -9 <PID>
```

---

## Build Optimization

### Faster Incremental Builds

**Cargo.toml** (already configured):
```toml
[profile.dev]
incremental = true
opt-level = 0

[profile.release]
incremental = false
opt-level = "z"  # Optimize for size
lto = true        # Link-time optimization
```

### Parallel Builds

```bash
# Use all CPU cores (default)
cargo build --release -j $(nproc)
```

### Caching

Rust builds cache in `target/` directory:
- Don't delete `target/` for faster rebuilds
- Can be large (1-2 GB)

---

## Environment Variables

### Development

`.env` file (already created):
```env
# AI Provider (optional)
GOOGLE_API_KEY=your_key_here
OPENAI_API_KEY=your_key_here

# Database (auto-managed)
DATABASE_URL=sqlite:./database.db
```

### Production

Tauri uses secure storage for production secrets (not `.env`).

---

## CI/CD Considerations

### GitHub Actions

Example workflow for automated builds:

```yaml
name: Build Tauri App

on:
  push:
    tags:
      - 'v*'

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - run: npm install
      - run: npm run tauri:build
      - uses: actions/upload-artifact@v3
        with:
          name: windows-installer
          path: src-tauri/target/release/bundle/nsis/*.exe
```

---

## Build Performance Benchmarks

**Development Build** (`tauri:dev`):
- First time: ~8-12 minutes (Rust compilation)
- Incremental: ~10-30 seconds

**Production Build** (`tauri:build`):
- First time: ~15-20 minutes
- Incremental: ~3-5 minutes
- Output size: ~8-12 MB

**Comparison** (vs Electron):
- Build time: ~2x faster (no node_modules bundling)
- Output size: ~15x smaller

---

## Distribution

### Windows Installer

**NSIS** (`.exe`):
- Recommended for end users
- Single file
- Auto-update support
- Install wizard

**MSI** (`.msi`):
- Enterprise deployment
- Group Policy support
- Silent install: `msiexec /i Rehber360.msi /quiet`

### Code Signing (Optional)

For production releases, sign binaries:
```bash
# Windows (requires certificate)
signtool sign /f cert.pfx /p password /t http://timestamp.digicert.com rehber360.exe
```

Unsigned apps show "Unknown Publisher" warning.

---

## Minimum Hardware Requirements

### Development Machine
- CPU: 4 cores (8 recommended)
- RAM: 8 GB (16 GB recommended)
- Disk: 10 GB free (for dependencies)
- SSD recommended (faster builds)

### End User Machine
- CPU: 2 cores
- RAM: 2 GB
- Disk: 50 MB
- Windows 10/11, macOS 10.13+, or modern Linux

---

## Resources

- [Tauri Build Documentation](https://v2.tauri.app/develop/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Vite Documentation](https://vitejs.dev/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
