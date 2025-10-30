# Rehber360 - Electron Masaüstü Uygulaması Dönüşüm Planı

Bu doküman, Rehber360 web uygulamasının tam özellikli bir Windows masaüstü uygulamasına dönüştürülmesi için gereken tüm görevleri içerir.

## 📋 Görev Listesi

### 1. Proje Kurulumu ve Bağımlılıklar

#### 1.1 Electron Bağımlılıklarını Yükle
```bash
npm install --save-dev electron electron-builder electron-devtools-installer
npm install --save electron-is-dev electron-log electron-store
```

**Paketlerin Açıklaması:**
- `electron`: Ana Electron framework
- `electron-builder`: Windows .exe ve installer oluşturma
- `electron-devtools-installer`: React DevTools entegrasyonu
- `electron-is-dev`: Development/production modu tespiti
- `electron-log`: Dosya bazlı loglama
- `electron-store`: Kullanıcı ayarlarını kaydetme

#### 1.2 TypeScript Tipleri
```bash
npm install --save-dev @types/electron-devtools-installer
```

---

### 2. Proje Yapısı Düzenleme

#### 2.1 Yeni Klasörler Oluştur
```
electron/
├── main.ts              # Ana process (Node.js ortamı)
├── preload.ts           # Güvenli köprü script
├── menu.ts              # Uygulama menüleri
├── tray.ts              # Sistem tepsisi
├── updater.ts           # Auto-update
├── ipc/                 # IPC handlers
│   ├── database.ts      # SQLite işlemleri
│   ├── file.ts          # Dosya işlemleri
│   └── window.ts        # Pencere kontrolleri
└── types/
    └── electron.d.ts    # TypeScript tipleri
```

#### 2.2 Build Çıktı Klasörleri
```
dist/
├── electron/            # Derlenmiş Electron kodu (main.js, preload.js)
├── spa/                 # Derlenmiş React frontend (Vite build çıktısı)
└── server/              # Derlenmiş Express backend (node-build.mjs + modules)
```

---

### 3. Ana Process (main.ts) Yapılandırması

#### 3.1 Temel Pencere Oluşturma
**Dosya:** `electron/main.ts`

**Görevler:**
- Electron app lifecycle yönetimi
- BrowserWindow oluşturma (1280x800, minWidth: 1024)
- Development/production URL yükleme
- Pencere durumu kaydetme ve geri yükleme
- DevTools otomatik açma (dev mode)
- Pencere olayları (close, minimize, maximize)

**Özellikler:**
```typescript
const mainWindow = new BrowserWindow({
  width: 1280,
  height: 800,
  minWidth: 1024,
  minHeight: 768,
  frame: true,
  backgroundColor: '#ffffff',
  icon: path.join(__dirname, '../renderer/icon.png'),
  webPreferences: {
    nodeIntegration: false,      // Güvenlik
    contextIsolation: true,      // Güvenlik
    preload: path.join(__dirname, 'preload.js'),
    webSecurity: true,
    devTools: isDev
  }
})
```

#### 3.2 SQLite Veritabanı Yönetimi
**Dosya:** `electron/ipc/database.ts`

**Görevler:**
- Veritabanı dosya konumu belirleme (userData dizini)
- İlk açılışta database.db kopyalama
- Better-sqlite3 bağlantısı kurma
- IPC handlers ekleme:
  - `db:query` - SQL sorgular
  - `db:backup` - Yedekleme
  - `db:restore` - Geri yükleme
  - `db:export` - Export işlemleri

**Veritabanı Konumu:**
```typescript
import { app } from 'electron'
import path from 'path'

const dbPath = path.join(app.getPath('userData'), 'database.db')
```

#### 3.3 Express Backend Entegrasyonu
**Görevler:**
- Express server'ı child process olarak başlatma
- Rastgele port seçme (3000-9000 arası)
- Frontend'e port bilgisi iletme
- Server kapanışını app quit ile senkronize etme

**ÖNEMLI:** node-build.mjs bir executable script olduğu için child process olarak çalıştırılmalı.

