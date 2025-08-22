## 📑 Manifest Formatı (The Manifest Format)

Her paketin `Cargo.toml` dosyasına **manifest** denir. Bu dosya **TOML formatında** yazılır ve paketin derlenmesi için gerekli metadata’yı içerir.
Cargo’nun manifest dosyasını nasıl bulduğunu öğrenmek için **cargo locate-project** bölümüne bakabilirsiniz.

---

### 📌 Manifest Dosyasının Bölümleri

* `cargo-features` → Kararsız (unstable), sadece nightly sürüm özellikleri
* `[package]` → Paketi tanımlar

  * `name` → Paketin adı
  * `version` → Paketin sürümü
  * `authors` → Paketin yazarları
  * `edition` → Rust sürüm dönemi
  * `rust-version` → Minimum desteklenen Rust sürümü
  * `description` → Paket açıklaması
  * `documentation` → Paket dokümantasyonu URL’si
  * `readme` → README dosyasının yolu
  * `homepage` → Paket ana sayfa URL’si
  * `repository` → Kaynak kod deposu URL’si
  * `license` → Paket lisansı
  * `license-file` → Lisans metninin yolu
  * `keywords` → Paket için anahtar kelimeler
  * `categories` → Paket kategorileri
  * `workspace` → Paketin çalışma alanı
  * `build` → Derleme betiği yolu
  * `links` → Paketin bağlandığı yerel (native) kütüphane adı
  * `exclude` → Yayınlarken hariç tutulacak dosyalar
  * `include` → Yayınlarken dahil edilecek dosyalar
  * `publish` → Paketin yayınlanmasını engellemek için kullanılabilir
  * `metadata` → Harici araçlar için ek ayarlar
  * `default-run` → `cargo run` için varsayılan binary
  * `autolib`, `autobins`, `autoexamples`, `autotests`, `autobenches` → Otomatik keşifleri devre dışı bırakır
  * `resolver` → Kullanılacak bağımlılık çözücü

**Hedef tablolar (Target tables):**

* `[lib]` → Kütüphane hedef ayarları
* `[[bin]]` → İkili (binary) hedef ayarları
* `[[example]]` → Örnek hedef ayarları
* `[[test]]` → Test hedef ayarları
* `[[bench]]` → Benchmark hedef ayarları

**Bağımlılık tabloları (Dependency tables):**

* `[dependencies]` → Paket bağımlılıkları
* `[dev-dependencies]` → Örnekler, testler ve benchmarklar için bağımlılıklar
* `[build-dependencies]` → Derleme betikleri için bağımlılıklar
* `[target]` → Platforma özel bağımlılıklar
* `[badges]` → Kayıt defterinde gösterilecek rozetler
* `[features]` → Koşullu derleme özellikleri
* `[lints]` → Linter yapılandırmaları
* `[patch]` → Bağımlılık geçersiz kılma
* `[replace]` → Bağımlılık geçersiz kılma (deprecated)
* `[profile]` → Derleyici ayarları ve optimizasyonlar
* `[workspace]` → Çalışma alanı tanımı

---

### 📦 `[package]` Bölümü

Her `Cargo.toml` dosyasının ilk bölümü `[package]` olur:

```toml
[package]
name = "hello_world" # paketin adı
version = "0.1.0"    # semver’e uyan sürüm
```

* Cargo için **tek zorunlu alan** `name`’dir.
* Eğer paket crates.io üzerinde yayınlanacaksa, bazı ek alanlar da zorunlu hale gelir.

---

### 🏷️ `name` Alanı

* Paket adı, pakete atıfta bulunmak için kullanılan tanımlayıcıdır.
* Başka bir pakette bağımlılık olarak listelenirken ve varsayılan lib/bin isimlendirmelerinde kullanılır.
* Yalnızca **alfanümerik karakterler**, `-` veya `_` kullanılabilir, boş olamaz.

**Ek kısıtlamalar:**

* `cargo new` ve `cargo init` paket adının geçerli bir Rust tanımlayıcısı olmasını ve anahtar sözcük olmamasını şart koşar.
* **crates.io kısıtlamaları:**

  * Yalnızca **ASCII karakterler** kullanılabilir.
  * Ayrılmış isimler kullanılamaz.
  * Windows özel adları (`nul` vb.) kullanılamaz.
  * Maksimum uzunluk: **64 karakter**

---

### 🏷️ `version` Alanı

* **SemVer (Semantic Versioning)** formatına göre yazılır.
* Üç sayıdan oluşur: `major.minor.patch`
* Ön-sürüm (pre-release) kısmı eklenebilir: `1.0.0-alpha`

  * Parçalara ayrılabilir: `1.0.0-alpha.11` > `1.0.0-alpha.4`
* Metadata kısmı eklenebilir: `1.0.0+21AF26D3` (bilgi amaçlıdır, Cargo dikkate almaz).
* Varsayılan değer: `0.0.0`
* **Yayınlamak için zorunludur.**

