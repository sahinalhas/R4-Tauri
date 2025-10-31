# Rehber360 - Auto-Updater Setup Guide

## ⚠️ Critical Security Notice

**Auto-updater is currently DISABLED** in `tauri.conf.json` because it requires proper signing key setup to prevent unsigned/malicious updates.

**DO NOT ENABLE** without completing all setup steps below.

---

## Prerequisites

- Tauri CLI installed: `npm install -g @tauri-apps/cli`
- Access to a server for hosting update manifests
- GitHub repository with releases enabled

---

## Step 1: Generate Signing Key Pair

Generate an ed25519 key pair for signing updates:

```bash
# Generate key pair (private key saved to file)
tauri signer generate -w ~/.tauri/rehber360.key

# Output will show the PUBLIC KEY - copy this
# Example output:
# Generated new private key ~/.tauri/rehber360.key
# Public key: dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6ABCD...
```

**⚠️ IMPORTANT:**
- **NEVER commit the private key** to git
- Store private key securely (password manager, secrets vault)
- Only share public key (goes in `tauri.conf.json`)

---

## Step 2: Update tauri.conf.json

Add the PUBLIC KEY to `tauri.conf.json`:

```json
{
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://releases.rehber360.com/latest.json"
      ],
      "dialog": true,
      "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6ABCD...",
      "windows": {
        "installMode": "passive"
      }
    }
  }
}
```

---

## Step 3: Create Update Manifest

Create a JSON file following Tauri's updater schema:

**File: `latest.json`** (host this on your server)

```json
{
  "version": "2.0.1",
  "notes": "### What's New\n- Bug fixes\n- Performance improvements\n- New features",
  "pub_date": "2025-01-15T10:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "SIGNATURE_GENERATED_IN_STEP_4",
      "url": "https://github.com/rehber360/rehber360/releases/download/v2.0.1/Rehber360_2.0.1_x64-setup.nsis.zip"
    }
  }
}
```

**Schema Explanation:**
- `version`: Semver version (must be > current version)
- `notes`: Markdown release notes
- `pub_date`: ISO 8601 timestamp
- `platforms`: Per-platform download URLs and signatures

---

## Step 4: Sign Release Bundles

After building the release (`npm run tauri:build`), sign the bundle:

### Windows NSIS Installer

```bash
# Navigate to bundle directory
cd src-tauri/target/release/bundle/nsis

# Sign the installer ZIP (not the .exe, the .zip Tauri creates)
tauri signer sign Rehber360_2.0.1_x64-setup.nsis.zip -k ~/.tauri/rehber360.key

# Output will show the SIGNATURE - copy this
# Example output:
# Signature: dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNpZ25pbmcga2V5...
```

**Update manifest:**
Copy the signature to `latest.json`:
```json
{
  "platforms": {
    "windows-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNpZ25pbmcga2V5...",
      "url": "https://github.com/rehber360/rehber360/releases/download/v2.0.1/Rehber360_2.0.1_x64-setup.nsis.zip"
    }
  }
}
```

---

## Step 5: Host Update Manifest

Upload `latest.json` to a publicly accessible HTTPS URL:

### Option A: GitHub Pages
```bash
# Create gh-pages branch
git checkout --orphan gh-pages
echo '{ "version": "2.0.1", ... }' > latest.json
git add latest.json
git commit -m "Update manifest"
git push origin gh-pages

# Enable GitHub Pages in repo settings
# URL: https://rehber360.github.io/rehber360/latest.json
```

### Option B: Custom Server
- Upload to `https://releases.rehber360.com/latest.json`
- Ensure HTTPS and CORS enabled
- Update `tauri.conf.json` endpoints

---

## Step 6: Upload Signed Bundle to GitHub Releases

1. Go to GitHub Releases: `https://github.com/rehber360/rehber360/releases`
2. Create new release: `v2.0.1`
3. Upload the **signed NSIS ZIP**:
   ```
   src-tauri/target/release/bundle/nsis/Rehber360_2.0.1_x64-setup.nsis.zip
   ```
