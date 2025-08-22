## ⚙️ Yapılandırma (configuration)

Bu belge, Cargo’nun yapılandırma sisteminin nasıl çalıştığını ve mevcut anahtarları (keys) veya ayarları açıklar. Bir paketin manifestosu (manifest) üzerinden yapılandırılması için manifest formatına bakınız.

---

## 🏗️ Hiyerarşik yapı (hierarchical structure)

Cargo, belirli bir paket için yerel yapılandırmaya (local configuration) ve küresel yapılandırmaya (global configuration) izin verir. Mevcut dizinde ve tüm üst dizinlerde yapılandırma dosyalarını arar.

Örneğin, Cargo `/projects/foo/bar/baz` içinde çağrıldığında şu dosyaları bu sırayla arar ve birleştirir:

* `/projects/foo/bar/baz/.cargo/config.toml`
* `/projects/foo/bar/.cargo/config.toml`
* `/projects/foo/.cargo/config.toml`
* `/projects/.cargo/config.toml`
* `/.cargo/config.toml`
* `$CARGO_HOME/config.toml` (varsayılan olarak):

  * Windows: `%USERPROFILE%\.cargo\config.toml`
  * Unix: `$HOME/.cargo/config.toml`

Bu yapı ile paket başına yapılandırma belirtebilir, hatta bunu sürüm kontrolüne dahil edebilirsiniz. Ayrıca kendi varsayılanlarınızı ev dizininizdeki yapılandırma dosyası ile belirleyebilirsiniz.

Bir anahtar birden fazla dosyada belirtilmişse değerler birleştirilir. Sayılar, dizeler ve boolean değerler için daha derin dizindeki dosya önceliklidir. Diziler ise birleştirilir, yüksek öncelikli öğeler daha sona eklenir.

Not: Cargo, `.toml` uzantısı olmayan `config` dosyalarını da okur. `.toml` desteği 1.39 sürümünde eklenmiştir ve tercih edilen biçimdir. İkisi birden varsa uzantısız dosya kullanılır.

---

## 📑 Yapılandırma formatı (configuration format)

Yapılandırma dosyaları, manifest gibi **TOML formatında** yazılır. Bölümler (tables) altında basit anahtar-değer çiftleri içerir.

Örnek genel ayarlar:

```toml
paths = ["/path/to/override"] # yol bağımlılığı geçersiz kılmaları

[alias]     # komut kısayolları
b = "build"
c = "check"
t = "test"
r = "run"
rr = "run --release"
recursive_example = "rr --example recursions"
space_example = ["run", "--release", "--", "\"command list\""]

[build]
jobs = 1                      # paralel iş sayısı (varsayılan: CPU sayısı)
rustc = "rustc"               # Rust derleyici aracı
rustc-wrapper = "…"           # rustc yerine çalıştırılacak wrapper
rustc-workspace-wrapper = "…" # workspace üyeleri için wrapper
rustdoc = "rustdoc"           # dokümantasyon üretici
target = "triple"             # hedef triple
target-dir = "target"         # derleme çıktıları konumu
rustflags = ["…", "…"]        # tüm derleyici çağrılarına ek bayraklar
rustdocflags = ["…", "…"]     # rustdoc için ek bayraklar
incremental = true            # artırımlı derleme
dep-info-basedir = "…"        # depfile taban dizini
```

Bu mantıkla tüm tablolar:

* `[credential-alias]` → kimlik bilgisi sağlayıcı alias’ları
* `[doc]` → `cargo doc --open` için tarayıcı
* `[env]` → ortam değişkenleri
* `[future-incompat-report]` → gelecekteki uyumsuzluk raporları
* `[cache]` → önbellek temizleme sıklığı
* `[cargo-new]` → VCS seçimi (git, hg, fossil, none vs.)
* `[http]` → HTTP ayarları (proxy, TLS sürümü, timeout vb.)
* `[install]` → `cargo install` hedef dizini
* `[net]` ve `[net.ssh]` → ağ ayarları ve SSH host key’leri
* `[patch.<registry>]` → registry yamaları
* `[profile.<name>]` → profil ayarları (örn. dev, release)
* `[resolver]` → çözümleyici ayarları
* `[registries.<name>]` → crates.io dışı registry’ler
* `[registry]` → varsayılan registry
* `[source.<name>]` → kaynak tanımı veya değiştirme
* `[target.<triple>]` → belirli target için derleyici/çalıştırıcı ayarları
* `[term]` → terminal çıktı ayarları (renk, progress bar, unicode, vs.)

