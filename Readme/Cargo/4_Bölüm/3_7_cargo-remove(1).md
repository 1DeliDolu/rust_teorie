## 📋 cargo-remove(1)

### 🏷️ İSİM (NAME)

cargo-remove — Bir `Cargo.toml` manifest dosyasından bağımlılıkları kaldırır

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo remove [options] dependency…
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bir veya birden fazla bağımlılığı `Cargo.toml` manifest dosyasından kaldırır.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 📂 Bölüm Seçenekleri (Section options)

* `--dev`
  Bağımlılığı geliştirme bağımlılığı (development dependency) olarak kaldır.

* `--build`
  Bağımlılığı derleme bağımlılığı (build dependency) olarak kaldır.

* `--target target`
  Bağımlılığı verilen hedef platformdan kaldır.
  Kabuk (shell) genişlemelerini önlemek için hedefi tırnak içinde kullanabilirsiniz.
  Örn: `--target 'cfg(unix)'`

---

### 🔧 Diğer Seçenekler (Miscellaneous Options)

* `--dry-run`
  Manifest dosyasına gerçekten yazmadan kaldırma işlemini simüle eder.

---

### 🖥️ Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose`
* `-q`, `--quiet`
* `--color when` (`auto`, `always`, `never`)

---

### 📂 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path`
* `--locked`
* `--offline`
* `--frozen`
* `--lockfile-path PATH` (yalnızca nightly + `-Z unstable-options`)

---

### 📦 Paket Seçimi (Package Selection)

* `-p spec…`, `--package spec…`
  Kaldırılacak paketi belirtir.

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

`regex` bağımlılığını kaldır:

```
cargo remove regex
```

👉 Bu komut `Cargo.toml` dosyasından `regex` bağımlılığını siler.

`trybuild` bağımlılığını geliştirme bağımlılığı olarak kaldır:

```
cargo remove --dev trybuild
```

👉 Bu komut `Cargo.toml` içindeki `[dev-dependencies]` bölümünden `trybuild` bağımlılığını siler.

`nom` bağımlılığını `x86_64-pc-windows-gnu` hedefinden kaldır:

```
cargo remove --target x86_64-pc-windows-gnu nom
```

👉 Bu komut belirtilen hedef platform bağımlılık tablosundan `nom` bağımlılığını siler.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-add(1)`
