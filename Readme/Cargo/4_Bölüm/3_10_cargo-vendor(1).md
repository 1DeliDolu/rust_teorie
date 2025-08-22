## 📋 cargo-vendor(1)

### 🏷️ İSİM (NAME)

cargo-vendor — Tüm bağımlılıkları yerel olarak kopyalar (vendor)

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo vendor [options] [path]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu Cargo alt komutu, bir proje için tüm `crates.io` ve `git` bağımlılıklarını `<path>` ile belirtilen dizine kopyalar (vendor işlemi). İşlem tamamlandıktan sonra belirtilen `vendor` dizini, bağımlılıkların tüm uzak kaynaklarını içerir. Varsayılan manifest dışında ek manifestler `-s` seçeneği ile belirtilebilir.

Gerekli yapılandırma (`.cargo/config.toml`) dosyası için ayarlar, işlem tamamlandıktan sonra `stdout` üzerine yazdırılır. Bunları yerel konfigürasyon dosyanıza eklemeniz veya yönlendirmeniz gerekir.

Cargo, kopyalanan (vendored) kaynakları salt-okunur olarak ele alır. Eğer uzak bir kaynaktan alınan crate üzerinde değişiklik yapmak istiyorsanız, `[patch]` veya yerel kopyaya işaret eden `path dependency` kullanmalısınız.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 📦 Vendor Seçenekleri (Vendor Options)

* `-s manifest`, `--sync manifest`
  Ek bir `Cargo.toml` manifest dosyası belirler. Birden fazla kez kullanılabilir.

* `--no-delete`
  Vendor dizinini silmeden mevcut içerik üzerine kopyalama yapar.

* `--respect-source-config`
  Varsayılan olarak yok sayılan `[source]` yapılandırmasını `.cargo/config.toml` içinden dikkate alır.

* `--versioned-dirs`
  Normalde sadece birden fazla sürüm varsa sürüm eklenir. Bu seçenek ile her paketin dizin adı sürümlü olur.

---

### 📂 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path`
* `--locked`
* `--offline`
* `--frozen`
* `--lockfile-path PATH` (yalnızca nightly + `-Z unstable-options`)

---

### 🖥️ Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose`
* `-q`, `--quiet`
* `--color when` (`auto`, `always`, `never`)

---

### ⚙️ Ortak Seçenekler (Common Options)

* `+toolchain`
* `--config KEY=VALUE or PATH`
* `-C PATH` (yalnızca nightly + `-Z unstable-options`)
* `-h`, `--help`
* `-Z flag`

---

## 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🚦 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarıyla tamamlandı.
* `101`: Cargo hata ile tamamlandı.

---

## 📚 ÖRNEKLER (EXAMPLES)

Tüm bağımlılıkları yerel bir `vendor` dizinine kopyala:

```
cargo vendor
```

👉 Bu komut bağımlılıkları `vendor/` klasörüne indirir.

Tüm bağımlılıkları `third-party/vendor` dizinine kopyala:

```
cargo vendor third-party/vendor
```

👉 Bu komut bağımlılıkları belirtilen özel klasöre indirir.

Mevcut çalışma alanı ile birlikte başka bir manifest’i de vendor dizinine kopyala:

```
cargo vendor -s ../path/to/Cargo.toml
```

👉 Bu komut birden fazla `Cargo.toml` dosyasını vendor içine dahil eder.

Vendor yapılandırmasını otomatik olarak konfigürasyon dosyasına yönlendir:

```
cargo vendor > path/to/my/cargo/config.toml
```

👉 Bu komut gerekli ayarları doğrudan Cargo konfigürasyon dosyasına yazar.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`
