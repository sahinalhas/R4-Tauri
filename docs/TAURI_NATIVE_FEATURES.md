# Rehber360 - Tauri Native Desktop Features

## Overview

Rehber360'ın Tauri versiyonu, modern masaüstü uygulama yetenekleri sunar. Bu dok man, tüm native özelliklerin kullanımını ve entegrasyonunu açıklar.

## Table of Contents

- [System Tray](#system-tray)
- [Native Notifications](#native-notifications)
- [Window Management](#window-management)
- [Window State Persistence](#window-state-persistence)
- [Application Menu](#application-menu)
- [Auto-Updater](#auto-updater)
- [Frontend Integration](#frontend-integration)

---

## System Tray

### Description
System tray icon ile minimize-to-tray özelliği. Uygulama kapatıldığında tray'de çalışmaya devam eder.

### Features
- **Left Click**: Window göster/gizle toggle
- **Double Click**: Window göster ve odaklan
- **Right Click**: Türkçe context menü
  - Göster
  - Gizle
  - Ayarlar
  - Hakkında
  - Çıkış

### Implementation
Rust tarafında `src-tauri/app/src/main.rs`:
```rust
TrayIconBuilder::new()
    .menu(&tray_menu)
    .tooltip("Rehber360")
    .on_menu_event(|app, event| { /* ... */ })
    .on_tray_icon_event(|tray, event| { /* ... */ })
    .build(app)?;
```

### Close Behavior
Window kapatma butonu (X) uygulamayı sonlandırmaz, tray'e minimize eder:
```rust
window.on_window_event(|event| {
    if let WindowEvent::CloseRequested { api, .. } = event {
        event.window().hide().unwrap();
        api.prevent_close();
    }
});
```

---

## Native Notifications

### Description
OS-native bildirimler (Windows Action Center, macOS Notification Center).

### Usage (Frontend)

```typescript
import { sendNativeNotification } from '@/services/desktop-integration';

// Basic notification
await sendNativeNotification({
  title: 'Yeni Öğrenci',
  body: 'Ahmet Yılmaz sisteme eklendi'
});

// Templated notification
import { sendTemplatedNotification } from '@/services/desktop-integration';

await sendTemplatedNotification('studentAdded', 'Ahmet Yılmaz');
await sendTemplatedNotification('riskAlert', 'Mehmet Kaya', 'Yüksek');
```

### Available Templates
- `studentAdded(studentName)` - Yeni öğrenci eklendiğinde
- `sessionReminder(sessionType, time)` - Görüşme hatırlatması
- `aiSuggestion(studentName, type)` - AI önerisi
- `riskAlert(studentName, level)` - Risk uyarısı (sesli)
- `taskDue(taskTitle)` - Görev hatırlatması
- `updateAvailable(version)` - Güncelleme mevcut

### Permission Request
İlk kullanımda otomatik izin ister:
```typescript
import { requestNotificationPermission } from '@/services/desktop-integration';

const granted = await requestNotificationPermission();
```

---

## Window Management

### Description
Programatik window kontrolü: minimize, maximize, fullscreen, pozisyon, boyut.

### Usage (Frontend)

```typescript
import { WindowManager } from '@/services/desktop-integration';

// Minimize to taskbar
await WindowManager.minimize();

// Maximize
await WindowManager.maximize();

// Toggle fullscreen
await WindowManager.toggleFullscreen();

// Hide to tray
await WindowManager.hide();

// Show and focus
await WindowManager.show();
await WindowManager.focus();

// Set size
await WindowManager.setSize(1200, 800);

// Center window
await WindowManager.center();

// Check state
const isMaximized = await WindowManager.isMaximized();
const isFullscreen = await WindowManager.isFullscreen();
```

### Keyboard Shortcuts
Otomatik olarak aktif:
- **F11**: Fullscreen toggle
- **Ctrl+M / Cmd+M**: Minimize

---

## Window State Persistence

### Description
Window pozisyonu, boyutu ve state'i otomatik kaydedilir ve uygulama yeniden başlatıldığında geri yüklenir.

### Auto-Save
Her değişiklikten 500ms sonra otomatik kaydedilir:
- Window position (x, y)
- Window size (width, height)
- Maximized state
- Fullscreen state

### Storage
Tauri Store plugin kullanır: `window-state.json`

### Manual Control

```typescript
import { WindowStateManager } from '@/services/desktop-integration';

// Manual save
await WindowStateManager.saveState();

// Manual restore
await WindowStateManager.restoreState();

// Reset to defaults
await WindowStateManager.resetToDefault();

// Clear saved state
await WindowStateManager.clearState();
```

### Initialization
App başlangıcında otomatik çalışır:
```typescript
import { initializeDesktopIntegration } from '@/services/desktop-integration';

// This includes window state persistence
await initializeDesktopIntegration();
```

---

## Application Menu

### Description
Native application menu bar (Windows: title bar altında, macOS: screen üstünde).

### Menu Structure (Turkish)

**Dosya**
- Yeni Öğrenci (Ctrl+N)
- İçe Aktar
- Dışa Aktar
- ---
- Tercihler
- ---
- Çıkış (Ctrl+Q)

**Görünüm**
- Öğrenciler
- Görüşmeler
- Analitik
- ---
- Tam Ekran (F11)

**Yardım**
- Dokümantasyon
- Güncellemeleri Kontrol Et
- ---
- Hakkında

### Implementation
Rust tarafında `main.rs`:
```rust
let file_menu = SubmenuBuilder::new(app, "Dosya")
    .text("new_student", "Yeni Öğrenci")
    .text("import", "İçe Aktar")
    /* ... */
    .build()?;
```

### Menu Actions
Frontend'de otomatik routing:
- "Yeni Öğrenci" → `#/students/new`
- "Tam Ekran" → Fullscreen toggle
- "Güncellemeleri Kontrol Et" → Auto-updater trigger

---

## Auto-Updater

### Description
GitHub Releases üzerinden otomatik güncelleme. Kullanıcıya bildirim gösterir, indirir ve yeniden başlatma seçeneği sunar.

### Configuration
`tauri.conf.json`:
```json
{
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://api.github.com/repos/rehber360/rehber360/releases/latest"
      ],
      "dialog": true
    }
  }
}
```

### Usage (Frontend)

```typescript
import { 
  checkForUpdates, 
  downloadAndInstall,
  setupPeriodicUpdateChecks 
} from '@/services/desktop-integration';

// Manual check
const updateInfo = await checkForUpdates();
if (updateInfo) {
  console.log(`Update available: ${updateInfo.version}`);
}

// Download and install
await downloadAndInstall((progress) => {
  console.log(`Download progress: ${progress}%`);
});

// Auto-check every 6 hours
setupPeriodicUpdateChecks();
```

### Auto-Check
Varsayılan olarak:
- App başlangıcında kontrol
- Her 6 saatte bir otomatik kontrol

### Update Flow
1. Güncelleme tespit edilir
2. Kullanıcıya dialog gösterilir
3. Kullanıcı onaylarsa indirilir
4. İndirme tamamlandığında yeniden başlatma teklifi

### GitHub Release Format
Release notları `body` alanında gösterilir. Tag formatı: `v2.0.0`

---

## Frontend Integration

### Unified Initialization

Tüm desktop features'ı tek seferde başlat:

```typescript
// client/src/main.tsx or App.tsx
import { initializeDesktopIntegration } from '@/services/desktop-integration';

// On app startup (only runs in desktop mode)
initializeDesktopIntegration().then(() => {
  console.log('Desktop features initialized');
});
```

Bu otomatik olarak başlatır:
- Window keyboard shortcuts
- Window state persistence
- Periodic update checks

### Conditional Desktop Features

```typescript
import { WindowManager } from '@/services/desktop-integration';

// Check if running in desktop mode
if (WindowManager.isDesktopMode()) {
  // Desktop-only features
  await WindowManager.requestAttention();
} else {
  // Web fallback
  document.title = 'New Message!';
}
```

### Web Fallback
Tüm desktop servisleri web modunda güvenli bir şekilde devre dışı kalır:
- `isTauriApp()` kontrolü
- Hata fırlatmaz, sessizce atlar
- Console uyarıları verir

---

## Testing & Debugging

### Development Mode
```bash
# Frontend (web)
npm run dev

# Tauri (desktop)
npm run tauri:dev
```

### Build
```bash
# Production build
npm run tauri:build
```

Çıktılar:
- Windows: `.exe` installer (NSIS)
- `.msi` installer

### Logs
**Development**: Console'da
**Production**: `%APPDATA%/com.rehber360.app/logs/`

---

## Security Considerations

1. **Permissions**: Minimal permissions kullanılıyor
2. **CSP**: Sıkı Content Security Policy
3. **IPC**: Tüm Tauri commands type-safe
4. **Updates**: GitHub releases HTTPS üzerinden
5. **Store**: Local storage, şifreli değil (hassas veri için uygun değil)

---

## Troubleshooting

### System Tray Icon Görünmüyor
- Windows: Taskbar settings → "Select which icons appear"
- İkon dosyaları eksikse default icon kullanılır

### Bildirimler Çalışmıyor
- Windows Settings → Notifications → Rehber360 enabled olmalı
- İlk kullanımda permission request onaylanmalı

### Güncelleme Başarısız
- GitHub releases kontrol et
- Network bağlantısı kontrol et
- Firewall/Antivirus ayarları

### Window State Geri Yüklenmiyor
- `window-state.json` dosyasını sil ve yeniden başlat
- Location: `%APPDATA%/com.rehber360.app/window-state.json`

---

## Best Practices

1. **Desktop First**: Desktop özellikleri önce test et, sonra web fallback ekle
2. **User Consent**: Update ve notification için kullanıcı onayı al
3. **Graceful Degradation**: Web modunda özellikler devre dışıysa alternatif sun
4. **Error Handling**: Desktop API hataları için try-catch kullan
5. **Performance**: Window state auto-save debounced (500ms)

---

## Roadmap

Gelecekte eklenebilecek özellikler:
- [ ] Global keyboard shortcuts (Tauri 2.x)
- [ ] System theme detection
- [ ] Multi-window support
- [ ] Drag & drop file handling
- [ ] Clipboard integration
- [ ] System idle detection

---

## Resources

- [Tauri 2.x Documentation](https://v2.tauri.app/)
- [System Tray Guide](https://v2.tauri.app/learn/system-tray/)
- [Plugin Notification](https://v2.tauri.app/plugin/notification/)
- [Plugin Updater](https://v2.tauri.app/plugin/updater/)
- [Plugin Store](https://v2.tauri.app/plugin/store/)
