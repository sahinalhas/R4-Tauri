# Rehber360 - Electron Desktop Build Guide

## 📋 Genel Bakış

Bu doküman, Rehber360 web uygulamasının Windows masaüstü uygulamasına dönüştürülmesi için gereken build sürecini açıklar.

## 🔧 Sistem Gereksinimleri

### Windows Build için:
- **İşletim Sistemi:** Windows 10/11 veya Linux (Wine ile)
- **Node.js:** v20.x veya üzeri
- **RAM:** Minimum 8 GB (16 GB önerilir)
- **Disk Alanı:** Minimum 5 GB boş alan

### Geliştirme Ortamı:
```bash
node --version  # v20.19.3 veya üzeri
npm --version   # v10.x veya üzeri
```

## 🚀 Build Süreci

### 1. Bağımlılıkları Yükle

```bash
npm install
```

### 2. Full Production Build

Build süreci 3 aşamadan oluşur:

```bash
# Adım 1: React Frontend Build (Vite)
npm run build:client
# Çıktı: dist/spa/

# Adım 2: Express Backend Build (TypeScript + Vite)
npm run build:server
# Çıktı: dist/server/

# Adım 3: Electron Main/Preload Build (TypeScript)
npm run build:electron
# Çıktı: dist/electron/

# Veya tek komutla:
npm run build
```

### 3. Electron Installer Oluşturma

#### Windows Installer (NSIS + Portable):
```bash
npm run electron:build:win
```

**Çıktılar** (`release/` klasöründe):
- `Rehber360-Setup-1.0.0.exe` - NSIS Installer (önerilen)
- `Rehber360-Portable-1.0.0.exe` - Portable versiyon

#### Unpacked Directory (Test için):
```bash
npm run electron:build:dir
```

**Çıktı:** `release/win-unpacked/` klasöründe paketlenmemiş dosyalar

## 📦 Build Çıktıları

### Klasör Yapısı:
```
dist/
├── spa/                 # React frontend (Vite build)
│   ├── index.html
│   ├── assets/
│   └── ...
├── server/              # Express backend (TypeScript compiled)
│   ├── node-build.mjs
│   ├── modules/
│   └── ...
└── electron/            # Electron main process
    ├── main.js
    ├── preload.js
    ├── ipc/
    └── ...

release/
├── Rehber360-Setup-1.0.0.exe       # Windows installer
├── Rehber360-Portable-1.0.0.exe    # Portable version
└── win-unpacked/                    # Unpacked files (test)
```

## 🔍 Build Doğrulama

### TypeScript Derleme Kontrolü:
```bash
# LSP diagnostics kontrol
npm run typecheck

# Electron TypeScript build
npm run build:electron

# Build çıktılarını kontrol
ls -lh dist/electron/
```

### Build Çıktılarını Test Etme:
```bash
# Unpacked build'i test et
npm run electron:build:dir
cd release/win-unpacked
./Rehber360.exe
```

## ⚙️ electron-builder Konfigürasyonu

Build ayarları `electron-builder.json` dosyasında tanımlıdır:

```json
{
  "appId": "com.rehber360.app",
  "productName": "Rehber360",
  "directories": {
    "output": "release",
    "buildResources": "build"
  },
  "win": {
    "target": ["nsis", "portable"],
    "icon": "build/icon.png"
  },
  "nsis": {
    "oneClick": false,
    "allowToChangeInstallationDirectory": true,
    "createDesktopShortcut": true,
    "language": "1055"
  }
}
```

## 🎨 Icon Dosyaları

Icon dosyaları `build/` klasöründe:

- `icon.png` (512x512) - Ana uygulama ikonu
- `icon-256.png` (256x256) - Windows ICO için
- `tray.png` (16x16) - System tray icon
- `tray@2x.png` (32x32) - Retina tray icon
- `installerHeader.png` (150x57) - NSIS installer header
- `installerSidebar.png` (164x314) - NSIS installer sidebar

**Icon'ları yeniden oluşturmak için:**
```bash
cd build
node generate-icons.cjs
```

## 🐛 Sorun Giderme

### Memory Hatası (ENOMEM, Killed):
```bash
# Node.js memory limitini artır
export NODE_OPTIONS="--max-old-space-size=8192"
npm run electron:build:win
```

### Build Temizleme:
```bash
# Tüm build çıktılarını temizle
rm -rf dist release

# Node modules temizle (gerekirse)
rm -rf node_modules package-lock.json
npm install
```

### Linux/Mac'te Windows Build:
```bash
# Wine gerekebilir (Linux)
sudo apt-get install wine64

# Build
npm run electron:build:win
```

## 📊 Build Performansı

Ortalama build süreleri (16 GB RAM, SSD):

| Build Type | Süre |
|-----------|------|
| `build:client` | ~45 saniye |
| `build:server` | ~30 saniye |
| `build:electron` | ~10 saniye |
| `electron:build:win` (full) | ~3-5 dakika |
| `electron:build:dir` (unpacked) | ~2 dakika |

## 🔐 Code Signing (Opsiyonel)

Production release için code signing önerilir:

```json
// electron-builder.json
{
  "win": {
    "certificateFile": "path/to/certificate.pfx",
    "certificatePassword": "${env.WINDOWS_CERTIFICATE_PASSWORD}",
    "signingHashAlgorithms": ["sha256"]
  }
}
```

## 📝 Development Build

Development modunda Electron çalıştırmak için:

```bash
# İlk seferde backend ve electron build et
npm run build:server
npm run build:electron

# Development modunda çalıştır
npm run electron:dev
```

**Not:** Development modunda frontend hot-reload aktif (Vite HMR).

## 🚢 Release Checklist

Build öncesi kontrol listesi:

- [ ] `package.json` versiyonu güncellendi
- [ ] CHANGELOG.md hazırlandı
- [ ] Tüm testler geçti (`npm test`)
- [ ] LSP hataları yok (`npm run typecheck`)
- [ ] Build temiz (`rm -rf dist release`)
- [ ] Icon dosyaları güncel (`build/`)
- [ ] `database.db` güncel (demo data)
- [ ] Build başarılı (`npm run electron:build:win`)
- [ ] Installer test edildi (temiz Windows)
- [ ] Portable versiyon test edildi

## 💡 İpuçları

1. **Incremental Build:** Sadece değişen kısımları build etmek için:
   ```bash
   npm run build:client  # Sadece frontend değişti ise
   npm run build:electron  # Sadece Electron kodu değişti ise
   ```

2. **Cache Temizleme:** Build sorunları yaşarsanız:
   ```bash
   rm -rf node_modules/.vite
   rm -rf dist
   ```

3. **Hızlı Test:** Unpacked build daha hızlıdır:
   ```bash
   npm run electron:build:dir
   ```

## 📚 Ek Kaynaklar

- [Electron Builder Dokümantasyonu](https://www.electron.build/)
- [Vite Build Dokümantasyonu](https://vite.dev/guide/build.html)
- [TypeScript Compiler Seçenekleri](https://www.typescriptlang.org/tsconfig)

---

**Son Güncelleme:** 30 Ekim 2025
**Versiyon:** 1.0.0
