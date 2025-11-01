# ⚡ GitHub Actions - Hızlı Başlangıç

## 🎯 3 Adımda Release Yap

### 1️⃣ Tag Oluştur
```bash
git tag v2.0.1
git push origin v2.0.1
```

### 2️⃣ Bekle
- GitHub Actions otomatik başlar
- ~20-30 dakika sürer
- Actions sekmesinden izleyebilirsiniz

### 3️⃣ İndir
- **Releases** sekmesine gidin
- `.exe` (Windows), `.dmg` (macOS), `.AppImage` (Linux) indirin

---

## 🔥 Sadece Windows Build

```bash
# GitHub'da: Actions → Windows Build (Fast) → Run workflow
```

veya kod değiştirdikten sonra `main` branch'e push edin:
```bash
git push origin main
```

Artifacts 7 gün boyunca Actions sekmesinde bulunur.

---

## 📚 Detaylı Bilgi

Tüm detaylar için: [RELEASE_GUIDE.md](./RELEASE_GUIDE.md)

---

## 🆘 Hızlı Sorun Giderme

| Sorun | Çözüm |
|-------|-------|
| Build failed | Actions'da hata loglarını inceleyin |
| Permission denied | Settings → Actions → Read/write permissions |
| Tag çalışmıyor | `v` ile başladığından emin olun (örn: `v2.0.1`) |
| Artifact yok | 7 günden eski mi kontrol edin |

---

**💡 İpucu**: İlk build uzun sürer (~30 dk), sonrakiler cache sayesinde daha hızlı (~15 dk).
