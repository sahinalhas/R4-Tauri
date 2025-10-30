# Rehber360 Electron - Deployment Checklist

## 📋 Pre-Deployment Checklist

### 1. Version ve Metadata Kontrolü
- [ ] `package.json` version numarası güncellenmiş (semantic versioning: MAJOR.MINOR.PATCH)
- [ ] `package.json` metadata eksiksiz:
  - [ ] `name`: "rehber360"
  - [ ] `version`: "1.0.0" (veya güncel versiyon)
  - [ ] `description`: Açıklama mevcut
  - [ ] `author`: Yazar bilgisi mevcut
  - [ ] `license`: Lisans tipi belirtilmiş
  - [ ] `homepage`: Website URL'i eklenmiş
- [ ] `electron-builder.json` yapılandırması doğru:
  - [ ] `appId`: "com.rehber360.app"
  - [ ] `productName`: "Rehber360"
  - [ ] `copyright`: Güncel yıl
  - [ ] `publisherName`: "Rehber360"

### 2. Code Quality Kontrolü
- [ ] LSP diagnostics temiz (hiç hata yok)
- [ ] TypeScript derlemesi başarılı: `npm run typecheck`
- [ ] Lint kontrolü temiz: `npm run lint`
- [ ] Format kontrolü temiz: `npm run format`
- [ ] Tüm testler geçiyor: `npm run test`
- [ ] Coverage yeterli: `npm run test:coverage`

### 3. Build Test
- [ ] Client build başarılı: `npm run build:client`
- [ ] Server build başarılı: `npm run build:server`
- [ ] Electron build başarılı: `npm run build:electron`
- [ ] Full build test: `npm run electron:build:dir`
- [ ] `dist/` klasörü yapısı doğru:
  - [ ] `dist/spa/` - Frontend dosyaları
  - [ ] `dist/server/` - Backend dosyaları
  - [ ] `dist/electron/` - Electron dosyaları

### 4. Icon ve Asset Kontrolü
- [ ] `build/icon.png` (512x512) mevcut ve doğru
- [ ] `build/icon-16.png` (16x16) mevcut
- [ ] `build/icon-32.png` (32x32) mevcut
- [ ] `build/icon-64.png` (64x64) mevcut
- [ ] `build/icon-128.png` (128x128) mevcut
- [ ] `build/icon-256.png` (256x256) mevcut
- [ ] `build/installerHeader.png` (150x57) mevcut
- [ ] Tüm iconlar kaliteli ve net görünüyor

### 5. Database Kontrolü
- [ ] `database.db` kök dizinde mevcut
- [ ] Database şeması güncel
- [ ] Seed data (varsa) doğru
- [ ] Migration scriptleri çalışıyor
- [ ] Database dosyası 50MB'dan küçük

### 6. Environment Variables
- [ ] Üretim için gerekli env değişkenleri tanımlı
- [ ] API anahtarları güvenli (hardcoded değil)
- [ ] `.env` dosyası .gitignore'da
- [ ] Electron ortamında NODE_ENV production olarak ayarlanmış

### 7. Security Checklist
- [ ] `nodeIntegration: false` ayarlanmış
- [ ] `contextIsolation: true` ayarlanmış
- [ ] `webSecurity: true` ayarlanmış
- [ ] Preload script sadece gerekli API'ları expose ediyor
- [ ] IPC handler'lar input validasyonu yapıyor
- [ ] SQL injection koruması var
- [ ] XSS koruması var
- [ ] Dosya yolu validasyonu var

### 8. Performance Checks
- [ ] Başlangıç süresi <8 saniye
- [ ] İlk bellek kullanımı <200 MB
- [ ] Bundle boyutu makul (<100 MB)
- [ ] Lazy loading uygulanmış
- [ ] Gereksiz dependencies temizlenmiş

### 9. Documentation
- [ ] README.md güncel
- [ ] CHANGELOG.md oluşturulmuş
- [ ] Kullanıcı kurulum rehberi hazır
- [ ] Kullanıcı kullanım kılavuzu hazır
- [ ] Manuel test rehberi hazır
- [ ] API dokümantasyonu güncel

---

## 🔨 Build Process

### Local Build (Development)

#### 1. Development Test
```bash
# Development modda test et
npm run electron:dev
```

