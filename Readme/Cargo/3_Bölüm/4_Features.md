## ⚙️ Özellikler (features)

Cargo “özellikler” (features), koşullu derleme (conditional compilation) ve isteğe bağlı bağımlılıkları (optional dependencies) ifade etmenin bir yolunu sağlar. Bir paket, `Cargo.toml` dosyasındaki `[features]` tablosunda adlandırılmış özellikler tanımlar ve her özellik etkinleştirilebilir veya devre dışı bırakılabilir. Derlenen paket için özellikler komut satırında `--features` gibi bayraklarla etkinleştirilebilir. Bağımlılıkların özellikleri ise `Cargo.toml` içindeki bağımlılık bildiriminde etkinleştirilebilir.

Not: crates.io üzerinde yayımlanan yeni crate’ler veya sürümler artık en fazla 300 özelliğe sahip olabilir. İstisnalar duruma göre değerlendirilir. Ayrıntılar için blog yazısına bakınız. Çözüm tartışmalarına crates.io Zulip akışı üzerinden katılım teşvik edilmektedir.

Ayrıca özelliklerin nasıl kullanılabileceğine dair örnekler için **Features Examples** bölümüne bakınız.

---

## 📑 \[features] bölümü

Özellikler, `Cargo.toml` içindeki `[features]` tablosunda tanımlanır. Her özellik, etkinleştirdiği diğer özelliklerin veya isteğe bağlı bağımlılıkların bir dizisini belirtir. Aşağıdaki örnekler, farklı görüntü formatlarını isteğe bağlı olarak dahil edebilen bir 2D görüntü işleme kütüphanesi için özelliklerin nasıl kullanılabileceğini göstermektedir:

```toml
[features]
# `webp` adında herhangi bir özelliği etkinleştirmeyen bir özellik tanımlar.
webp = []
```

Bu özellik tanımlandığında, `cfg` ifadeleri derleme zamanında istenilen özelliği destekleyen kodu koşullu olarak dahil etmek için kullanılabilir. Örneğin, paketin `lib.rs` dosyasında şu kod bulunabilir:

```rust
// Bu, WEBP desteğini uygulayan bir modülü koşullu olarak dahil eder.
#[cfg(feature = "webp")]
pub mod webp;
```

Cargo, özellikleri `rustc --cfg` bayrağı ile ayarlar ve kodda bunların varlığı `cfg` özniteliği veya `cfg` makrosu ile test edilebilir.

Özellikler, etkinleştirilmesi gereken diğer özellikleri listeleyebilir. Örneğin, ICO görüntü formatı BMP ve PNG resimleri içerebilir, bu nedenle etkinleştirildiğinde bu diğer özelliklerin de etkinleştirildiğinden emin olunmalıdır:

```toml
[features]
bmp = []
png = []
ico = ["bmp", "png"]
webp = []
```

Özellik adları Unicode XID standardındaki karakterleri (çoğu harf) içerebilir, ayrıca `_` veya `0`–`9` rakamları ile başlayabilir, ilk karakterden sonra ise `-`, `+` veya `.` içerebilir.

Not: crates.io ek kısıtlamalar uygular; özellik adları yalnızca ASCII alfasayısal karakterler veya `_`, `-`, `+` içerebilir.

---

## ⭐ Varsayılan özellik (default feature)

Varsayılan olarak, tüm özellikler açıkça etkinleştirilmedikçe devre dışıdır. Bu, `default` özelliği belirtilerek değiştirilebilir:

```toml
[features]
default = ["ico", "webp"]
bmp = []
png = []
ico = ["bmp", "png"]
webp = []
```

Paket derlendiğinde, `default` özelliği etkinleştirilir ve bu da listelenen özellikleri etkinleştirir. Bu davranış şu yollarla değiştirilebilir:

* `--no-default-features` komut satırı bayrağı, paketin varsayılan özelliklerini devre dışı bırakır.
* `default-features = false` seçeneği bağımlılık bildiriminde belirtilebilir.

Not: Varsayılan özellik kümesini seçerken dikkatli olun. Varsayılan özellikler, yaygın kullanım için kullanıcıyı hangi özellikleri etkinleştireceğini dikkatle seçmeye zorlamadan paketi kullanmayı kolaylaştırır. Ancak bağımlılıklar otomatik olarak varsayılan özellikleri etkinleştirir (aksi belirtilmezse). Bu, özellikle bağımlılık grafiğinde birden fazla kez görünen bir bağımlılık için varsayılan özelliklerin devre dışı bırakıldığından emin olmayı zorlaştırır. Her paket, `default-features = false` belirtmelidir.

