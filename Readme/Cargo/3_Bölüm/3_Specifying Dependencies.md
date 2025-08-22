## 📦 Bağımlılık Belirtme (specifying dependencies)

Crate’leriniz, **crates.io** veya diğer kayıt defterlerindeki (registry) kütüphanelere, git depolarına ya da yerel dosya sisteminizdeki alt dizinlere bağımlı olabilir. Ayrıca, bir bağımlılığın konumunu geçici olarak geçersiz kılabilirsiniz — örneğin, üzerinde yerel olarak çalıştığınız bir hata düzeltmesini test etmek için. Farklı platformlar için farklı bağımlılıklar tanımlayabilir ve yalnızca geliştirme sırasında kullanılan bağımlılıklara sahip olabilirsiniz. Şimdi her birini nasıl yapacağımıza bakalım.

---

## 📥 crates.io’dan Bağımlılık Belirtme

Cargo, varsayılan olarak bağımlılıkları **crates.io** üzerinde arayacak şekilde yapılandırılmıştır. Bu durumda yalnızca ad ve sürüm (version) dizesi yeterlidir. Cargo kılavuzunda `time` crate’ine bağımlılığı şu şekilde belirtmiştik:

```toml
[dependencies]
time = "0.1.12"
```

Buradaki `"0.1.12"` ifadesi bir **sürüm gereksinimi** (version requirement) olarak adlandırılır. Bu, bağımlılık çözülürken seçilebilecek sürüm aralığını belirtir. Bu durumda `"0.1.12"`, `>=0.1.12, <0.2.0` aralığını temsil eder. Güncelleme, yalnızca bu aralık içindeyse yapılabilir. Örneğin, `cargo update time` çalıştırırsak, Cargo bizi `0.1.z` serisindeki en son sürüm olan `0.1.13` sürümüne yükseltir, ancak `0.2.0` sürümüne yükseltmez.

---

## 🔢 Sürüm Gereksinimi Söz Dizimi (version requirement syntax)

### ✅ Varsayılan gereksinimler (default requirements)

Varsayılan gereksinimler, minimum bir sürümü ve SemVer uyumlu sürümlere güncelleme imkânını belirtir.
Sürümler, en soldaki sıfır olmayan major/minor/patch bileşeni aynıysa uyumlu kabul edilir.
(Not: Bu, tüm `1.0.0` öncesi paketleri uyumsuz kabul eden SemVer’den farklıdır.)

Örnekler:

```
1.2.3  := >=1.2.3, <2.0.0
1.2    := >=1.2.0, <2.0.0
1      := >=1.0.0, <2.0.0
0.2.3  := >=0.2.3, <0.3.0
0.2    := >=0.2.0, <0.3.0
0.0.3  := >=0.0.3, <0.0.4
0.0    := >=0.0.0, <0.1.0
0      := >=0.0.0, <1.0.0
```

---

### 🎯 Caret gereksinimleri (caret requirements)

Caret gereksinimleri varsayılan sürüm gereksinimi stratejisidir. Bu strateji, SemVer uyumlu güncellemelere izin verir. Başında `^` işareti ile belirtilir.

Örnek:

```
^1.2.3
```

Caret (`^`) bırakıldığında, bu kısaltılmış eşdeğer sözdizimidir. Varsayılan olarak caret gereksinimleri kullanılsa da, mümkün olduğunda sade sözdizimi tercih edilir.

```
log = "^1.2.3"   # eşdeğer
log = "1.2.3"
```

---

### 🔄 Tilde gereksinimleri (tilde requirements)

Tilde gereksinimleri, minimum sürümü ve sınırlı bir güncelleme aralığını belirtir.

* Eğer major, minor ve patch sürümü belirttiyseniz, yalnızca patch seviyesinde değişikliklere izin verilir.
* Eğer yalnızca major ve minor sürüm belirtilirse yine patch seviyesinde güncellemeler yapılır.
* Eğer yalnızca major sürüm belirtilirse, minor ve patch seviyesinde güncellemeler yapılabilir.