4. **Do NOT upload unsigned .exe files**
5. Publish release

---

## Step 7: Test Update Flow

### Manual Test

```bash
# In development build (old version)
npm run tauri:dev

# Open DevTools, run:
import { check } from '@tauri-apps/plugin-updater';
const update = await check();
console.log(update);

# Should return update info if manifest version > current version
```

### End-to-End Test

1. Install old version (e.g., 2.0.0) on test machine
2. Update manifest to 2.0.1
3. Launch app
4. Wait for auto-check (6 hours) or trigger manually
5. Verify:
   - Update notification appears
   - Download progress works
   - Signature verification passes
   - Relaunch installs new version

---

## Automated CI/CD Setup

### GitHub Actions Workflow

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-and-release:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install dependencies
        run: npm install
      
      - name: Build Tauri app
        run: npm run tauri:build
      
      - name: Sign bundle
        env:
          TAURI_PRIVATE_KEY: ${{ secrets.TAURI_PRIVATE_KEY }}
        run: |
          echo "$TAURI_PRIVATE_KEY" > private.key
          tauri signer sign src-tauri/target/release/bundle/nsis/*.nsis.zip -k private.key
          rm private.key
      
      - name: Generate manifest
        run: |
          # Script to generate latest.json with signature
      
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            src-tauri/target/release/bundle/nsis/*.nsis.zip
      
      - name: Deploy manifest to GitHub Pages
        # Upload latest.json to gh-pages branch
```

**Setup GitHub Secrets:**
- `TAURI_PRIVATE_KEY`: Your private key content (from `~/.tauri/rehber360.key`)

---

## Security Best Practices

### Key Management
- ✅ Private key stored in secrets vault (GitHub Secrets, AWS Secrets Manager, etc.)
- ✅ Private key NEVER committed to git
- ✅ Public key in `tauri.conf.json` (safe to commit)
- ✅ Key rotation plan (generate new key yearly)

### Manifest Security
- ✅ Manifest served over HTTPS
- ✅ Manifest URL never changes (stable endpoint)
- ✅ Manifest signed with same private key
- ✅ Signature verified before installation

### Bundle Security
- ✅ Only upload signed bundles to releases
- ✅ Verify signature locally before upload
- ✅ Delete unsigned .exe files from release
- ✅ Code signing certificate (optional, for Windows SmartScreen)

---

## Troubleshooting

### Error: "Signature verification failed"
- **Cause**: Public key in `tauri.conf.json` doesn't match private key used to sign
- **Fix**: Re-generate signature with correct private key

### Error: "Invalid updater manifest"
- **Cause**: Endpoint returns GitHub REST API response instead of Tauri manifest
- **Fix**: Host proper `latest.json` manifest, update endpoints

### Error: "Version X is not greater than current version Y"
- **Cause**: Manifest version ≤ installed version
- **Fix**: Bump version in manifest and `Cargo.toml`

### No update check happens
- **Cause**: `active: false` or invalid endpoint
- **Fix**: Set `active: true`, verify endpoint URL

---

## Production Checklist

Before enabling auto-updater in production:

- [ ] Private key generated and stored securely
- [ ] Public key added to `tauri.conf.json`
- [ ] Update manifest created and hosted (HTTPS)
- [ ] Release bundle signed with private key
- [ ] Signature added to manifest
- [ ] Manifest URL stable and accessible
- [ ] `active: true` in `tauri.conf.json`
- [ ] End-to-end test completed successfully
- [ ] CI/CD pipeline configured (optional)
- [ ] Key rotation plan documented
- [ ] Incident response plan (compromised key)

---

## Resources

- [Tauri Updater Plugin](https://v2.tauri.app/plugin/updater/)
- [Tauri Signer](https://v2.tauri.app/reference/cli/#signer)
- [Updater Manifest Schema](https://v2.tauri.app/plugin/updater/#manifest)
- [GitHub Actions Example](https://github.com/tauri-apps/tauri-action)