---

## 🌍 Ortam değişkenleri (environment variables)

Cargo ayrıca **ortam değişkenleri** ile de yapılandırılabilir.

Her yapılandırma anahtarı `foo.bar` → `CARGO_FOO_BAR` biçimine dönüştürülür.

* Noktalar (`.`) ve tireler (`-`) alt çizgiye (`_`) çevrilir.
* Anahtar büyük harfe dönüştürülür.

Örneğin:
`target.x86_64-unknown-linux-gnu.runner` →
`CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER`

Öncelik sırası:

1. Komut satırı `--config` ile verilenler
2. Ortam değişkenleri
3. TOML dosyaları

---

## 💻 Komut satırı geçersiz kılmaları (command-line overrides)

Cargo, `--config` bayrağı ile ek yapılandırma alır.

```bash
# KEY=VALUE ile
cargo --config net.git-fetch-with-cli=true fetch

# dosya ile
cargo --config ./path/to/my/extra-config.toml fetch
```

Birden fazla `--config` belirtilebilir, soldan sağa birleştirilir.

Örnekler:

```bash
# Proxy ayarı
cargo --config http.proxy=\"http://example.com\" …

# Boşluk kullanımı
cargo --config "net.git-fetch-with-cli = true" …

# TOML array
cargo --config 'build.rustdocflags = ["--html-in-header", "header.html"]' …

# Karmaşık TOML anahtarı
cargo --config "target.'cfg(all(target_arch = \"arm\", target_os = \"none\"))'.runner = 'my-runner'" …

# Bir profil ayarını geçersiz kılma
cargo --config profile.dev.package.image.opt-level=3 …
```

---

İstersen ben sana bu yapılandırma tablolarının (örn. `[build]`, `[http]`, `[env]`) hepsini tek tek ayrıntılı açıklayarak Türkçe liste halinde açabilirim. İstiyor musun?

## 📂 Yapılandırma ile İlgili Yollar (config-relative paths)

Yapılandırma dosyalarındaki yollar (paths) **mutlak (absolute)**, **göreli (relative)** veya **ayrıştırıcı (path separator) olmadan yalın isimler (bare name)** olabilir.

* Ayrıştırıcı içermeyen çalıştırılabilir dosya yolları `PATH` ortam değişkeni kullanılarak aranır.
* Çalıştırılabilir olmayan yollar, yapılandırma değerinin tanımlandığı yere göre yorumlanır.

Kurallar:

* **Ortam değişkenlerinde (environment variables)** → yollar mevcut çalışma dizinine göre ayarlanır.
* **`--config KEY=VALUE` ile doğrudan yüklenen ayarlarda** → yollar mevcut çalışma dizinine göre ayarlanır.
* **Yapılandırma dosyalarında** → yollar `.cargo/config.toml` dosyasının bulunduğu dizinin bir üstüne göre ayarlanır.

Not: Tutarlılık için, `--config <path>` ile belirtilen dosyalardaki yollar da aynı şekilde dosyanın iki seviye üstüne göre değerlendirilir.

Öneri: Ek yapılandırma dosyalarını, projenizde keşfedilen `.cargo/config.toml` ile aynı seviyeye koyun.
Örneğin `/my/project` projesinde:

* `/my/project/.cargo` altında
* veya `/my/project/.config` altında

---

### 📌 Göreli yol örnekleri

```toml
[target.x86_64-unknown-linux-gnu]
runner = "foo"  # `PATH` içinde foo aranır.

[source.vendored-sources]
# Dizin, `.cargo/config.toml` dosyasının bulunduğu yerin üstüne göre hesaplanır.
# Örn: `/my/project/.cargo/config.toml` → `/my/project/vendor`
directory = "vendor"
```

---

## ⚙️ Argümanlı Çalıştırılabilir Yollar (executable paths with arguments)

Bazı Cargo komutları harici programlar çağırır. Bunlar:

