## 📦 cargo-generate-lockfile(1)

### 🏷️ İSİM (NAME)

cargo-generate-lockfile — Bir paket için kilit dosyası (`lockfile`) oluşturma

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo generate-lockfile [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut mevcut paket veya çalışma alanı (workspace) için `Cargo.lock` dosyasını oluşturur.
Eğer `Cargo.lock` dosyası zaten varsa, her paketin en son kullanılabilir sürümüyle yeniden oluşturulur.

Ayrıca bkz: `cargo-update(1)` — o da `Cargo.lock` oluşturabilir ve güncelleme davranışını kontrol etmek için daha fazla seçeneğe sahiptir.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🖥️ Görüntüleme Seçenekleri (Display Options)

* `-v, --verbose` → Ayrıntılı çıktı. İki kez belirtilirse çok ayrıntılı (bağımlılık uyarıları ve build script çıktısı dahil).
* `-q, --quiet` → Cargo günlük mesajlarını bastırır.
* `--color when` → Renkli çıktıyı kontrol et (`auto`, `always`, `never`).

### 📂 Bildirim Seçenekleri (Manifest Options)

* `--manifest-path path` → `Cargo.toml` dosya yolu. Varsayılan olarak geçerli veya üst dizinlerde aranır.
* `--ignore-rust-version` → Paketlerdeki `rust-version` bilgisini yok say.
* `--locked` → `Cargo.lock` dosyasıyla aynı bağımlılıkların kullanılmasını zorunlu kılar.
* `--offline` → Ağ erişimini engeller.
* `--frozen` → Hem `--locked` hem `--offline` ile eşdeğer.
* `--lockfile-path PATH` → Kilit dosyası yolunu değiştirir (yalnızca nightly).

### 🌍 Ortak Seçenekler (Common Options)

* `+toolchain` → Belirtilen Rustup araç zinciriyle çalıştır.
* `--config KEY=VALUE or PATH` → Cargo yapılandırmasını geçersiz kıl.
* `-C PATH` → Çalışma dizinini değiştirme. *(yalnızca nightly)*
* `-h, --help` → Yardım bilgisini yazdır.
* `-Z flag` → Deneysel (nightly) bayraklar.

---

## 🌱 ORTAM DEĞİŞKENLERİ (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🚦 ÇIKIŞ DURUMU (EXIT STATUS)

* `0` → Cargo başarıyla tamamlandı.
* `101` → Cargo hata ile tamamlandı.

---

## 📚 ÖRNEKLER (EXAMPLES)

Mevcut paket veya çalışma alanı için kilit dosyasını oluştur veya güncelle:

```
cargo generate-lockfile
```

👉 Bu komut, `Cargo.lock` dosyasını oluşturur ya da mevcutsa en güncel bağımlılıklarla yeniden yazar.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-update(1)`