Örnekler:

```
~1.2.3  := >=1.2.3, <1.3.0
~1.2    := >=1.2.0, <1.3.0
~1      := >=1.0.0, <2.0.0
```

---

### 🌐 Joker gereksinimleri (wildcard requirements)

Joker gereksinimleri, joker karakterin (`*`) konumuna bağlı olarak herhangi bir sürüme izin verir.

Örnekler:

```
*      := >=0.0.0
1.*    := >=1.0.0, <2.0.0
1.2.*  := >=1.2.0, <1.3.0
```

Not: **crates.io** çıplak `*` sürümlerine izin vermez.

---

### ⚖️ Karşılaştırma gereksinimleri (comparison requirements)

Karşılaştırma gereksinimleri, manuel olarak sürüm aralığı veya tam sürüm belirtmeye izin verir.

Örnekler:

```
>= 1.2.0
> 1
< 2
= 1.2.3
```

---

### ➕ Çoklu sürüm gereksinimleri (multiple version requirements)

Birden fazla sürüm gereksinimi, virgül ile ayrılarak belirtilebilir.

Örnek:

```
>= 1.2, < 1.5
```

---

## 🧪 Ön sürümler (pre-releases)

(Devamında ön sürümler için kurallar anlatılacak.)

## 🧪 Ön Sürümler (pre-releases)

Sürüm gereksinimleri, **özellikle belirtilmedikçe** `1.0.0-alpha` gibi ön sürümleri kapsamaz.
Örneğin, `foo` paketinin `1.0.0-alpha` sürümü yayımlandıysa, `foo = "1.0"` ifadesi bu sürümü eşleştirmez ve hata döner. Ön sürüm açıkça belirtilmelidir, örneğin:

```toml
foo = "1.0.0-alpha"
```

Benzer şekilde, `cargo install` komutu da özellikle istenmediği sürece ön sürümleri yüklemez.

Cargo, **daha yeni ön sürümleri** otomatik olarak kullanmaya izin verir.
Örneğin, `foo = "1.0.0-alpha"` belirtildiğinde ve `1.0.0-beta` yayımlandığında, Cargo `beta` sürümüne güncellemeye izin verir. Ancak bu yalnızca **aynı ana sürüm için** geçerlidir:

* `foo = "1.0.0-alpha"` → `foo = "1.0.0-beta"` yükseltmesi yapılabilir.
* `foo = "1.0.0-alpha"` → `foo = "1.0.1-alpha"` yükseltmesi yapılamaz.

Ayrıca Cargo, ön sürümlerden **semver uyumlu kararlı sürümlere** otomatik geçiş yapar.
Örneğin:
`foo = "1.0.0-alpha"` → `foo = "1.0.0"` veya `foo = "1.2.0"` güncellemesine izin verir.

⚠️ Ön sürümlerin kararsız olabileceğini unutmayın.

* Bazı projeler, ön sürümler arasında geriye dönük uyumluluğu bozan değişiklikler yayımlayabilir.
* Kütüphaneniz ön sürüm değilse, ön sürüm bağımlılıkları kullanmanız önerilmez.
* `Cargo.lock` güncellemelerinde dikkatli olun, çünkü bir ön sürüm güncellemesi sorunlara neden olabilir.

---

## 🏷️ Sürüm Metadatası (version metadata)

`1.0.0+21AF26D3` gibi sürüm metadatası yok sayılır ve sürüm gereksinimlerinde kullanılmamalıdır.

👉 Tavsiye: Emin olmadığınızda, varsayılan sürüm gereksinim operatörünü kullanın.

Nadir durumlarda, **kamuya açık bağımlılık** (public dependency) içeren bir paket (örneğin `Id` gibi sürümler arasında değişmeyen basit bir tip kullanan) farklı, SemVer uyumsuz sürümleri destekleyebilir. Bu durumda aşağıdaki gibi sürüm aralıkları ilgi çekici olabilir:

```toml
">=0.4, <2"
```

