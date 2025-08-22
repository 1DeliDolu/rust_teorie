## 📋 cargo-clean(1)

### 🏷️ İSİM (NAME)

cargo-clean — Üretilen çıktıları (artifacts) kaldırır

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo clean [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Cargo’nun geçmişte oluşturduğu `target` dizinindeki çıktı dosyalarını kaldırır.

Hiçbir seçenek verilmezse, `cargo clean` tüm `target` dizinini siler.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 📦 Paket Seçimi (Package Selection)

* `-p spec`, `--package spec` → Yalnızca belirtilen paketleri temizler. Birden fazla kez belirtilebilir.

Varsayılan olarak paket seçilmezse, çalışma alanındaki tüm paketler ve bağımlılıkları temizlenir.

---

### 🧹 Temizleme Seçenekleri (Clean Options)

* `--dry-run` → Dosyaları silmeden, hangi dosyaların silineceğini özetler.
* `--doc` → Yalnızca `target` içindeki `doc` dizinini siler.
* `--release` → `release` dizinindeki tüm çıktıları siler.
* `--profile name` → Belirtilen profil dizinindeki çıktıları siler.
* `--target-dir directory` → Üretilen çıktılar için kullanılacak dizin.
* `--target triple` → Belirtilen mimari için temizleme yapar.

---

### 👀 Görüntüleme Seçenekleri (Display Options)

* `-v, --verbose` → Ayrıntılı çıktı.
* `-q, --quiet` → Sessiz mod.
* `--color when` → Renkli çıktı ayarı (`auto`, `always`, `never`).

---

### 📄 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path` → Kullanılacak `Cargo.toml` dosyası.
* `--locked` → `Cargo.lock` dosyasını değiştirmeden kullan.
* `--offline` → Ağ erişimini kapat.
* `--frozen` → Hem `--locked` hem `--offline`.
* `--lockfile-path PATH` → Kilit dosyası yolunu değiştir (yalnızca `nightly`).

---

### ⚡ Genel Seçenekler (Common Options)

* `+toolchain` → Belirli bir Rust sürüm zinciri (örn. `+stable`, `+nightly`).
* `--config KEY=VALUE` → Konfigürasyon değerini geçersiz kıl.
* `-C PATH` → Çalışma dizinini değiştir.
* `-h, --help` → Yardım bilgisini yazdır.
* `-Z` → Kararsız (nightly) bayrakları.

---

## 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🚪 ÇIKIŞ DURUMU (EXIT STATUS)

* `0` → Başarılı.
* `101` → Başarısız.

---

## 📚 ÖRNEKLER (EXAMPLES)

Tüm `target` dizinini sil:

```
cargo clean
```

Yalnızca `release` çıktıları sil:

```
cargo clean --release
```

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-build(1)`
