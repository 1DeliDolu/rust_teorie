## 📋 cargo(1)

### 🏷️ İSİM (NAME)

cargo — Rust paket yöneticisi (package manager)

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo [options] command [args]  
cargo [options] --version  
cargo [options] --list  
cargo [options] --help  
cargo [options] --explain code  
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu program Rust dili için bir paket yöneticisi (package manager) ve derleme aracı (build tool) olup [https://rust-lang.org](https://rust-lang.org) adresinde mevcuttur.

---

## 🏗️ KOMUTLAR (COMMANDS)

### 🔧 Derleme Komutları (Build Commands)

* **cargo-bench(1)**
  Bir paketin kıyaslama testlerini (benchmarks) çalıştırır.
* **cargo-build(1)**
  Bir paketi derler (compile).
* **cargo-check(1)**
  Yerel bir paketi ve tüm bağımlılıklarını (dependencies) hatalar için kontrol eder.
* **cargo-clean(1)**
  Cargo’nun geçmişte oluşturduğu artıkları (artifacts) siler.
* **cargo-doc(1)**
  Bir paketin belgelerini (documentation) oluşturur.
* **cargo-fetch(1)**
  Bir paketin bağımlılıklarını ağdan indirir.
* **cargo-fix(1)**
  `rustc` tarafından bildirilen lint uyarılarını otomatik olarak düzeltir.
* **cargo-run(1)**
  Yerel paketin bir binary’sini veya örneğini çalıştırır.
* **cargo-rustc(1)**
  Bir paketi derler ve derleyiciye (compiler) ek seçenekler geçirir.
* **cargo-rustdoc(1)**
  Bir paketin belgelerini özel bayraklar (flags) ile oluşturur.
* **cargo-test(1)**
  Bir paketin birim (unit) ve entegrasyon (integration) testlerini çalıştırır.

---

### 📑 Manifesto Komutları (Manifest Commands)

* **cargo-add(1)**
  `Cargo.toml` dosyasına bağımlılıklar ekler.
* **cargo-generate-lockfile(1)**
  Bir proje için `Cargo.lock` dosyası oluşturur.
* **cargo-info(1)**
  Kayıttaki (registry) bir paket hakkında bilgi gösterir. Varsayılan kayıt `crates.io`’dur.
* **cargo-locate-project(1)**
  `Cargo.toml` dosyasının konumunu JSON biçiminde yazdırır.
* **cargo-metadata(1)**
  Bir paketin çözümlenmiş bağımlılıklarını makine tarafından okunabilir formatta çıkarır.
* **cargo-pkgid(1)**
  Tam nitelikli (fully qualified) bir paket belirtimini yazdırır.
* **cargo-remove(1)**
  `Cargo.toml` dosyasından bağımlılıkları kaldırır.
* **cargo-tree(1)**
  Bağımlılık grafiğinin ağaç görselleştirmesini (tree visualization) gösterir.
* **cargo-update(1)**
  Yerel `Cargo.lock` dosyasında kayıtlı bağımlılıkları günceller.
* **cargo-vendor(1)**
  Tüm bağımlılıkları yerelde kopyalar (vendor).

---

### 📦 Paket Komutları (Package Commands)

* **cargo-init(1)**
  Var olan bir dizinde yeni bir Cargo paketi oluşturur.
* **cargo-install(1)**
  Bir Rust binary’sini derler ve kurar (install).
* **cargo-new(1)**
  Yeni bir Cargo paketi oluşturur.
* **cargo-search(1)**
  `crates.io` üzerinde paket araması yapar.
* **cargo-uninstall(1)**
  Bir Rust binary’sini kaldırır.

---

### 🚀 Yayınlama Komutları (Publishing Commands)

* **cargo-login(1)**
  Kayıttan (registry) alınan bir API jetonunu (token) yerel olarak kaydeder.
* **cargo-logout(1)**
  Yerelde saklanan API jetonunu siler.
* **cargo-owner(1)**
  Kayıtta bir crate’in sahiplerini yönetir.
* **cargo-package(1)**
  Yerel paketi dağıtılabilir bir tarball içine toplar.
* **cargo-publish(1)**
  Bir paketi kayıt sistemine yükler.
* **cargo-yank(1)**
  Daha önce yüklenmiş bir crate’i dizinden (index) kaldırır.

---

### 📋 Genel Komutlar (General Commands)

* **cargo-help(1)**
  Cargo hakkında yardım bilgisi gösterir.
* **cargo-version(1)**
  Sürüm bilgisini gösterir.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🔑 Özel Seçenekler (Special Options)

* `-V`, `--version`
  Sürüm bilgisini yazdırır ve çıkar. `--verbose` ile kullanılırsa ek bilgiler gösterir.
* `--list`
  Yüklü tüm Cargo alt komutlarını listeler. `--verbose` ile ek bilgi gösterir.
* `--explain code`
  `rustc --explain CODE` çalıştırır ve hata mesajı hakkında ayrıntılı açıklama verir (ör. `E0004`).

### 🎨 Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose`
  Ayrıntılı çıktı. İki kez verilirse çok ayrıntılı çıktı sağlar.
* `-q`, `--quiet`
  Cargo günlük (log) mesajlarını yazdırmaz.
* `--color when`
  Renkli çıktının ne zaman kullanılacağını kontrol eder: `auto`, `always`, `never`.

### 📑 Manifest Seçenekleri (Manifest Options)

* `--locked`
  `Cargo.lock` dosyasındaki bağımlılıkların ve sürümlerin birebir aynı kullanılmasını garanti eder.
* `--offline`
  Cargo’nun ağ erişimini engeller.
* `--frozen`
  Hem `--locked` hem `--offline` seçeneklerini aynı anda etkinleştirir.

### ⚙️ Genel Seçenekler (Common Options)

* `+toolchain`
  `rustup` ile belirli bir araç zinciri (toolchain) seçmek için.
* `--config KEY=VALUE` veya `PATH`
  Cargo yapılandırma (configuration) değerlerini geçersiz kılar.
* `-C PATH`
  Çalışma dizinini değiştirir. Yalnızca nightly sürümde kullanılabilir.
* `-h`, `--help`
  Yardım bilgisini yazdırır.
* `-Z flag`
  Yalnızca nightly’de mevcut olan deneysel (unstable) bayraklar.

---

## 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri (environment variables) için referansa bakınız.

---

## 📤 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarılı oldu.
* `101`: Cargo işlemi tamamlayamadı.

---

## 📂 DOSYALAR (FILES)

* `~/.cargo/` → Cargo’nun ana dizini.
* `$CARGO_HOME/bin/` → `cargo-install(1)` ile kurulan binary’ler burada bulunur.
* `$CARGO_HOME/config.toml` → Küresel yapılandırma dosyası.
* `.cargo/config.toml` → Yerel yapılandırma dosyası.
* `$CARGO_HOME/credentials.toml` → Kayıt için özel kimlik doğrulama bilgileri.
* `$CARGO_HOME/registry/` → Kayıt indeksleri ve indirilen bağımlılıklar.
* `$CARGO_HOME/git/` → Git bağımlılıklarının önbellekleri.

---

## 📚 ÖRNEKLER (EXAMPLES)

Yerel paketi ve tüm bağımlılıklarını derle:

```
cargo build
```

👉 Bu komut, varsayılan modda derleme yapar.

Optimizasyonlarla derle:

```
cargo build --release
```

👉 Bu komut, üretim (release) modunda derleme yapar.

Çapraz derlenen (cross-compiled) hedef için testleri çalıştır:

```
cargo test --target i686-unknown-linux-gnu
```

👉 Bu komut, farklı bir hedef mimari için testleri çalıştırır.

Yürütülebilir (executable) oluşturan yeni bir paket oluştur:

```
cargo new foobar
```

👉 Bu komut, `foobar` adlı yeni bir paket oluşturur.

Mevcut dizinde bir paket oluştur:

```
mkdir foo && cd foo
cargo init .
```

👉 Bu komut, mevcut dizinde yeni bir Cargo paketi başlatır.

Bir komutun seçeneklerini ve kullanımını öğren:

```
cargo help clean
```

👉 Bu komut, `cargo clean` için yardım bilgisini gösterir.

---

## 🐞 HATALAR (BUGS)

Hatalar için: [https://github.com/rust-lang/cargo/issues](https://github.com/rust-lang/cargo/issues)

---

## 🔗 BAKINIZ (SEE ALSO)

`rustc(1)`, `rustdoc(1)`

---

İstersen bir sonraki adımda sadece **Build Commands (Derleme Komutları)** kısmını detaylı Türkçe çevirip ayrı ayrı açıklayayım mı?
