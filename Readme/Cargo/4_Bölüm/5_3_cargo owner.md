## 📋 cargo-owner(1)

### 🏷️ İSİM (NAME)

cargo-owner — Bir kayıt defterinde (registry) bir crate’in sahiplerini yönet

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo owner [options] --add login [crate]
cargo owner [options] --remove login [crate]
cargo owner [options] --list [crate]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, kayıt defterinde bir `crate`’in sahiplerini değiştirir.
Bir `crate`’in sahipleri yeni sürümler yükleyebilir ve eski sürümleri `yank` edebilir. Takım dışı sahipler ayrıca sahip listesini de değiştirebilir, bu yüzden dikkatli olunmalıdır.

Bu komut, kimlik doğrulaması için `--token` seçeneği veya `cargo-login(1)` aracılığıyla oturum açmış olmayı gerektirir.

Eğer `crate` adı belirtilmezse, mevcut dizindeki paket adı kullanılır.

Sahipler ve yayınlama hakkında daha fazla bilgi için referansa bakınız.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 👤 Sahiplik Seçenekleri (Owner Options)

* `-a`, `--add login…`
  Belirtilen kullanıcıyı veya takımı sahip olarak davet eder.

* `-r`, `--remove login…`
  Belirtilen kullanıcıyı veya takımı sahiplikten kaldırır.

* `-l`, `--list`
  Bir crate’in sahiplerini listeler.

* `--token token`
  Kimlik doğrulamada kullanılacak API anahtarı. Bu, `cargo-login(1)` tarafından oluşturulan kimlik dosyasındaki anahtarı geçersiz kılar.

  Ayrıca, Cargo yapılandırma ortam değişkenleri ile de bu değer geçersiz kılınabilir:

  * `crates.io` için: `CARGO_REGISTRY_TOKEN`
  * Diğer kayıt defterleri için: `CARGO_REGISTRIES_NAME_TOKEN` (NAME büyük harflerle).

* `--index index`
  Kullanılacak kayıt defteri `index` URL’si.

* `--registry registry`
  Kullanılacak kayıt defterinin adı. Belirtilmezse varsayılan kayıt defteri (`crates-io`) kullanılır.

---

### 👀 Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose` → Ayrıntılı çıktı verir. İki kez belirtilirse çok ayrıntılı olur.
* `-q`, `--quiet` → Cargo günlük mesajlarını bastırır.
* `--color when` → Renkli çıktının ne zaman kullanılacağını kontrol eder:

  * `auto` (varsayılan)
  * `always`
  * `never`

---

### 🔨 Ortak Seçenekler (Common Options)

* `+toolchain` → Eğer Cargo `rustup` ile kurulmuşsa, `+stable` veya `+nightly` gibi bir araç zinciri seçilebilir.
* `--config KEY=VALUE or PATH` → Bir Cargo yapılandırma değerini geçersiz kılar.
* `-C PATH` → Çalışma dizinini değiştirir (sadece `nightly` sürümünde, `-Z unstable-options` ile).
* `-h`, `--help` → Yardım bilgisini gösterir.
* `-Z flag` → `nightly` için kararsız bayraklar.

---

## 🌍 ORTAM DEĞİŞKENLERİ (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🔚 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarıyla tamamlandı.
* `101`: Cargo tamamlanamadı.

---

## 📚 ÖRNEKLER (EXAMPLES)

Bir paketin sahiplerini listele:

```
cargo owner --list foo
```

👉 Bu komut, `foo` adlı crate’in sahiplerini listeler.

Bir pakete sahip ekle:

```
cargo owner --add username foo
```

👉 Bu komut, `foo` adlı crate’e `username` kullanıcısını sahip olarak ekler.

Bir paketten sahip kaldır:

```
cargo owner --remove username foo
```

👉 Bu komut, `foo` adlı crate’ten `username` kullanıcısını sahiplikten kaldırır.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-login(1)`, `cargo-publish(1)`
