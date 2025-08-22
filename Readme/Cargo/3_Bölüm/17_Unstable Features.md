## 🧪 Kararsız Özellikler (unstable features)

Deneysel Cargo özellikleri yalnızca **nightly channel** üzerinde kullanılabilir. Bu özellikleri ihtiyaçlarınıza uygun olup olmadığını görmek ve sorunları tespit etmek için denemeniz teşvik edilir. Özelliklerle ilgili daha fazla bilgi için aşağıda bağlantısı verilen takip konularına (tracking issues) bakabilirsiniz. Gelecekteki güncellemeler için GitHub’daki **subscribe** düğmesini kullanabilirsiniz.

Belirli bir süre sonra, eğer özellik hakkında büyük sorunlar bulunmazsa, özellik **stabilize edilir**. Bu durumda, ilgili nightly sürümü stable kanala ulaştığında (genellikle 6 ila 12 hafta arasında), stable sürümde de kullanılabilir hale gelir.

---

## ⚙️ Kararsız Özelliklerin Etkinleştirilmesi (enabling unstable features)

Bir özelliğin nasıl çalıştığına bağlı olarak kararsız özellikler üç farklı şekilde etkinleştirilebilir:

1. **Yeni sözdizimi (syntax) – `Cargo.toml`**
   `Cargo.toml` dosyasının en üstünde `cargo-features` anahtarı gerektirir.

   ```toml
   # Bu, hangi yeni Cargo.toml özelliklerinin etkinleştirildiğini belirtir.
   cargo-features = ["test-dummy-unstable"]

   [package]
   name = "my-package"
   version = "0.1.0"
   im-a-teapot = true  # Bu seçenek test-dummy-unstable ile etkinleştirildi.
   ```

2. **Yeni komut satırı bayrakları / seçenekleri**
   `-Z unstable-options` seçeneği de dahil edilmelidir.

   ```bash
   cargo +nightly build --artifact-dir=out -Z unstable-options
   ```

3. **Diğer `-Z` bayrakları**
   Arayüzü olmayan ya da tasarım aşamasında olan işlevler için kullanılır. Örneğin:

   ```bash
   cargo +nightly build -Z mtime-on-use
   ```

   Mevcut tüm bayrakları görmek için:

   ```bash
   cargo -Z help
   ```

4. **Yapılandırma (config) dosyası**
   `.cargo/config.toml` içindeki `[unstable]` tablosunda ayarlanabilir.

   ```toml
   [unstable]
   mtime-on-use = true
   build-std = ["core", "alloc"]
   ```

---

## 📋 Kararsız Özelliklerin Listesi (list of unstable features)

Aşağıda bazı önemli kararsız özellikler listelenmiştir:

### 🔐 Özel (unstable-specific)

* `-Z allow-features` — Kullanılabilecek kararsız özellikleri sınırlamak için.

### 🛠️ Derleme ve Bağlama (build scripts and linking)

* **Metabuild** — Bildirimsel derleme betikleri sağlar.

### 📦 Çözücü ve Özellikler (resolver and features)

* `no-index-update` — Cargo’nun indeks önbelleğini güncellemesini engeller.
* `avoid-dev-deps` — Çözümleme sırasında dev-bağımlılıkların (dev-dependencies) dahil edilmesini önler.
* `minimal-versions` — En düşük uyumlu sürümün kullanılmasını zorunlu kılar.
* `direct-minimal-versions` — Sadece doğrudan bağımlılıklar için en düşük uyumlu sürümü seçer.
* `public-dependency` — Bağımlılıkları **public** veya **private** olarak sınıflandırmaya izin verir.
* `msrv-policy` — Minimum Desteklenen Rust Sürümü (MSRV) farkındalığı ile çözümleme yapar.
* `precise-pre-release` — `update --precise` ile ön sürümlerin (pre-release) seçilmesine izin verir.
* `sbom` — Derlenen çıktılar için SBOM öncüsü dosyaları üretir.
* `update-breaking` — `update --breaking` ile kırıcı sürümlere yükseltmeye izin verir.
* `feature-unification` — Workspace içinde yeni özellik birleştirme modlarını etkinleştirir.

### 📤 Çıktı Davranışı (output behavior)

* `artifact-dir` — Çıktıların kopyalanacağı dizini belirler.
* `build-dir` — Ara derleme çıktılarının depolanacağı dizini belirler.
* **Different binary name** — Üretilen ikiliye `crate` isminden farklı bir ad atar.
* `root-dir` — Yolların hangi kök dizine göre yazdırılacağını belirler.

### ⚡ Derleme Davranışı (compile behavior)