Ancak bu, kullanıcıların hata almasına yol açabilir ve onların `cargo update` ile manuel sürüm seçmeleri gerekebilir. Çünkü Cargo, aynı bağımlılığın farklı sürümlerini seçebilir.

❌ Kaçınılması gereken üst sınırlandırmalar:

* `">=2.0, <2.4"`
* `"2.0.*"`
* `"~2.0"`

Bunlar, diğer paketlerin daha yeni sürüm gereksinimlerinden dolayı **çözümlenemeyen hatalara** yol açabilir. Daha uygun olan, sürümü `Cargo.lock` üzerinden kontrol etmektir.

Bunun önemsiz olabileceği veya faydaların maliyetlerden ağır bastığı bazı durumlar:

* Paketinizin kimseye bağımlı olmadığı durumlar (sadece `[[bin]]` içeriyorsa).
* Ön sürüm bir pakete bağımlıysanız ve kırıcı değişikliklerden kaçınmak istiyorsanız, `=1.2.3-alpha.3` gibi tam sürüm belirtimi gerekebilir.
* Bir kütüphane bir `proc-macro` yeniden dışa aktarıyorsa ve bu `proc-macro` yeniden dışa aktaran kütüphaneye kod üretiyorsa, `=1.2.3` gibi tam sürüm belirtimi gerekebilir.

---

## 📚 Diğer Kayıt Defterlerinden Bağımlılık Belirtme

Varsayılan `crates.io` dışında bir kayıt defterinden bağımlılık belirtmek için `registry` anahtarı kullanılır:

```toml
[dependencies]
some-crate = { version = "1.0", registry = "my-registry" }
```

Buradaki `my-registry`, `.cargo/config.toml` dosyasında yapılandırılmış kayıt defteri adıdır.
Daha fazla bilgi için `registries` belgelerine bakın.

⚠️ Not: `crates.io`, dış kayıt defterlerinden bağımlılık içeren paketlerin yayımlanmasına izin vermez.

---

## 🌐 Git Depolarından Bağımlılık Belirtme

Bir git deposunda bulunan kütüphaneye bağımlı olmak için, minimum gerekli bilgi `git` anahtarı ile deponun adresidir:

```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git" }
```

Cargo, depoyu indirir ve `Cargo.toml` dosyasını bulmak için dosya ağacını tarar. Örneğin `regex-lite` ve `regex-syntax`, aynı `rust-lang/regex` reposunun üyeleridir ve repo kök URL’si ile belirtilebilir:

```toml
regex-lite   = { git = "https://github.com/rust-lang/regex.git" }
regex-syntax = { git = "https://github.com/rust-lang/regex.git" }
```

⚠️ Bu kural **path bağımlılıkları** için geçerli değildir.

---

## 🔀 Commit Seçimi (choice of commit)

Eğer yalnızca repo URL’si belirtilirse, Cargo varsayılan olarak **ana dalın (default branch) en son commit’ini** kullanır.

Daha belirgin olmak için `rev`, `tag` veya `branch` anahtarları eklenebilir.

Örneğin, `next` adlı bir branch’in en son commit’i:

```toml
[dependencies]
regex = { git = "https://github.com/rust-lang/regex.git", branch = "next" }
```

* `rev` anahtarı → commit hash veya özel referans olabilir.
  Örn: `rev = "4c59b707"` veya `rev = "refs/pull/493/head"`.
* GitHub, her pull request için en son commit’e bir referans sağlar.

---

## 📂 Git Bağımlılığı Örnekleri

```toml
# .git uzantısı opsiyoneldir
regex = { git = "https://github.com/rust-lang/regex" }
regex = { git = "https://github.com/rust-lang/regex.git" }

# Etiketlenmiş commit
regex = { git = "https://github.com/rust-lang/regex.git", tag = "1.10.3" }

# SHA1 hash ile commit
regex = { git = "https://github.com/rust-lang/regex.git", rev = "0c0990399270277832fbb5b91a1fa118e6f63dba" }

# PR 493’ün HEAD commit’i
regex = { git = "https://github.com/rust-lang/regex.git", rev = "refs/pull/493/head" }
```

