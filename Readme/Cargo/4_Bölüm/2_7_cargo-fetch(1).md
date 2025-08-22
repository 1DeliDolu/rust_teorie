## 📋 cargo-fetch(1)

### 🏷️ İSİM (NAME)

cargo-fetch — Bir paketin bağımlılıklarını ağ üzerinden indirir

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo fetch [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Eğer bir `Cargo.lock` dosyası mevcutsa, bu komut tüm git ve/veya kayıt (registry) bağımlılıklarının indirilip yerel olarak erişilebilir olmasını sağlar. Böylece `cargo fetch` sonrası, kilit dosyası değişmediği sürece diğer Cargo komutları çevrimdışı çalıştırılabilir.

Eğer `Cargo.lock` dosyası yoksa, bu komut bağımlılıkları indirmeden önce bir kilit dosyası oluşturur.

`--target` belirtilmezse, tüm hedef bağımlılıkları indirilir.

Ayrıca popüler paketleri önceden indirmek için `cargo-prefetch` eklentisine bakabilirsiniz. Bu, Cargo’yu `--offline` bayrağıyla ağ olmadan kullanmayı planlıyorsanız faydalı olabilir.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🔽 İndirme Seçenekleri (Fetch Options)

* `--target triple` → Belirtilen mimari için bağımlılıkları indir. Varsayılan tüm mimarilerdir.

---

### 👀 Görüntüleme Seçenekleri (Display Options)

* `-v, --verbose` → Ayrıntılı çıktı.
* `-q, --quiet` → Sessiz mod.
* `--color when` → Renkli çıktı ayarı (`auto`, `always`, `never`).

---

### 📄 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path` → Kullanılacak `Cargo.toml` dosyası.
* `--locked` → `Cargo.lock` dosyasındaki bağımlılık sürümlerini değiştirmeden kullan.
* `--offline` → Ağ erişimini engeller.
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

Tüm bağımlılıkları indir:

```
cargo fetch
```

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-update(1)`, `cargo-generate-lockfile(1)`