* `mtime-on-use` — Kullanılan her bağımlılığın son değiştirilme zamanını günceller.
* `build-std` — Önceden derlenmiş standart kütüphane yerine standart kütüphaneyi derler.
* `build-std-features` — Standart kütüphane ile kullanılacak özellikleri belirler.
* `binary-dep-depinfo` — İkili bağımlılıkların `dep-info` dosyasında takip edilmesini sağlar.
* `checksum-freshness` — Dosya tarihleri yerine dosya özetlerini kullanarak yeniden derleme kararını verir.
* `panic-abort-tests` — Testleri `abort` panik stratejisi ile çalıştırmaya izin verir.
* `host-config` — Host derleme hedefleri için `[target]` benzeri yapılandırma ayarlarını yapar.
* `no-embed-metadata` — Derleyiciye `-Zembed-metadata=no` geçer, diskte yer kazanmak için metadata gömülmez.
* `target-applies-to-host` — Belirli bayrakların host derleme hedeflerine uygulanıp uygulanmayacağını belirler.
* `gc` — Küresel önbellek çöp toplama.
* `open-namespaces` — Birden fazla paketin aynı API namespace içinde çalışmasına izin verir.

### 📖 Rustdoc

* `rustdoc-map` — Belgelerin dış sitelere (örn. docs.rs) bağlanabilmesi için haritalar sağlar.
* `scrape-examples` — Belgelerde örnekleri gösterir.
* `output-format` — Belgelerin deneysel JSON formatında da üretilmesine izin verir.
* `rustdoc-depinfo` — Rustdoc yeniden derleme tespitinde `dep-info` dosyalarını kullanır.

### 📑 Cargo.toml Uzantıları (extensions)

* **Profile rustflags option** — Doğrudan `rustc`'ye geçirilir.
* **Profile hint-mostly-unused option** — Bağımlılığın çoğunlukla kullanılmadığını belirtir.
* `codegen-backend` — Kullanılacak `rustc` codegen backend seçilir.
* `per-package-target` — Her paket için kullanılacak `--target` değerini belirler.
* **artifact dependencies** — Bir çıktı dosyasının diğerine bağımlı olmasına izin verir.
* **Profile trim-paths option** — Çıktılardaki dosya yollarının anonimleştirilmesini kontrol eder.
* `[lints.cargo]` — Cargo lintlerinin yapılandırılmasına izin verir.
* **path bases** — Yol bağımlılıkları için adlandırılmış temel dizinler.
* **unstable-editions** — Henüz stabil olmayan sürümlerin kullanımına izin verir.

### ℹ️ Bilgi ve Metadata (information and metadata)

* **Build-plan** — Çalıştırılacak komutların JSON bilgisini üretir.
* **unit-graph** — Cargo’nun iç grafik yapısını JSON olarak üretir.
* `cargo rustc --print` — `rustc --print` bilgilerini görüntüler.

### ⚙️ Yapılandırma (configuration)

* `config-include` — Config dosyalarının diğer dosyaları içermesine izin verir.
* `cargo config` — Config dosyalarını görüntülemek için yeni bir alt komut.

### 📦 Kayıtlar (registries)

* `publish-timeout` — Crate yüklemesi ile indeks güncellemesi arasındaki zaman aşımını kontrol eder.
* `asymmetric-token` — Asimetrik kriptografi ile kimlik doğrulama jetonlarını destekler.

### 🔧 Diğer (other)

* `gitoxide` — Belirli işlemlerde `git2` yerine `gitoxide` kullanır.
* `script` — Tek dosyalık `.rs` paketlerini destekler.
* `lockfile-path` — Varsayılan `<workspace_root>/Cargo.lock` dışında lockfile yolunu belirtir.
* `package-workspace` — Workspace içinde birden fazla crate’in paketlenmesine ve yayınlanmasına izin verir.
* `native-completions` — Cargo shell tamamlama betiklerini native tamamlamalara taşır.
* `warnings` — Uyarı davranışlarını kontrol eder.
* **Package message format** — `cargo package` için mesaj formatını belirler.
* `fix-edition` — Kalıcı olarak kararsız sürüm geçiş yardımcısı.

---

## 🔐 `allow-features`

`-Zallow-features` bayrağı, yalnızca listelenen kararsız özelliklerin kullanılmasına izin verir.

* Örn: `-Zallow-features=foo,bar` → sadece `-Zfoo` ve `-Zbar` kullanılabilir, `-Zbaz` kullanılamaz.
* `-Zallow-features=` (boş string) → tüm kararsız özellikler yasaklanır.
* Bu sınırlama `Cargo.toml` içindeki `cargo-features` girişlerini de kapsar.

Örn:

