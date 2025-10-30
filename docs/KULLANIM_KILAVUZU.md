# Rehber360 - Kullanım Kılavuzu

## 📚 İçindekiler

1. [Hızlı Başlangıç](#hızlı-başlangıç)
2. [Öğrenci Yönetimi](#öğrenci-yönetimi)
3. [Sınav ve Not Yönetimi](#sınav-ve-not-yönetimi)
4. [Anket Yönetimi](#anket-yönetimi)
5. [AI Analiz Özellikleri](#ai-analiz-özellikleri)
6. [Raporlama](#raporlama)
7. [Veri İçe ve Dışa Aktarma](#veri-içe-ve-dışa-aktarma)
8. [İleri Düzey Özellikler](#ileri-düzey-özellikler)

---

## 🚀 Hızlı Başlangıç

### Ana Sayfa (Dashboard)

Uygulama açıldığında karşınıza çıkan **Ana Sayfa**, okul rehberlik süreçlerinizin özet görünümünü sunar:

#### Üst Bilgi Kartları
1. **Toplam Öğrenci Sayısı**
   - Kayıtlı toplam öğrenci
   - Önceki aya göre değişim (↑ veya ↓)

2. **Risk Altındaki Öğrenciler**
   - Dikkat gerektiren öğrenci sayısı
   - Akademik, davranışsal veya sosyal risk göstergeleri

3. **Toplam Sınav Sayısı**
   - Sisteme girilen sınav sayısı

4. **Tamamlanan Anketler**
   - Doldurulmuş anket sayısı
   - Tamamlanma oranı

#### Son Aktiviteler
- Son eklenen öğrenciler
- Son girilen sınavlar
- Son tamamlanan anketler
- Risk değişimleri

#### Hızlı Erişim Butonları
- ➕ **Yeni Öğrenci Ekle**
- 📊 **Sınav Notu Gir**
- 📋 **Anket Oluştur**
- 📈 **Rapor Oluştur**

---

## 👥 Öğrenci Yönetimi

### Öğrenci Listesi

Sol menüden **Öğrenciler** sayfasına gittiğinizde:

#### Filtreleme ve Arama
1. **Arama Çubuğu** (üst kısım)
   - Ad, soyad veya TC ile arama yapın
   - Anlık arama: Yazdıkça filtreler

2. **Filtreler** (Sağ üst köşe)
   - **Sınıf:** 9, 10, 11, 12
   - **Risk Durumu:** Düşük, Orta, Yüksek
   - **Cinsiyet:** Kız, Erkek
   - **Durum:** Aktif, Pasif, Mezun

3. **Sıralama**
   - Ad/Soyad (A-Z)
   - Sınıf (9-12)
   - Not Ortalaması (Yüksek-Düşük)
   - Risk Seviyesi

#### Öğrenci Listesi Görünümü

Her öğrenci kartında:
- **Profil Fotoğrafı** (varsa)
- **Ad Soyad**
- **Sınıf ve Numara**
- **Not Ortalaması**
- **Risk Göstergesi** (renkli çubuk)
- **Hızlı İşlem Butonları:**
  - 👁️ Görüntüle
  - ✏️ Düzenle
  - 🗑️ Sil

### Yeni Öğrenci Ekleme

#### Manuel Ekleme

1. **➕ Yeni Öğrenci** butonuna tıklayın
2. **Temel Bilgiler** sekmesini doldurun:

**Zorunlu Alanlar:**
- **Ad:** Öğrencinin adı
- **Soyad:** Öğrencinin soyadı
- **TC Kimlik No:** 11 haneli TC kimlik numarası
- **Sınıf:** 9, 10, 11 veya 12
- **Okul Numarası:** Öğrenci numarası
- **Cinsiyet:** Kız veya Erkek
- **Doğum Tarihi:** GG/AA/YYYY formatında

**Opsiyonel Alanlar:**
- **E-posta:** İletişim için
- **Telefon:** Veli telefonu
- **Adres:** Ev adresi
- **Profil Fotoğrafı:** Fotoğraf yükleyin

3. **İletişim Bilgileri** sekmesine geçin:
- **Veli Adı Soyadı**
- **Veli Telefonu**
- **Veli E-posta**
- **Acil Durum Kişisi**
- **Acil Durum Telefonu**

4. **Kaydet** butonuna tıklayın

> ✅ **İpucu:** Kırmızı yıldız (*) işaretli alanlar zorunludur.

#### Excel ile Toplu Ekleme

1. Menü çubuğundan **Dosya** → **Veri İçe Aktar** → **Öğrenci Listesi**
2. Excel dosyanızı seçin (.xlsx veya .xls)
3. **Sütun Eşleştirme** penceresinde:
   - Excel sütunlarını sistem alanlarıyla eşleştirin
   - Örnek: "Ad" sütunu → "Ad" alanı
4. **Önizleme** bölümünde ilk 5 kaydı kontrol edin
5. **İçe Aktar** butonuna tıklayın

**Excel Formatı Örneği:**
| Ad | Soyad | TC Kimlik No | Sınıf | Okul No | Cinsiyet | Doğum Tarihi |
|----|-------|--------------|-------|---------|----------|--------------|
| Ahmet | Yılmaz | 12345678901 | 9 | 1001 | Erkek | 01/01/2010 |
| Ayşe | Demir | 98765432109 | 10 | 1002 | Kız | 15/03/2009 |

> 📥 **Excel Şablonu İndirin:** Ayarlar → Yardım → Şablonları İndir

### Öğrenci Profili

Bir öğrencinin detay sayfasında:

#### Sekme Yapısı

**1. Genel Bilgiler**
- Temel demografik bilgiler
- İletişim bilgileri
- Düzenle butonu

**2. Akademik Bilgiler**
- Not ortalaması
- Başarı grafiği
- Ders bazlı performans
- Eksik notlar

**3. Sınavlar**
- Tüm sınav kayıtları
- Sınav türüne göre filtreleme
- Grafik gösterim
- Not girişi yapma

**4. Anketler**
- Tamamlanan anketler
- Anket sonuçları
- Yeni anket başlatma

**5. AI Analizi**
- Öğrenci profil analizi
- Risk değerlendirmesi
- Öneriler
- Güçlü/zayıf yönler

**6. Görüşmeler**
- Geçmiş görüşme notları
- Yeni görüşme kaydı
- Görüşme takvimi

**7. Dökümanlar**
- Yüklenen dosyalar
- Raporlar
- Belgeler

---

## 📊 Sınav ve Not Yönetimi

### Sınav Türleri

Rehber360'da desteklenen sınav türleri:

1. **MEB Sınavları**
   - Merkezi sınavlar
   - Yazılı sınavlar
   - Performans ödevleri

2. **Okul İçi Sınavlar**
   - Sözlü sınavlar
   - Kısa sınavlar (Quiz)
   - Projeler

3. **Deneme Sınavları**
   - TYT Denemeleri
   - AYT Denemeleri
   - YKS Denemeleri

### Not Girişi

#### Tekli Not Girişi

1. **Öğrenciler** → Öğrenci seç → **Sınavlar** sekmesi
2. **➕ Not Ekle** butonuna tıklayın
3. Formu doldurun:
   - **Sınav Türü:** Dropdown'dan seçin
   - **Ders:** İlgili dersi seçin
   - **Not:** 0-100 arası sayı girin
   - **Tarih:** Sınav tarihi
   - **Açıklama:** (Opsiyonel) Ek notlar

4. **Kaydet** butonuna tıklayın

#### Toplu Not Girişi

Bir sınıfın tamamı için not girmek:

1. **Sınavlar** sayfasına gidin
2. **Toplu Not Girişi** butonuna tıklayın
3. **Sınıf seçin:** 9A, 9B, vb.
4. **Sınav türü ve ders seçin**
5. Tablo formatında notları girin:

| Öğrenci | Okul No | Not |
|---------|---------|-----|
| Ahmet Yılmaz | 1001 | 85 |
| Ayşe Demir | 1002 | 92 |

6. **Toplu Kaydet** butonuna tıklayın

#### Excel ile Not İçe Aktarma

1. **Dosya** → **Veri İçe Aktar** → **Sınav Notları**
2. Excel dosyasını seçin
3. Sütunları eşleştirin:
   - Okul Numarası
   - Ders
   - Not
   - Sınav Tarihi
4. **İçe Aktar**

### Not Analizi

#### Ders Bazlı Analiz
- En yüksek/düşük notlar
- Ortalama
- Başarı yüzdesi
- Grafik gösterim

#### Öğrenci Bazlı Analiz
- Genel ortalama
- Ders bazlı performans
- Gelişim trendi
- Güçlü/zayıf dersler

---

## 📋 Anket Yönetimi

### Anket Türleri

Rehber360'da hazır anket şablonları:

1. **Çoklu Zeka Anketi**
   - 8 zeka türü değerlendirmesi
   - Otomatik skorlama
   - Grafik sonuç

2. **Kariyer İlgi Anketi**
   - Mesleki ilgi alanları
   - Holland Kodu hesaplama
   - Kariyer önerileri

3. **Kişilik Analizi**
   - MBTI benzeri test
   - Kişilik tipi belirleme
   - Özellik profili

4. **Okul İklimi Anketi**
   - Okul memnuniyeti
   - Sosyal ilişkiler
   - Güvenlik algısı

5. **Özel Anketler**
   - Kendi anketinizi oluşturun
   - Özel sorular ekleyin

### Anket Oluşturma

#### Hazır Şablon Kullanma

1. **Anketler** → **Yeni Anket** butonuna tıklayın
2. **Şablon Seç** penceresinde istediğiniz anketi seçin
3. **Öğrenci veya Sınıf Seç:**
   - Tekli: Bir öğrenci için
   - Toplu: Tüm sınıf için
4. **Anket Oluştur**

#### Özel Anket Oluşturma

1. **Anketler** → **Özel Anket Oluştur**
2. **Anket Bilgileri:**
   - **Başlık:** Anket adı
   - **Açıklama:** Anketin amacı
   - **Tür:** Gizli/Açık
3. **Soru Ekleme:**
   - **➕ Soru Ekle** butonuna tıklayın
   - **Soru metni** yazın
   - **Soru tipi** seçin:
     - Çoktan seçmeli (Tek)
     - Çoktan seçmeli (Çoklu)
     - Likert ölçeği (1-5)
     - Açık uçlu
     - Evet/Hayır
   - Seçenekleri ekleyin (gerekiyorsa)
4. **Kaydet ve Yayınla**

### Anket Uygulama

#### Manuel Anket Doldurma

1. Öğrenci profilinde **Anketler** sekmesi
2. Mevcut anketi seçin veya yeni anket başlatın
3. Soruları sırayla cevaplayın
4. **Anketi Tamamla** butonuna tıklayın

#### Öğrencinin Kendi Doldurması (QR Kod)

1. Anket detay sayfasında **QR Kod Oluştur**
2. QR kodu yazdırın veya ekranda gösterin
3. Öğrenci kendi cihazından okutup anketi doldurur
4. Sonuçlar otomatik kaydedilir

### Anket Sonuçları

#### Bireysel Sonuçlar

Öğrenci profilinde:
- Anket puanı
- Detaylı cevaplar
- Grafik gösterim
- Yorum ve öneriler

#### Toplu Sonuçlar

**Anketler** sayfasında:
- Katılım oranı
- Ortalama skorlar
- Sınıf karşılaştırması
- İstatistiksel analiz
- Grafik ve tablolar

---

## 🤖 AI Analiz Özellikleri

### AI Provider Ayarları

AI özelliklerini kullanmak için önce ayarları yapın:

1. **⚙️ Ayarlar** → **AI Yapılandırması**
2. **AI Provider Seçin:**
   - **OpenAI (ChatGPT):** En güçlü, ücretli
   - **Google Gemini:** Ücretsiz kota mevcut
   - **Ollama:** Lokal, internet gerektirmez

3. **API Anahtarı Girin**
   - OpenAI için: https://platform.openai.com/api-keys
   - Gemini için: https://makersuite.google.com/app/apikey

4. **Model Seçin:**
   - GPT-4 (en iyi, pahalı)
   - GPT-3.5-Turbo (iyi, ekonomik)
   - Gemini Pro (ücretsiz)

5. **Test Et** butonuyla bağlantıyı kontrol edin

### Öğrenci Profil Analizi

#### Kapsamlı Profil Analizi

1. Öğrenci profilinde **AI Analizi** sekmesine gidin
2. **Analiz Başlat** butonuna tıklayın
3. AI şu verileri analiz eder:
   - Akademik performans
   - Sınav notları ve trendleri
   - Anket sonuçları
   - Devamsızlık kayıtları
   - Davranış notları

**Analiz Sonucu:**
- **Genel Değerlendirme:** Öğrenci hakkında özet
- **Güçlü Yönler:** Öne çıkan özellikler
- **Gelişim Alanları:** İyileştirme gereken noktalar
- **Risk Değerlendirmesi:** Potansiyel sorunlar
- **Öneriler:** Somut aksiyon planları
- **Kariyer Önerileri:** Uygun meslekler

#### Erken Uyarı Sistemi

AI sürekli olarak öğrencileri izler ve risk tespit eder:

**Risk Göstergeleri:**
1. **Akademik Risk:**
   - Not düşüşü trendi
   - Belirli derslerde başarısızlık
   - Sınav kaygısı

2. **Sosyal-Duygusal Risk:**
   - Anket yanıtlarında değişim
   - İzolasyon işaretleri
   - Motivasyon kaybı

3. **Davranışsal Risk:**
   - Devamsızlık artışı
   - Disiplin kayıtları
   - Öğretmen geri bildirimleri

**Bildirimler:**
- Yüksek risk tespit edildiğinde bildirim gelir
- Risk seviyesi: Düşük (🟢), Orta (🟡), Yüksek (🔴)
- Önerilen müdahaleler listelenir

### AI Asistan

Sohbet tabanlı AI yardımcısı:

1. Sağ alt köşedeki **💬 AI Asistan** butonuna tıklayın
2. Sorularınızı yazın, örneğin:
   - "9A sınıfının genel durumu nedir?"
   - "Risk altındaki öğrencileri listele"
   - "Matematik notları düşen öğrencileri göster"
   - "Ahmet Yılmaz için öneriler neler?"

3. AI anında analiz yapıp cevap verir

**Kullanım Örnekleri:**
- "Bu hafta kimlerle görüşmem gerekiyor?"
- "Sınav ortalaması en düşük 5 öğrenci kimler?"
- "Kariyer anketi tamamlamayanları listele"
- "Matematik dersinde başarılı ama Türkçe'de zayıf öğrenciler?"

### Rapor Önerileri

AI, raporlarınızı zenginleştirir:

1. **Raporlar** sayfasında rapor oluştururken
2. **AI Önerileri Ekle** seçeneğini işaretleyin
3. AI rapora şunları ekler:
   - Yorumlar ve değerlendirmeler
   - Öğrenci potansiyeli analizi
   - Gelişim önerileri
   - Veli görüşme konuları

---

## 📈 Raporlama

### Rapor Türleri

#### 1. Bireysel Öğrenci Raporu

**İçerik:**
- Genel bilgiler
- Akademik performans
- Anket sonuçları
- AI analizi
- Öneriler

**Oluşturma:**
1. Öğrenci profilinde **Rapor Oluştur** butonuna tıklayın
2. Rapor türü seçin:
   - Dönem Sonu Raporu
   - Veli Görüşme Raporu
   - Kariyer Rehberlik Raporu
3. **Oluştur** → PDF veya Excel olarak kaydedin

#### 2. Sınıf Raporu

**İçerik:**
- Sınıf istatistikleri
- Başarı dağılımı
- Risk analizi
- Öne çıkan öğrenciler

**Oluşturma:**
1. **Raporlar** → **Sınıf Raporu**
2. Sınıf seçin (9A, 10B, vb.)
3. Dönem seçin
4. **Rapor Oluştur**

#### 3. Okul Geneli Rapor

**İçerik:**
- Genel istatistikler
- Sınıf karşılaştırması
- Başarı trendleri
- Risk haritası

#### 4. Özel Raporlar

İhtiyacınıza göre filtreli raporlar:
- Belirli not aralığındaki öğrenciler
- Anketi tamamlayanlar/tamamlamayanlar
- Risk seviyesine göre
- Mezuniyet senesi göre

### Rapor Formatları

#### PDF Rapor
- Yazdırılabilir format
- Resmi dokümanlarda kullanılır
- Veli görüşmelerinde paylaşılır

**Export:**
1. Rapor oluşturduktan sonra **PDF Olarak Kaydet**
2. Konum ve dosya adı belirleyin
3. Kaydet

#### Excel Rapor
- Veri analizi için uygun
- Pivot tablolar oluşturulabilir
- Başka sistemlere aktarılabilir

**Export:**
1. Rapor sayfasında **Excel'e Aktar**
2. Kaydetme konumu seçin
3. Excel ile açın ve düzenleyin

#### Word Rapor
- Düzenlenebilir format
- Kişiselleştirme yapılabilir
- Şablon olarak kullanılabilir

---

## 📥 Veri İçe ve Dışa Aktarma

### Excel İçe Aktarma

#### Öğrenci Listesi

1. **Dosya** → **Veri İçe Aktar** → **Öğrenci Listesi**
2. Excel dosyanızı seçin
3. Sütun eşleştirme:
   - Her Excel sütununu doğru alana eşleştirin
4. **Önizleme** ile kontrol edin
5. **İçe Aktar**

**Excel Formatı:**
- İlk satır başlıklar olmalı
- Tarih formatı: GG/AA/YYYY
- TC Kimlik 11 haneli olmalı

#### Sınav Notları

1. **Dosya** → **Veri İçe Aktar** → **Sınav Notları**
2. Excel'de şu sütunlar olmalı:
   - Okul Numarası
   - Ders
   - Sınav Türü
   - Not
   - Tarih
3. İçe aktar

#### Anket Yanıtları

MEBBİS veya diğer sistemlerden anket sonuçları:
1. **Dosya** → **Veri İçe Aktar** → **Anket Yanıtları**
2. Format eşleştirmesi yapın
3. İçe aktar

### Excel Dışa Aktarma

#### Tüm Öğrenci Listesi

1. **Öğrenciler** sayfasında **Excel'e Aktar**
2. Sütun seçimi:
   - Hangi bilgilerin yer alacağını seçin
3. **Export** → Kaydet

#### Filtrelenmiş Liste

1. Önce filtre uygulayın (sınıf, risk, vb.)
2. **Excel'e Aktar**
3. Sadece filtrelenmiş öğrenciler export edilir

#### Notlar ve Anketler

1. İlgili sayfada **Excel'e Aktar** butonuna tıklayın
2. Kaydetme konumunu seçin

### Veritabanı Yedekleme

#### Manuel Yedek

1. **Veritabanı** → **Yedek Al**
2. Konum seçin (USB bellekte tavsiye edilir)
3. Dosya adı: `rehber360-yedek-2025-10-30.db`
4. **Kaydet**

> 🔒 **Önemli:** Düzenli yedek alın! Haftalık yedek önerilir.

#### Otomatik Yedek

1. **Ayarlar** → **Veritabanı** → **Otomatik Yedekleme**
2. Etkinleştir
3. Sıklık: Günlük/Haftalık/Aylık
4. Konum belirleyin
5. Kaydet

Artık belirtilen sıklıkta otomatik yedek alınacak.

#### Yedek Geri Yükleme

⚠️ **Dikkat:** Mevcut veriler silinecek!

1. Mevcut veritabanını yedekleyin (güvenlik için)
2. **Veritabanı** → **Yedek Geri Yükle**
3. Yedek dosyasını seçin (.db)
4. **Onaylıyorum** seçeneğini işaretleyin
5. **Geri Yükle**
6. Uygulama yeniden başlayacak

---

## 🔧 İleri Düzey Özellikler

### Görüşme Notları

#### Yeni Görüşme Kaydı

1. Öğrenci profilinde **Görüşmeler** sekmesi
2. **➕ Yeni Görüşme** butonuna tıklayın
3. Formu doldurun:
   - **Tarih ve Saat**
   - **Görüşme Türü:** Birey/Veli/Grup
   - **Konu:** Dropdown'dan seçin
   - **Notlar:** Detaylı açıklama
   - **Sonuç:** Olumlu/Olumsuz/Nötr
   - **Takip Gerekli:** Evet/Hayır
4. **Kaydet**

#### Görüşme Takvimi

1. **Raporlar** → **Görüşme Takvimi**
2. Tüm geçmiş ve gelecek görüşmeler takvim görünümünde
3. Filtreleme:
   - Tarih aralığı
   - Görüşme türü
   - Öğrenci/Sınıf

### Dosya Yönetimi

#### Dosya Yükleme

Öğrenci profiline belge ekleme:

1. Öğrenci profilinde **Dökümanlar** sekmesi
2. **📁 Dosya Yükle** butonuna tıklayın
3. Dosya seçin (PDF, Word, resim, Excel)
4. Kategori seçin:
   - Rapor
   - Belge
   - Sertifika
   - Diğer
5. **Yükle**

#### Dosya İndirme

1. **Dökümanlar** sekmesinde dosyayı bulun
2. **İndir** simgesine tıklayın
3. Dosya bilgisayarınıza indirilir

### Bildirim Ayarları

#### Bildirim Türleri

1. **Ayarlar** → **Bildirimler**
2. İstediğiniz bildirimleri aktif edin:

**Sistem Bildirimleri:**
- ☑ Uygulama güncellemeleri
- ☑ Veritabanı yedekleme hatırlatıcıları

**Öğrenci Bildirimleri:**
- ☑ Yeni risk öğrenci tespiti
- ☑ Not düşüşü uyarıları
- ☑ Eksik veri bildirimleri

**Anket Bildirimleri:**
- ☐ Anket tamamlama hatırlatıcıları
- ☐ Yeni anket yanıtları

**Günlük Raporlar:**
- ☐ Günlük özet raporu (saat seçimi yapılabilir)

#### Bildirim Görünümü

Windows native bildirimleri kullanılır:
- Sağ alt köşede görünür
- Tıklanabilir (ilgili sayfaya yönlendirir)
- Sesli uyarı (ayarlanabilir)

### Tema ve Görünüm

#### Tema Değişimi

1. **Ayarlar** → **Görünüm**
2. Tema seçin:
   - 🌞 **Açık Tema:** Beyaz arkaplan, günlük kullanım için
   - 🌙 **Koyu Tema:** Siyah arkaplan, göz yorgunluğunu azaltır

#### Zoom/Büyütme

Yazıları büyütmek/küçültmek için:
- **Ctrl +** → Yakınlaştır
- **Ctrl -** → Uzaklaştır
- **Ctrl 0** → Varsayılana döndür

Veya:
1. **Görünüm** → **Zoom**
2. Seviye seçin: %75, %100, %125, %150

#### Font Boyutu

1. **Ayarlar** → **Erişilebilirlik**
2. Font boyutu: Küçük/Normal/Büyük/Çok Büyük

### Klavye Kısayolları

#### Genel
- **Ctrl+N** → Yeni öğrenci
- **Ctrl+S** → Kaydet
- **Ctrl+F** → Ara
- **Ctrl+P** → Yazdır
- **Ctrl+Q** → Çıkış
- **F5** → Yenile
- **F11** → Tam ekran

#### Navigasyon
- **Alt+1** → Ana sayfa
- **Alt+2** → Öğrenciler
- **Alt+3** → Sınavlar
- **Alt+4** → Anketler
- **Alt+5** → Raporlar

#### Düzenleme
- **Ctrl+Z** → Geri al
- **Ctrl+Y** → İleri al
- **Ctrl+C** → Kopyala
- **Ctrl+V** → Yapıştır

### Performans Ayarları

Eski veya yavaş bilgisayarlarda:

1. **Ayarlar** → **Performans**
2. **Düşük Kaynak Modu** aktif edin
3. Şu optimizasyonlar uygulanır:
   - Animasyonlar devre dışı
   - Grafik efektleri azaltılır
   - Önbellek kullanımı optimize edilir

### Veri Güvenliği

#### Veritabanı Şifreleme

1. **Ayarlar** → **Güvenlik**
2. **Veritabanı Şifreleme** aktif edin
3. Güçlü bir şifre belirleyin
4. **Uygula**

Artık her uygulama açılışında şifre sorulur.

#### Oturum Zaman Aşımı

1. **Ayarlar** → **Güvenlik**
2. **Otomatik Kilitleme** süresini ayarlayın:
   - 5 dakika
   - 15 dakika
   - 30 dakika
   - 1 saat
   - Hiçbir zaman

---

## ❓ Sık Kullanılan İşlemler

### Hızlı Görevler

#### Öğrenci Not Durumunu Kontrol Etme

1. **Öğrenciler** → Öğrenci seç
2. **Sınavlar** sekmesine git
3. Grafik ve liste görünümünde tüm notlar

#### Risk Öğrencileri Bulma

1. **Öğrenciler** sayfasında **Filtre** aç
2. **Risk Durumu:** Yüksek seç
3. Liste filtrelenir

#### Toplu Rapor Oluşturma

1. **Raporlar** → **Toplu Rapor**
2. Sınıf seçin
3. **Tüm Öğrenciler için Rapor Oluştur**
4. PDF'ler klasöre kaydedilir

#### Excel'den Hızlı Veri İçe Aktarma

1. **Dosya** → **Hızlı İçe Aktar**
2. Excel'i sürükle-bırak
3. Otomatik eşleştirme
4. **Tamam**

---

## 🆘 Sorun Giderme

### Sık Karşılaşılan Sorunlar

#### Uygulama Yavaş Çalışıyor

**Çözüm 1:** Düşük Kaynak Modu
1. Ayarlar → Performans
2. Düşük Kaynak Modu aktif et

**Çözüm 2:** Cache Temizle
1. Ayarlar → Gelişmiş
2. Önbelleği Temizle

#### AI Analizi Çalışmıyor

**Kontroller:**
1. Internet bağlantınızı kontrol edin
2. Ayarlar → AI → API anahtarını kontrol edin
3. **Test Bağlantısı** butonuna tıklayın
4. API kotanızı kontrol edin (OpenAI/Gemini)

#### Excel İçe Aktarma Hatası

**Kontroller:**
1. Excel formatı doğru mu?
2. Tarih formatı: GG/AA/YYYY
3. TC Kimlik 11 haneli mi?
4. Boş satırlar var mı? (Kaldırın)

#### Veritabanı Hatası

1. Uygulamayı tamamen kapatın (sistem tepsisinden çık)
2. Veritabanını yedekleyin
3. Uygulamayı yeniden başlatın
4. Sorun devam ederse yedekten geri yükleyin

---

## 📞 Yardım ve Destek

### Yardım Kaynakları

1. **Uygulama İçi Yardım**
   - **F1** tuşuna basın
   - Bulunduğunuz sayfaya özel yardım gösterilir

2. **Video Eğitimler**
   - Yardım → Video Eğitimler
   - YouTube kanalımız

3. **Dokümantasyon**
   - https://rehber360.com/dokumantasyon

4. **Sık Sorulan Sorular**
   - https://rehber360.com/sss

### İletişim

**Email:** support@rehber360.com  
**Telefon:** +90 (212) XXX XX XX (09:00-18:00)  
**Canlı Destek:** Web sitesinde sağ alt köşe  
**Forum:** https://rehber360.com/forum

### Hata Bildirimi

Hata bulduğunuzda:

1. **Ekran görüntüsü** alın (Print Screen tuşu)
2. **Hata mesajını** kopyalayın
3. **Log dosyasını** bulun:
   - Windows: `%APPDATA%\Rehber360\logs\main.log`
4. **Email gönderin:** support@rehber360.com
   - Konu: "Hata Raporu - [Kısa açıklama]"
   - İçerik:
     - Ne yaptığınızda hata oluştu
     - Hata mesajı
     - Ekran görüntüsü
     - Log dosyası (ek olarak)

---

## 📚 Ek Kaynaklar

### Şablonlar

İndirebileceğiniz şablonlar:

1. **Excel Şablonları**
   - Öğrenci listesi şablonu
   - Not girişi şablonu
   - Anket yanıt şablonu

2. **PDF Şablonları**
   - Veli görüşme formu
   - Öğrenci izleme formu
   - Dönem sonu rapor şablonu

**İndirme:** Ayarlar → Yardım → Şablonları İndir

### Eğitim Materyalleri

- 📹 **Video Eğitimler:** 15 dakikalık modüller
- 📖 **PDF Kılavuzlar:** Yazdırılabilir rehberler
- 💡 **İpuçları:** Günlük kullanım ipuçları
- 📊 **Örnek Veriler:** Test amaçlı örnek dataset

---

## ✅ Başarı İpuçları

### Verimli Kullanım için Öneriler

1. **Düzenli Veri Girişi**
   - Notları hemen girin, biriktirmeyin
   - Haftalık veri kontrolü yapın

2. **Yedekleme Alışkanlığı**
   - Otomatik yedekleme açın
   - Aylık manuel yedek alın (USB'ye)

3. **AI'dan Yararlanın**
   - Öğrenci analizlerini düzenli inceleyin
   - Risk uyarılarını ciddiye alın

4. **Raporları Zamanında Hazırlayın**
   - Dönem sonu raporlarını erken hazırlayın
   - Veli görüşmelerinde raporları kullanın

5. **Anketleri Aktif Kullanın**
   - Dönem başı/sonu anketleri uygulayın
   - Sonuçları değerlendirin

6. **Kısa Yolları Öğrenin**
   - Klavye kısayollarını kullanın
   - Daha hızlı çalışın

---

**Rehber360 Kullanım Kılavuzu**  
**Versiyon:** 1.0  
**Tarih:** 30 Ekim 2025  
**Hazırlayan:** Rehber360 Geliştirme Ekibi

**İyi Kullanımlar! 🎯**
