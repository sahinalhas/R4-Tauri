# Rehber360 - Güncelleme ve Yayınlama Stratejisi

## 🔄 Geliştirme Akışı

### 1. Geliştirme Ortamı

Rehber360 projesini **iki şekilde** geliştirebilirsiniz:

#### A) Replit'te Web Geliştirme
```bash
npm run dev
```
- Tarayıcıda çalışır
- Hızlı test ve geliştirme
- Tüm değişiklikler otomatik kaydedilir

#### B) Lokal Masaüstü Geliştirme
```bash
npm run electron:dev
```
- Masaüstü uygulaması olarak çalışır
- Desktop özellikleri test edilir
- Gerçek kullanıcı deneyimi

**Önemli:** Kod tabanı aynı - her iki ortamda da aynı değişiklikler geçerli!

---

## 🚀 Yeni Sürüm Yayınlama

### Adım 1: Versiyonu Artırın

`package.json` dosyasını güncelleyin:

```json
{
  "version": "1.0.1"  // Önceki: 1.0.0
}
```

**Versiyon Numaralandırma (Semantic Versioning):**
- `1.0.0` → `1.0.1` - Küçük hata düzeltmeleri (patch)
- `1.0.0` → `1.1.0` - Yeni özellikler (minor)
- `1.0.0` → `2.0.0` - Büyük değişiklikler (major)

### Adım 2: Build Oluşturun

Windows bilgisayarınızda:

```bash
# Tam build (installer + portable)
npm run electron:build:win
```

Oluşan dosyalar (`release/` klasörü):
- `Rehber360-Setup-1.0.1.exe` - Kurulum programı
- `Rehber360-Portable-1.0.1.exe` - Taşınabilir sürüm
- `latest.yml` - Güncelleme metadata
- `Rehber360-Setup-1.0.1.exe.blockmap` - Yama dosyası

### Adım 3: GitHub Release Oluşturun

1. **GitHub Repository → Releases → "Create a new release"**
2. **Tag version:** `v1.0.1`
3. **Release title:** `Rehber360 v1.0.1 - [Açıklama]`
4. **Description:** Değişiklikleri yazın (changelog)
5. **Dosyaları yükleyin:**
   - `Rehber360-Setup-1.0.1.exe`
   - `Rehber360-Portable-1.0.1.exe`
   - `latest.yml`
   - `*.blockmap` dosyaları

6. **Publish release**

### Adım 4: Kullanıcılar Otomatik Güncellenir

- ✅ Uygulama her 2 saatte bir güncelleme kontrol eder
- ✅ Yeni sürüm bulunca kullanıcıya bildirim gösterir
- ✅ Kullanıcı "Güncelle" dediğinde otomatik indirir
- ✅ "Yeniden başlat" ile güncelleme yüklenir

---

## 🔧 Güncelleme Yapılandırması

### Auto-Updater Ayarları

`electron/updater.ts` dosyasında:

```typescript
// Güncelleme kontrolü sıklığı
setInterval(() => {
  autoUpdater.checkForUpdates()
}, 2 * 60 * 60 * 1000)  // 2 saat
```

### GitHub'ı Güncelleme Sunucusu Olarak Kullanma

`package.json` içinde:

```json
{
  "repository": {
    "type": "git",
    "url": "https://github.com/kullanici-adi/rehber360"
  }
}
```

electron-updater otomatik olarak bu repository'den güncellemeleri çeker.

---

## 📝 Changelog Örneği

Her release için değişiklikleri dokümante edin:

```markdown
## v1.0.1 - 30 Kasım 2025

### Yeni Özellikler
- ✨ Öğrenci rapor şablonları eklendi
- ✨ Excel export hızlandırıldı

### Hata Düzeltmeleri
- 🐛 Sınav notu hesaplama hatası düzeltildi
- 🐛 Bildirim gösterim sorunu çözüldü

### İyileştirmeler
- ⚡ Veritabanı sorgu performansı artırıldı
- 🎨 Arayüz renkleri iyileştirildi
```

---

## 🔒 Güvenlik

### Code Signing (İsteğe Bağlı)

Profesyonel kullanım için sertifika alabilirsiniz:

```json
// electron-builder.json
{
  "win": {
    "certificateFile": "path/to/cert.pfx",
    "certificatePassword": "şifre"
  }
}
```

**Faydaları:**
- Windows SmartScreen uyarısını engeller
- Kullanıcılar güvenle yükler
- Profesyonel görünüm

---

## 📊 Sürüm Yönetimi İpuçları

### Test Sürümleri (Beta)

```json
{
  "version": "1.1.0-beta.1"
}
```

### Release Checklist

- [ ] Versiyon numarası güncellendi
- [ ] Changelog hazırlandı
- [ ] Tüm testler geçti (`docs/ELECTRON_TESTING_GUIDE.md`)
- [ ] Build başarılı
- [ ] Dosyalar GitHub'a yüklendi
- [ ] Release yayınlandı
- [ ] Kullanıcılara duyuru yapıldı

---

## 🎯 Özet

**Geliştirme:**
1. Replit veya lokal ortamda kod yazın
2. Her iki modda da (web/desktop) test edin

**Yayınlama:**
1. Versiyon artırın
2. Build oluşturun (`npm run electron:build:win`)
3. GitHub Release yapın
4. Kullanıcılar otomatik güncellenir

**Sürekli geliştirme için bu döngüyü tekrarlayın!** 🔄

---

## 📞 Destek

- Güncelleme sorunları için: `electron/updater.ts`
- Build sorunları için: `docs/DEPLOYMENT_CHECKLIST.md`
- Test için: `docs/ELECTRON_TESTING_GUIDE.md`