**Implementasyon (electron/main.ts):**
```typescript
import { app } from 'electron'
import path from 'path'
import { spawn, ChildProcess } from 'child_process'
import isDev from 'electron-is-dev'
import log from './logger'

let serverPort: number = 3000
let serverProcess: ChildProcess | null = null

function startBackend(): Promise<void> {
  return new Promise((resolve, reject) => {
    // Build edilmiş server script yolu
    const serverScript = isDev 
      ? path.join(__dirname, '../../dist/server/node-build.mjs')
      : path.join(process.resourcesPath, 'dist/server/node-build.mjs')
    
    // node-build.mjs zaten schedulers'ı başlatıyor
    serverProcess = spawn('node', [serverScript], {
      env: {
        ...process.env,
        PORT: serverPort.toString(),
        NODE_ENV: isDev ? 'development' : 'production'
      },
      stdio: ['ignore', 'pipe', 'pipe']
    })
    
    serverProcess.stdout?.on('data', (data) => {
      const output = data.toString()
      log.info(`[Backend] ${output}`)
      
      // Server başladığını algıla
      if (output.includes('running on port')) {
        resolve()
      }
    })
    
    serverProcess.stderr?.on('data', (data) => {
      log.error(`[Backend Error] ${data.toString()}`)
    })
    
    serverProcess.on('error', (error) => {
      log.error('Backend process error:', error)
      reject(error)
    })
    
    serverProcess.on('exit', (code) => {
      log.info(`Backend process exited with code ${code}`)
      serverProcess = null
    })
    
    // 5 saniye timeout
    setTimeout(() => {
      if (serverProcess && !serverProcess.killed) {
        resolve() // Server büyük ihtimalle başladı
      }
    }, 5000)
  })
}

function stopBackend() {
  if (serverProcess && !serverProcess.killed) {
    serverProcess.kill('SIGTERM')
    
    // Eğer 5 saniyede kapanmazsa zorla kapat
    setTimeout(() => {
      if (serverProcess && !serverProcess.killed) {
        serverProcess.kill('SIGKILL')
      }
    }, 5000)
  }
}
```

---

### 4. Preload Script (preload.ts)

#### 4.1 Güvenli API Köprüsü
**Dosya:** `electron/preload.ts`

**Görevler:**
- contextBridge ile güvenli API expose etme
- IPC invoke metodları
- Type-safe API tanımları

**API İşlevleri:**
```typescript
// Pencere kontrolleri
window.electronAPI = {
  minimize: () => ipcRenderer.invoke('window:minimize'),
  maximize: () => ipcRenderer.invoke('window:maximize'),
  close: () => ipcRenderer.invoke('window:close'),
  
  // Sistem bilgileri
  getAppVersion: () => ipcRenderer.invoke('app:getVersion'),
  getPlatform: () => ipcRenderer.invoke('app:getPlatform'),
  
  // Dosya işlemleri
  selectFile: (options) => ipcRenderer.invoke('file:select', options),
  saveFile: (data, filename) => ipcRenderer.invoke('file:save', data, filename),
  openExternal: (url) => ipcRenderer.invoke('shell:openExternal', url),
  
  // Veritabanı
  dbBackup: () => ipcRenderer.invoke('db:backup'),
  dbRestore: (path) => ipcRenderer.invoke('db:restore', path),
  
  // Bildirimler
  showNotification: (title, body) => ipcRenderer.invoke('notification:show', title, body),
  
  // Backend port
  getServerPort: () => ipcRenderer.invoke('server:getPort')
}
```

#### 4.2 TypeScript Tipleri
**Dosya:** `electron/types/electron.d.ts`

```typescript
export interface ElectronAPI {
  minimize: () => Promise<void>
  maximize: () => Promise<void>
  close: () => Promise<void>
  getAppVersion: () => Promise<string>
  getPlatform: () => Promise<string>
  selectFile: (options: any) => Promise<string | null>
  saveFile: (data: any, filename: string) => Promise<boolean>
  openExternal: (url: string) => Promise<void>
  dbBackup: () => Promise<string>
  dbRestore: (path: string) => Promise<boolean>
  showNotification: (title: string, body: string) => Promise<void>
  getServerPort: () => Promise<number>
}

declare global {
  interface Window {
    electronAPI: ElectronAPI
  }
}
```

