## 🎯 Cargo Targets / Cargo Hedefleri

Cargo paketleri, derlenebilecek kaynak dosyalarına karşılık gelen **target**’lardan oluşur. Bir pakette şunlar bulunabilir:

* 📚 **Library (kütüphane)**
* ⚙️ **Binary (ikili programlar)**
* 📝 **Examples (örnekler)**
* 🧪 **Tests (testler)**
* 📊 **Benchmarks (kıyaslamalar)**

Bu hedefler genellikle dosya dizin yapısına göre Cargo tarafından otomatik keşfedilir, ama `Cargo.toml` içinde yapılandırılabilir.

---

### 📚 Library (`[lib]`)

* Varsayılan dosya: `src/lib.rs`
* Varsayılan isim: paket adı (`-` karakterleri `_` olur).
* Bir pakette **sadece 1 library** olabilir.
* `[lib]` tablosu ile özelleştirilebilir.

```toml
[lib]
crate-type = ["cdylib"]
bench = false
```

---

### ⚙️ Binaries (`[[bin]]`)

* Varsayılan dosya: `src/main.rs` veya `src/bin/` altındaki dosyalar.
* Varsayılan isim: paket adı.
* `[[bin]]` tabloları ile birden fazla binary tanımlanabilir.
* `cargo run --bin <isim>` ile çalıştırılır.
* `cargo install` ile sisteme yüklenebilir.

```toml
[[bin]]
name = "cool-tool"
test = false
bench = false

[[bin]]
name = "frobnicator"
required-features = ["frobnicate"]
```

---

### 📝 Examples (`[[example]]`)

* `examples/` dizininde bulunur.
* Varsayılan olarak çalıştırılabilir binary’dir.
* `crate-type` ile library yapılabilir.
* `cargo run --example <isim>` veya `cargo build --example <isim>` ile çalıştırılır.
* `cargo test` örnekleri de derler, ama test çalıştırmaz.

```toml
[[example]]
name = "foo"
crate-type = ["staticlib"]
```

---

### 🧪 Tests (`[[test]]`)

Cargo’da 3 test türü vardır:

1. **Unit tests** → `#[test]` fonksiyonları, aynı crate içinde (private API’ye erişebilir).
2. **Integration tests** → `tests/` klasöründe ayrı dosyalar (sadece public API’ye erişir).

   * Ortak kod için `tests/common/mod.rs` kullanılabilir.
   * Her dosya ayrı binary olur → `cargo test` bunları sırayla çalıştırır.
   * `CARGO_BIN_EXE_<isim>` değişkeni ile binary çalıştırılabilir.
3. **Documentation tests** → `rustdoc` ile üretilir.

---

### 📊 Benchmarks (`[[bench]]`)

* `benches/` dizininde bulunur.
* Fonksiyonlar `#[bench]` ile işaretlenir.
* `cargo bench` ile çalıştırılır.
* Nightly gerektirir (`#[bench]` unstable).
* Stable için `criterion` gibi crate’ler kullanılabilir.

```toml
[[bench]]
name = "my_bench"
harness = false
```

---

## ⚙️ Hedefleri Yapılandırma

Tüm `[lib]`, `[[bin]]`, `[[example]]`, `[[test]]`, `[[bench]]` tablolarında benzer ayarlar bulunur:

```toml
[lib]
name = "foo"           # Hedef adı
path = "src/lib.rs"    # Kaynak dosya
test = true            # Varsayılan test edilir mi?
doctest = true         # Dokümantasyon örnekleri test edilir mi?
bench = true           # Varsayılan benchmark yapılır mı?
doc = true             # Dokümantasyona dahil mi?
proc-macro = false     # Procedural macro mu?
harness = true         # libtest harness kullanılsın mı?
crate-type = ["lib"]   # crate tipleri
required-features = [] # Derleme için gerekli feature'lar
```

* **name** → çıktı dosyası veya crate adı.
* **path** → kaynak dosya konumu.
* **test** → `cargo test`’te dahil edilir mi?
* **doctest** → yalnızca `[lib]` için geçerli.
* **bench** → `cargo bench`’te dahil edilir mi?
* **doc** → `cargo doc`’a dahil edilir mi?
* **proc-macro** → yalnızca `[lib]` için procedural macro.
* **harness** → libtest ile otomatik test çalıştırma.
* **crate-type** → `bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib`, `proc-macro`.
* **required-features** → feature’lar aktif değilse derlenmez.

---

### 🔍 Target Auto-Discovery

Cargo dosya yapısına göre otomatik target keşfi yapar.

* Varsayılan olarak: `src/lib.rs`, `src/main.rs`, `src/bin/`, `tests/`, `benches/`, `examples/` dizinleri.

Otomatik keşfi kapatmak için `Cargo.toml` içinde:

```toml
[package]
autolib = false
autobins = false
autoexamples = false
autotests = false
autobenches = false
```

👉 Örn: `src/bin/mod.rs` kütüphanede `bin` modülü olarak kullanılacaksa, `autobins = false` yapılmalı.

---

📌 **MSRV (Minimum Supported Rust Version)**

* `autobins`, `autoexamples`, `autotests`, `autobenches` → 1.27’den itibaren
* `autolib` → 1.83’den itibaren

---

Ben sana şimdi her target türü için ayrı ayrı **örnek Cargo.toml parçaları** hazırlayayım mı? (library, bin, example, test, bench için tek tek)
