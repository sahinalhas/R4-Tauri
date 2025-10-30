# Rehber360 Electron - Manuel Test Rehberi

## 📋 Test Öncesi Hazırlık

### Gereksinimler
- ✅ Windows 10/11 (64-bit)
- ✅ En az 4 GB RAM
- ✅ 500 MB boş disk alanı
- ✅ Ekran çözünürlüğü: Minimum 1024x768

### Test Ortamı Hazırlığı
1. Temiz bir Windows makinesinde test yapın (VM kullanabilirsiniz)
2. Önceki Rehber360 kurulumlarını kaldırın
3. Antivirüs yazılımını geçici olarak devre dışı bırakın
4. Internet bağlantısını kontrol edin (auto-update için)

---

## 🔧 Kurulum Testleri

### Test 1.1: NSIS Installer - İlk Kurulum
**Adımlar:**
1. `Rehber360-Setup-1.0.0.exe` dosyasını çalıştırın
2. UAC (User Account Control) uyarısını kabul edin
3. Lisans sözleşmesini okuyun ve kabul edin
4. Kurulum konumunu seçin (varsayılan: `C:\Users\<kullanıcı>\AppData\Local\Programs\Rehber360`)
5. "Masaüstü kısayolu oluştur" seçeneğini işaretleyin
6. "Başlat menüsü kısayolu oluştur" seçeneğini işaretleyin
7. Kurulumu tamamlayın

**Beklenen Sonuçlar:**
- ✅ Kurulum başarıyla tamamlanır
- ✅ Masaüstünde "Rehber360" kısayolu oluşturulur
- ✅ Başlat menüsünde "Rehber360" görünür
- ✅ Program otomatik olarak başlatılır

**Test Edilecek Noktalar:**
- [ ] UAC isteği görünüyor mu?
- [ ] Kurulum süresi makul mü? (<2 dakika)
- [ ] Masaüstü ikonu doğru görünüyor mu?
- [ ] Program ilk açılışta çalışıyor mu?
- [ ] Splash screen görünüyor mu?

### Test 1.2: Portable Versiyon
**Adımlar:**
1. `Rehber360-Portable-1.0.0.exe` dosyasını USB belleğe kopyalayın
2. USB bellekten çalıştırın
3. İlk açılışı bekleyin

**Beklenen Sonuçlar:**
- ✅ Kurulum olmadan doğrudan çalışır
- ✅ Tüm dosyalar USB bellek içinde saklanır
- ✅ Farklı bilgisayarlarda çalışır

### Test 1.3: Güncelleme Kurulumu
**Adımlar:**
1. Eski versiyonu kurun (örn: 0.9.0)
2. Yeni versiyonu kurun (1.0.0)
3. Kurulum sırasında "Güncelle" seçeneğini seçin

**Beklenen Sonuçlar:**
- ✅ Eski versiyon kaldırılır
- ✅ Kullanıcı verileri korunur
- ✅ Veritabanı migrate edilir
- ✅ Ayarlar korunur

---

## 🚀 Başlangıç Testleri

### Test 2.1: İlk Açılış
**Adımlar:**
1. Uygulamayı masaüstü kısayolundan başlatın
2. Splash screen'i izleyin
3. Ana pencerenin açılmasını bekleyin

**Beklenen Sonuçlar:**
- ✅ Splash screen 2-3 saniye görünür
- ✅ Ana pencere açılır (1280x800)
- ✅ Dashboard sayfası yüklenir
- ✅ Hiçbir hata mesajı görünmez

**Test Edilecek Noktalar:**
- [ ] Başlangıç süresi (<5 saniye)
- [ ] Pencere boyutu doğru mu?
- [ ] Uygulama ikonu taskbar'da doğru görünüyor mu?
- [ ] Sistem tepsisi ikonu var mı?

### Test 2.2: Pencere Kontrolleri
**Adımlar:**
1. Minimize butonuna tıklayın → Pencere taskbar'a gitmeli
2. Maximize butonuna tıklayın → Tam ekran olmalı
3. Restore butonuna tıklayın → Normal boyuta dönmeli
4. Pencerenin köşesinden yeniden boyutlandırın
5. Close (X) butonuna tıklayın