❌ Geçersiz örnekler:

```toml
# # sonrası commit ID’sini yok sayar ve uyarı üretir
regex = { git = "https://github.com/rust-lang/regex.git#4c59b70" }

# git ve path aynı anda kullanılamaz
regex = { git = "https://github.com/rust-lang/regex.git#4c59b70", path = "../regex" }
```

Cargo, git bağımlılıklarının commit’lerini eklenme anında `Cargo.lock` dosyasında kilitler ve yalnızca `cargo update` çalıştırıldığında güncellemeleri kontrol eder.

---

## 🔑 version Anahtarının Rolü

`version` anahtarı, paket ne olursa olsun bir kayıt defterinde mevcut olduğunu ima eder.

* Git bağımlılığı için hangi commit’in kullanılacağını etkilemez.
* Ancak Cargo, bağımlılığın `Cargo.toml` içindeki sürüm bilgisini kontrol eder.
* Eğer uyumsuzsa hata döner.

Örneğin:

```toml
[dependencies]
regex = { version = "1.10.3", git = "https://github.com/rust-lang/regex.git", branch = "next" }
```

Cargo, `next` branch’in HEAD commit’ini indirir ve crate’in sürümünü `"1.10.3"` ile uyumlu olup olmadığını kontrol eder.

Not: `version`, `git` ve `path` bağımsız konumlar olarak değerlendirilir.

⚠️ crates.io, dış bağımlılıklarla yayımlamaya izin vermez (ancak `dev-dependencies` yok sayılır).

---

## 🔒 Özel Git Depolarına Erişim

Özel git depolarına erişim için kimlik doğrulama bilgileri gerekir. Bunun için **Git Authentication** belgelerine bakın.

## 📂 Yol Bağımlılıklarını Belirtme (specifying path dependencies)

Zamanla, kılavuzdaki `hello_world` paketimiz oldukça büyüdü! Bu noktada, başkalarının da kullanabileceği ayrı bir crate’e ayırmak isteyebiliriz. Bunu yapmak için Cargo, genellikle tek bir depoda yaşayan alt-crate’ler olan **yol bağımlılıklarını (path dependencies)** destekler.

Örneğin, `hello_world` paketi içinde yeni bir crate oluşturalım:

```bash
# hello_world/ dizini içinde
$ cargo new hello_utils
```

Bu komut, içinde `Cargo.toml` ve `src` klasörü hazır olan `hello_utils` adlı yeni bir klasör oluşturur. Cargo’ya bunu bildirmek için `hello_world/Cargo.toml` dosyasını açıp `hello_utils`’i bağımlılıklara ekleyelim:

```toml
[dependencies]
hello_utils = { path = "hello_utils" }
```

Bu, Cargo’ya `hello_utils` adlı crate’in, bu `Cargo.toml` dosyasına göreceli olarak `hello_utils` klasöründe bulunduğunu söyler.

Bir sonraki `cargo build`, `hello_utils` ve onun bağımlılıklarını otomatik olarak derleyecektir.

---

## 🚫 Yerel Yol Geçişi Yok (no local path traversal)

Yerel yollar, bağımlılığın `Cargo.toml` dosyasının bulunduğu klasörü **doğrudan** işaret etmelidir. Git bağımlılıklarının aksine, Cargo yerel yolları taramaz.

Örneğin, `regex-lite` ve `regex-syntax` aynı repodaysa:

```toml
# git bağımlılıkları için repo kökü yeterlidir
[dependencies]
regex-lite   = { git = "https://github.com/rust-lang/regex.git" }
regex-syntax = { git = "https://github.com/rust-lang/regex.git" }

# path bağımlılıkları için tam yol verilmelidir
[dependencies]
regex-lite   = { path = "../regex/regex-lite" }
regex-syntax = { path = "../regex/regex-syntax" }
```

---

## 📦 Yayınlanan Crate’lerde Yerel Yollar (local paths in published crates)