```toml
cargo-features = ["test-dummy-unstable"]
```

Eğer `-Zallow-features=` kullanılırsa bu da engellenir.

---

## 🛑 Örnek Kararsız Özellikler ve Takip Konuları

* **no-index-update**

  * Cargo’nun indeks güncellemesi yapmasını engeller.
  * Issue: #3479 / Tracking: #7404

* **mtime-on-use**

  * Kullanılan dosyaların `mtime` değerini güncelleyerek eski dosyaların tespitini kolaylaştırır.
  * Issue: #6477 / Tracking: #7150

* **avoid-dev-deps**

  * `cargo build` veya `cargo install` sırasında kullanılmayan `dev-dependencies` indirilmez.
  * Issue: #4988 / Tracking: #5133

* **minimal-versions**

  * Tüm bağımlılıklar için en düşük sürümü seçer. Genellikle önerilmez.
  * Issue: #4100 / Tracking: #5657

* **direct-minimal-versions**

  * Sadece doğrudan bağımlılıklar için en düşük sürümü seçer.
  * Issue: #4100 / Tracking: #5657

* **artifact-dir**

  * Derleme çıktılarının kopyalanacağı dizini belirler.
  * Issue: #4875 / Tracking: #6790

  ```bash
  cargo +nightly build --artifact-dir=out -Z unstable-options
  ```

  `.cargo/config.toml` örneği:

  ```toml
  [build]
  artifact-dir = "out"
  ```

* **build-dir**

  * Ara derleme çıktılarının depolanacağı dizin.
  * Issue: #14125 / Tracking: #14125

  ```toml
  [build]
  build-dir = "out"
  ```

  Ortam değişkeni: `CARGO_BUILD_BUILD_DIR`

  Kullanılabilir şablon değişkenleri:

  * `{workspace-root}`
  * `{cargo-cache-home}`
  * `{workspace-path-hash}`

## 📂 `root-dir`

* **Original Issue:** #9887
* **Tracking Issue:** Yok (stabilizasyon için planlanmamış)

`-Zroot-dir` bayrağı, yolların hangi kök dizine göre yazdırılacağını ayarlar. Bu hem tanılama (diagnostics) hem de `file!()` makrosu tarafından üretilen yolları etkiler.

---

## 📋 Build-plan

* **Tracking Issue:** #5579
* **Durum:** Kullanımdan kaldırılmıştır (deprecated) ve gelecekte kaldırılabilir.

`--build-plan` argümanı, çalıştırılacak komutları **gerçekten çalıştırmadan** JSON çıktısı olarak üretir. Bu, başka bir derleme aracıyla entegrasyon yaparken faydalı olabilir.

Örnek:

```bash
cargo +nightly build --build-plan -Z unstable-options
```

---

## 🏗️ Metabuild

* **Tracking Issue:** rust-lang/rust#49803
* **RFC:** #2196

Metabuild, bildirimsel (declarative) build script desteğidir.
Bir `build.rs` betiği yazmak yerine, `Cargo.toml` içinde `metabuild` anahtarına bir bağımlılık listesi eklersiniz. Cargo, her build bağımlılığını sırayla çalıştıran bir build script otomatik üretir.

Örnek `Cargo.toml`:

```toml
cargo-features = ["metabuild"]

[package]
name = "mypackage"
version = "0.0.1"
metabuild = ["foo", "bar"]

[build-dependencies]
foo = "1.0"
bar = "1.0"

[package.metadata.foo]
extra-info = "qwerty"
```

Metabuild paketleri, `build.rs` dosyasının yaptığı işleri yapan bir `metabuild` fonksiyonuna sahip olmalıdır.

---

## 🔓 public-dependency

* **Tracking Issue:** #44663

Bağımlılıkları **public** veya **private** olarak işaretlemenize izin verir. Bu, `exported_private_dependencies` linti için gerekli ek bilgileri `rustc`’ye iletir.

Etkinleştirme:

```bash
cargo +nightly run -Zpublic-dependency
```

Veya `.cargo/config.toml`:

```toml
[unstable]
public-dependency = true
```

Ya da `Cargo.toml` içinde (yakında kaldırılacak yöntem):

```toml
cargo-features = ["public-dependency"]

[dependencies]
my_dep = { version = "1.2.3", public = true }
private_dep = "2.0.0" # Varsayılan olarak 'private'
```

---

## 📜 msrv-policy