---

### 5. Frontend (React) Entegrasyonu

#### 5.1 Electron Algılama
**Dosya:** `client/utils/electron.ts`

```typescript
export const isElectron = () => {
  return !!(window && window.electronAPI)
}

export const getBackendUrl = async () => {
  if (isElectron()) {
    const port = await window.electronAPI.getServerPort()
    return `http://127.0.0.1:${port}`
  }
  return '' // Web'de relative URL
}
```

#### 5.2 API Client Güncelleme
**Dosya:** `client/lib/api/core/client.ts`

**Görevler:**
- Electron'da backend URL'ini dinamik al
- Web'de mevcut relative URL kullan
- ApiClient class'ına baseURL desteği ekle

```typescript
import { getBackendUrl, isElectron } from '@/utils/electron'

class ApiClient {
  private baseURL: string = '';
  
  async initialize() {
    if (isElectron()) {
      this.baseURL = await getBackendUrl()
    }
  }
  
  async request<TResponse = unknown, TBody = unknown>(
    endpoint: string,
    config: ApiRequestConfig<TBody> = {},
    isRetry = false
  ): Promise<TResponse> {
    // URL'yi baseURL ile birleştir
    const url = this.baseURL ? `${this.baseURL}${endpoint}` : endpoint
    
    // ...mevcut kod
  }
}

// Initialize on app start
export const apiClient = new ApiClient()
if (isElectron()) {
  apiClient.initialize()
}
```

#### 5.3 Özel Pencere Kontrolleri (Opsiyonel)
**Dosya:** `client/components/ElectronTitleBar.tsx`

**Görevler:**
- Custom title bar komponenti
- Minimize, maximize, close butonları
- Uygulama adı ve versiyonu gösterimi

---

### 6. Uygulama Menüsü

#### 6.1 Menü Yapısı
**Dosya:** `electron/menu.ts`

**Menüler:**
```
Dosya
├── Yeni Öğrenci
├── Veri İçe Aktar
├── Veri Dışa Aktar
├── ---
├── Ayarlar
├── ---
├── Çıkış

Düzenle
├── Geri Al
├── İleri Al
├── ---
├── Kes
├── Kopyala
├── Yapıştır

Görünüm
├── Ana Sayfa
├── Öğrenciler
├── Sınavlar
├── Anketler
├── Raporlar
├── ---
├── Tam Ekran
├── Yenile
├── DevTools (dev mode only)

Veritabanı
├── Yedek Al
├── Yedek Geri Yükle
├── Export (Excel/PDF)

Yardım
├── Dokümantasyon
├── Sürüm Notları
├── Hakkında
```

#### 6.2 Klavye Kısayolları
- `Ctrl+N`: Yeni öğrenci
- `Ctrl+S`: Kaydet
- `Ctrl+P`: Yazdır
- `Ctrl+F`: Ara
- `Ctrl+R` / `F5`: Yenile
- `Ctrl+Q`: Çıkış
- `F11`: Tam ekran
- `Ctrl+Shift+I`: DevTools (dev mode)

---

### 7. Sistem Tepsisi (System Tray)

#### 7.1 Tray İkonu ve Menü
**Dosya:** `electron/tray.ts`

**Görevler:**
- Uygulama ikonu oluşturma (16x16, 32x32 PNG)
- Tray menüsü
- Tray tıklama olayları
- Balon bildirimleri

**Tray Menüsü:**
```
📊 Rehber360
├── Göster
├── ---
├── Hızlı Erişim
│   ├── Öğrenciler
│   ├── Sınavlar
│   └── Raporlar
├── ---
├── Ayarlar
├── Çıkış
```

**Davranışlar:**
- Pencere kapatıldığında sistem tepsisine gitme (minimize to tray)
- Tray ikonuna çift tıklayınca pencereyi gösterme
- Balon bildirimler (yeni risk öğrenci, eksik veri vb.)

---

### 8. Bildirimler (Notifications)

#### 8.1 Native Bildirimler
**Dosya:** `electron/ipc/notifications.ts`

**Bildirim Tipleri:**
- Yeni risk öğrenci tespiti
- Eksik veri uyarıları
- Günlük rapor hazır
- Sistem yedekleme tamamlandı
- Güncelleme mevcut

**Implementasyon:**
```typescript
import { Notification } from 'electron'