Sadece `path` bağımlılığı kullanan crate’ler **crates.io**’da yayımlanamaz.

Eğer `hello_world` paketini yayımlamak istersek, `hello_utils` paketini ayrıca crates.io’da yayımlamamız ve sürümünü belirtmemiz gerekir:

```toml
[dependencies]
hello_utils = { path = "hello_utils", version = "0.1.0" }
```

Burada `path` ve `version` anahtarlarının birlikte kullanımı, **Birden Fazla Konum (Multiple locations)** bölümünde açıklanır.

⚠️ Not: `crates.io`, yalnızca `dev-dependencies` hariç olmak üzere, dış koddaki bağımlılıklarla yayımlamaya izin vermez.

---

## 🌍 Birden Fazla Konum (multiple locations)

Bir bağımlılığın hem sürümünü hem de `git` veya `path` konumunu belirtebilirsiniz.

* Yerel geliştirmede git veya path sürümü kullanılır.
* Yayımlandığında ise crates.io’daki sürüm kullanılır.

Diğer kombinasyonlara izin verilmez.

Örnekler:

```toml
[dependencies]
# Yerelde my-bitflags, crates.io’da sürüm 1.0
bitflags = { path = "my-bitflags", version = "1.0" }

# Yerelde git repo, crates.io’da sürüm 1.0
smallvec = { git = "https://github.com/servo/rust-smallvec.git", version = "1.0" }
```

⚠️ Eğer sürüm uyuşmazsa, Cargo derleme sırasında hata verir.

Bu yöntem, aynı çalışma alanında (workspace) bölünmüş bir kütüphaneyi geliştirmek için faydalıdır:

* Yerel geliştirmede `path` kullanılır.
* Yayımlandığında crates.io sürümü devreye girer.

---

## 💻 Platforma Özgü Bağımlılıklar (platform specific dependencies)

Platforma özel bağımlılıklar, Rust’taki `#[cfg]` sözdizimine benzer şekilde `target` bölümü altında tanımlanır:

```toml
[target.'cfg(windows)'.dependencies]
winhttp = "0.4.0"

[target.'cfg(unix)'.dependencies]
openssl = "1.0.1"

[target.'cfg(target_arch = "x86")'.dependencies]
native-i686 = { path = "native/i686" }

[target.'cfg(target_arch = "x86_64")'.dependencies]
native-x86_64 = { path = "native/x86_64" }
```

Desteklenen operatörler: `not`, `any`, `all`.

* Mevcut platform için hedefleri görmek:

  ```bash
  rustc --print=cfg
  ```
* Belirli bir hedef için (ör. Windows 64-bit):

  ```bash
  rustc --print=cfg --target=x86_64-pc-windows-msvc
  ```

⚠️ Ancak Rust kaynak kodunda kullanılabilen `cfg(feature = "...")` burada geçerli değildir. Bunun için `[features]` bölümü kullanılır:

```toml
[dependencies]
foo = { version = "1.0", optional = true }
bar = { version = "1.0", optional = true }

[features]
fancy-feature = ["foo", "bar"]
```

Aynı durum `cfg(debug_assertions)`, `cfg(test)` ve `cfg(proc_macro)` için de geçerlidir.

---

## 🎯 Özel Hedef Tanımları (custom target specifications)

Özel bir hedef tanımı (`--target foo/bar.json`) kullanıyorsanız, `.json` uzantısı olmadan taban adını kullanın:

```toml
[target.bar.dependencies]
winhttp = "0.4.0"

[target.my-special-i686-platform.dependencies]
openssl = "1.0.1"
native = { path = "native/i686" }
```

⚠️ Not: Özel hedef tanımları stable kanalında kullanılamaz.

---

## 🧪 Geliştirme Bağımlılıkları (development dependencies)

`Cargo.toml` içine `[dev-dependencies]` bölümü eklenebilir:

```toml
[dev-dependencies]
tempdir = "0.3"
```

