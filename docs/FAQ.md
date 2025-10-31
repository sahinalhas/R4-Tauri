# Sık Sorulan Sorular (FAQ)

## 📋 İçindekiler
- [Genel Sorular](#genel-sorular)
- [Kurulum ve Sistem](#kurulum-ve-sistem)
- [Öğrenci Yönetimi](#öğrenci-yönetimi)
- [AI ve Analiz](#ai-ve-analiz)
- [Veri Güvenliği](#veri-güvenliği)
- [Teknik Sorunlar](#teknik-sorunlar)
- [Lisans ve Fiyatlandırma](#lisans-ve-fiyatlandırma)

---

## 🌟 Genel Sorular

### Rehber360 nedir?
Rehber360, okul rehberlik servislerinin öğrenci takibi, danışmanlık yönetimi ve AI destekli analiz yapmasını sağlayan kapsamlı bir sistemdir.

### v2.0.0'da ne değişti?
Electron'dan Tauri'ye geçiş ile:
- %93 daha küçük boyut (150MB → 10MB)
- %67 daha az RAM kullanımı
- %50 daha hızlı başlangıç
- Gelişmiş güvenlik
- Yerli masaüstü özellikleri

### Hangi platformlarda çalışır?
- **Windows:** 10 ve üzeri (64-bit)
- **macOS:** 10.15 (Catalina) ve üzeri
- **Linux:** Ubuntu 20.04+, Debian 11+, Fedora 35+

### Ücretsiz mi?
Rehber360 **açık kaynak** bir projedir ve **ücretsiz** kullanılabilir. AI özellikleri için API anahtarı gerektiren servislerde (OpenAI) kendi API hesabınızı kullanmanız gerekir.

---

## 💻 Kurulum ve Sistem

### Minimum sistem gereksinimleri nedir?
**Minimum:**
- İşlemci: Dual-core 2.0 GHz
- RAM: 2 GB
- Disk: 500 MB boş alan
- İnternet: AI özellikleri için gerekli

**Önerilen:**
- İşlemci: Quad-core 2.5 GHz+
- RAM: 4 GB+
- Disk: 1 GB+ (veri için)
- SSD kullanımı önerilir

### Kurulum hatası alıyorum, ne yapmalıyım?
1. Eski sürümü tamamen kaldırın
2. Antivirüs programını geçici olarak kapatın
3. Kurulum dosyasını **Yönetici olarak çalıştırın**
4. Windows için: Microsoft Visual C++ Redistributable yükleyin
5. macOS için: Sistem Tercihleri → Güvenlik ve Gizlilik → "İzin Ver"

### Windows Defender uyarı veriyor
Bu normaldir çünkü uygulama yeni ve dijital imza henüz yaygın değil. Güvenli olduğundan emin olmak için:
- Resmi GitHub repodan indirin
- Checksum'ı kontrol edin
- "Daha fazla bilgi" → "Yine de çalıştır" seçin

### macOS'ta "Geliştirici doğrulanamadı" hatası
**Çözüm:**
1. Uygulamaya sağ tıklayın
2. "Aç" seçeneğini seçin
3. Gelen uyarıda "Aç" butonuna tıklayın
4. Bir sonraki açılışlarda sorun olmaz

---

## 👨‍🎓 Öğrenci Yönetimi

### Kaç öğrenci ekleyebilirim?
Teknik limit yoktur. Test ortamında 10,000+ öğrenci ile başarıyla çalışıldı. Performans bilgisayarınızın özelliklerine bağlıdır.

### Excel'den toplu öğrenci yükleyebilir miyim?
Evet! **Öğrenciler → İçe Aktar → Excel** seçeneğini kullanın. Şablon dosyasını indirip doldurun.

### Öğrenci fotoğrafları nerede saklanıyor?
Fotoğraflar veritabanında Base64 formatında saklanır. Alternatif olarak dosya sistemi kullanımı da mevcuttur (**Ayarlar → Depolama**).

### Mezun olan öğrencileri nasıl arşivlerim?
**Öğrenci profili → ⋯ Menü → Arşivle**. Arşivlenen öğrenciler normal listede gözükmez ama veriler korunur. **Arşiv → Arşivlenen Öğrenciler**'den erişebilirsiniz.

### Öğrenci verilerini silebilir miyim?
Evet, ancak **kalıcı silme** işlemidir ve geri alınamaz. KVKK uyumluluğu için veri silme talepleri için kullanılır.

---

## 🤖 AI ve Analiz

### Hangi AI sağlayıcılarını destekliyorsunuz?
1. **OpenAI (GPT-4/GPT-3.5):** En güçlü, ücretli
2. **Google Gemini:** Ücretsiz kota yüksek, iyi performans
3. **Ollama (Llama 3.1):** Tamamen lokal, %100 gizli

### AI için API anahtarı nasıl alırım?
**OpenAI:**
1. [platform.openai.com](https://platform.openai.com) → Kaydol
2. API Keys → Create new key
3. Anahtarı kopyalayın
4. Rehber360 → Ayarlar → AI Ayarları → Yapıştırın

**Gemini:**
1. [ai.google.dev](https://ai.google.dev) → Get API Key
2. Free tier için Google hesabıyla giriş
3. Anahtarı oluşturun ve kopyalayın

### Ollama nasıl kurulur?
1. [ollama.ai](https://ollama.ai) → İndirin
2. Kurulumu tamamlayın
3. Terminal: `ollama pull llama3.1`
4. Rehber360 → AI Ayarları → Provider: Ollama

### AI analizleri ne kadar güvenilir?
AI analizleri **karar destek aracıdır**, kesin teşhis değildir. Profesyonel rehber öğretmen değerlendirmesi şarttır. AI sadece kalıpları tespit eder ve öneriler sunar.

### AI önerileri kişisel verileri paylaşıyor mu?
**Hayır.** Analizler sırasında:
- Öğrenci isimleri kodlanır
- Hassas veriler anonimleştirilir
- Ollama kullanırsanız hiçbir veri internete gitmez

### AI analiz maliyeti ne kadar?
**OpenAI:** ~$0.01-0.05 per öğrenci analizi (GPT-3.5), ~$0.10-0.30 (GPT-4)
**Gemini:** Aylık ücretsiz kota: 60 istek/dakika
**Ollama:** Tamamen ücretsiz (sadece elektrik)

---

## 🔒 Veri Güvenliği

### Veriler nerede saklanıyor?
**Lokal:** Tüm veriler bilgisayarınızda SQLite veritabanında saklanır.
**Konum:**
- Windows: `%APPDATA%/com.rehber360.app/database.db`
- macOS: `~/Library/Application Support/com.rehber360.app/database.db`
- Linux: `~/.local/share/com.rehber360.app/database.db`

### Bulut desteği var mı?
Şu anda hayır. v2.1.0'da opsiyonel cloud sync planlanıyor.

### KVKK uyumlu mu?
Evet. Rehber360:
- ✅ Veri minimizasyonu
- ✅ Lokal depolama (3. taraflara aktarılmaz)
- ✅ Veri silme hakkı desteği
- ✅ Şifreli yedekleme
- ✅ Rol bazlı erişim kontrolü

### Veritabanı şifrelenmiş mi?
Varsayılan olarak hayır (SQLite plain-text). Ancak:
- Disk şifreleme (BitLocker, FileVault) kullanabilirsiniz
- Şifreli yedekleme özelliği mevcuttur
- v2.1.0'da native database encryption planlanıyor

### Yedekleme nasıl yapılır?
**Otomatik:**
- Ayarlar → Veri Yönetimi → Otomatik Yedekleme: Günlük/Haftalık

**Manuel:**
- Ayarlar → Veri Yönetimi → Yedek Al → JSON veya .db dosyası

### Veri kaybı yaşarsam ne yapmalıyım?
1. **Ayarlar → Veri Yönetimi → Geri Yükle**
2. Son yedek dosyasını seçin
3. Geri yükleme işlemini onaylayın

---

## 🔧 Teknik Sorunlar

### Uygulama çöküyor / donuyor
**Çözümler:**
1. Veritabanını optimize edin: **Ayarlar → Veritabanı → Optimize**
2. Log dosyasını kontrol edin: **Ayarlar → Hakkında → Logları Görüntüle**
3. Eski verileri arşivleyin
4. Uygulamayı temiz kurulum yapın

### Güncellemeler otomatik mi?
Evet. Varsayılan olarak uygulama başlangıçta güncelleme kontrolü yapar. **Ayarlar → Güncellemeler** seçeneğinden kapatılabilir.

### Güncelleme hatası alıyorum
**Manuel güncelleme:**
1. [GitHub Releases](https://github.com/your-repo/rehber360/releases) → Son sürümü indirin
2. Eski uygulamayı kapatın
3. Yeni installer'ı çalıştırın (veriler korunur)

### Ses kaydı çalışmıyor
**Kontroller:**
1. Mikrofon izni: **Ayarlar → Gizlilik → Mikrofon**
2. Doğru mikrofon seçili mi: **Ayarlar → Ses Kayıt**
3. AI provider API anahtarı doğru mu
4. İnternet bağlantısı stabil mi (cloud STT için)

### Uygulama başlatılırken hata veriyor
**Muhtemel Sebepler:**
- Veritabanı bozulmuş: Yedekten geri yükleyin
- Port çakışması: 3000 portu kullanımda olmayabilir
- Yetersiz izinler: Yönetici olarak çalıştırın

### Loglara nasıl ulaşırım?
**Yol:**
- Windows: `%APPDATA%/com.rehber360.app/logs/`
- macOS: `~/Library/Logs/com.rehber360.app/`
- Linux: `~/.local/share/com.rehber360.app/logs/`

VEYA **Ayarlar → Hakkında → Logları Aç**

---

## 💰 Lisans ve Fiyatlandırma

### Rehber360 açık kaynak mı?
Evet! [GitHub](https://github.com/your-repo/rehber360) üzerinde MIT lisansı altında yayınlanmıştır.

### Ticari kullanım için izin gerekli mi?
Hayır. MIT lisansı ticari kullanıma izin verir. İsterseniz kaynak kodu değiştirip ticari kullanabilirsiniz.

### Destek hizmeti satın alabilir miyim?
Şu anda resmi ücretli destek sunmuyoruz. Ancak:
- Community desteği ücretsiz (GitHub, Forum)
- Özel eğitim/danışmanlık için iletişime geçin

### AI kullanımının maliyeti var mı?
**OpenAI:** Kendi API hesabınızdan ücretlendirilir (pay-as-you-go)
**Gemini:** Ücretsiz kota geniş (aylık 1M+ token)
**Ollama:** Tamamen ücretsiz

### Mobil uygulama ne zaman çıkacak?
v2.1.0 (2025 Q2) için planlanıyor. iOS ve Android desteği gelecek.

---

## 🆘 Hala Sorunuz mu Var?

### Destek Kanalları
- 📧 **Email:** support@rehber360.com
- 💬 **Forum:** [forum.rehber360.com](https://forum.rehber360.com)
- 🐛 **Bug Raporu:** [GitHub Issues](https://github.com/your-repo/rehber360/issues)
- 📖 **Dokümantasyon:** [docs.rehber360.com](https://docs.rehber360.com)

### Topluluk
- 💡 **Öneriler:** GitHub Discussions
- 🎥 **Video Eğitimler:** YouTube kanalımız
- 📱 **Sosyal Medya:** @rehber360

---

**Son Güncelleme:** 31 Ekim 2025
**Versiyon:** 2.0.0

Daha fazla bilgi için kullanıcı kılavuzunu inceleyin: [USER_GUIDE.md](./USER_GUIDE.md)