export function showNotification(title: string, body: string, type?: string) {
  const notification = new Notification({
    title,
    body,
    icon: getIconForType(type),
    silent: false,
    urgency: type === 'urgent' ? 'critical' : 'normal'
  })
  
  notification.on('click', () => {
    // Pencereyi göster ve ilgili sayfaya git
  })
  
  notification.show()
}
```

---

### 9. Dosya İşlemleri

#### 9.1 Dosya Dialog
**Dosya:** `electron/ipc/file.ts`

**IPC Handlers:**
```typescript
// Excel import için dosya seçme
ipcMain.handle('file:select', async (event, options) => {
  const result = await dialog.showOpenDialog({
    properties: ['openFile'],
    filters: [
      { name: 'Excel Files', extensions: ['xlsx', 'xls'] },
      { name: 'All Files', extensions: ['*'] }
    ],
    ...options
  })
  
  if (!result.canceled) {
    return result.filePaths[0]
  }
  return null
})

// PDF/Excel export için kaydetme
ipcMain.handle('file:save', async (event, data, defaultName) => {
  const result = await dialog.showSaveDialog({
    defaultPath: defaultName,
    filters: [
      { name: 'PDF', extensions: ['pdf'] },
      { name: 'Excel', extensions: ['xlsx'] },
      { name: 'CSV', extensions: ['csv'] }
    ]
  })
  
  if (!result.canceled) {
    fs.writeFileSync(result.filePath, data)
    return true
  }
  return false
})

// Harici URL açma
ipcMain.handle('shell:openExternal', async (event, url) => {
  await shell.openExternal(url)
})
```

#### 9.2 Frontend Entegrasyonu
**Güncellenecek Yerler:**
- `client/components/features/students/` - Export butonları için Electron API kullanımı
- `client/components/features/exam-management/` - Export için Electron API
- `client/components/features/surveys/` - File picker için Electron dialog

```typescript
// Örnek kullanım (Export butonlarında)
const handleExport = async () => {
  if (window.electronAPI) {
    const pdfData = await generatePDF()
    await window.electronAPI.saveFile(pdfData, 'ogrenci-raporu.pdf')
  } else {
    // Web için mevcut download metodu
  }
}

// Örnek kullanım (File upload için)
const handleFileSelect = async () => {
  if (window.electronAPI) {
    const filePath = await window.electronAPI.selectFile({
      filters: [{ name: 'Excel', extensions: ['xlsx', 'xls'] }]
    })
    if (filePath) {
      // Dosyayı işle
    }
  } else {
    // Web için mevcut file input
  }
}
```

---

### 10. Kullanıcı Ayarları

#### 10.1 Electron Store
**Dosya:** `electron/store.ts`

**Kaydedilecek Ayarlar:**
- Pencere boyutu ve konumu
- Tema (light/dark)
- Dil tercihi
- Son açılan sayfalar
- Bildirim tercihleri
- Veritabanı yedekleme ayarları
- AI provider ayarları

```typescript
import Store from 'electron-store'

interface StoreSchema {
  windowBounds: { width: number; height: number; x: number; y: number }
  theme: 'light' | 'dark'
  language: 'tr'
  notifications: {
    riskStudents: boolean
    missingData: boolean
    dailyReports: boolean
  }
  lastOpenedPages: string[]
}