* `dev-dependencies`, paket inşa edilirken **kullanılmaz**.
* Testler, örnekler ve benchmark’lar derlenirken kullanılır.
* Bu bağımlılıklar, bu pakete bağlı diğer paketlere aktarılmaz.

Hedefe özel geliştirme bağımlılıkları da mümkündür:

```toml
[target.'cfg(unix)'.dev-dependencies]
mio = "0.0.1"
```

⚠️ Bir paket yayımlandığında, yalnızca sürümü belirtilmiş `dev-dependencies` dahil edilir.

---

## 🏗️ Derleme Bağımlılıkları (build dependencies)

Build script’lerde kullanılacak bağımlılıklar `[build-dependencies]` bölümünde tanımlanır:

```toml
[build-dependencies]
cc = "1.0.3"
```

Hedefe özel build bağımlılıkları da mümkündür:

```toml
[target.'cfg(unix)'.build-dependencies]
cc = "1.0.3"
```

⚠️ Build script bağımlılıkları, normal `dependencies` veya `dev-dependencies` altında tanımlananlara erişemez.

* Paket ve build script bağımsız şekilde derlenir.
* Dolayısıyla bağımlılıkları da bağımsız tanımlanır.

---

## ⚙️ Özellik Seçimi (choosing features)

Bağımlı olduğunuz paket koşullu özellikler sunuyorsa, kullanılacak özellikler belirtilebilir:

```toml
[dependencies.awesome]
version = "1.3.5"
default-features = false # varsayılan özellikler dahil edilmez
features = ["secure-password", "civet"]
```

---

## 📝 Bağımlılıkları Yeniden Adlandırma (renaming dependencies)

Normalde `[dependencies]` içindeki anahtar, crate’in kodda kullandığınız adıyla eşleşir. Ancak:

* `foo as bar` yazmaktan kaçınmak,
* aynı crate’in birden fazla sürümüne bağımlı olmak,
* farklı kayıt defterlerinden aynı ada sahip crate’lere bağımlı olmak için yeniden adlandırma yapılabilir.

Bunun için `package` anahtarı kullanılır:

```toml
[package]
name = "mypackage"
version = "0.0.1"

[dependencies]
foo = "0.1"
bar = { git = "https://github.com/example/project.git", package = "foo" }
baz = { version = "0.1", registry = "custom", package = "foo" }
```

Bu durumda:

```rust
extern crate foo; // crates.io
extern crate bar; // git repo
extern crate baz; // custom registry
```

Hepsi kendi `Cargo.toml` dosyalarında `foo` paket adını kullansa da, biz `package` anahtarıyla Cargo’ya hangi paketi kastettiğimizi bildiriyoruz.

⚠️ Eğer opsiyonel bağımlılık yeniden adlandırılırsa:

```toml
[dependencies]
bar = { version = "0.1", package = "foo", optional = true }
```

* Aslında `foo` crate’ine bağımlıyız.
* Ancak crate içinde `bar` adlı bir özellik oluşur (feature adları, bağımlılığın yerel adıyla eşleşir).

Örneğin transitif bağımlılıklar etkinleştirilirken:

```toml
[features]
log-debug = ["bar/log-debug"] # 'foo/log-debug' hata olur!
```

---

## 🏢 Çalışma Alanından Bağımlılık Devralma (inheriting a dependency from a workspace)

Bağımlılıklar, çalışma alanındaki `[workspace.dependencies]` tablosunda tanımlanıp üye paketlerde `workspace = true` ile devralınabilir.

Ek anahtarlar:

* `optional` → kullanılabilir, fakat `[workspace.dependencies]` içinde tanımlanamaz.
* `features` → `[workspace.dependencies]` içindekilerle birleştirilir.

Diğer anahtarlar (`version`, `default-features` vs.) devralmada kullanılamaz.

Örnek:

```toml
[package]
name = "bar"
version = "0.2.0"

[dependencies]
regex = { workspace = true, features = ["unicode"] }

[build-dependencies]
cc.workspace = true

[dev-dependencies]
rand = { workspace = true, optional = true }
```
