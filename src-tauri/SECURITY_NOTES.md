# Security Notes - Rehber360 Tauri

## ⚠️ Important Security Considerations

### 1. API Key Storage

**Current Status (Development):**
- AI provider API keys are stored in **plaintext JSON** in `{app_data_dir}/settings.json`
- This is acceptable for **development/testing** on trusted local machines
- File permissions are managed by the operating system

**Production Requirements:**
API keys MUST be moved to secure storage before production deployment.

**Recommended Solutions:**

#### Option A: Tauri Secure Storage (Recommended)
```rust
// Use Tauri's secure storage plugin
use tauri_plugin_store::StoreBuilder;

let store = StoreBuilder::new(app, "secure.dat")
    .build()
    .expect("Failed to build secure store");

// Store API key securely
store.insert("openai_api_key".to_string(), api_key)?;

// Retrieve API key
let api_key: String = store.get("openai_api_key")?;
```

**Benefits:**
- Encrypted at rest
- Platform-specific secure storage (Windows Credential Manager, macOS Keychain, Linux Secret Service)
- Integrated with Tauri

**Implementation:**
```toml
# Add to Cargo.toml
[dependencies]
tauri-plugin-store = "2.0"
```

#### Option B: OS Keychain (Native)
Use platform-specific keychain libraries:

**Windows:** Windows Credential Manager
```rust
use keyring::Entry;

let entry = Entry::new("Rehber360", "openai_api_key")?;
entry.set_password(&api_key)?;
let stored_key = entry.get_password()?;
```

**macOS:** macOS Keychain
**Linux:** GNOME Keyring / Secret Service

**Benefits:**
- Native OS-level encryption
- Requires user authentication for access
- Best security practice

**Implementation:**
```toml
[dependencies]
keyring = "2.3"
```

#### Option C: Environment Variables (Less Secure)
For development/testing only:
```bash
# Set environment variables
OPENAI_API_KEY=sk-...
GEMINI_API_KEY=...

# Access in Rust
use std::env;
let api_key = env::var("OPENAI_API_KEY")?;
```

⚠️ Not recommended for production distribution

---

### 2. File Operations Security

**Status:** ✅ Secure

File upload/download commands are hardened against path traversal attacks:

✅ **Implemented Protections:**
- Strict filename validation (alphanumeric + dash + underscore + dot only)
- Rejection of absolute paths, "..", and drive letters
- UUID-based unique filenames
- Path sandboxing to `{app_data_dir}/uploads/`
- Double validation with `starts_with()` checks

**Code Location:** `src-tauri/app/src/commands/file.rs`

---

### 3. Database Security

**Status:** ✅ Secure

- SQLite database with SQLx compile-time query checking
- All queries use prepared statements (no SQL injection risk)
- Password hashing with bcrypt
- Database file located in protected app data directory

**File Path:** `{app_data_dir}/rehber360.db`

---

### 4. Network Security

**AI HTTP Clients:**
- OpenAI API: HTTPS only
- Gemini API: HTTPS only
- Ollama: HTTP localhost (local-only, acceptable)

**Recommendations:**
- Always use HTTPS for external APIs
- Validate SSL certificates
- Implement request timeouts
- Add retry logic with exponential backoff

---

### 5. Desktop Application Security

**Tauri Security Features:**
- `nodeIntegration: false` (disabled Node.js in renderer)
- `contextIsolation: true` (isolated execution contexts)
- CSP (Content Security Policy) headers
- IPC message validation
- Allowlist-based API access

**Current Configuration:**
```json
{
  "security": {
    "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline';"
  }
}
```

---

## 🔒 Security Checklist for Production

Before deploying to production, ensure:

- [ ] **API keys moved to secure storage** (Tauri plugin or OS keychain)
- [ ] Remove plaintext API key storage from settings.json
- [ ] Enable HTTPS for all external API calls
- [ ] Implement rate limiting for API requests
- [ ] Add request/response logging (without exposing secrets)
- [ ] Set up automatic security updates
- [ ] Code signing for Windows executable
- [ ] Notarization for macOS app
- [ ] Regular security audits
- [ ] Penetration testing

---

## 🐛 Reporting Security Issues

**DO NOT** open public GitHub issues for security vulnerabilities.

Instead, contact:
- Email: security@rehber360.com (if available)
- Private disclosure via GitHub Security Advisories

---

## 📚 Resources

- [Tauri Security Best Practices](https://tauri.app/v1/guides/development/security)
- [OWASP Desktop App Security](https://owasp.org/www-project-desktop-app-security/)
- [Tauri Secure Storage Plugin](https://github.com/tauri-apps/tauri-plugin-store)
- [Keyring Rust Crate](https://docs.rs/keyring/)

---

*Last Updated: October 31, 2025*
*Version: 2.0.0 (Tauri Migration)*