---

### 🏷️ `authors` Alanı

⚠️ Bu alan **deprecated** (artık önerilmiyor).

* Paket yazarlarını (ve opsiyonel e-posta adreslerini) listeler.
* Örnek:

```toml
[package]
# ...
authors = ["Graydon Hoare", "Fnu Lnu <no-reply@rust-lang.org>"]
```

* Bu alan, paket metadata’sında ve `CARGO_PKG_AUTHORS` ortam değişkeninde hâlâ geriye dönük uyumluluk için mevcuttur.

## 📑 Manifest Formatı: Alanlar (Manifest Fields)

### 🦀 `edition` Alanı

* `edition` anahtarı, paketin hangi **Rust Edition** sürümüyle derleneceğini belirler.
* `[package]` bölümünde ayarlandığında, paket içindeki tüm hedefleri (testler, benchmarklar, binary’ler, örnekler vb.) etkiler.

```toml
[package]
edition = "2024"
```

* `cargo new` komutu ile oluşturulan manifest dosyalarında bu alan genellikle en güncel kararlı sürüm olarak otomatik doldurulur (şu anda **2024 edition**).
* Eğer bu alan belirtilmezse, geriye dönük uyumluluk için **2015 edition** varsayılır. Ancak `cargo new` ile oluşturulan tüm yeni projelerde bu alan açıkça yazıldığı için bu durum pratikte görülmez.

---

### 🦀 `rust-version` Alanı

* Bu alan, paketinizi destekleyen minimum **Rust sürümünü** belirtir.
* Ayrıntılar için *Rust version* bölümüne bakılabilir.

---

### 📝 `description` Alanı

* Paket hakkında kısa bir açıklama içerir.
* crates.io üzerinde paketinizin sayfasında gösterilir.
* Düz metin olmalıdır (Markdown olmamalı).

```toml
[package]
description = "A short description of my package"
```

⚠️ crates.io üzerinde yayınlamak için bu alan **zorunludur**.

---

### 📚 `documentation` Alanı

* Paket dokümantasyonunun URL’sini belirtir.

```toml
[package]
documentation = "https://docs.rs/bitflags"
```

* Eğer belirtilmezse, crates.io otomatik olarak **docs.rs** bağlantısını kullanır (dokümantasyon derlendikten sonra).

---

### 📖 `readme` Alanı

* Paketin kök dizinindeki bir dosyaya (örn. `README.md`) giden yol belirtilir.

```toml
[package]
readme = "README.md"
```

* Eğer belirtilmezse, kökte `README.md`, `README.txt` veya `README` dosyalarından biri aranır ve varsayılan olarak kullanılır.
* `false` verilirse bu davranış bastırılır.
* `true` verilirse varsayılan olarak `README.md` kabul edilir.

---

### 🏠 `homepage` Alanı

* Paket için özel bir **ana sayfa URL’si** belirtilir.

```toml
[package]
homepage = "https://serde.rs"
```

⚠️ Bu alan sadece paket için ayrı bir web sitesi varsa kullanılmalıdır. `repository` veya `documentation` ile aynı olmamalıdır.

---

### 📦 `repository` Alanı

* Kaynak kodun bulunduğu depo URL’si belirtilir.

```toml
[package]
repository = "https://github.com/rust-lang/cargo"
```

---

### ⚖️ `license` ve `license-file` Alanları

* `license`: Paket lisansının adını belirtir (SPDX 2.3 formatında).
* `license-file`: Lisans metnini içeren dosyanın yolunu belirtir.

```toml
[package]
license = "MIT OR Apache-2.0"
```

👉 Örnek lisans ifadeleri:

* `MIT OR Apache-2.0`
* `LGPL-2.1-only AND MIT AND BSD-2-Clause`
* `GPL-2.0-or-later WITH Bison-exception-2.2`

```toml
[package]
license-file = "LICENSE.txt"
```

⚠️ crates.io için `license` veya `license-file` alanlarından biri **zorunludur**.

---

### 🏷️ `keywords` Alanı

* Paketle ilişkili en fazla **5 anahtar kelime** içerir.

```toml
[package]
keywords = ["gamedev", "graphics"]
```

Kurallar:

* Her kelime ASCII olmalı
* En fazla 20 karakter
* Alfanümerik karakterle başlamalı
* `letters, numbers, _, -, +` dışında karakter içeremez

---

### 🗂️ `categories` Alanı

* Paketinizin ait olduğu kategorileri listeler.

```toml
[package]
categories = ["command-line-utilities", "development-tools::cargo-plugins"]
```

