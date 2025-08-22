## 📋 cargo-init(1)

### 🏷️ İSİM (NAME)

cargo-init — Var olan bir dizinde yeni bir Cargo paketi oluşturur

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo init [options] [path]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, geçerli dizinde yeni bir Cargo manifesti (Cargo.toml) oluşturur. Bir dizin yolu argüman olarak verilirse, işlem o dizinde yapılır.

Dizinde tipik olarak adlandırılmış Rust kaynak dosyaları varsa, onlar kullanılır. Yoksa, varsayılan olarak `src/main.rs` dosyası oluşturulur; eğer `--lib` seçeneği verilirse bunun yerine `src/lib.rs` dosyası oluşturulur.

Dizin halihazırda bir VCS (sürüm kontrol sistemi) deposu değilse, yeni bir depo oluşturulur (aşağıda `--vcs` seçeneğine bakınız).

Benzer bir komut olan `cargo-new(1)`, yeni bir paketi yeni bir dizin içinde oluşturur.

---

### ⚙️ SEÇENEKLER (OPTIONS)

#### Init Options

* `--bin`
  Bir ikili hedef (binary target) ile paket oluştur (`src/main.rs`). Varsayılan davranıştır.

* `--lib`
  Bir kütüphane hedefi (library target) ile paket oluştur (`src/lib.rs`).

* `--edition edition`
  Kullanılacak Rust sürümünü belirtir. Varsayılan 2024’tür. Olası değerler: `2015`, `2018`, `2021`, `2024`.

* `--name name`
  Paket adını ayarlar. Varsayılan olarak dizin adı kullanılır.

* `--vcs vcs`
  Belirtilen sürüm kontrol sistemi (`git`, `hg`, `pijul` veya `fossil`) için yeni bir VCS deposu başlatır veya hiç başlatmaz (`none`).
  Belirtilmezse, varsayılan olarak `git` veya Cargo yapılandırmasındaki `cargo-new.vcs` değeri kullanılır. Eğer dizin zaten bir VCS içindeyse `none` olur.

* `--registry registry`
  `Cargo.toml` dosyasındaki `publish` alanını ayarlayarak yalnızca belirtilen kayıt defterine (registry) yayın yapılmasını sağlar.
  Kayıt defteri adları Cargo yapılandırma dosyalarında tanımlanır. Belirtilmezse, varsayılan kayıt defteri (`registry.default`) kullanılır. Varsayılan ayarlanmamışsa ve `--registry` de verilmemişse, `publish` alanı ayarlanmadan kalır ve yayınlama kısıtlanmaz.

---

#### Display Options

* `-v`, `--verbose`
  Ayrıntılı çıktı kullan. İki kez belirtilirse çok ayrıntılı çıktı verir (bağımlılık uyarıları, build script çıktıları gibi).

* `-q`, `--quiet`
  Cargo günlük mesajlarını bastırır.

* `--color when`
  Renkli çıktının ne zaman kullanılacağını denetler. Olası değerler:

  * `auto` (varsayılan): Terminalde renk desteği varsa otomatik algılar.
  * `always`: Her zaman renkli gösterir.
  * `never`: Hiç renkli göstermez.

---

#### Common Options

* `+toolchain`
  Cargo `rustup` ile kurulduysa ve `cargo` komutunun ilk argümanı `+` ile başlıyorsa, bu `rustup` araç zinciri (`+stable`, `+nightly`) olarak yorumlanır.

* `--config KEY=VALUE` veya `PATH`
  Cargo yapılandırma değerini geçersiz kılar. Argüman TOML sözdiziminde `KEY=VALUE` şeklinde olmalı veya ek bir yapılandırma dosyasına yol olarak verilmelidir. Birden fazla kez belirtilebilir.

* `-C PATH`
  Komut çalıştırılmadan önce geçerli çalışma dizinini değiştirir.
  Bu, Cargo’nun varsayılan olarak `Cargo.toml` aradığı yeri ve `.cargo/config.toml` aradığı dizinleri etkiler.
  Bu seçenek yalnızca nightly kanalında kullanılabilir ve `-Z unstable-options` bayrağı gerektirir.

* `-h`, `--help`
  Yardım bilgisini yazdırır.

* `-Z flag`
  Cargo için kararsız (yalnızca nightly) bayraklar. Ayrıntılar için `cargo -Z help` çalıştırılabilir.

---

### 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri hakkında ayrıntılar için referansa bakınız.

---

### 🚪 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarılı oldu.
* `101`: Cargo işlemi tamamlayamadı.

---

### 📚 ÖRNEKLER (EXAMPLES)

Geçerli dizinde ikili (binary) Cargo paketi oluştur:

```
cargo init
```

👉 Bu komut, bulunduğunuz dizinde `Cargo.toml` ve `src/main.rs` içeren yeni bir Cargo projesi oluşturur.

---

### 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-new(1)`