* **Dizi (array) biçiminde** belirtilebilir:
  `['/path/to/program', 'somearg']`
* **Boşlukla ayrılmış string** olarak belirtilebilir:
  `'/path/to/program somearg'`

Eğer yol boşluk içeriyorsa **dizi biçimi zorunludur**.

Cargo kendi argümanlarını, sizin tanımladığınız son argümandan sonra ekler.

---

## 🔑 Kimlik Bilgileri (credentials)

Hassas bilgileri içeren ayarlar `$CARGO_HOME/credentials.toml` içinde tutulur.

* Bu dosya `cargo login` ve `cargo logout` komutlarıyla otomatik olarak güncellenir.
* `cargo publish` gibi komutlarda uzak registry ile kimlik doğrulaması için kullanılır.

Örnek:

```toml
[registry]
token = "…"   # crates.io erişim token'ı

[registries.<name>]
token = "…"   # belirli registry için erişim token'ı
```

Ortam değişkeni ile de belirtilebilir:

* `CARGO_REGISTRY_TOKEN` → crates.io için
* `CARGO_REGISTRIES_<NAME>_TOKEN` → diğer registry’ler için

Not: `.cargo/credentials` uzantısız dosya da okunur/yazılır.

* `.toml` desteği 1.39 sürümünde eklenmiştir.
* 1.68’den itibaren varsayılan olarak `.toml` dosyası yazılır.
* Geriye dönük uyumluluk için, her ikisi varsa Cargo uzantısız dosyayı kullanır.

---

## 📑 Yapılandırma Anahtarları (configuration keys)

### 🔗 `paths`

* Tür: dizi (array)
* Varsayılan: yok
* Ortam: desteklenmiyor
* Yerel paket yollarını bağımlılıkların üzerine yazmak için kullanılır.

---

### 🏷️ `[alias]`

* Tür: string veya string dizisi
* Ortam: `CARGO_ALIAS_<name>`
* CLI komutlarına kısayol tanımlar.

Örnek:

```toml
[alias]
b = "build"
c = "check"
t = "test"
r = "run"
rr = "run --release"
recursive_example = "rr --example recursions"
```

> Alias’lar özyinelemelidir (recursive).
> Yerleşik komutlar yeniden tanımlanamaz.

---

### 🛠️ `[build]`

Derleme zamanı işlemleri ve derleyici ayarlarını kontrol eder.

* `build.jobs` → paralel derleyici işlemleri (default: CPU sayısı)
* `build.rustc` → kullanılacak `rustc` programı
* `build.rustc-wrapper` → `rustc` yerine çalıştırılacak wrapper
* `build.rustc-workspace-wrapper` → yalnızca workspace üyeleri için wrapper
* `build.rustdoc` → kullanılacak `rustdoc` programı
* `build.target` → hedef platform triple listesi
* `build.target-dir` → çıktı dizini (default: `target`)
* `build.rustflags` → `rustc` için ek bayraklar
* `build.rustdocflags` → `rustdoc` için ek bayraklar
* `build.incremental` → artırımlı derleme
* `build.dep-info-basedir` → bağımlılık bilgi dosyalarındaki taban dizin ayarı

---

### 🗝️ `[credential-alias]`

Kimlik bilgisi sağlayıcı alias’ları tanımlar.

Örnek:

```toml
[credential-alias]
my-alias = ["/usr/bin/cargo-credential-example", "--argument", "value", "--flag"]
```

---

### 📚 `[doc]`

`cargo doc` için ayarlar.

* `doc.browser` → tarayıcı programı

---

### 🌱 `[cargo-new]`

`cargo new` komutunun varsayılanlarını tanımlar.

* `cargo-new.vcs` → kullanılacak VCS (`git`, `hg`, `pijul`, `fossil`, `none`)

---

### 🌍 `[env]`

Ek ortam değişkenleri tanımlar.

Örnek:

```toml
[env]
OPENSSL_DIR = "/opt/openssl"
TMPDIR = { value = "/home/tmp", force = true }
OPENSSL_DIR = { value = "vendor/openssl", relative = true }
```

---

### 🚨 `[future-incompat-report]`

Gelecekteki uyumsuzluk raporlarını kontrol eder.