⚠️ Maksimum 5 kategori belirtilebilir.
⚠️ Değerler, [crates.io/category\_slugs](https://crates.io/category_slugs) sayfasındaki kategorilerle birebir eşleşmelidir.

---

### 🗄️ `workspace` Alanı

* Paketinizin dahil olduğu **workspace**’i belirtmek için kullanılır.

```toml
[package]
workspace = "path/to/workspace/root"
```

Kurallar:

* Eğer zaten `[workspace]` tablosu tanımlıysa bu alan kullanılamaz.
* Yani bir crate aynı anda hem workspace root’u hem de başka bir workspace’in üyesi olamaz.

Daha fazla bilgi için **workspaces** bölümüne bakabilirsiniz.

## 📑 Manifest Formatı: İleri Düzey Alanlar (The build field, links, include/exclude, publish, metadata, vb.)

### 🔨 `build` Alanı

* Paket kökünde bulunan bir **derleme betiğini (build script)** belirtir.
* Varsayılan değer `build.rs` dosyasıdır.

```toml
[package]
build = "build.rs"
```

* Farklı bir dosya belirtmek için: `build = "custom_build.rs"`
* Otomatik algılamayı kapatmak için: `build = false`

---

### 🔗 `links` Alanı

* Bağlandığınız yerel (native) kütüphanenin adını belirtir.

```toml
[package]
links = "git2"
```

👉 Örn: `libgit2.a` (Linux)

---

### 📂 `exclude` ve `include` Alanları

* Yayınlama sırasında hangi dosyaların dahil edileceğini veya hariç tutulacağını kontrol eder.

```toml
[package]
exclude = ["/ci", "images/", ".*"]
```

```toml
[package]
include = ["/src", "COPYRIGHT", "/examples", "!/examples/big_example"]
```

* `include` → sadece belirtilen dosyaları dahil eder (ve `exclude`’yu geçersiz kılar).
* Kalıplar **gitignore stili** kullanır (`*`, `?`, `[]`, `!` gibi).
* Daima hariç tutulanlar:

  * Alt paketler (`Cargo.toml` içeren alt klasörler)
  * `target` klasörü
* Daima dahil edilenler:

  * `Cargo.toml`
  * Küçültülmüş `Cargo.lock`
  * `license-file` varsa, o dosya

---

### 🚫 `publish` Alanı

* Paketin hangi kayıt defterlerine (registries) yüklenebileceğini kontrol eder.

```toml
[package]
publish = ["some-registry-name"]
```

* Yayınlamayı tamamen engellemek için:

```toml
[package]
publish = false
```

---

### ⚙️ `metadata` Tablosu

* Cargo tarafından yok sayılır, **harici araçlar** için kullanılabilir.

```toml
[package.metadata.android]
package-name = "my-awesome-android-app"
assets = "path/to/static"
```

👉 Workspace seviyesinde de `workspace.metadata` bulunabilir.

---

### ▶️ `default-run` Alanı

* `cargo run` için varsayılan binary’yi belirtir.

```toml
[package]
default-run = "a"
```

👉 Örn: `src/bin/a.rs` ve `src/bin/b.rs` varsa, `cargo run` çalıştırıldığında `a` seçilir.

---

### 🚨 `[lints]` Bölümü

* Lint seviyelerini ayarlamak için kullanılır.

```toml
[lints.rust]
unsafe_code = "forbid"
```

Bu, eşdeğerdir:

```toml
[lints.rust]
unsafe_code = { level = "forbid", priority = 0 }
```

* Olası `level` değerleri:

  * `forbid`
  * `deny`
  * `warn`
  * `allow`

* Lint tablosu → lint adındaki `::` öncesine göre belirlenir.

  * `lints.rust.unsafe_code`
  * `lints.clippy.enum_glob_use`

```toml
[lints.clippy]
enum_glob_use = "deny"
```

⚠️ Bu ayarlar yalnızca **geçerli paket** için geçerlidir, bağımlılıklara uygulanmaz.

---

### 🏷️ `[badges]` Bölümü

* Registry üzerinde **durum rozetleri** göstermek için kullanılır.

```toml
[badges]
maintenance = { status = "actively-developed" }
```

Olası `status` değerleri:

* `actively-developed`
* `passively-maintained`
* `as-is`
* `experimental`
* `looking-for-maintainer`
* `deprecated`
* `none`

⚠️ crates.io rozetleri artık göstermiyor, onun yerine README’ye eklenmesi öneriliyor.

---

### 📦 Bağımlılık Bölümleri (Dependency sections)

* `[dependencies]` → Normal bağımlılıklar
* `[dev-dependencies]` → Test, örnek, benchmark için bağımlılıklar
* `[build-dependencies]` → Derleme betikleri için bağımlılıklar
* `[target.*.dependencies]` → Platforma özel bağımlılıklar

---

### ⚙️ `[profile.*]` Bölümleri

* Derleyici ayarlarını (optimizasyon, debug vb.) özelleştirir.
* Ayrıntılar için **Profiles** bölümüne bakın.

---

⚠️ Önceden birden fazla lisans `/` ile ayrılabiliyordu, ancak bu kullanım **deprecated**.
