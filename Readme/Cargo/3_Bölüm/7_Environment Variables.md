## 🌍 Ortam Değişkenleri (environment variables)

Cargo birçok ortam değişkenini (environment variables) **okur** veya **ayarlar**. Bunlar kodunuz tarafından algılanabilir ya da Cargo’nun davranışını değiştirmek için kullanılabilir.

---

## 📥 Cargo’nun okuduğu ortam değişkenleri

Bu değişkenleri ayarlayarak Cargo’nun sisteminizdeki davranışını değiştirebilirsiniz:

* **`CARGO_LOG`** → debug log seviyesini belirler (`trace`, `debug`, `warn`).
* **`CARGO_HOME`** → Cargo’nun global önbellek dizini (varsayılan: `$HOME/.cargo` veya Windows’ta `%USERPROFILE%\.cargo`).
* **`CARGO_TARGET_DIR`** → derlenen tüm çıktılar için dizin.
* **`CARGO`** → Cargo’nun kendi binary’si yerine kullanılacak binary.
* **`RUSTC`** → kullanılacak `rustc` derleyicisi.
* **`RUSTC_WRAPPER`** → `rustc` çağrıları için wrapper (örn. `sccache`).
* **`RUSTC_WORKSPACE_WRAPPER`** → yalnızca workspace üyeleri için `rustc` wrapper.
* **`RUSTDOC`** → kullanılacak `rustdoc`.
* **`RUSTDOCFLAGS`**, **`CARGO_ENCODED_RUSTDOCFLAGS`** → tüm `rustdoc` çağrılarına gönderilecek ek bayraklar.
* **`RUSTFLAGS`**, **`CARGO_ENCODED_RUSTFLAGS`** → tüm `rustc` çağrılarına gönderilecek ek bayraklar.
* **`CARGO_INCREMENTAL`** → artırımlı derlemeyi zorlama (1 = açık, 0 = kapalı).
* **`CARGO_CACHE_RUSTC_INFO`** → derleyici sürüm bilgisinin önbelleğe alınmasını kapatma (0).
* **`HTTPS_PROXY`, `https_proxy`, `http_proxy`** → HTTP proxy.
* **`HTTP_TIMEOUT`** → HTTP zaman aşımı (saniye).
* **`TERM`** → `dumb` ise ilerleme çubuğu kapalı.
* **`BROWSER`** → `cargo doc --open` için tarayıcı.
* **`RUSTFMT`** → kullanılacak `rustfmt`.

---

## ⚙️ Yapılandırma ortam değişkenleri

Bazı config değerleri doğrudan ortam değişkenleriyle ayarlanabilir:

* **`CARGO_ALIAS_<name>`** → komut kısayolları.
* **`CARGO_BUILD_*`** → derleme ayarları (`jobs`, `rustc`, `rustdoc`, `target`, `incremental`, vb.).
* **`CARGO_CACHE_AUTO_CLEAN_FREQUENCY`** → otomatik önbellek temizleme sıklığı.
* **`CARGO_CARGO_NEW_VCS`** → `cargo new` için varsayılan VCS.
* **`CARGO_FUTURE_INCOMPAT_REPORT_FREQUENCY`** → gelecekteki uyumsuzluk rapor sıklığı.
* **`CARGO_HTTP_*`** → HTTP ayarları (`debug`, `proxy`, `timeout`, `cainfo`, `ssl-version`, `multiplexing`, vb.).
* **`CARGO_INSTALL_ROOT`** → `cargo install` kök dizini.
* **`CARGO_NET_*`** → ağ ayarları (`retry`, `git-fetch-with-cli`, `offline`).
* **`CARGO_PROFILE_<name>_*`** → profil ayarları (`debug`, `opt-level`, `lto`, vb.).
* **`CARGO_REGISTRIES_<name>_*`** → registry ayarları (`index`, `token`, `credential-provider`).
* **`CARGO_REGISTRY_*`** → varsayılan registry ayarları.
* **`CARGO_TARGET_<triple>_*`** → hedef ayarları (`linker`, `runner`, `rustflags`).
* **`CARGO_TERM_*`** → terminal ayarları (`quiet`, `verbose`, `color`, `progress`).

---

## 📤 Cargo’nun crate’ler için ayarladığı ortam değişkenleri

Cargo, crate derlenirken bu ortam değişkenlerini set eder:

* **`CARGO`** → Cargo binary’si yolu.
* **`CARGO_MANIFEST_DIR`**, **`CARGO_MANIFEST_PATH`** → manifest’in bulunduğu dizin/yol.
* **`CARGO_PKG_*`** → paket bilgileri (`VERSION`, `NAME`, `AUTHORS`, `LICENSE`, vb.).
* **`CARGO_CRATE_NAME`** → crate adı.
* **`CARGO_BIN_NAME`** → binary adı (yalnızca binary hedefleri için).
* **`OUT_DIR`** → build script çıktı dizini.
* **`CARGO_BIN_EXE_<name>`** → binary hedefinin mutlak yolu.
* **`CARGO_PRIMARY_PACKAGE`** → derlenen paket ana paket mi.
* **`CARGO_TARGET_TMPDIR`** → integration test/benchmark için geçici dizin.

### 📚 Dinamik kütüphane yolları

Cargo `cargo run` ve `cargo test` sırasında sistem kütüphane yollarını da ayarlar:

* Windows: `PATH`
* macOS: `DYLD_FALLBACK_LIBRARY_PATH`
* Unix: `LD_LIBRARY_PATH`
* AIX: `LIBPATH`

---

## 🛠️ Build script’ler için ortam değişkenleri

Cargo, `build.rs` çalıştırılırken özel değişkenler ayarlar:

* **`CARGO`**, **`CARGO_MANIFEST_DIR`**, **`CARGO_MANIFEST_PATH`**, **`CARGO_MANIFEST_LINKS`**
* **`CARGO_MAKEFLAGS`** → jobserver parametreleri
* **`CARGO_FEATURE_<name>`** → etkinleştirilmiş özellikler
* **`CARGO_CFG_<cfg>`** → yapılandırma seçenekleri (`TARGET_OS`, `TARGET_ARCH`, `TARGET_FAMILY`, `TARGET_FEATURE`, vb.)
* **`OUT_DIR`**, **`TARGET`**, **`HOST`**, **`NUM_JOBS`**
* **`OPT_LEVEL`**, **`DEBUG`**, **`PROFILE`**
* **`DEP_<name>_<key>`** → bağlı kütüphanelerden build script link bilgileri
* **`RUSTC`**, **`RUSTDOC`**, **`RUSTC_WRAPPER`**, **`RUSTC_WORKSPACE_WRAPPER`**, **`RUSTC_LINKER`**
* **`CARGO_ENCODED_RUSTFLAGS`** → `rustc` ek bayrakları

---

## 🔗 3. parti alt komutlar için ortam değişkenleri

Cargo, üçüncü taraf alt komutlarına (örn. `cargo-foobar`) şu değişkenleri verir:

* **`CARGO`** → Cargo binary’si yolu
* **`CARGO_MAKEFLAGS`** → jobserver parametreleri

---

İstersen ben sana şimdi bu **tüm ortam değişkenlerini** özetleyen **büyük bir tablo** (sütunlar: *Değişken*, *Açıklama*, *Varsayılan*, *İlgili config*) hazırlayabilirim. İstiyor musun?