* `frequency = "always"` → her zaman bildir
* `frequency = "never"` → asla bildirme

---

İstersen buradan devam edip `[http]`, `[install]`, `[net]` gibi kalan tüm config tablolarını tek tek Türkçe açıklayabilirim. İstiyor musun?
## 🗄️ \[cache] – Önbellek (cache)

`[cache]` tablosu Cargo’nun önbellek ayarlarını tanımlar.

### 🌍 Küresel önbellek (global caches)

Cargo, küresel önbellekte kullanılan dosyaları otomatik olarak takip eder ve belirli bir süre kullanılmayanları siler:

* **Ağdan indirilen dosyalar** → 3 ay kullanılmazsa silinir.
* **Ağ erişimi olmadan yeniden üretilebilen dosyalar** → 1 ay kullanılmazsa silinir.

Bu silme işlemi yalnızca **zaten yoğun iş yapan komutlarla** (örn. `cargo build`, `cargo test`, `cargo check`, `cargo fetch`) tetiklenir.

Eğer Cargo çevrimdışı (`--offline`, `--frozen`) çalıştırılırsa silme işlemi devre dışıdır.

Not: Şu anda bu takip yalnızca Cargo’nun ev dizinindeki küresel önbellek için geçerlidir (registry index’leri, registry’den indirilen kaynak dosyalar, git bağımlılıkları). Derleme çıktılarının takibi henüz uygulanmamıştır.

### 🔄 `cache.auto-clean-frequency`

* Tür: string
* Varsayılan: `"1 day"`
* Ortam: `CARGO_CACHE_AUTO_CLEAN_FREQUENCY`
* Kullanılmayan dosyaların ne sıklıkla silinmesi gerektiğini belirler.

Seçenekler:

* `"never"` → asla silme
* `"always"` → her Cargo çalıştırıldığında kontrol et
* Belirli zaman dilimi → `"30 days"`, `"2 weeks"`, `"12 hours"` gibi

---

## 🌐 \[http] – HTTP

`[http]` tablosu, Cargo’nun HTTP davranışlarını kontrol eder (crate bağımlılıklarının indirilmesi, uzak git repo erişimleri).

* **`http.debug`** → HTTP isteklerini debug modunda çalıştırır (`CARGO_LOG=network=debug`).
* **`http.proxy`** → proxy sunucusu (`protocol://host:port`).
* **`http.timeout`** → her HTTP isteği için zaman aşımı (saniye, varsayılan: 30).
* **`http.cainfo`** → TLS sertifika doğrulaması için CA bundle dosyası.
* **`http.check-revoke`** → TLS sertifika iptal kontrolü (yalnızca Windows).
* **`http.ssl-version`** → TLS sürüm aralığı (örn. `tlsv1.1`–`tlsv1.3`).
* **`http.low-speed-limit`** → yavaş bağlantı zaman aşımı (default: 10 B/s).
* **`http.multiplexing`** → HTTP/2 çoklama (varsayılan: true).
* **`http.user-agent`** → HTTP User-Agent başlığı (default: Cargo sürümü).

---

## 📦 \[install] – Kurulum (install)

`[install]` tablosu `cargo install` komutu için varsayılanları tanımlar.

* **`install.root`** → yükleme kök dizini (`bin/` altına çalıştırılabilirler eklenir).

  * Varsayılan: Cargo’nun ev dizini (`$HOME/.cargo`).
  * CLI ile `--root` bayrağıyla değiştirilebilir.

---

## 🌍 \[net] – Ağ (network)

`[net]` tablosu ağ yapılandırmasını kontrol eder.

* **`net.retry`** → ağ hatalarında tekrar deneme sayısı (varsayılan: 3).
* **`net.git-fetch-with-cli`** → `git` komutunu kullanarak fetch işlemi (true ise).
* **`net.offline`** → çevrimdışı çalış (yalnızca yerel önbellek).

### 🔑 \[net.ssh]

* **`net.ssh.known-hosts`** → kabul edilecek SSH host anahtarları listesi.
* OpenSSH `known_hosts` formatında olmalıdır.

---

## 🩹 \[patch] – Yama (patch)

`Cargo.toml` içindeki `[patch]` bölümü gibi çalışır. Bağımlılıkları geçici olarak değiştirir.