**Beklenen Sonuçlar:**
- ✅ Tüm pencere kontrolleri çalışır
- ✅ Minimum boyut korunur (1024x768)
- ✅ Close butonu uygulamayı sistem tepsisine gönderir (kapatmaz)
- ✅ Pencere boyutu kaydedilir ve hatırlanır

### Test 2.3: Sistem Tepsisi
**Adımlar:**
1. Uygulamayı kapat (X butonu)
2. Sistem tepsisindeki Rehber360 ikonunu bul
3. Ikona çift tıkla
4. Sağ tıklayıp menüyü aç

**Beklenen Sonuçlar:**
- ✅ Uygulama sistem tepsisinde görünür
- ✅ Çift tıklama pencereyi geri getirir
- ✅ Sağ tık menüsü şu seçenekleri gösterir:
  - Göster
  - Hızlı Erişim (Öğrenciler, Sınavlar, Raporlar)
  - Ayarlar
  - Çıkış

---

## 💾 Veritabanı Testleri

### Test 3.1: İlk Veritabanı Oluşturma
**Adımlar:**
1. Uygulamayı ilk kez başlatın
2. `%APPDATA%\Rehber360\` klasörünü açın
3. `database.db` dosyasını kontrol edin

**Beklenen Sonuçlar:**
- ✅ `database.db` dosyası oluşturulur
- ✅ Dosya boyutu >0 KB
- ✅ Veritabanı şeması doğru yüklenir

### Test 3.2: Veri Ekleme ve Okuma
**Adımlar:**
1. Yeni bir öğrenci ekleyin
2. Öğrenciye sınav notu girin
3. Anket yanıtı ekleyin
4. Uygulamayı kapatın
5. Uygulamayı tekrar açın
6. Eklenen verileri kontrol edin

**Beklenen Sonuçlar:**
- ✅ Tüm veriler kaydedilir
- ✅ Uygulama tekrar açıldığında veriler korunur
- ✅ Hiçbir veri kaybı olmaz

### Test 3.3: Veritabanı Yedekleme
**Adımlar:**
1. Menü → Veritabanı → Yedek Al
2. Kaydetme konumunu seçin
3. Yedek dosyasını kontrol edin

**Beklenen Sonuçlar:**
- ✅ `.db` dosyası başarıyla kaydedilir
- ✅ Dosya adı: `rehber360-backup-YYYY-MM-DD.db`
- ✅ Dosya boyutu orijinal veritabanı ile aynı

### Test 3.4: Veritabanı Geri Yükleme
**Adımlar:**
1. Veritabanını yedekleyin
2. Bazı verileri silin veya değiştirin
3. Menü → Veritabanı → Yedek Geri Yükle
4. Yedek dosyasını seçin
5. Onay verin

**Beklenen Sonuçlar:**
- ✅ Veriler yedeklenen haline döner
- ✅ Onay mesajı gösterilir
- ✅ Uygulama yeniden başlatılır

---

## 📊 Temel İşlevsellik Testleri

### Test 4.1: Öğrenci Yönetimi
**Adımlar:**
1. "Öğrenciler" sayfasına git
2. "Yeni Öğrenci" butonuna tıkla
3. Form doldur (ad, soyad, sınıf, numara)
4. Kaydet

**Beklenen Sonuçlar:**
- ✅ Form doğrulama çalışır
- ✅ Öğrenci başarıyla eklenir
- ✅ Liste otomatik güncellenir
- ✅ Bildirim gösterilir

### Test 4.2: Sınav Notu Girişi
**Adımlar:**
1. Bir öğrenci seç
2. "Not Ekle" butonuna tıkla
3. Sınav türü, ders, not gir
4. Kaydet

**Beklenen Sonuçlar:**
- ✅ Not eklenir
- ✅ Ortalaması otomatik hesaplanır
- ✅ Grafik güncellenir

### Test 4.3: AI Analiz
**Adımlar:**
1. Öğrenci profil sayfasına git
2. "AI Analizi" sekmesine tıkla
3. Analiz başlat

**Beklenen Sonuçlar:**
- ✅ Yükleme göstergesi çalışır
- ✅ AI analizi gösterilir
- ✅ Öneriler listelenir

### Test 4.4: Rapor Oluşturma
**Adımlar:**
1. "Raporlar" sayfasına git
2. Rapor türü seç
3. "PDF Oluştur" butonuna tıkla
4. Kaydetme konumunu seç

**Beklenen Sonuçlar:**
- ✅ PDF oluşturulur
- ✅ Dosya seçme dialogu açılır
- ✅ PDF kaydedilir
- ✅ Başarı mesajı gösterilir

---

## 📁 Dosya İşlemleri Testleri

### Test 5.1: Excel Import
**Adımlar:**
1. Menü → Dosya → Veri İçe Aktar
2. Excel dosyası seç (.xlsx)
3. Import işlemini başlat

**Beklenen Sonuçlar:**
- ✅ Dosya seçme dialogu açılır
- ✅ Sadece .xlsx ve .xls dosyaları görünür
- ✅ Import başarılı olur
- ✅ Öğrenciler listeye eklenir

### Test 5.2: PDF Export
**Adımlar:**
1. Bir rapor sayfasında "PDF Olarak Kaydet" tıkla
2. Kaydetme konumu ve dosya adı belirle
3. Kaydet

**Beklenen Sonuçlar:**
- ✅ Kaydetme dialogu açılır
- ✅ Varsayılan dosya adı önerilir
- ✅ PDF başarıyla kaydedilir
- ✅ Dosya boyutu makul (<5 MB)

### Test 5.3: Excel Export
**Adımlar:**
1. Öğrenci listesinde "Excel'e Aktar" tıkla
2. Kaydetme konumu seç
3. Excel dosyasını aç ve kontrol et

**Beklenen Sonuçlar:**
- ✅ .xlsx dosyası oluşturulur
- ✅ Tüm sütunlar mevcut
- ✅ Veriler doğru formatta
- ✅ Excel ile açılabilir

---

## 🔔 Bildirim Testleri

### Test 6.1: Native Bildirimler
**Adımlar:**
1. Ayarlar → Bildirimler → "Risk Öğrenci Bildirimleri" aktif et
2. Risk profilinde bir öğrenci ekle
3. Bildirimi bekle

**Beklenen Sonuçlar:**
- ✅ Windows bildirimi görünür
- ✅ Bildirim başlığı ve içeriği doğru
- ✅ Bildirime tıklayınca ilgili sayfa açılır
- ✅ Ses çalınır (ayarlara göre)

### Test 6.2: Günlük Rapor Bildirimi
**Adımlar:**
1. Ayarlar → Bildirimler → "Günlük Rapor" aktif et
2. Saati ayarla (örn: 17:00)
3. Belirlenen saati bekle

**Beklenen Sonuçlar:**
- ✅ Bildirim belirlenen saatte gelir
- ✅ Rapor özeti gösterilir

---

## 🎨 Görünüm Testleri

### Test 7.1: Tema Değişimi
**Adımlar:**
1. Ayarlar → Görünüm → "Koyu Tema" seç
2. Tüm sayfaları gez
3. "Açık Tema" seç

**Beklenen Sonuçlar:**
- ✅ Tema anında değişir
- ✅ Tüm bileşenler doğru renklendirilir
- ✅ Tercih kaydedilir

### Test 7.2: Tam Ekran Modu
**Adımlar:**
1. F11 tuşuna bas
2. Menü → Görünüm → Tam Ekran

**Beklenen Sonuçlar:**
- ✅ Uygulama tam ekran olur
- ✅ Title bar gizlenir
- ✅ F11 veya ESC ile normal moda dönülür

### Test 7.3: Zoom/Büyütme
**Adımlar:**
1. Ctrl + (yakınlaştır)
2. Ctrl - (uzaklaştır)
3. Ctrl 0 (sıfırla)

**Beklenen Sonuçlar:**
- ✅ Zoom seviyesi değişir
- ✅ Tüm bileşenler oransal büyür/küçülür
- ✅ Tercih kaydedilir

---

## ⌨️ Klavye Kısayolları Testleri

### Test 8.1: Temel Kısayollar
Test edilecek kısayollar:
- [ ] Ctrl+N → Yeni öğrenci
- [ ] Ctrl+S → Kaydet
- [ ] Ctrl+P → Yazdır
- [ ] Ctrl+F → Ara
- [ ] Ctrl+R / F5 → Yenile
- [ ] Ctrl+Q → Çıkış
- [ ] F11 → Tam ekran
- [ ] Ctrl+Shift+I → DevTools (geliştirici modu)

---

## 🔄 Auto-Update Testleri

### Test 9.1: Güncelleme Kontrolü
**Adımlar:**
1. Uygulamayı başlat
2. 5 saniye bekle (otomatik kontrol)
3. Menü → Yardım → Güncellemeleri Kontrol Et

**Beklenen Sonuçlar:**
- ✅ "Güncellemeler kontrol ediliyor..." mesajı
- ✅ "Güncel versiyon kullanıyorsunuz" veya
- ✅ "Yeni versiyon mevcut" bildirimi

### Test 9.2: Güncelleme İndirme
**Adımlar:**
1. Yeni versiyon mevcut olduğunda
2. "İndir" butonuna tıkla
3. İndirme ilerlemesini izle

**Beklenen Sonuçlar:**
- ✅ İndirme çubuğu gösterilir
- ✅ İlerleme %0'dan %100'e gider
- ✅ "Güncelleme hazır" bildirimi gelir

### Test 9.3: Güncelleme Kurulumu
**Adımlar:**
1. "Yeniden Başlat ve Güncelle" tıkla
2. Uygulama kapanmasını bekle
3. Güncellemenin kurulmasını bekle
4. Uygulama otomatik başlasın

**Beklenen Sonuçlar:**
- ✅ Uygulama düzgün kapanır
- ✅ Güncelleme kurulur
- ✅ Uygulama yeni versiyonla başlar
- ✅ Veriler korunur

---

## 🛡️ Güvenlik Testleri

### Test 10.1: Pencere Güvenliği
**Adımlar:**
1. F12 veya Ctrl+Shift+I (DevTools) dene
2. Sağ tık → "Sayfa Kaynağını Görüntüle" dene

**Beklenen Sonuçlar:**
- ✅ Production build'de DevTools açılmaz
- ✅ Context menu kısıtlıdır

### Test 10.2: Dosya Sistemi Güvenliği
**Adımlar:**
1. Frontend'den dosya yolu girişi dene
2. SQL injection test et
3. XSS saldırısı dene

**Beklenen Sonuçlar:**
- ✅ Dosya yolu validasyonu çalışır
- ✅ SQL injection engellenir
- ✅ XSS filtrelenir

---

## 📊 Performans Testleri

### Test 11.1: Başlangıç Süresi
**Adımlar:**
1. Uygulamayı kapat
2. Kronometre başlat
3. Uygulamayı aç
4. Dashboard tam yüklenene kadar bekle

**Beklenen Sonuçlar:**
- ✅ Başlangıç <5 saniye (SSD)
- ✅ Başlangıç <8 saniye (HDD)

### Test 11.2: Büyük Veri Performansı
**Adımlar:**
1. 1000+ öğrenci ekle
2. Listeleme hızını test et
3. Arama performansını test et
4. Rapor oluşturma süresini ölç

**Beklenen Sonuçlar:**
- ✅ Listeleme <2 saniye
- ✅ Arama <1 saniye
- ✅ Rapor oluşturma <10 saniye

### Test 11.3: Bellek Kullanımı
**Adımlar:**
1. Task Manager'ı aç
2. Rehber360 bellek kullanımını izle
3. Tüm sayfaları gez
4. 30 dakika kullan
5. Bellek kullanımını tekrar kontrol et

**Beklenen Sonuçlar:**
- ✅ İlk bellek <200 MB
- ✅ 30 dakika sonra <400 MB
- ✅ Bellek sızıntısı yok

---

## 🧪 Stres Testleri

### Test 12.1: Hızlı Pencere Değişimleri
**Adımlar:**
1. Hızlıca minimize/maximize yap (20 kez)
2. Tam ekran modu aç/kapat (10 kez)
3. Pencereyi köşelerden yeniden boyutlandır (10 kez)

**Beklenen Sonuçlar:**
- ✅ Uygulama crash olmaz
- ✅ Pencere kontrolleri düzgün çalışır

### Test 12.2: Hızlı Sayfa Geçişleri
**Adımlar:**
1. Menüdeki tüm sayfalara sırayla git (3 tur)
2. Geri/ileri butonlarını hızlıca kullan

**Beklenen Sonuçlar:**
- ✅ Sayfa geçişleri sorunsuz
- ✅ Veri kaybolmaz
- ✅ UI donmaz

### Test 12.3: Çoklu İşlem
**Adımlar:**
1. AI analizi başlat
2. Aynı anda PDF export yap
3. Aynı anda Excel import yap

**Beklenen Sonuçlar:**
- ✅ Tüm işlemler eşzamanlı çalışır
- ✅ Uygulama donmaz
- ✅ Tüm işlemler başarılı olur

---

## 🌐 Çevrimdışı Mod Testleri

### Test 13.1: Internet Bağlantısı Yok
**Adımlar:**
1. Internet bağlantısını kes
2. Uygulamayı aç
3. Tüm temel işlevleri test et

**Beklenen Sonuçlar:**
- ✅ Uygulama çalışır
- ✅ Temel işlevler kullanılabilir
- ✅ AI işlevleri devre dışı kalır

### Test 13.2: Bağlantı Kesintisi
**Adımlar:**
1. Uygulama çalışırken interneti kes
2. AI analizi başlatmayı dene
3. Interneti tekrar aç

**Beklenen Sonuçlar:**
- ✅ Bağlantı hatası mesajı gösterilir
- ✅ Uygulama crash olmaz
- ✅ Bağlantı dönünce işlevler çalışır

---

## 📝 Test Sonuç Şablonu

### Test Sonucu Raporu
```
Tarih: _____________
Tester: _____________
Platform: Windows 10/11
Versiyon: 1.0.0