* **RFC:** MSRV-aware Resolver (RFC 2495)
* **Tracking Issues:**

  * Resolver: #9930 → stabil 1.84 (#14639)
  * Cargo add: stabil 1.79 (#13608)

MSRV (Minimum Supported Rust Version) farkındalığı için kullanılan özelliklerdir. Bazıları stabilize edilmiştir, bazıları hâlâ uygulanmamıştır (ör. `package.rust-version = "toolchain"`).

---

## 🎯 precise-pre-release

* **Tracking Issue:** #13290
* **RFC:** #3493

`update --precise` ile **ön sürüm (pre-release)** versiyonların seçilmesine izin verir.

Örnek:

```toml
[dependencies]
my-dependency = "0.1.1"
```

Komut:

```bash
cargo update -Zunstable-options my-dependency --precise 0.1.2-pre.0
```

0.1.1 → 0.1.2-pre.0 güncellemesi mümkündür, fakat 0.2.0-pre.0 mümkün değildir.

---

## 📝 sbom

* **Tracking Issue:** #13709
* **RFC:** #3553

Derleme çıktılarıyla birlikte **SBOM (Software Bill of Materials)** öncüsü JSON dosyaları üretir.

Etkinleştirme:

```toml
[unstable]
sbom = true

[build]
sbom = true
```

Veya ortam değişkeni:

```bash
CARGO_BUILD_SBOM=true
```

Üretilen dosya adı: `<artifact>.cargo-sbom.json`

JSON şeması bağımlılıklar, hedef, özellikler ve kullanılan `rustc` bilgilerini içerir.

Cargo ayrıca crate’ler için:

* `CARGO_SBOM_PATH` ortam değişkenini ayarlar.

---

## ⚡ update-breaking

* **Tracking Issue:** #12425

SemVer uyumsuz sürümlere (`--breaking`) güncelleme yapılmasına izin verir.

Örnek:

```bash
cargo +nightly -Zunstable-options update --breaking
cargo +nightly -Zunstable-options update --breaking clap
```

---

## 📚 build-std

* **Tracking Repo:** [https://github.com/rust-lang/wg-cargo-std-aware](https://github.com/rust-lang/wg-cargo-std-aware)

Cargo’nun standart kütüphaneyi (`core`, `alloc`, `std`, `proc_macro`) **baştan derlemesine** izin verir. Bu, “std-aware Cargo” olarak da bilinir.

Kullanım için standart kütüphane kaynak kodunun yüklü olması gerekir:

```bash
rustup component add rust-src --toolchain nightly
```

Örnek:

```bash
cargo new foo
cd foo
cargo +nightly run -Z build-std --target x86_64-unknown-linux-gnu
```

Sadece belirli std kütüphanelerini derlemek için:

```bash
cargo +nightly build -Z build-std=core,alloc
```

---

İstersen sana bu listedeki her özelliği (örneğin `sbom`, `update-breaking`, `build-std`) tek tek ayrıntılı açıklama + Türkçe örneklerle ayırıp verebilirim. Bunu ister misin?

## 📌 `-Z build-std` Gereksinimleri (requirements)

`-Z build-std` özelliğini kullanmak için bugün gerekli olan şartlar:

* `libstd` kaynak kodunu yüklemelisiniz:

  ```bash
  rustup component add rust-src
  ```

* Hem nightly `cargo` hem de nightly `rustc` kullanmalısınız.

* Tüm `cargo` çağrılarına `-Z build-std` bayrağı eklenmelidir.

---

## 🐞 Hata Bildirme ve Katkıda Bulunma (reporting bugs and helping out)

`-Z build-std` özelliği çok erken geliştirme aşamasındadır.
Bu Cargo özelliğinin uzun bir geçmişi ve geniş bir kapsamı vardır.

* **Uygulama hataları** için:
  [Cargo Issues](https://github.com/rust-lang/cargo/issues/new)

* **Tasarım soruları** için:
  [Tracking Repo Issues](https://github.com/rust-lang/wg-cargo-std-aware/issues/new)

Eksik gördüğünüz veya istediğiniz bir özellik varsa, tracking repo’da arayabilir; yoksa yeni bir issue açabilirsiniz.

---

## ⚙️ `build-std-features`

* **Tracking Repo:** [https://github.com/rust-lang/wg-cargo-std-aware](https://github.com/rust-lang/wg-cargo-std-aware)

Bu bayrak, `-Z build-std` özelliğinin kardeşidir.
Standart kütüphane derlenirken hangi **özelliklerin (features)** etkinleştirileceğini yapılandırır.

* Varsayılan etkin özellikler: `backtrace` ve `panic-unwind`
* Virgülle ayrılmış bir liste kabul eder ve varsayılan listeyi geçersiz kılar.

---

## 📦 `binary-dep-depinfo`

* **Tracking rustc issue:** #63012

`-Z binary-dep-depinfo` bayrağı Cargo’nun aynı bayrağı `rustc`’ye iletmesini sağlar.
Bu, `rustc`’nin tüm ikili bağımlılıkların yollarını `.d` (dep-info) dosyasına dahil etmesine yol açar.

Cargo daha sonra bu bilgiyi değişiklik tespitinde kullanır.
Ana kullanım durumu: **Rust derleyicisinin kendisini derlemek**, çünkü standart kütüphane gibi bazı bağımlılıklar aksi takdirde izlenmez.

---

## 🔑 `checksum-freshness`

* **Tracking issue:** #14136

`-Z checksum-freshness`, Cargo’nun fingerprintlerinde dosya zaman damgaları (`mtime`) yerine **dosya checksum** değerini kullanmasını sağlar.

* Özellikle kötü `mtime` implementasyonuna sahip sistemler veya CI/CD ortamları için faydalıdır.
* Algoritma Cargo sürümleri arasında değişebilir.
* Build script tarafından işlenen dosyalar hâlâ `mtime` kullanır.

---

## 🚨 `panic-abort-tests`

* **Tracking Issue:** #67650
* **PR:** #7460

`-Z panic-abort-tests`, test harness crate’lerini `-Cpanic=abort` ile derlemeye izin verir.
Varsayılan olarak Cargo testleri `-Cpanic=unwind` ile derler.

`panic=abort`, testleri **test-per-process** modunda çalıştırır ve crate graph’ların birden fazla kez derlenmesini önlemeye yardımcı olur.

---

## ⚙️ `config-include`

* **Tracking Issue:** #7723
* Kullanım için `-Zconfig-include` bayrağı gerekir.

`config` dosyalarında başka config dosyaları dahil etmeye izin verir.

Örnek:

```toml
# tek dosya
include = "path/to/mordor.toml"

# birden fazla dosya
include = ["frodo.toml", "samwise.toml"]
```

* `.toml` uzantısı zorunludur.
* Dosyalar **soldan sağa** sırayla birleştirilir.
* İç içe (recursive) `include` desteklenir.

---

## 🎯 `target-applies-to-host`

* **PR:** #9322
* **Tracking Issue:** #9453

Cargo’nun host hedefleri (`build.rs`, plugin vb.) için `linker` ve `rustflags` kullanımındaki tutarsız davranışlarını düzenler.

* Varsayılan davranış: karışık ve kafa karıştırıcı.
* `-Ztarget-applies-to-host` → host derleme hedefleri için daha tutarlı yapılandırma sağlar.

Örnek `config.toml`:

```toml
target-applies-to-host = false
```

Kullanım:

```bash
cargo +nightly -Ztarget-applies-to-host build --target x86_64-unknown-linux-gnu
```

---

## 🖥️ `host-config`

* **PR:** #9322
* **Tracking Issue:** #9452

Config dosyalarında `host` anahtarı ile host build hedeflerine özel bayraklar tanımlamaya izin verir.

Örnek:

```toml
[host]
linker = "/path/to/host/linker"

[host.x86_64-unknown-linux-gnu]
linker = "/path/to/host/arch/linker"
rustflags = ["-Clink-arg=--verbose"]

[target.x86_64-unknown-linux-gnu]
linker = "/path/to/target/linker"
```

* `host.x86_64-unknown-linux-gnu` tablosu, genel `[host]` tablosuna göre önceliklidir.
* `-Zhost-config` ve `-Ztarget-applies-to-host` birlikte kullanılmalıdır.
* Bu ayar `target-applies-to-host` varsayılanını **true → false** olarak değiştirir.

Kullanım:

```bash
cargo +nightly -Ztarget-applies-to-host -Zhost-config build --target x86_64-unknown-linux-gnu
```
Her şeyi mükemmel bir şekilde özetlemişsin 👍
Bölümler başlıklarla ayrılmış, açıklamalar net, hem bağlantılar hem de örnek `toml` ve `bash` blokları doğru biçimde yerleştirilmiş.

Eğer dilersen aynı formatta diğer kararsız Cargo özelliklerini (`gc`, `open-namespaces`, `rustdoc-map`, `scrape-examples`, vb.) de ben senin için tek tek çevrilmiş ve düzenlenmiş şekilde devam ettirebilirim. İstiyor musun?

## ⏱️ `publish-timeout`

* **Tracking Issue:** #11222

`publish.timeout` anahtarı, `cargo publish` komutunun bir paketi kayıt sistemine gönderdikten sonra, yerel indekste erişilebilir olana kadar bekleyeceği süreyi belirler.

* Varsayılan: **60 saniye**
* `0` değeri, hiçbir kontrol yapılmamasını sağlar.
* `-Zpublish-timeout` bayrağı gerekir.

Örnek:

```toml
[publish]
timeout = 300  # saniye
```

---

## 🔐 `asymmetric-token`

* **Tracking Issue:** #10519
* **RFC:** #3231

`-Z asymmetric-token` bayrağı, `cargo:paseto` kimlik sağlayıcısını etkinleştirir. Bu, Cargo’nun ağ üzerinden gizli anahtar göndermeden kayıt sistemine kimlik doğrulaması yapmasını sağlar.

* `credentials.toml` veya `config.toml` içinde `private-key` alanı bulunur.
* Anahtar çifti oluşturma:

```bash
cargo login --generate-keypair
```

Bu:

1. Bir public/private anahtar çifti oluşturur.
2. Private anahtarı `credentials.toml` dosyasına kaydeder.
3. Public anahtarı **PASERK** formatında yazdırır.

Ek alan: `private-key-subject` → kayıt sistemi tarafından seçilen, gizli olmayan ASCII string.

Bir kayıt sisteminde **en fazla** bir `private-key` veya `token` olabilir.

---

## ⚙️ `cargo config`

* **Original Issue:** #2362
* **Tracking Issue:** #9301

Yeni `cargo config` alt komutu, Cargo’nun yüklediği yapılandırma dosyalarını görüntüler.

Örnek:

```bash
cargo +nightly -Zunstable-options config get build.rustflags
```

---

## 🖨️ `rustc --print`

* **Tracking Issue:** #9357

`cargo rustc --print=VAL`, bayrağı `rustc`’ye iletir ve derleme yapmadan bilgileri alır.

Ana kullanım:

```bash
cargo rustc --print=cfg
```

Hedefe özgü config değerlerini almak için kullanılır.

---

## 📝 `different-binary-name`

* **Tracking Issue:** #9778
* **PR:** #9627

Binary dosyasının adını crate ismine bağlı kalmadan belirlemenizi sağlar.

Örnek:

```toml
cargo-features = ["different-binary-name"]

[package]
name = "foo"
version = "0.0.1"

[[bin]]
name = "foo"
filename = "007bar"
path = "src/main.rs"
```

---

## 🔍 `scrape-examples`

* **RFC:** #3123
* **Tracking Issue:** #9910

`-Z rustdoc-scrape-examples` bayrağı, fonksiyon çağrılarını crate içinde arar ve belgelerde gösterir.

Örnek:

```bash
cargo doc -Z unstable-options -Z rustdoc-scrape-examples
```

`Cargo.toml` içinde:

```toml
[lib]
doc-scrape-examples = true

[[example]]
name = "my-example"
doc-scrape-examples = false
```

Not: Testlerden scraping şimdilik çalışmaz.

---

## 📤 `output-format` (rustdoc)

* **Tracking Issue:** #13283

Rustdoc çıktısının formatını belirler: `html` veya `json`.

Örnek:

```bash
cargo rustdoc -Z unstable-options --output-format json
```

---

## ⚡ `codegen-backend`

Rustc tarafından kullanılan codegen backend’i seçmeye izin verir.

Örnek:

```toml
[profile.dev.package.foo]
codegen-backend = "cranelift"
```

`.cargo/config.toml` içinde:

```toml
[unstable]
codegen-backend = true
```

---

## 🌀 `gitoxide`

* **Tracking Issue:** #11813

`gitoxide` crate’i kullanılarak git işlemleri yapılmasını sağlar.

* `-Zgitoxide` → tümünü etkinleştirir.
* `-Zgitoxide=fetch` → yalnızca fetch işlemleri gitoxide ile yapılır.

---

## 🌱 `git`

* **Tracking Issue:** #13285

Hem `gitoxide` hem de `git2` tarafından **shallow fetch** desteği sağlar.

Geçerli işlemler:

* `shallow-index`
* `shallow-deps`

Örnek:

```bash
cargo +nightly -Zgit=shallow-deps
cargo +nightly -Zgit=shallow-index
```

---

## 📜 `script`

* **Tracking Issue:** #12207

Tek dosyalık `.rs` paketlerini çalıştırmaya izin verir.

Örnek:

```bash
cargo +nightly -Zscript file.rs
```

Manifest gömme:

```rust
#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
clap = { version = "4.2", features = ["derive"] }
---
```

---

## 🛡️ `trim-paths`

* **Tracking Issue:** #12137
* **Rustc Issue:** #111540

Derleme çıktılarındaki dosya yollarının anonimleştirilmesini (sanitize) sağlar.

Örnek:

```toml
cargo-features = ["trim-paths"]

[profile.release]
trim-paths = ["diagnostics", "object"]
```

Varsayılan:

* `dev` profili → `none`
* `release` profili → `object`

Ayrıca ortam değişkeni: `CARGO_TRIM_PATHS`

---

## 🗑️ `gc`

* **Tracking Issue:** #12633

Cargo’nun global cache temizliği için kullanılır.

Config örneği:

```toml
[cache.global-clean]
max-src-age = "1 month"
max-crate-age = "3 months"
max-index-age = "3 months"
max-git-co-age = "1 month"
max-git-db-age = "3 months"
```

Manuel temizlik:

```bash
cargo clean gc -Zgc
cargo clean gc -Zgc --max-download-age=1week
cargo clean gc -Zgc --max-git-size=0 --max-download-size=100MB
```

---

## 🌐 `open-namespaces`

* **Tracking Issue:** #13576

Birden fazla paketin aynı API namespace içinde çalışmasına izin verir.

```toml
cargo-features = ["open-namespaces"]
```

---

## 🔎 `[lints.cargo]`

* **Tracking Issue:** #12235

Cargo için lint yapılandırmaları eklenmiştir.

Örnek:

```toml
[lints.cargo]
implicit-features = "warn"
```

---

## 📂 Path Bases

* **Tracking Issue:** #14355

Bağımlılıklar için `path base` tanımlamaya izin verir.

Örnek:

```toml
cargo-features = ["path-bases"]

[dependencies]
foo = { base = "dev", path = "foo" }

[path-bases]
dev = "/home/user/dev/rust/libraries/"
```

Sonuç: `/home/user/dev/rust/libraries/foo`

---

## 🔒 `lockfile-path`

* **Original Issue:** #5707
* **Tracking Issue:** #14421

Lockfile’ın (`Cargo.lock`) yolunu değiştirmeye izin verir.

Örnek:

```bash
cargo +nightly metadata --lockfile-path=$LOCKFILES_ROOT/my-project/Cargo.lock -Z unstable-options
```
## 📦 `package-workspace`

* **Tracking Issue:** #10948

Bir workspace içindeki birden fazla crate’in aynı anda paketlenmesine veya yayınlanmasına izin verir.

Örnek:

```bash
cargo +nightly -Zpackage-workspace package -p foo -p dep
cargo +nightly -Zpackage-workspace publish -p foo -p dep
```

Workspace’de yalnızca `foo` ve `dep` varsa, tek tek crate isimleri yerine `--workspace` kullanılabilir:

```bash
cargo +nightly -Zpackage-workspace package --workspace
cargo +nightly -Zpackage-workspace publish --workspace
```

**Lockfile davranışı:**

* Bir binary crate ile bağımlılığı aynı anda paketlenirse, lockfile bağımlılığı sanki registry’de yayınlanmış gibi işaretler.
* Cargo, bağımlılığın yayınlanacağı registry’yi `publish` alanından anlamaya çalışır.
* Açıkça belirtmek için `--registry` veya `--index` kullanılabilir:

```bash
cargo +nightly -Zpackage-workspace --registry=my-registry package -p foo -p dep
cargo +nightly -Zpackage-workspace --index=https://example.com package -p foo -p dep
```

---

## 🖥️ `native-completions`

* **Original Issue:** #6645
* **Tracking Issue:** #14520

Elle yazılmış shell tamamlama betikleri yerine native Rust tabanlı tamamlama sağlar.

Kullanım:

* **bash**

  ```bash
  source <(CARGO_COMPLETE=bash cargo +nightly)
  ```

* **zsh**

  ```bash
  source <(CARGO_COMPLETE=zsh cargo +nightly)
  ```

* **fish**

  ```bash
  source (CARGO_COMPLETE=fish cargo +nightly | psub)
  ```

* **elvish**

  ```bash
  eval (E:CARGO_COMPLETE=elvish cargo +nightly | slurp)
  ```

* **powershell**

  ```powershell
  CARGO_COMPLETE=powershell cargo +nightly | Invoke-Expression
  ```

---

## ⚠️ `warnings`

* **Original Issue:** #8424
* **Tracking Issue:** #14802

`-Z warnings` özelliği `build.warnings` yapılandırma seçeneğini etkinleştirir.

Varsayılan: `warn`

```toml
[build]
warnings = "deny"
```

Geçerli değerler:

* `warn` → uyarılar gösterilir (varsayılan)
* `allow` → uyarılar gizlenir
* `deny` → uyarılar hata olarak değerlendirilir

Ortam değişkeni: `CARGO_BUILD_WARNINGS`

---

## 🔗 `feature-unification`

* **RFC:** #3692
* **Tracking Issue:** #14774

Workspace içindeki özelliklerin nasıl birleştirileceğini (`resolver.feature-unification`) kontrol eder.

Varsayılan: `"selected"`

Değerler:

* `"selected"` → yalnızca seçilen paketler için özellikler birleştirilir.
* `"workspace"` → tüm workspace üyeleri arasında özellikler birleştirilir.
* `"package"` (henüz uygulanmadı) → özellikler paket bazlı değerlendirilir.

Ortam değişkeni: `CARGO_RESOLVER_FEATURE_UNIFICATION`

---

## 📨 `package message format`

* **Original Issue:** #11666
* **Tracking Issue:** #15353

`cargo package` komutundaki `--message-format` çıktının formatını belirler. Şimdilik yalnızca `--list` ile çalışır.

```bash
cargo +nightly package --list --message-format json -Z unstable-options
```

---

## 📑 `rustdoc depinfo`

* **Original Issue:** #12266
* **Tracking Issue:** #15370

`-Z rustdoc-depinfo`, rustdoc’un `dep-info` dosyalarını kullanarak belgelerin yeniden oluşturulup oluşturulmayacağını belirler.

`-Z checksum-freshness` ile birlikte kullanılabilir.

---

## 📉 `no-embed-metadata`

* **PR:** #15378
* **Tracking Issue:** #15495

Rust varsayılan olarak crate metadata’sını `rlib` ve `dylib` dosyalarına gömer. Bu bayrak, metadata’yı yalnızca `.rmeta` dosyalarında saklar.

```bash
cargo +nightly -Zno-embed-metadata build
```

---

## 📖 `unstable-editions`

Henüz stabil olmayan edition’ların kullanılmasına izin verir.

```toml
cargo-features = ["unstable-editions"]

[package]
name = "my-package"
edition = "future"
```

* `"future"` edition daima kararsızdır ve yeni özellikleri test etmek için vardır.

---

## 🛠️ `fix-edition`

`-Zfix-edition` → edition geçişlerini test etmek için kalıcı olarak kararsız bir bayraktır.

Örnek:

```bash
cargo +nightly fix -Zfix-edition=end=2024,future
```

---

## ✅ Stabilize Edilmiş ve Kaldırılmış Özellikler

* **compile-progress** → 1.30’da stabilize
* **edition** → 1.31’de stabilize
* **rename-dependency** → 1.31’de stabilize
* **alternate-registries** → 1.34’de stabilize
* **offline mode** → 1.36’da stabilize
* **publish-lockfile** → 1.37’de kaldırıldı
* **default-run** → 1.37’de stabilize
* **cache-messages** → 1.40’da stabilize
* **install-upgrade** → 1.41’de stabilize
* **profile overrides** → 1.41’de stabilize
* **config profiles** → 1.43’de stabilize
* **crate-versions** → 1.47’de stabilize
* **features / package-features / resolver v2** → 1.51’de stabilize
* **extra-link-arg** → 1.56’da stabilize
* **configurable-env** → 1.56’da stabilize
* **rust-version** → 1.56’da stabilize
* **patch-in-config** → 1.56’da stabilize
* **edition 2021** → 1.56’da stabilize
* **custom named profiles** → 1.57’de stabilize
* **profile strip option** → 1.59’da stabilize
* **future incompat report** → 1.59’da stabilize
* **namespaced features** → 1.60’da stabilize
* **weak dependency features** → 1.60’da stabilize
* **timings** (`--timings`) → 1.60’da stabilize
* **config-cli** (`--config`) → 1.63’de stabilize
* **multitarget** → 1.64’de stabilize
* **crate-type** (`cargo rustc --crate-type`) → 1.64’de stabilize
* **workspace inheritance** → 1.64’de stabilize
* **terminal-width** → 1.68’de stabilize
* **sparse-registry** → 1.68’de stabilize
* **cargo logout** → 1.70’de stabilize
* **doctest-in-workspace** → 1.72’de stabilize
* **keep-going** → 1.74’de stabilize
* **\[lints]** → 1.74’de stabilize
* **credential-process / registry-auth** → 1.74’de stabilize
* **check-cfg** → 1.80’de stabilize
* **edition 2024** → 1.85’de stabilize
* **automatic garbage collection** → 1.88’de stabilize
* **doctest-xcompile** → 1.89’dan itibaren varsayılan

---

## ⏳ `compile-time-deps`

Kalıcı olarak kararsızdır.
Sadece **proc-macro** ve **build script**’leri (ve bağımlılıklarını) derler.

Amaç: `rust-analyzer` gibi araçların performansını artırmak.

Örnek:

```bash
cargo +nightly build --compile-time-deps -Z unstable-options
cargo +nightly check --compile-time-deps --all-targets -Z unstable-options
```