* Tercihen `Cargo.toml` içinde tanımlanmalı, çünkü `.cargo/config.toml` genelde sürüm kontrolüne dahil edilmez.
* Aynı bağımlılık hem `Cargo.toml` hem config dosyasında yamalanmışsa **config dosyası önceliklidir**.

---

## 📂 \[profile] – Profiller

Config dosyasında `[profile]` ile tüm profiller global olarak değiştirilebilir.

Alt tablolar:

* **`[profile.<name>.build-override]`** → build-script ve proc-macro ayarları için.
* **`[profile.<name>.package.<name>]`** → belirli bir paket için ayarlar (panic, lto, rpath hariç).

Desteklenen ayarlar:

* `codegen-units`, `debug`, `split-debuginfo`, `debug-assertions`, `incremental`, `lto`, `overflow-checks`, `opt-level`, `panic`, `rpath`, `strip`

---

## 🔎 \[resolver] – Çözücü (resolver)

Yerel geliştirme sırasında bağımlılık çözümlemesini kontrol eder.

* **`resolver.incompatible-rust-versions`**

  * `"allow"` → uyumsuz sürümleri normal kabul et
  * `"fallback"` → yalnızca başka sürüm bulunamazsa kullan

MSRV:

* `"allow"` → her sürümde
* `"fallback"` → 1.84’ten itibaren

---

## 📚 \[registries] – Registry’ler

Ek registry’ler tanımlanır.

* **`registries.<name>.index`** → registry index URL’si
* **`registries.<name>.token`** → registry erişim token’ı
* **`registries.<name>.credential-provider`** → kimlik bilgisi sağlayıcı

Özel crates.io ayarı:

* **`registries.crates-io.protocol`** → `"git"` veya `"sparse"` (varsayılan: `"sparse"`)

---

## 🏷️ \[registry] – Varsayılan registry

Varsayılan registry ayarları:

* **`registry.default`** → kullanılacak varsayılan registry adı
* **`registry.credential-provider`** → crates.io kimlik bilgisi sağlayıcı
* **`registry.token`** → crates.io erişim token’ı
* **`registry.global-credential-providers`** → küresel kimlik bilgisi sağlayıcı listesi

---

Burada durdum çünkü sırada `[source]`, `[target]`, `[term]` bölümleri var.
İstersen sana devamında **\[source]**, **\[target]** ve **\[term]** yapılandırmalarını da Türkçeye çevirebilirim. Onları da çevirmemi ister misin?

## 📦 \[source] – Kaynaklar (sources)

`[source]` tablosu, kullanılabilir **registry kaynaklarını** tanımlar. Her kaynak için alt tablo oluşturulur. Bir kaynak yalnızca tek bir tür tanımlamalıdır: `directory`, `registry`, `local-registry` veya `git`.

* **`source.<name>.replace-with`**

  * Tür: string
  * Varsayılan: none
  * Bu kaynağı başka bir kaynak veya registry ile değiştirir.

* **`source.<name>.directory`**

  * Tür: string (path)
  * Yerel dizin kaynağı yolu.

* **`source.<name>.registry`**

  * Tür: string (url)
  * Registry kaynağı için URL.

* **`source.<name>.local-registry`**

  * Tür: string (path)
  * Yerel registry dizini yolu.

* **`source.<name>.git`**

  * Tür: string (url)
  * Git deposu URL’si.

* **`source.<name>.branch`**

  * Tür: string
  * Git deposu için kullanılacak branch adı.

* **`source.<name>.tag`**

  * Tür: string
  * Git deposu için kullanılacak tag adı.

* **`source.<name>.rev`**

  * Tür: string
  * Git deposu için kullanılacak belirli revizyon.

> Eğer `branch`, `tag` veya `rev` belirtilmezse varsayılan **master branch** kullanılır.

---

## 🎯 \[target] – Hedefler (targets)

`[target]` tablosu belirli platform hedefleri için ayarları tanımlar. Alt tablo olarak ya bir platform **triple** ya da bir **cfg() ifadesi** kullanılır.

Örnek:

