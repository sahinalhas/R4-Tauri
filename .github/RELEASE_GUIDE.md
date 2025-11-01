# 🚀 GitHub Actions ile Release ve Build Kılavuzu

Bu kılavuz, Rehber360 projesinde GitHub Actions kullanarak otomatik build ve release işlemlerinin nasıl yapılacağını açıklar.

## 📋 İçindekiler

- [Otomatik Release (Tavsiye Edilen)](#otomatik-release)
- [Manuel Windows Build](#manuel-windows-build)
- [Workflow Dosyaları](#workflow-dosyaları)
- [Sorun Giderme](#sorun-giderme)

---

## 🎯 Otomatik Release

### Adım 1: Version Tag Oluşturma

```bash
# Yeni bir versiyon tag'i oluşturun
git tag v2.0.1

# Tag'i GitHub'a gönderin
git push origin v2.0.1
```

### Adım 2: GitHub Actions İzleme

1. GitHub repository'nize gidin
2. **Actions** sekmesine tıklayın
3. **Release Build** workflow'unun çalıştığını göreceksiniz
4. Build süreci yaklaşık 15-30 dakika sürer

### Adım 3: Release İndirme

1. **Releases** sekmesine gidin (sağ tarafta)
2. Yeni oluşturulan release'i bulun
3. **Assets** bölümünden dosyaları indirin:
   - **Windows**: `Rehber360_2.0.1_x64-setup.exe` (NSIS - Önerilen)
   - **Windows**: `Rehber360_2.0.1_x64_tr-TR.msi` (MSI)
   - **macOS**: `Rehber360_2.0.1.dmg` (Universal Binary)
   - **Linux**: `Rehber360_2.0.1.AppImage` veya `.deb`

---

## ⚡ Manuel Windows Build (Hızlı)

Sadece Windows için hızlı build yapmak isterseniz:

### Workflow Dispatch ile

1. GitHub'da **Actions** → **Windows Build (Fast)** sekmesine gidin
2. **Run workflow** butonuna tıklayın
3. Branch seçin (genelde `main`)
4. **Run workflow** ile başlatın

### Artifacts İndirme

1. Workflow tamamlandığında
2. Workflow detay sayfasına gidin
3. En altta **Artifacts** bölümünü bulun
4. `windows-build-XXXXXXX.zip` dosyasını indirin
5. Zip içinde `.exe` ve `.msi` dosyalarını bulacaksınız

**Not**: Bu artifact'lar 7 gün sonra otomatik silinir.

---

## 📁 Workflow Dosyaları

### 1. `release.yml` - Ana Release Workflow

**Ne yapar?**
- Multi-platform build (Windows, macOS, Linux)
- Otomatik GitHub Release oluşturur
- Tüm platformlar için installer üretir

**Ne zaman çalışır?**
- Version tag push edildiğinde (`v*`)
- Manuel tetikleme ile

**Kullanım:**
```bash
git tag v2.0.1
git push origin v2.0.1
```

### 2. `build-windows.yml` - Windows Hızlı Build

**Ne yapar?**
- Sadece Windows için build
- Artifact olarak kaydeder (release oluşturmaz)
- Daha hızlı tamamlanır (~10-15 dakika)

**Ne zaman çalışır?**
- `main` veya `develop` branch'e push
- Manuel tetikleme ile
- Sadece kod değişikliği olduğunda

### 3. `ci.yml` - Continuous Integration

**Ne yapar?**
- Type checking, linting, testing
- Frontend + Backend kontrolü
- Her commit'te kod kalitesi kontrolü

**Ne zaman çalışır?**
- Her push ve pull request

### 4. `test.yml` - Detaylı Test Suite

**Ne yapar?**
- Frontend unit + integration testleri
- Backend Rust testleri
- Code coverage raporu

**Ne zaman çalışır?**
- Push ve pull request
- Manuel tetikleme ile

---

## 🛠️ Özellikler ve Optimizasyonlar

### ✅ Cache Sistemi

Tüm workflow'larda Rust ve npm cache aktif:
- İlk build: ~20-30 dakika
- Sonraki build'ler: ~10-15 dakika

### ✅ Multi-Platform Desteği

Tek bir tag ile 3 platform için otomatik build:
```
Windows → .exe (NSIS) + .msi
macOS   → .dmg (Universal: Intel + ARM)
Linux   → .AppImage + .deb
```

### ✅ Code Signing Hazır

`release.yml` code signing için hazır. Sadece GitHub Secrets'a ekleyin:
- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

---

## 🔧 Sorun Giderme

### ❌ "Build Failed" Hatası

**1. Rust cache hatası:**
```bash
# Lokal olarak test edin
cd src-tauri/app
cargo clean
cargo build --release
```

**2. Frontend build hatası:**
```bash
# Lokal olarak test edin
npm ci
npm run build:frontend
```

**3. GitHub Actions loglarını inceleyin:**
- Actions sekmesinde failed workflow'a tıklayın
- Kırmızı adımlara bakın
- Hata mesajlarını okuyun

### ❌ "Artifacts Not Found" Hatası

Windows build workflow'unda artifact bulunamıyorsa:
- Build'in başarılı tamamlandığından emin olun
- Workflow loglarında "Upload artifacts" adımını kontrol edin
- 7 günden eski artifact'lar otomatik silinir

### ❌ Permission Hatası

GitHub Actions'da "Permission denied" hatası:
1. Repository **Settings** → **Actions** → **General**
2. **Workflow permissions** → **Read and write permissions** seçin
3. Değişikliği kaydedin

---

## 📦 Build Çıktıları

### Windows

```
src-tauri/app/target/release/bundle/
├── nsis/
│   └── Rehber360_2.0.1_x64-setup.exe    (NSIS Installer)
└── msi/
    └── Rehber360_2.0.1_x64_tr-TR.msi     (MSI Installer)
```

### macOS

```
src-tauri/app/target/release/bundle/
└── dmg/
    └── Rehber360_2.0.1.dmg               (Universal Binary)
```

### Linux

```
src-tauri/app/target/release/bundle/
├── appimage/
│   └── Rehber360_2.0.1.AppImage          (Portable)
└── deb/
    └── rehber360_2.0.1_amd64.deb         (Debian Package)
```

---

## 🎓 En İyi Pratikler

### 1. Version Tag Formatı

✅ **Doğru:**
```bash
git tag v2.0.1
git tag v2.1.0-beta
git tag v3.0.0-rc.1
```

❌ **Yanlış:**
```bash
git tag 2.0.1        # 'v' prefix gerekli
git tag version-2.0  # Geçersiz format
```

### 2. Release Sıklığı

- **Patch** (v2.0.1, v2.0.2): Bug fix'ler için
- **Minor** (v2.1.0, v2.2.0): Yeni özellikler için
- **Major** (v3.0.0): Breaking changes için

### 3. Testing

Release öncesi mutlaka test edin:
```bash
# Lokal test
npm run dev           # Frontend çalışıyor mu?
npm run build         # Build başarılı mı?
npm run test          # Tüm testler geçiyor mu?
```

### 4. Changelog Güncellemesi

Her release öncesi `CHANGELOG.md` güncelleyin:
```markdown
## [2.0.1] - 2025-11-01
### Fixed
- Login hatası düzeltildi
- PDF export sorunu giderildi
```

---

## 🔐 Code Signing (İleride)

### Windows Code Signing

Windows SmartScreen uyarılarını engellemek için:

1. **Sertifika alın** (DigiCert, Sectigo vb.)
2. **GitHub Secrets'a ekleyin:**
   ```
   WINDOWS_CERTIFICATE          # Base64 encoded .pfx
   WINDOWS_CERTIFICATE_PASSWORD # Password
   ```

### macOS Notarization

macOS Gatekeeper için:

1. **Apple Developer hesabı** gerekli
2. **Secrets:**
   ```
   APPLE_CERTIFICATE           # Base64 encoded .p12
   APPLE_CERTIFICATE_PASSWORD  # Password
   APPLE_SIGNING_IDENTITY      # Developer ID
   APPLE_ID                    # Your Apple ID
   APPLE_PASSWORD              # App-specific password
   APPLE_TEAM_ID               # Team ID
   ```

---

## 📞 Yardım

Sorun yaşarsanız:
1. Bu kılavuzu tekrar okuyun
2. GitHub Actions loglarını inceleyin
3. [Tauri v2 Docs](https://v2.tauri.app) kontrol edin
4. Issue açın veya team'e bildirin

---

**Son Güncelleme**: 2025-11-01  
**Tauri Version**: 2.1  
**Node Version**: 20.x  
**Rust Version**: Stable (latest)