export const store = new Store<StoreSchema>({
  defaults: {
    windowBounds: { width: 1280, height: 800, x: 0, y: 0 },
    theme: 'light',
    language: 'tr',
    notifications: {
      riskStudents: true,
      missingData: true,
      dailyReports: false
    },
    lastOpenedPages: []
  }
})
```

---

### 11. Güvenlik Yapılandırması

#### 11.1 Content Security Policy (CSP)
**Dosya:** `electron/main.ts`

```typescript
session.defaultSession.webRequest.onHeadersReceived((details, callback) => {
  callback({
    responseHeaders: {
      ...details.responseHeaders,
      'Content-Security-Policy': [
        "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline';"
      ]
    }
  })
})
```

#### 11.2 Güvenlik Önlemleri Checklist
- ✅ `nodeIntegration: false`
- ✅ `contextIsolation: true`
- ✅ `webSecurity: true`
- ✅ Preload script ile sınırlı API
- ✅ IPC validasyonu
- ✅ Dosya yolu validasyonu
- ✅ SQL injection koruması (zaten mevcut)

---

### 12. Loglama Sistemi

#### 12.1 Electron Log
**Dosya:** `electron/logger.ts`

```typescript
import log from 'electron-log'
import path from 'path'
import { app } from 'electron'

log.transports.file.level = 'info'
log.transports.file.maxSize = 10 * 1024 * 1024 // 10MB
log.transports.file.format = '[{y}-{m}-{d} {h}:{i}:{s}] [{level}] {text}'
log.transports.file.resolvePathFn = () => 
  path.join(app.getPath('userData'), 'logs', 'main.log')

export default log
```

**Log Dosyası Konumu:**
- Windows: `%APPDATA%/Rehber360/logs/main.log`

**Log Seviyeleri:**
- `error`: Kritik hatalar
- `warn`: Uyarılar
- `info`: Genel bilgiler
- `debug`: Debug bilgileri (dev mode)

---

### 13. Build ve Paketleme

#### 13.1 package.json Güncellemeleri
**Dosya:** `package.json`

**Yeni scriptler ekle (mevcut build scriptlerini koruyarak):**
```json
{
  "scripts": {
    "electron:dev": "concurrently \"npm run dev\" \"wait-on http://localhost:5000 && electron .\"",
    "electron:build": "npm run build:client && npm run build:server && npm run build:electron && electron-builder",
    "electron:build:win": "npm run build:client && npm run build:server && npm run build:electron && electron-builder --win",
    "electron:build:dir": "npm run build:client && npm run build:server && npm run build:electron && electron-builder --dir",
    "build:electron": "tsc -p tsconfig.electron.json"
  },
  "main": "dist/electron/main.js"
}

NOT: 
- Mevcut "build", "build:client" ve "build:server" scriptleri değişmeden kalacak.
- Electron build sürecinde:
  1. npm run build:client → React frontend build (dist/spa/)
  2. npm run build:server → Express backend build (dist/server/)
  3. npm run build:electron → Electron main/preload build (dist/electron/)
  4. electron-builder → Windows installer oluşturma
