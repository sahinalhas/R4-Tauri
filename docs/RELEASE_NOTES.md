# Rehber360 v2.0.0 - Sürüm Notları

## 🎉 Tauri'ye Geçiş - Büyük Güncelleme

**Yayın Tarihi:** 2025

Rehber360, tamamen yeniden tasarlandı! Electron'dan Tauri'ye geçiş ile birlikte uygulama çok daha hızlı, hafif ve güvenli hale geldi.

---

## 🚀 Neler Yeni?

### Performans İyileştirmeleri
- **%93 Daha Küçük:** 150MB → ~10MB uygulama boyutu
- **%67 Daha Az Bellek:** 600MB → 200MB RAM kullanımı
- **%50 Daha Hızlı:** 2 saniye → <1 saniye başlangıç süresi
- **%50 Daha Az CPU:** CPU kullanımı yarı yarıya azaltıldı

### Yeni Özellikler
- ✨ **Yerli Masaüstü Deneyimi:** Sistem tepsisi, yerel bildirimler, pencere kontrolü
- 🔒 **Gelişmiş Güvenlik:** Multi-layer güvenlik (SQLx, input validation, sandboxing)
- 📦 **Veri Export/Import:** JSON ve CSV formatında tam veri yedekleme
- 🧪 **Test Altyapısı:** 18 otomatik test ile kararlılık garantisi
- 🔄 **Otomatik Güncelleme:** GitHub üzerinden otomatik uygulama güncellemeleri
- 📱 **Daha İyi Responsive:** Mobil ve tablet uyumlu modern arayüz

### Teknik İyileştirmeler
- **Rust Backend:** Yüksek performans ve güvenlik
- **SQLx Database:** Type-safe veritabanı işlemleri
- **Tauri v2.0.0:** En güncel teknoloji stack
- **Multi-platform:** Windows, macOS, Linux desteği
- **CI/CD Pipeline:** Otomatik build ve release süreci

---

## ⚠️ Breaking Changes (Önemli Değişiklikler)

### Veritabanı Şeması Güncellemeleri
- Tüm tablolar SQLx uyumlu hale getirildi
- Yeni `settings` tablosu eklendi
- Foreign key ilişkileri güçlendirildi

### API Değişiklikleri
- Express.js API'den Tauri Command API'ye geçiş
- Tüm endpoint'ler Rust tarafında yeniden implemente edildi
- Response formatları standartlaştırıldı

### Dosya Konumları
- **Eski (Electron):** `%APPDATA%/rehber360-electron/`
- **Yeni (Tauri):** `%APPDATA%/com.rehber360.app/`

### Konfigürasyon
- Environment variables yerine Tauri Config kullanımı
- API anahtarları `settings` tablosunda saklanıyor

---

## 🔄 Migration Guide (Geçiş Kılavuzu)

### Otomatik Veri Aktarımı
Rehber360 v2.0.0 ilk açılışta otomatik olarak eski verilerinizi tespit edip aktarır:

1. **Veritabanı:** Tüm öğrenci, seans ve dosya verileri
2. **Ayarlar:** Kullanıcı tercihleri ve tema ayarları
3. **Dosyalar:** Öğrenci belgeleri ve raporlar

### Manuel Veri Aktarımı
Otomatik aktarım başarısız olursa:

1. Eski uygulamadan **Ayarlar → Veri İçe/Dışa Aktar → JSON Export**
2. Yeni uygulamada **Ayarlar → Veri İçe/Dışa Aktar → JSON Import**
3. Export edilen dosyayı seçin ve içe aktarın

### AI Provider Ayarları
API anahtarlarınızı yeniden girmeniz gerekebilir:
- **Ayarlar → AI Ayarları → API Anahtarları** bölümünden güncelleyin

---

## 🐛 Bilinen Sorunlar

### v2.0.0
- **macOS:** İlk açılışta "Geliştirici doğrulanamadı" uyarısı (sağ tık → Aç ile çözülür)
- **Linux:** Sistem tepsisi bazı masaüstü ortamlarında görünmeyebilir
- **Windows 7:** Desteklenmiyor (Windows 10+ gereklidir)

---

## 📊 Karşılaştırma Tablosu

| Özellik | Electron (v1.x) | Tauri (v2.0.0) |
|---------|----------------|----------------|
| Uygulama Boyutu | 150MB | ~10MB |
| RAM (Boşta) | 300MB | <100MB |
| RAM (Aktif) | 600MB | <250MB |
| Başlangıç Süresi | 2 saniye | <1 saniye |
| CPU Kullanımı | ~20% | <10% |
| Güvenlik | Orta | Yüksek |
| Platform Desteği | Windows, macOS, Linux | Windows, macOS, Linux |
| Otomatik Güncelleme | ✅ | ✅ |
| Sistem Tepsisi | ✅ | ✅ |
| Yerel Bildirimler | Kısıtlı | Tam Destek |

---

## 🎯 Gelecek Sürümler

### v2.1.0 (Planlanan)
- 📱 Mobil uygulama (iOS/Android)
- 🌐 Web versiyonu
- 🤖 Daha gelişmiş AI analiz özellikleri
- 📊 Gelişmiş raporlama ve dashboard

### v2.2.0 (Planlanan)
- 👥 Multi-user desteği
- ☁️ Cloud sync
- 🔗 Okul yönetim sistemleri entegrasyonu

---

## 🙏 Teşekkürler

Bu büyük güncelleme için geri bildirimleriniz ve sabırınız için teşekkür ederiz!

**Destek ve Geri Bildirim:**
- 🐛 Bug Raporu: [GitHub Issues](https://github.com/your-repo/rehber360/issues)
- 💬 Öneriler: support@rehber360.com
- 📖 Dokümantasyon: [docs/](./docs/)

---

## 📝 Değişiklik Geçmişi

### v2.0.0 (2025-10-31)
- 🎉 **MAJOR:** Electron'dan Tauri'ye tam geçiş
- ✨ **NEW:** Rust backend ile yeniden yazıldı
- ✨ **NEW:** SQLx database layer
- ✨ **NEW:** 85+ Tauri command API
- ✨ **NEW:** Multi-layer güvenlik sistemi
- ✨ **NEW:** Otomatik test suite (18 test)
- ✨ **NEW:** JSON/CSV export/import
- ✨ **NEW:** GitHub Actions CI/CD
- ✨ **NEW:** Yerli masaüstü özellikleri (tray, notifications)
- 📚 **DOCS:** Kapsamlı teknik dokümantasyon
- ⚡ **PERF:** %93 daha küçük, %67 daha az RAM
- 🔒 **SEC:** XSS, SQL injection, path traversal koruması

### v1.5.0 (2025-03-15) - Son Electron Sürümü
- ✨ AI Assistant özellikleri
- 🐛 Bug düzeltmeleri
- 📊 Geliştirilmiş raporlama

---

**Not:** v1.x (Electron) sürümleri artık desteklenmeyecektir. Lütfen v2.0.0'a geçiş yapın.