**Kontroller:**
- [ ] Uygulama açılıyor
- [ ] Hot reload çalışıyor
- [ ] DevTools açılabiliyor
- [ ] Backend bağlantısı çalışıyor
- [ ] Veritabanı işlemleri çalışıyor

#### 2. Directory Build
```bash
# Installer olmadan build klasörü oluştur
npm run electron:build:dir
```

**Kontroller:**
- [ ] `release/win-unpacked/` klasörü oluştu
- [ ] `Rehber360.exe` çalışıyor
- [ ] Tüm dosyalar mevcut
- [ ] Veritabanı kopyalandı
- [ ] Backend server başlıyor

### Production Build

#### 3. Full Build (Windows)
```bash
# Tam installer build
npm run electron:build:win
```

**Oluşacak Dosyalar:**
- [ ] `release/Rehber360-Setup-1.0.0.exe` (NSIS installer)
- [ ] `release/Rehber360-Portable-1.0.0.exe` (Portable version)
- [ ] `release/win-unpacked/` (Unpacked directory)

**Kontroller:**
- [ ] NSIS installer boyutu makul (<150 MB)
- [ ] Portable versiyon boyutu makul (<100 MB)
- [ ] Installer çalışıyor
- [ ] Portable versiyon çalışıyor

#### 4. Build Validation Script
```bash
# Build çıktılarını validate et
node scripts/validate-build.cjs
```

**Validation Kriterleri:**
- [ ] Tüm gerekli dosyalar mevcut
- [ ] Icon dosyaları doğru
- [ ] Database.db mevcut
- [ ] Package.json doğru
- [ ] Electron main.js mevcut
- [ ] Server dosyaları mevcut
- [ ] Frontend dosyları mevcut

---

## 🧪 Pre-Release Testing

### Installer Testing (NSIS)

#### Test 1: Temiz Kurulum
- [ ] Temiz Windows VM hazırla
- [ ] `Rehber360-Setup-1.0.0.exe` çalıştır
- [ ] UAC uyarısını kabul et
- [ ] Tüm seçenekleri varsayılan bırak
- [ ] Kurulumu tamamla
- [ ] Uygulama otomatik başlasın
- [ ] Masaüstü kısayolu mevcut
- [ ] Başlat menüsü kısayolu mevcut

**Başarı Kriterleri:**
- ✅ Kurulum hatasız tamamlanır
- ✅ Uygulama çalışır
- ✅ Tüm özellikler çalışır

#### Test 2: Özel Konum Kurulumu
- [ ] Farklı kurulum konumu seç (örn: D:\Programs\)
- [ ] Kısayol oluşturma seçeneklerini değiştir
- [ ] Kurulumu tamamla

**Başarı Kriterleri:**
- ✅ Uygulama seçilen konumda kurulur
- ✅ Kısayol tercihleri uygulanır

#### Test 3: Güncelleme Kurulumu
- [ ] Eski versiyon kur (örn: 0.9.0)
- [ ] Bazı veri ekle (öğrenci, not, vb.)
- [ ] Yeni versiyonu kur (1.0.0)
- [ ] Uygulamayı aç

**Başarı Kriterleri:**
- ✅ Eski veri korunur
- ✅ Güncelleme sorunsuz
- ✅ Veritabanı migrate edilir

### Portable Version Testing

#### Test 4: USB Bellekten Çalıştırma
- [ ] Portable exe'yi USB'ye kopyala
- [ ] 3 farklı bilgisayarda çalıştır
- [ ] Veri ekle ve kontrol et

**Başarı Kriterleri:**
- ✅ Her bilgisayarda çalışır
- ✅ Veriler USB'de saklanır
- ✅ Kurulum gerektirmez

### Functional Testing

#### Test 5: Temel İşlevsellik
- [ ] Öğrenci ekleme
- [ ] Sınav notu girişi
- [ ] Anket yanıtı ekleme
- [ ] AI analizi
- [ ] Rapor oluşturma
- [ ] Excel import
- [ ] PDF export

**Başarı Kriterleri:**
- ✅ Tüm işlevler çalışır
- ✅ Veri kaybı yok
- ✅ Performans kabul edilebilir

#### Test 6: Pencere Yönetimi
- [ ] Minimize
- [ ] Maximize
- [ ] Restore
- [ ] Close (sistem tepsisine)
- [ ] Tam ekran (F11)
- [ ] Yeniden boyutlandırma