```

**Yeni bağımlılıklar:**
```json
{
  "devDependencies": {
    "concurrently": "^8.2.0",
    "wait-on": "^7.0.1"
  }
}
```

#### 13.2 electron-builder Yapılandırması
**Dosya:** `electron-builder.json`

```json
{
  "appId": "com.rehber360.app",
  "productName": "Rehber360",
  "copyright": "Copyright © 2025 Rehber360",
  "directories": {
    "output": "release",
    "buildResources": "build"
  },
  "files": [
    "dist/**/*",
    "package.json"
  ],
  "extraResources": [
    {
      "from": "database.db",
      "to": "database.db"
    },
    {
      "from": "dist/server",
      "to": "dist/server"
    }
  ],
  "win": {
    "target": [
      {
        "target": "nsis",
        "arch": ["x64"]
      },
      {
        "target": "portable",
        "arch": ["x64"]
      }
    ],
    "icon": "build/icon.ico",
    "artifactName": "${productName}-Setup-${version}.${ext}",
    "publisherName": "Rehber360",
    "verifyUpdateCodeSignature": false
  },
  "nsis": {
    "oneClick": false,
    "allowToChangeInstallationDirectory": true,
    "createDesktopShortcut": true,
    "createStartMenuShortcut": true,
    "shortcutName": "Rehber360",
    "perMachine": false,
    "deleteAppDataOnUninstall": false,
    "installerIcon": "build/icon.ico",
    "uninstallerIcon": "build/icon.ico",
    "installerHeader": "build/installerHeader.bmp",
    "language": "1055"
  },
  "portable": {
    "artifactName": "${productName}-Portable-${version}.${ext}"
  }
}
```

#### 13.3 TypeScript Config (Electron)
**Dosya:** `tsconfig.electron.json`

```json
{
  "extends": "./tsconfig.json",
  "compilerOptions": {
    "outDir": "dist/electron",
    "module": "commonjs",
    "target": "ES2020",
    "moduleResolution": "node",
    "skipLibCheck": true,
    "esModuleInterop": true,
    "resolveJsonModule": true
  },
  "include": ["electron/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

---

### 14. İkonlar ve Görseller

#### 14.1 Gerekli İkon Dosyaları
**Klasör:** `build/`

**Dosyalar:**
- `icon.ico` - 256x256 (Windows app icon)
- `icon.png` - 512x512 (Kaynak dosya)
- `tray.png` - 16x16 (Sistem tepsisi normal)
- `tray@2x.png` - 32x32 (Sistem tepsisi retina)
- `installerHeader.bmp` - 150x57 (NSIS installer header)
- `installerSidebar.bmp` - 164x314 (NSIS installer sidebar)

#### 14.2 İkon Oluşturma Görevi
- Rehber360 logosu tasarımı
- Multi-size icon export (16, 24, 32, 48, 64, 128, 256, 512)
- ICO formatına dönüştürme
- Tray icon için monochrome versiyon

---

### 15. Auto-Update Sistemi

#### 15.1 Electron Updater
**Dosya:** `electron/updater.ts`

```typescript
import { autoUpdater } from 'electron-updater'
import log from './logger'

export function setupAutoUpdater(mainWindow: BrowserWindow) {
  autoUpdater.logger = log
  
  autoUpdater.on('update-available', () => {
    mainWindow.webContents.send('update:available')
  })
  
  autoUpdater.on('update-downloaded', () => {
    mainWindow.webContents.send('update:downloaded')
  })
  
  autoUpdater.on('error', (error) => {
    log.error('Auto-updater error:', error)
  })
  
  // Uygulama başladıktan 5 saniye sonra güncelleme kontrolü
  setTimeout(() => {
    autoUpdater.checkForUpdates()
  }, 5000)
  
  // Her 2 saatte bir güncelleme kontrolü
  setInterval(() => {
    autoUpdater.checkForUpdates()
  }, 2 * 60 * 60 * 1000)
}
```

#### 15.2 Frontend Güncelleme UI
**Dosya:** `client/components/UpdateNotification.tsx`

**Görevler:**
- "Güncelleme mevcut" bildirimi
- İndirme durumu gösterimi
- "Yeniden başlat ve güncelle" butonu

---

### 16. Development Ortamı

#### 16.1 Development Workflow
**Development modunda Electron çalıştırmak için:**

1. **İlk seferde backend'i build et:**
```bash
npm run build:server
npm run build:electron
```

2. **Development server başlat:**
```bash
npm run electron:dev
```

**Çalışma Sırası:**
1. Vite dev server başlar (port 5000) - frontend hot reload
2. wait-on server hazır olmasını bekler
3. Electron başlar
4. Electron içinde Express backend build edilmiş dosyalardan çalışır
5. Frontend değişikliklerinde hot reload (Vite HMR)
6. Backend değişikliklerinde `npm run build:server` tekrar çalıştır

**NOT:** Backend değişikliklerinde otomatik rebuild için nodemon eklenebilir:
```json
"scripts": {
  "watch:server": "vite build --config vite.config.server.ts --watch",
  "electron:dev": "concurrently \"npm run watch:server\" \"npm run dev\" \"wait-on http://localhost:5000 && electron .\""
}
```

---

### 17. Production Build

#### 17.1 Build Adımları
```bash
# 1. Frontend build (React + Vite)
npm run build:client

# 2. Backend build (Express + TypeScript)
npm run build:server

# 3. Electron TypeScript build
npm run build:electron

# 4. Electron Builder - Windows installer
npm run electron:build:win
```

#### 17.2 Çıktılar
**Klasör:** `release/`

- `Rehber360-Setup-1.0.0.exe` - NSIS Installer (kullanıcı kurulum)
- `Rehber360-Portable-1.0.0.exe` - Portable versiyon (kuruluma gerek yok)
- `win-unpacked/` - Paketlenmemiş dosyalar (test için)

---

### 18. Test Senaryoları

#### 18.1 Fonksiyonel Testler
- [ ] Uygulama başlatma ve kapanma
- [ ] Pencere minimize, maximize, close
- [ ] Sistem tepsisine gitme ve geri dönme
- [ ] Menü öğeleri çalışıyor
- [ ] Klavye kısayolları çalışıyor
- [ ] Dosya seçme dialog
- [ ] Dosya kaydetme dialog
- [ ] PDF export
- [ ] Excel export
- [ ] Excel import
- [ ] SQLite veritabanı okuma/yazma
- [ ] Veritabanı yedekleme
- [ ] Veritabanı geri yükleme
- [ ] Native bildirimler
- [ ] Ayarlar kaydetme/yükleme
- [ ] Harici link açma

#### 18.2 Performans Testleri
- [ ] Başlangıç süresi < 3 saniye
- [ ] Bellek kullanımı < 500 MB (boşta)
- [ ] CPU kullanımı < 5% (boşta)
- [ ] Büyük veritabanı yükleme (10,000+ öğrenci)
- [ ] Bulk operations performansı

#### 18.3 Güvenlik Testleri
- [ ] Node integration kapalı
- [ ] Context isolation aktif
- [ ] IPC parametreleri validate ediliyor
- [ ] Dosya yolları sanitize ediliyor
- [ ] SQL injection koruması

---

### 19. Deployment Checklist

#### 19.1 Yayınlama Öncesi
- [ ] Versiyon numarası güncelle (package.json)
- [ ] Changelog hazırla
- [ ] Tüm testler başarılı
- [ ] Production build test edildi
- [ ] Installer test edildi (temiz Windows)
- [ ] Portable versiyon test edildi
- [ ] Icon dosyaları hazır
- [ ] Lisans dosyası eklendi
- [ ] README.md güncellendi

#### 19.2 Yayınlama Kanalları
- [ ] GitHub Releases
- [ ] Microsoft Store (opsiyonel)
- [ ] Okul içi dağıtım sunucusu
- [ ] Web sitesi download linki

---

### 20. Kullanıcı Dokümantasyonu

#### 20.1 Kurulum Rehberi
**Dosya:** `docs/KURULUM.md`

**İçerik:**
- Sistem gereksinimleri
- İndirme linki
- Adım adım kurulum
- İlk açılış ayarları
- Veritabanı import
- Troubleshooting

#### 20.2 Kullanım Kılavuzu
**Dosya:** `docs/KULLANIM.md`

**İçerik:**
- Menü ve kısayollar
- Dosya işlemleri
- Yedekleme ve geri yükleme
- Ayarlar
- Güncelleme
- SSS

---

## 🔧 Teknik Detaylar

### Önemli Kararlar

**1. Express Backend Electron İçinde**
- Web versiyonunda Express ayrı sunucu
- Electron versiyonunda Express main process içinde
- Rastgele port kullanımı (çakışma önleme)
- 127.0.0.1 binding (güvenlik)

**2. SQLite Veritabanı Konumu**
- Development: Proje klasöründe `database.db`
- Production (Electron): `%APPDATA%/Rehber360/database.db`
- İlk açılışta şablon veritabanı kopyalama
- Auto-backup her gece saat 02:00

**3. Dosya Yolları**
- Uploads: `%APPDATA%/Rehber360/uploads/`
- Backups: `%APPDATA%/Rehber360/backups/`
- Logs: `%APPDATA%/Rehber360/logs/`

**4. Güvenlik Modeli**
- Preload script ile sınırlı API
- IPC üzerinden kontrollü erişim
- Dosya sistem işlemleri validation
- SQL injection koruması (zaten mevcut)

---

## 📅 Tahmini Süre

| Görev | Süre |
|-------|------|
| 1-2. Kurulum ve yapı | 2 saat |
| 3-4. Main ve preload | 4 saat |
| 5. Frontend entegrasyon | 3 saat |
| 6-7. Menü ve tray | 2 saat |
| 8-9. Bildirimler ve dosyalar | 2 saat |
| 10-12. Ayarlar, güvenlik, log | 2 saat |
| 13-15. Build ve updater | 3 saat |
| 16-17. Dev ve prod ortam | 2 saat |
| 18. Test | 4 saat |
| 19-20. Deployment ve döküman | 2 saat |
| **TOPLAM** | **~26 saat** |

---

## 🚀 Hızlı Başlangıç

Electron entegrasyonuna hemen başlamak için:

```bash
# 1. Bağımlılıkları yükle
npm install --save-dev electron electron-builder concurrently wait-on
npm install --save electron-is-dev electron-log electron-store

# 2. Electron klasörü oluştur
mkdir electron

# 3. main.ts ve preload.ts oluştur
# (Dosya içeriklerini bu dokümandan kopyala)

# 4. package.json'a scriptler ekle

# 5. Development modunda çalıştır
npm run electron:dev
```

---

## 📝 Notlar

- Bu plan, mevcut web uygulamasını Electron'a taşırken **tüm özellikleri korur**
- Backend API'leri değişmeden kalır (Express)
- Frontend React kodu minimal değişiklik gerektirir
- Veritabanı yapısı aynı kalır
- Web versiyonu ayrı deploy edilebilir (Electron opsiyonel)

---

## ✅ İlerleme Takibi

Görevler tamamlandıkça bu bölümü güncelleyin:

- [x] 1. Proje kurulumu ✅ **Tamamlandı** (30 Ekim 2025)
  - ✅ Electron bağımlılıkları yüklendi (electron, electron-builder, electron-devtools-installer)
  - ✅ Runtime bağımlılıkları eklendi (electron-is-dev, electron-log, electron-store)
  - ✅ TypeScript tipleri eklendi (@types/electron-devtools-installer)
  - ✅ Development araçları kuruldu (concurrently, wait-on)
  - ✅ package.json güncellendi:
    - "main": "dist/electron/main.js" field'ı eklendi
    - electron:dev scripti eklendi (build + concurrent dev server)
    - electron:build scripti eklendi (full build + packaging)
    - electron:build:win scripti eklendi (Windows specific)
    - electron:build:dir scripti eklendi (unpacked directory)
    - build:electron scripti eklendi (TypeScript compilation)
  - ✅ Electron klasör yapısı oluşturuldu (electron/, electron/ipc/, electron/types/)
  - ✅ tsconfig.electron.json yapılandırıldı (CommonJS, ES2020, Node resolution)
  - ✅ electron-builder.json yapılandırıldı (NSIS + Portable, Windows x64)
  - ✅ .gitignore güncellendi (release/ klasörü eklendi)
  - ✅ Minimal çalışan Electron yapısı oluşturuldu:
    - electron/main.ts: BrowserWindow oluşturma, IPC handler'lar
    - electron/preload.ts: Güvenli API köprüsü (contextBridge)
    - electron/types/electron.d.ts: TypeScript tip tanımları
  - ✅ Build süreci test edildi ve çalışıyor
  - ✅ Mimar tarafından onaylandı ✓
- [ ] 2. Proje yapısı
- [ ] 3. Main process
- [ ] 4. Preload script
- [ ] 5. Frontend entegrasyon
- [ ] 6. Uygulama menüsü
- [ ] 7. Sistem tepsisi
- [ ] 8. Bildirimler
- [ ] 9. Dosya işlemleri
- [ ] 10. Kullanıcı ayarları
- [ ] 11. Güvenlik
- [ ] 12. Loglama
- [ ] 13. Build yapılandırması
- [ ] 14. İkonlar
- [ ] 15. Auto-update
- [ ] 16. Development ortamı
- [ ] 17. Production build
- [ ] 18. Testler
- [ ] 19. Deployment
- [ ] 20. Dokümantasyon

---

**Son Güncelleme:** 30 Ekim 2025
