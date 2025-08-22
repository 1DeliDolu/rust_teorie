## 📋 cargo-pkgid(1)

### 🏷️ İSİM (NAME)

cargo-pkgid — Tam nitelikli (fully qualified) bir paket tanımlayıcısı (package specification) yazdırır

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo pkgid [options] [spec]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bir `spec` argümanı verildiğinde, mevcut çalışma alanındaki (workspace) bir paket veya bağımlılık için tam nitelikli paket kimliği (package ID specifier) yazdırılır. Eğer `spec` bağımlılık grafiğinde hangi pakete karşılık geldiği belirsizse hata üretilir. Eğer `spec` verilmezse, yerel paket için tanımlayıcı yazdırılır.

Bu komut bir `lockfile` (Cargo.lock) dosyasının mevcut olmasını ve bağımlılıkların indirilmiş olmasını gerektirir.

Bir paket tanımlayıcısı (package specifier), `name`, `version` ve `source URL`’den oluşur. Yalnızca tek bir paketi eşleştirdiği sürece kısmi tanımlayıcılar (partial specifier) kullanmanıza izin verilir. Bu tanımlayıcı Cargo’nun diğer bölümlerinde de kullanılır, örneğin `cargo-metadata(1)` ve Cargo’nun ürettiği JSON mesajlarında.

Bir `spec` aşağıdaki formatlardan birinde olabilir:

| SPEC Yapısı        | Örnek SPEC                                              |
| ------------------ | ------------------------------------------------------- |
| `name`             | `bitflags`                                              |
| `name@version`     | `bitflags@1.0.4`                                        |
| `url`              | `https://github.com/rust-lang/cargo`                    |
| `url#version`      | `https://github.com/rust-lang/cargo#0.33.0`             |
| `url#name`         | `https://github.com/rust-lang/crates.io-index#bitflags` |
| `url#name@version` | `https://github.com/rust-lang/cargo#crates-io@0.21.0`   |

Tanımlama dilbilgisi (specification grammar) için *Package ID Specifications* bölümüne bakınız.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 📦 Paket Seçimi (Package Selection)

* `-p spec`, `--package spec`
  Mevcut paket yerine verilen paket için paket kimliğini alır.

### 🖥️ Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose`
* `-q`, `--quiet`
* `--color when` (`auto`, `always`, `never`)

### 📂 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path`
* `--locked`
* `--offline`
* `--frozen`
* `--lockfile-path PATH` (yalnızca nightly + `-Z unstable-options`)

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

`foo` paketi için paket tanımlayıcısını al:

```
cargo pkgid foo
```

👉 Bu komut, `foo` paketi için paket kimliğini verir.

`foo` paketinin `1.0.0` sürümü için paket tanımlayıcısını al:

```
cargo pkgid foo@1.0.0
```

👉 Bu komut, `foo` paketinin belirli sürümüne ait paket kimliğini verir.

`foo` paketini crates.io’dan al:

```
cargo pkgid https://github.com/rust-lang/crates.io-index#foo
```

👉 Bu komut, `foo` paketini crates.io kaynağından eşler.

Yerel bir paket için `foo` tanımlayıcısını al:

```
cargo pkgid file:///path/to/local/package#foo
```

👉 Bu komut, yerel dosya yolundan `foo` paketini eşler.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-generate-lockfile(1)`, `cargo-metadata(1)`, *Package ID Specifications*, JSON messages
