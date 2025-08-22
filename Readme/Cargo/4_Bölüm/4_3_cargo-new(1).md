## 📋 cargo-new(1)

### 🏷️ İSİM (NAME)

cargo-new — Yeni bir Cargo paketi oluştur

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo new [options] path
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, belirtilen dizinde yeni bir Cargo paketi oluşturur. Bu paket, bir `Cargo.toml` manifest dosyası, örnek kaynak dosyası ve bir VCS `ignore` dosyası içeren basit bir şablonla gelir. Eğer dizin zaten bir VCS deposu değilse, yeni bir depo oluşturulur (aşağıdaki `--vcs` seçeneğine bakınız).

Benzer bir komut için bkz. `cargo-init(1)`; bu komut, mevcut bir dizine yalnızca yeni bir manifest ekler.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🆕 Yeni Paket Seçenekleri (New Options)

* `--bin`
  İkili (binary) hedefli bir paket oluşturur (`src/main.rs`). Varsayılan davranıştır.

* `--lib`
  Kütüphane (library) hedefli bir paket oluşturur (`src/lib.rs`).

* `--edition edition`
  Kullanılacak Rust sürümünü belirtir. Varsayılan `2024`. Geçerli değerler: `2015`, `2018`, `2021`, `2024`.

* `--name name`
  Paket adını belirler. Varsayılan olarak dizin adı kullanılır.

* `--vcs vcs`
  Belirtilen sürüm kontrol sistemi (`git`, `hg`, `pijul`, `fossil`) için yeni bir VCS deposu başlatır ya da hiçbir sürüm kontrolü oluşturmaz (`none`).
  Eğer belirtilmezse, varsayılan `git` olur veya `cargo-new.vcs` yapılandırma değeri kullanılır. Eğer dizin zaten bir VCS içindeyse, varsayılan `none` olur.

* `--registry registry`
  `Cargo.toml` içindeki `publish` alanını belirtilen kayıt defterine ayarlayarak yalnızca bu kayıt defterine yayınlamaya izin verir.
  Kayıt defteri isimleri Cargo yapılandırma dosyalarında tanımlanır. Eğer belirtilmezse `registry.default` anahtarında tanımlanan varsayılan kayıt defteri kullanılır. Varsayılan kayıt defteri tanımlı değilse ve `--registry` kullanılmazsa, `publish` alanı ayarlanmaz, yani yayınlama kısıtlanmaz.

---

### 👀 Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose` → Ayrıntılı çıktı kullanır. İki kez belirtilirse çok ayrıntılı olur.
* `-q`, `--quiet` → Cargo günlük mesajlarını bastırır.
* `--color when` → Renkli çıktının ne zaman kullanılacağını kontrol eder.

  * `auto` (varsayılan): Terminal renk desteğini otomatik algılar.
  * `always`: Her zaman renkli çıktı.
  * `never`: Hiç renkli çıktı kullanma.

---

### 🔨 Ortak Seçenekler (Common Options)

* `+toolchain` → Eğer Cargo `rustup` ile kurulmuşsa, `+stable` veya `+nightly` gibi bir araç zinciri (toolchain) belirtilebilir.
* `--config KEY=VALUE or PATH` → Bir Cargo yapılandırma değerini geçersiz kılar. Birden fazla kez belirtilebilir.
* `-C PATH` → Komut yürütülmeden önce çalışma dizinini değiştirir. Yalnızca `nightly` sürümünde mevcuttur ve `-Z unstable-options` gerektirir.
* `-h`, `--help` → Yardım bilgisini gösterir.
* `-Z flag` → Yalnızca `nightly` için kararsız (unstable) bayraklar.

---

## 🌍 ORTAM DEĞİŞKENLERİ (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🔚 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarıyla tamamlandı.
* `101`: Cargo tamamlanamadı.

---

## 📚 ÖRNEKLER (EXAMPLES)

Belirtilen dizinde yeni bir ikili Cargo paketi oluştur:

```
cargo new foo
```

👉 Bu komut, `foo` adlı dizinde `Cargo.toml` ve `src/main.rs` içeren yeni bir proje oluşturur.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-init(1)`