Başarılı Testler: ___ / ___
Başarısız Testler: ___

Kritik Hatalar:
- 
- 

Minör Hatalar:
- 
- 

İyileştirme Önerileri:
- 
- 

Genel Değerlendirme:
[ ] Üretime hazır
[ ] Düzeltme gerekli
[ ] Major sorunlar var

Notlar:


Imza: _____________
```

---

## ✅ Kabul Kriterleri

Uygulamanın production'a çıkabilmesi için:

### Zorunlu (Must-Have)
- ✅ Tüm kurulum testleri başarılı
- ✅ Temel işlevsellik testlerinin %100'ü başarılı
- ✅ Kritik hata yok
- ✅ Veri kaybı yok
- ✅ Güvenlik testleri başarılı
- ✅ Başlangıç süresi <8 saniye

### İsteğe Bağlı (Nice-to-Have)
- ✅ Tüm performans testleri hedefleri tutuyor
- ✅ Auto-update çalışıyor
- ✅ Tüm klavye kısayolları çalışıyor

---

## 🐛 Hata Raporlama

Hata bulunduğunda:

1. **Ekran görüntüsü** alın
2. **Hata mesajı** tam metnini kaydedin
3. **Adımları** tekrarlayın ve kaydedin
4. **Log dosyalarını** kontrol edin:
   - `%APPDATA%\Rehber360\logs\main.log`
5. GitHub Issues'a detaylı rapor açın

**Hata Raporu Şablonu:**
```
### Hata Tanımı
Kısa açıklama

### Adımlar
1. 
2. 
3. 

### Beklenen Davranış
Ne olması gerekiyordu?

### Gerçek Davranış
Ne oldu?

### Ortam
- OS: Windows 10/11
- Versiyon: 1.0.0
- Kurulum Türü: NSIS/Portable

### Log
```
Log içeriği
```

### Ekran Görüntüsü
(Ekle)
```

---

## 📞 Destek

Test sürecinde sorularınız için:
- GitHub Issues: [repository]/issues
- Email: support@rehber360.com

**Mutlu Testler! 🎉**
