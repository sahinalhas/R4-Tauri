# Rehber360 Kullanım Kılavuzu

## 📖 İçindekiler
1. [Kurulum](#kurulum)
2. [İlk Kullanım](#ilk-kullanım)
3. [Temel Özellikler](#temel-özellikler)
4. [Öğrenci Yönetimi](#öğrenci-yönetimi)
5. [Görüşme Takibi](#görüşme-takibi)
6. [AI Asistan](#ai-asistan)
7. [Raporlama](#raporlama)
8. [Ayarlar](#ayarlar)
9. [Sık Karşılaşılan Sorunlar](#sık-karşılaşılan-sorunlar)

---

## 🚀 Kurulum

### Windows
1. `Rehber360-Setup-2.0.0.exe` dosyasını indirin
2. Kurulum dosyasını çalıştırın
3. Kurulum sihirbazını takip edin
4. "Başlat" menüsünden Rehber360'ı açın

### macOS
1. `Rehber360-2.0.0.dmg` dosyasını indirin
2. DMG dosyasını açın ve Rehber360'ı Applications klasörüne sürükleyin
3. İlk açılışta "Geliştirici doğrulanamadı" uyarısı alırsanız: Sağ tık → Aç

### Linux
1. `.deb` (Debian/Ubuntu) veya `.AppImage` dosyasını indirin
2. **DEB:** `sudo dpkg -i Rehber360-2.0.0.deb`
3. **AppImage:** Çalıştırılabilir izni verin ve açın

---

## 🎯 İlk Kullanım

### 1. İlk Açılış Sihirbazı
Uygulama ilk kez açıldığında karşınıza çıkacak:

**Adım 1: Veritabanı Kurulumu**
- Yeni bir veritabanı oluşturun
- VEYA eski verilerinizi içe aktarın

**Adım 2: Yönetici Hesabı**
- Email: `rehber@okul.edu.tr`
- Şifre: Güçlü bir şifre belirleyin
- Ad ve soyad bilgilerinizi girin

**Adım 3: AI Provider Seçimi**
- **OpenAI:** Güçlü, ücretli (API anahtarı gerekir)
- **Gemini:** Google'ın AI'ı, ücretsiz kotası yüksek
- **Ollama:** Tamamen lokal, gizlilik odaklı (kurulum gerektirir)

**Adım 4: Demo Veri (Opsiyonel)**
- Test için örnek öğrenci verisi eklensin mi?

### 2. Giriş Yapma
- Email ve şifrenizi girin
- "Beni Hatırla" seçeneğini işaretleyebilirsiniz
- İlk girişte tüm özellikleri keşfetmek için tur başlatılır

---

## 📚 Temel Özellikler

### Dashboard (Ana Sayfa)
Dashboard'da şunları görebilirsiniz:
- 📊 **İstatistikler:** Toplam öğrenci, risk durumu, yaklaşan görüşmeler
- 📅 **Takvim:** Bugünün ve yaklaşan etkinlikler
- ⚠️ **Uyarılar:** Dikkat gerektiren öğrenciler
- 📈 **Grafikler:** Trend analizleri

### Hızlı İşlemler
- **Yeni Öğrenci Ekle:** ➕ butonu
- **Görüşme Oluştur:** 📝 butonu
- **AI Analiz:** 🤖 butonu
- **Arama:** 🔍 arama çubuğu (Ctrl+K)

---

## 👨‍🎓 Öğrenci Yönetimi

### Öğrenci Ekleme
1. **Sol menü → Öğrenciler → Yeni Öğrenci**
2. Zorunlu alanları doldurun:
   - Ad, Soyad
   - TC Kimlik No / Öğrenci No
   - Sınıf, Şube
   - Doğum Tarihi
3. Opsiyonel bilgiler:
   - Veli iletişim bilgileri
   - Sağlık durumu
   - Fotoğraf
4. **Kaydet** butonuna tıklayın

### Öğrenci Profili
Her öğrenci için:
- **Genel Bilgiler:** Kişisel ve iletişim bilgileri
- **Akademik Durum:** Not ortalaması, devamsızlık
- **Görüşme Geçmişi:** Tüm yapılan görüşmeler
- **Davranış Kayıtları:** Olumlu/olumsuz davranışlar
- **AI Profil:** Otomatik oluşturulan psikolojik profil
- **Belgeler:** Yüklenen raporlar ve belgeler

### Toplu İşlemler
1. Listeden öğrencileri seçin (checkbox)
2. Üstteki **Toplu İşlemler** menüsünden seçin:
   - Excel'e aktar
   - PDF rapor oluştur
   - Toplu mesaj gönder
   - Risk durumu güncelle

### Filtreleme ve Arama
- **Hızlı Filtreler:** Risk durumu, sınıf, cinsiyet
- **Gelişmiş Arama:** Ctrl+K → çoklu kriter
- **Sıralama:** Sütun başlıklarına tıklayarak

---

## 💬 Görüşme Takibi

### Yeni Görüşme Oluşturma
1. **Öğrenci profili → Yeni Görüşme**
2. Görüşme türünü seçin:
   - Bireysel danışmanlık
   - Grup çalışması
   - Veli görüşmesi
   - Kriz müdahalesi
3. **Tarih ve Süre:** Planlı veya tamamlandı olarak işaretleyin
4. **Notlar:** 
   - Manuel yazın
   - VEYA ses kaydından otomatik transkript (🎤 butonu)
5. **AI Analiz (Opsiyonel):**
   - Duygu analizi
   - Risk anahtar kelimeleri
   - Öneriler

### Görüşme Şablonları
Sık kullanılan görüşme formatları için şablonlar:
- **Ayarlar → Görüşme Şablonları**
- Yeni şablon oluşturun veya mevcut şablonları düzenleyin

### Ses Kaydı ve Transkript
1. Görüşme sırasında 🎤 **Ses Kaydı Başlat**
2. Görüşme bitince **Durdur**
3. Otomatik olarak metin dönüşümü yapılır
4. AI özeti ve analiz oluşturulur

---

## 🤖 AI Asistan

### AI Profil Analizi
Sistem otomatik olarak öğrenci verilerinden analiz yapar:
- **Akademik Performans:** Trend ve riskler
- **Sosyal-Duygusal Durum:** Davranış kalıpları
- **Risk Değerlendirmesi:** Erken uyarı sistemi

### AI Asistan ile Konuşma
1. **Sol menü → AI Asistan**
2. Sorunuzu yazın veya ses kaydı yapın
3. Örnek sorular:
   - "Ahmet Yılmaz'ın akademik durumu nasıl?"
   - "Bu hafta hangi öğrencilerle görüşmeliyim?"
   - "Sınıf 9A'nın genel durumunu analiz et"
   - "Risk altındaki öğrenciler kimler?"

### AI Önerileri
AI asistan size proaktif öneriler sunar:
- ⚠️ **Risk Uyarıları:** Dikkat edilmesi gereken öğrenciler
- 📅 **Görüşme Önerileri:** Kiminle ne zaman görüşmeli?
- 📊 **Müdahale Planları:** Kanıt tabanlı öneriler
- 📈 **İlerleme Takibi:** Müdahalelerin etkisi

### AI Provider Değiştirme
**Ayarlar → AI Ayarları:**
- **OpenAI (GPT-4):** En güçlü, ücretli
- **Gemini:** Ücretsiz kota, iyi performans
- **Ollama (Llama 3.1):** Lokal, tamamen gizli

---

## 📊 Raporlama

### Hızlı Raporlar
**Sol menü → Raporlar:**
- **Öğrenci Listesi:** Filtrelenmiş liste (PDF/Excel)
- **Görüşme Raporu:** Belirli tarih aralığı
- **Risk Raporu:** Yüksek risk öğrenciler
- **Akademik Rapor:** Başarı analizleri

### Özel Rapor Oluşturma
1. **Raporlar → Özel Rapor**
2. Kriterlerinizi seçin:
   - Öğrenci grubu
   - Tarih aralığı
   - Dahil edilecek bilgiler
3. Format seçin: PDF, Excel, CSV
4. **Oluştur ve İndir**

### Otomatik Raporlar
**Ayarlar → Otomatik Raporlar:**
- Haftalık özet rapor (email ile)
- Aylık genel durum raporu
- Dönem sonu değerlendirme raporu

---

## ⚙️ Ayarlar

### Genel Ayarlar
- **Dil:** Türkçe (varsayılan)
- **Tema:** Açık / Koyu / Sistem
- **Bildirimler:** Masaüstü bildirimleri açık/kapalı
- **Otomatik Yedekleme:** Günlük/Haftalık

### Kullanıcı Yönetimi
**Ayarlar → Kullanıcılar (Admin):**
- Yeni kullanıcı ekle
- Roller: Admin, Rehber Öğretmen, Öğretmen, Gözlemci
- İzinleri düzenle

### AI Ayarları
- **Provider:** OpenAI / Gemini / Ollama
- **API Anahtarı:** Providerdan aldığınız anahtar
- **Model:** Kullanılacak AI modeli
- **Sıcaklık:** 0.1-1.0 (yaratıcılık seviyesi)

### Veri Yönetimi
**Ayarlar → Veri Yönetimi:**
- **Export:** Tüm veriyi JSON/Excel olarak dışa aktar
- **Import:** Eski sistemden veri içe aktar
- **Yedekleme:** Manuel veya otomatik yedek alma
- **Geri Yükleme:** Yedekten geri yükleme

---

## 🔍 Sık Karşılaşılan Sorunlar

### Uygulama Açılmıyor
**Çözüm:**
1. Bilgisayarı yeniden başlatın
2. Antivirüs programını geçici olarak kapatın
3. Uygulamayı yönetici olarak çalıştırın
4. Eski sürümü kaldırıp yeniden kurun

### AI Çalışmıyor
**Kontrol Edin:**
- ✅ API anahtarı doğru girilmiş mi?
- ✅ İnternet bağlantısı var mı?
- ✅ API limitiniz dolmamış mı?
- ✅ Doğru provider seçili mi?

### Veriler Gözükmüyor
**Çözüm:**
1. **Ayarlar → Veri Yönetimi → Veritabanı Kontrolü**
2. Hata varsa "Onar" butonuna tıklayın
3. Sorun devam ederse yedekten geri yükleyin

### Yavaş Çalışıyor
**Optimizasyon:**
- Eski verileri arşivleyin
- Gereksiz dosyaları silin
- Veritabanını optimize edin (**Ayarlar → Veritabanı → Optimize**)
- RAM 4GB altındaysa bilgisayarı yükseltin

### Sesli Transkript Çalışmıyor
**Kontrol Edin:**
- Mikrofon izni verilmiş mi?
- AI provider ayarları doğru mu?
- İnternet bağlantısı stabil mi?

---

## 📞 Destek

### Yardım Kaynakları
- 📖 **Dokümantasyon:** [docs.rehber360.com](https://docs.rehber360.com)
- 💬 **Topluluk Forum:** [forum.rehber360.com](https://forum.rehber360.com)
- 📧 **Email Destek:** support@rehber360.com
- 🐛 **Bug Raporu:** [GitHub Issues](https://github.com/your-repo/rehber360/issues)

### Videolu Eğitimler
YouTube kanalımızda adım adım videolar:
- Temel kullanım
- Gelişmiş özellikler
- AI asistan kullanımı
- Raporlama ipuçları

---

## 🎓 İpuçları ve Püf Noktaları

### Klavye Kısayolları
- **Ctrl+K:** Hızlı arama
- **Ctrl+N:** Yeni öğrenci
- **Ctrl+M:** Yeni görüşme
- **Ctrl+S:** Kaydet
- **Ctrl+Z:** Geri al
- **F11:** Tam ekran

### Verimlilik Artırma
1. **Şablonlar kullanın:** Tekrar eden görüşmeler için
2. **Toplu işlemler:** Çoklu öğrenci için tek seferde
3. **Filtreleri kaydedin:** Sık kullanılan filtreler için
4. **AI'dan yararlanın:** Manuel analiz yerine otomatik

### Veri Güvenliği
- 🔒 Düzenli yedek alın (haftada 1)
- 🔐 Güçlü şifre kullanın
- 🚪 Bilgisayardan ayrılırken kilitleyin
- 👥 Kullanıcı rollerini doğru ayarlayın

---

**Son Güncelleme:** 31 Ekim 2025
**Versiyon:** 2.0.0

Daha fazla bilgi için dokümantasyon sitesini ziyaret edin: [docs.rehber360.com](https://docs.rehber360.com)