**Başarı Kriterleri:**
- ✅ Tüm kontroller çalışır
- ✅ Pencere durumu kaydedilir

#### Test 7: Sistem Tepsisi
- [ ] Uygulama sistem tepsisine gider
- [ ] Çift tık pencereyi geri getirir
- [ ] Sağ tık menüsü çalışır
- [ ] Bildirimler gösterilir

**Başarı Kriterleri:**
- ✅ Tray ikonu görünür
- ✅ Tüm menü öğeleri çalışır

#### Test 8: Veritabanı İşlemleri
- [ ] Yedek alma
- [ ] Yedek geri yükleme
- [ ] Excel export
- [ ] PDF export
- [ ] Veri bütünlüğü

**Başarı Kriterleri:**
- ✅ Yedekler oluşturulur
- ✅ Geri yükleme çalışır
- ✅ Export işlemleri başarılı

---

## 🚀 Release Process

### 1. Version Bump
```bash
# Version numarasını güncelle
# package.json içinde version değiştir:
# - PATCH: 1.0.0 → 1.0.1 (bug fixes)
# - MINOR: 1.0.0 → 1.1.0 (new features)
# - MAJOR: 1.0.0 → 2.0.0 (breaking changes)
```

### 2. Changelog Güncelle
```bash
# CHANGELOG.md dosyasını güncelle
# Yeni versiyondaki değişiklikleri ekle
```

**Changelog Template:**
```markdown
## [1.0.0] - 2025-10-30

### Added
- Electron masaüstü uygulaması desteği
- Sistem tepsisi entegrasyonu
- Native dosya dialogları
- Veritabanı yedekleme/geri yükleme

### Changed
- Backend port yönetimi iyileştirildi
- Performans optimizasyonları

### Fixed
- Pencere boyutu kaydetme sorunu giderildi
- Memory leak düzeltildi

### Security
- NodeIntegration güvenlik ayarları
- Input validasyonu eklendi
```

### 3. Git Tag Oluştur
```bash
# Version için git tag oluştur
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

### 4. Final Build
```bash
# Son production build
npm run electron:build:win
```

### 5. Build Artifact Kontrolü

**Oluşan Dosyalar:**
- [ ] `Rehber360-Setup-1.0.0.exe` (~100-150 MB)
- [ ] `Rehber360-Portable-1.0.0.exe` (~80-120 MB)

**Checksum Oluştur:**
```bash
# Windows PowerShell
Get-FileHash .\release\Rehber360-Setup-1.0.0.exe -Algorithm SHA256
Get-FileHash .\release\Rehber360-Portable-1.0.0.exe -Algorithm SHA256
```

- [ ] SHA256 checksum'ları kaydet

### 6. Code Signing (Opsiyonel)
```bash
# Eğer code signing certificate varsa
# electron-builder.json içinde:
# "win": {
#   "certificateFile": "cert.pfx",
#   "certificatePassword": "password"
# }
```

### 7. Release Notes Hazırla

**Release Notes Template:**
```markdown
# Rehber360 v1.0.0 - İlk Resmi Sürüm

## 🎉 Öne Çıkan Özellikler
- **Masaüstü Uygulama**: Windows için native masaüstü uygulaması
- **Offline Çalışma**: Internet bağlantısı olmadan çalışır
- **Veritabanı Yedekleme**: Kolay yedekleme ve geri yükleme
- **Gelişmiş Performans**: Daha hızlı ve daha kararlı

## 📦 Kurulum
1. `Rehber360-Setup-1.0.0.exe` dosyasını indirin
2. Çift tıklayarak kurulumu başlatın
3. Kurulum sihirbazını takip edin

## 🔄 Güncelleme
Eski versiyondan güncelleme yapıyorsanız:
1. Verilerinizi yedekleyin
2. Yeni installer'ı çalıştırın
3. "Güncelle" seçeneğini seçin

## 🐛 Bilinen Sorunlar
- Yok

## 📊 Teknik Detaylar
- Electron: v39.0.0
- Node.js: v20+
- Minimum Sistem: Windows 10 64-bit
- Gerekli Alan: 500 MB

## 🔐 Güvenlik
SHA256 Checksums:
- Setup: [checksum]
- Portable: [checksum]