Başka bir sorun da, varsayılan kümeden bir özelliği kaldırmanın SemVer uyumsuz bir değişiklik olabilmesidir. Bu nedenle o özellikleri koruyacağınızdan emin olmalısınız.

---

## 📦 İsteğe bağlı bağımlılıklar (optional dependencies)

Bağımlılıklar “isteğe bağlı” olarak işaretlenebilir; bu durumda varsayılan olarak derlenmezler. Örneğin, 2D görüntü işleme kütüphanemizin GIF görüntülerini işlemek için harici bir paket kullandığını varsayalım:

```toml
[dependencies]
gif = { version = "0.11.1", optional = true }
```

Varsayılan olarak, bu isteğe bağlı bağımlılık şu şekilde örtük (implicit) bir özellik tanımlar:

```toml
[features]
gif = ["dep:gif"]
```

Bu, bağımlılığın yalnızca `gif` özelliği etkinleştirilirse dahil edileceği anlamına gelir. Aynı `cfg(feature = "gif")` sözdizimi kodda kullanılabilir ve bağımlılık `--features gif` ile etkinleştirilebilir.

Bazı durumlarda, isteğe bağlı bağımlılıkla aynı ada sahip bir özelliği açığa çıkarmak istemeyebilirsiniz. Bu durumda `[features]` tablosunda `dep:` öneki kullanıldığında örtük özellik devre dışı bırakılır.

Not: `dep:` sözdizimi yalnızca Rust 1.60 ve sonrasında kullanılabilir.

Örneğin, AVIF formatı için iki bağımlılığın etkinleştirilmesi gerekiyorsa:

```toml
[dependencies]
ravif = { version = "0.6.3", optional = true }
rgb = { version = "0.8.25", optional = true }

[features]
avif = ["dep:ravif", "dep:rgb"]
```

Bu durumda `avif` özelliği her iki bağımlılığı da etkinleştirir. Ayrıca `ravif` ve `rgb` için örtük özellikler oluşturulmaz.

Not: Bağımlılığı isteğe bağlı olarak dahil etmenin başka bir yolu da platforma özgü bağımlılıklardır.

---

## 🔗 Bağımlılık özellikleri (dependency features)

Bağımlılıkların özellikleri bağımlılık bildiriminde etkinleştirilebilir. `features` anahtarı, hangi özelliklerin etkinleştirileceğini belirtir:

```toml
[dependencies]
# serde'nin `derive` özelliğini etkinleştirir.
serde = { version = "1.0.118", features = ["derive"] }
```

Varsayılan özellikler `default-features = false` ile devre dışı bırakılabilir:

```toml
[dependencies]
flate2 = { version = "1.0.3", default-features = false, features = ["zlib-rs"] }
```

Not: Bu, varsayılan özelliklerin devre dışı bırakılmasını garanti etmeyebilir. Başka bir bağımlılık `flate2`’yi `default-features = false` olmadan dahil ederse varsayılan özellikler etkinleştirilecektir.

Bağımlılıkların özellikleri `[features]` tablosunda da etkinleştirilebilir. Sözdizimi `"paket-adı/özellik-adı"` şeklindedir:

```toml
[dependencies]
jpeg-decoder = { version = "0.1.20", default-features = false }

[features]
# jpeg-decoder’in `rayon` özelliğini etkinleştirerek paralel işlemeyi açar.
parallel = ["jpeg-decoder/rayon"]
```

Eğer isteğe bağlı bağımlılık ise `paket-adı?/özellik-adı` biçiminde `?` eklenebilir; bu yalnızca bağımlılık başka bir şekilde etkinleştirilmişse özelliği açar.

Not: `?` sözdizimi Rust 1.60 ile kullanılabilir hale gelmiştir.

---

## 💻 Komut satırı özellik seçenekleri (command-line feature options)

Aşağıdaki komut satırı bayrakları hangi özelliklerin etkinleştirileceğini kontrol eder:

* `--features FEATURES`: Listelenen özellikleri etkinleştirir. Birden fazla özellik virgül veya boşluk ile ayrılabilir. Boşluk kullanılıyorsa tırnak işareti gerekir (`--features "foo bar"`).
* `--all-features`: Komut satırında seçilen tüm paketlerin tüm özelliklerini etkinleştirir.
* `--no-default-features`: Seçilen paketlerin varsayılan özelliklerini etkinleştirmez.

---

## 🔄 Özellik birleşimi (feature unification)

Bir bağımlılık birden fazla paket tarafından kullanıldığında, Cargo bu bağımlılık için tüm etkin özelliklerin birleşimini (union) kullanır. Bu, bağımlılığın tek bir kopyasının kullanılmasını sağlar.