```toml
[target.thumbv7m-none-eabi]
linker = "arm-none-eabi-gcc"
runner = "my-emulator"
rustflags = ["…", "…"]

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "my-arm-wrapper"
rustflags = ["…", "…"]
```

### 🔑 Ayarlar

* **`target.<triple>.linker`**

  * Tür: string (program path)
  * `-C linker` ile `rustc`’ye iletilen bağlayıcı (linker).
  * Ortam: `CARGO_TARGET_<triple>_LINKER`

* **`target.<triple>.runner`**

  * Tür: string veya string dizisi (program + argümanlar)
  * Bu runner, derlenen yürütülebilirleri çalıştırır (`cargo run`, `cargo test`, `cargo bench`).
  * Ortam: `CARGO_TARGET_<triple>_RUNNER`

* **`target.<triple>.rustflags`**

  * Tür: string veya string dizisi
  * Bu hedef için `rustc`’ye gönderilecek özel bayraklar.
  * Ortam: `CARGO_TARGET_<triple>_RUSTFLAGS`

* **`target.<triple>.rustdocflags`**

  * Tür: string veya string dizisi
  * Bu hedef için `rustdoc`’a gönderilecek özel bayraklar.
  * Ortam: `CARGO_TARGET_<triple>_RUSTDOCFLAGS`

* **`target.<triple>.<links>`**

  * Belirli bir `links` kütüphanesi için build-script’i geçersiz kılar.

Örnek:

```toml
[target.x86_64-unknown-linux-gnu.foo]
rustc-link-lib = ["foo"]
rustc-link-search = ["/path/to/foo"]
rustc-flags = "-L /some/path"
rustc-cfg = ['key="value"']
rustc-env = { key = "value" }
rustc-cdylib-link-arg = ["…"]
metadata_key1 = "value"
metadata_key2 = "value"
```

---

## 🖥️ \[term] – Terminal (terminal output)

`[term]` tablosu Cargo’nun terminal çıktısını ve etkileşimini kontrol eder.

* **`term.quiet`**

  * Tür: boolean
  * Varsayılan: false
  * Cargo’nun log mesajlarını bastırır.
  * Ortam: `CARGO_TERM_QUIET`

* **`term.verbose`**

  * Tür: boolean
  * Varsayılan: false
  * Ayrıntılı log mesajlarını etkinleştirir.
  * Ortam: `CARGO_TERM_VERBOSE`

* **`term.color`**

  * Tür: string
  * Varsayılan: `"auto"`
  * Renkli çıktı ayarı: `"auto"`, `"always"`, `"never"`
  * Ortam: `CARGO_TERM_COLOR`

* **`term.hyperlinks`**

  * Tür: bool
  * Varsayılan: otomatik
  * Terminal çıktısında hyperlink desteği.
  * Ortam: `CARGO_TERM_HYPERLINKS`

* **`term.unicode`**

  * Tür: bool
  * Varsayılan: otomatik
  * Unicode karakter desteği.
  * Ortam: `CARGO_TERM_UNICODE`

* **`term.progress.when`**

  * Tür: string
  * Varsayılan: `"auto"`
  * İlerleme çubuğu gösterimi: `"auto"`, `"always"`, `"never"`
  * Ortam: `CARGO_TERM_PROGRESS_WHEN`

* **`term.progress.width`**

  * Tür: integer
  * İlerleme çubuğu genişliği.
  * Ortam: `CARGO_TERM_PROGRESS_WIDTH`

* **`term.progress.term-integration`**

  * Tür: bool
  * Varsayılan: otomatik
  * Terminal emülatörüne ilerleme durumunu raporlar (örn. görev çubuğu).
  * Ortam: `CARGO_TERM_PROGRESS_TERM_INTEGRATION`

---

Böylece **Cargo yapılandırma tablolarının (\[cache] → \[http] → \[install] → \[net] → \[patch] → \[profile] → \[resolver] → \[registries] → \[registry] → \[source] → \[target] → \[term])** tamamını Türkçe çevirmiş olduk.

İstersen şimdi sana tüm bu yapılandırma anahtarlarını **tek bir özet tablo** halinde çıkarabilirim (ör. sütunlar: *Anahtar*, *Tür*, *Varsayılan*, *Ortam değişkeni*). Bunu yapmamı ister misin?