## 📞 Destek
- Website: https://rehber360.com
- Email: support@rehber360.com
- GitHub Issues: [repository]/issues
```

---

## 📤 Distribution

### 1. GitHub Release Oluştur
- [ ] GitHub'da yeni release oluştur
- [ ] Tag: `v1.0.0`
- [ ] Title: `Rehber360 v1.0.0`
- [ ] Description: Release notes yapıştır
- [ ] Dosyaları upload et:
  - [ ] `Rehber360-Setup-1.0.0.exe`
  - [ ] `Rehber360-Portable-1.0.0.exe`
  - [ ] `CHECKSUMS.txt`
- [ ] "This is a pre-release" işaretle (eğer beta ise)
- [ ] Publish release

### 2. Website Güncelle
- [ ] Download linklerini güncelle
- [ ] Version numarasını güncelle
- [ ] Release notes yayınla
- [ ] Screenshots güncelle (eğer UI değiştiyse)

### 3. Auto-Update Server (Opsiyonel)
- [ ] Update server'a yeni versiyon upload et
- [ ] `latest.yml` dosyasını güncelle
- [ ] Auto-update test et

### 4. Dokümantasyon Güncelle
- [ ] README.md güncelle
- [ ] Kullanıcı dokümantasyonu güncelle
- [ ] API dokümantasyonu güncelle
- [ ] Video tutorials güncelle (eğer varsa)

---

## 📊 Post-Release

### 1. Monitoring
- [ ] GitHub releases download sayılarını izle
- [ ] Crash reports kontrol et (eğer crash reporting varsa)
- [ ] User feedback topla
- [ ] GitHub Issues takip et

### 2. Hotfix Hazırlığı
- [ ] Kritik bug'lar için hızlı patch planı hazır
- [ ] Hotfix branch oluştur (eğer gerekirse)
- [ ] Emergency rollback planı hazır

### 3. Metrikler
- [ ] İlk 24 saat download sayısı: _____
- [ ] İlk hafta download sayısı: _____
- [ ] Rapor edilen bug sayısı: _____
- [ ] Ortalama rating: _____

---

## ✅ Release Sign-Off

### Pre-Release Approval

**Technical Lead:**
- [ ] Code quality onayı
- [ ] Build validation onayı
- [ ] Security review onayı
- İmza: _____________ Tarih: _____________

**QA Lead:**
- [ ] Tüm testler başarılı
- [ ] Manual testing tamamlandı
- [ ] No critical bugs
- İmza: _____________ Tarih: _____________

**Product Owner:**
- [ ] Feature completeness
- [ ] Documentation onayı
- [ ] Release go/no-go
- İmza: _____________ Tarih: _____________

### Release Checklist Final

- [ ] Version numarası doğru
- [ ] Tüm testler geçti
- [ ] Documentation hazır
- [ ] Build artifacts oluşturuldu
- [ ] Checksums hesaplandı
- [ ] GitHub release hazır
- [ ] Website güncel
- [ ] Support team bilgilendirildi
- [ ] Monitoring aktif
- [ ] Rollback planı hazır

**Final Approval:** _____________ Tarih: _____________

---

## 🆘 Emergency Rollback

### Kritik Bug Senaryosu

**Adımlar:**
1. GitHub release'i "Pre-release" olarak işaretle
2. Download linklerini eski versiyona döndür
3. Website'de duyuru yap
4. Hotfix hazırla
5. Acil patch release yap

**Hotfix Version Naming:**
- Critical bug: 1.0.0 → 1.0.1
- Security issue: Immediate patch + announcement

---

## 📝 Notlar

### Build Süresi
- Full build süresi: ~15-30 dakika (makineye göre)
- Installer oluşturma: ~5-10 dakika

### Build Boyutları (Yaklaşık)
- dist/ klasörü: ~200 MB
- NSIS Installer: ~100-150 MB
- Portable: ~80-120 MB
- win-unpacked: ~250 MB

### Önemli Linkler
- Electron Builder Docs: https://www.electron.build/
- NSIS Documentation: https://nsis.sourceforge.io/
- Code Signing Guide: [link]
- Release Management: [link]

---

**Son Güncelleme:** 30 Ekim 2025
**Hazırlayan:** Rehber360 Development Team
**Versiyon:** 1.0