Özellikler eklemeli (additive) olmalıdır: bir özelliği etkinleştirmek işlevselliği devre dışı bırakmamalı ve herhangi bir kombinasyon güvenli olmalıdır.

Örneğin, `no_std` ortamlarını isteğe bağlı desteklemek için `no_std` özelliği tanımlamak yerine `std` özelliğini tanımlamak daha doğrudur:

```rust
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
pub fn function_that_requires_std() {
    // ...
}
```

---

## ⚔️ Birbirini dışlayan özellikler (mutually exclusive features)

Nadiren de olsa, bazı özellikler birbirleriyle uyumsuz olabilir. Bu durumdan mümkün olduğunca kaçınılmalıdır. Kaçınılamıyorsa, derleme hatası eklenebilir:

```rust
#[cfg(all(feature = "foo", feature = "bar"))]
compile_error!("feature \"foo\" and feature \"bar\" cannot be enabled at the same time");
```

Alternatifler:

* İşlevselliği ayrı paketlere bölmek
* Çakışma durumunda bir özelliği diğerine tercih etmek (`cfg-if` paketi kullanılabilir)
* Her iki özelliği aynı anda açılabilir hale getirmek, çalıştırma zamanı seçenekleri ile kontrol etmek

---

## 🌳 Çözümlenen özellikleri inceleme (inspecting resolved features)

Karmaşık bağımlılık grafiklerinde hangi özelliklerin nasıl etkinleştiğini anlamak zor olabilir. `cargo tree` komutu şu seçenekleri sunar:

* `cargo tree -e features`: Özellikleri bağımlılık grafiğinde gösterir.
* `cargo tree -f "{p} {f}"`: Paket başına etkin özellikleri virgülle ayrılmış biçimde gösterir.
* `cargo tree -e features -i foo`: Belirtilen paket için hangi yollarla özelliklerin etkinleştirildiğini gösterir.

---

## 🧩 Çözümleyici sürümü 2 (resolver version 2)

`Cargo.toml` içinde şu şekilde belirtilebilir:

```toml
[package]
name = "my-package"
version = "1.0.0"
resolver = "2"
```

Bu çözümleyici, bazı durumlarda özellik birleşimini engeller (ör. platforma özgü bağımlılıklar, build-dependencies, dev-dependencies). Ancak dezavantajı, aynı bağımlılığın farklı özelliklerle birden fazla kez derlenmesine yol açabilmesidir. `cargo tree --duplicates` ile tespit edilebilir.

---

## 🛠️ Build script’ler ve özellikler

Build script’ler, hangi özelliklerin etkin olduğunu `CARGO_FEATURE_<NAME>` ortam değişkenleri ile kontrol edebilir. (`<NAME>` büyük harf, `-` → `_` dönüşümü ile)

---

## ✅ Gerekli özellikler (required features)

`required-features` alanı, bir özellik etkin değilse belirli Cargo hedeflerini devre dışı bırakmak için kullanılabilir.

---

## 📐 SemVer uyumluluğu

Bir özelliği etkinleştirmek **SemVer uyumsuzluğu** getirmemelidir. Örneğin, mevcut API’yi kıracak şekilde değiştirmemelidir. Güvenli kabul edilen değişiklikler:

* Yeni bir özellik veya isteğe bağlı bağımlılık eklemek
* Bir bağımlılık üzerinde kullanılan özellikleri değiştirmek

Tehlikeli değişiklikler:

* Bir özelliği kaldırmak
* Mevcut kamuya açık kodu bir özelliğin arkasına taşımak
* Bir özellik listesinden özelliği çıkarmak

---

## 📖 Özellik belgeleri ve keşif (feature documentation and discovery)

Paketinizde hangi özelliklerin mevcut olduğunu belgelendirmeniz önerilir (örneğin `lib.rs` başına doc yorumları ekleyerek). Belgeler `docs.rs` üzerinde özelliklerin etkinleştirilmesiyle oluşturulabilir. Rustdoc ayrıca `doc_cfg` ile özellik gereksinimlerini API seviyesinde gösterebilir.

---

## 🧪 Özellik kombinasyonları (feature combinations)

Özellikler koşullu derleme olduğundan, %100 kapsama için üstel sayıda yapılandırma/test gerekir. Varsayılan olarak testler, belgeler ve Clippy yalnızca varsayılan özellik seti ile çalıştırılır. Stratejinizi, zaman ve kaynak durumunuza göre belirlemeniz önerilir.

Yaygın kombinasyonlar:

* Varsayılan özelliklerle / olmadan
* Belirli özellik kombinasyonları
* Tüm özelliklerin etkin olduğu durumlar
