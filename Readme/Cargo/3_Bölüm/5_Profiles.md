## 📂 Profiller (profiles)

Profiller, derleyici (compiler) ayarlarını değiştirmenin bir yolunu sağlar; optimizasyonlar ve hata ayıklama (debugging) sembolleri gibi şeyleri etkiler.

Cargo’nun 4 yerleşik profili vardır: `dev`, `release`, `test` ve `bench`. Komut satırında bir profil belirtilmezse, hangi komutun çalıştırıldığına bağlı olarak profil otomatik olarak seçilir. Yerleşik profillere ek olarak, kullanıcı tanımlı özel profiller de belirtilebilir.

Profil ayarları `Cargo.toml` dosyasında `[profile]` tablosu ile değiştirilebilir. Her adlandırılmış profilin içinde, ayarlar anahtar/değer (key/value) çiftleri ile değiştirilir:

```toml
[profile.dev]
opt-level = 1               # Biraz daha iyi optimizasyonlar kullan.
overflow-checks = false     # Tam sayı taşma kontrollerini kapat.
```

Cargo yalnızca çalışma alanının (workspace) kökündeki `Cargo.toml` içindeki profil ayarlarını dikkate alır. Bağımlılıklarda tanımlanan profil ayarları yok sayılır.

Ayrıca profiller, yapılandırma (config) tanımlarından da geçersiz kılınabilir. Bir yapılandırma dosyasında veya ortam değişkeninde profil belirtilmesi, `Cargo.toml` içindeki ayarları geçersiz kılar.

---

## ⚙️ Profil ayarları (profile settings)

### 🔧 `opt-level`

`opt-level` ayarı, derleyiciye `-C opt-level` bayrağını iletir ve optimizasyon seviyesini belirler. Yüksek optimizasyon seviyeleri, çalışma zamanı kodunu hızlandırabilir ancak derleme süresini uzatır.

Geçerli seçenekler:

* `0`: optimizasyon yok
* `1`: temel optimizasyonlar
* `2`: bazı optimizasyonlar
* `3`: tüm optimizasyonlar
* `"s"`: ikili dosya boyutu için optimize et
* `"z"`: ikili dosya boyutu için optimize et, ayrıca döngü vektörleştirmeyi kapat

> Projeye en uygun dengeyi bulmak için farklı seviyelerle deneme yapılması önerilir.

---

### 🐞 `debug`

`debug` ayarı, `-C debuginfo` bayrağını kontrol eder ve derlenen ikiliye dahil edilen hata ayıklama bilgisini belirler.

Geçerli seçenekler:

* `0`, `false`, `"none"` → hiç hata ayıklama bilgisi yok (varsayılan: release)
* `"line-directives-only"` → yalnızca satır bilgisi yönergeleri
* `"line-tables-only"` → geri izlemeler için minimum satır bilgisi
* `1`, `"limited"` → sınırlı debug bilgisi
* `2`, `true`, `"full"` → tam hata ayıklama bilgisi (varsayılan: dev)

MSRV: `1.71` sürümü gereklidir.

---

### 📂 `split-debuginfo`

`-C split-debuginfo` bayrağını kontrol eder; hata ayıklama bilgisinin çalıştırılabilir dosyaya mı yoksa yanına mı yerleştirileceğini belirler.

Varsayılan değer platforma bağlıdır ve macOS üzerinde `unpacked` olur.

---

### ✂️ `strip`

`strip`, `-C strip` bayrağını kontrol eder. İkili dosyadan semboller veya hata ayıklama bilgisi kaldırılabilir.

Örnek:

```toml
[profile.release]
strip = "debuginfo"
```

Geçerli seçenekler:

* `"none"` (varsayılan)
* `"debuginfo"`
* `"symbols"`

Ayrıca `true` (symbols) ve `false` (none) kullanılabilir.

---

### ✅ `debug-assertions`

`-C debug-assertions` bayrağını kontrol eder. Bu ayar, `debug_assert!` makrosunu etkinleştirir.

* `true`: etkin
* `false`: devre dışı

---

### 🔢 `overflow-checks`

`-C overflow-checks` bayrağını kontrol eder.

* `true`: taşma durumunda `panic` oluşur
* `false`: kapalı

---

### 🔗 `lto` (Link Time Optimization)

LLVM’nin bağlantı zamanı optimizasyonlarını kontrol eder.

Seçenekler:

* `false`: yalnızca yerel crate üzerinde “thin local LTO”
* `true` veya `"fat"`: tüm bağımlılık grafiğinde tam LTO
* `"thin"`: daha hızlı çalışan “thin LTO”
* `"off"`: LTO kapalı

---

### 🚨 `panic`

`-C panic` bayrağını kontrol eder.

Seçenekler:

* `"unwind"`: panic durumunda stack unwinding yapılır
* `"abort"`: panic durumunda işlem sonlandırılır

---

### ⚡ `incremental`

`-C incremental` bayrağını kontrol eder. Artırımlı derlemeyi etkinleştirir.

* `true`: etkin
* `false`: devre dışı

Artırımlı derleme yalnızca workspace üyeleri ve `path` bağımlılıkları için geçerlidir.

---

### 🧩 `codegen-units`

`-C codegen-units` bayrağını kontrol eder. Bir crate’in kaç kod üretim birimine (code generation units) bölüneceğini belirler.

Varsayılan:

* `256` (artırımlı derlemeler için)
* `16` (normal derlemeler için)

---

### 📍 `rpath`

`-C rpath` bayrağını kontrol eder. rpath’in etkin olup olmadığını belirler.

---

## 🛠️ Varsayılan profiller (default profiles)

### 🔨 `dev` profili

Varsayılan geliştirme ve debug profili.

```toml
[profile.dev]
opt-level = 0
debug = true
split-debuginfo = '...'  # Platforma özgü
strip = "none"
debug-assertions = true
overflow-checks = true
lto = false
panic = 'unwind'
incremental = true
codegen-units = 256
rpath = false
```

---

### 🚀 `release` profili

Üretim için optimize edilmiş derlemeler.

```toml
[profile.release]
opt-level = 3
debug = false
split-debuginfo = '...'  # Platforma özgü
strip = "none"
debug-assertions = false
overflow-checks = false
lto = false
panic = 'unwind'
incremental = false
codegen-units = 16
rpath = false
```

---

### 🧪 `test` profili

`cargo test` için varsayılan profil. `dev` profilinden miras alır.

---

### 📊 `bench` profili

`cargo bench` için varsayılan profil. `release` profilinden miras alır.

---

## 🧱 Derleme bağımlılıkları (build dependencies)

Varsayılan olarak, tüm profiller derleme bağımlılıklarını optimize etmez.

```toml
[profile.dev.build-override]
opt-level = 0
codegen-units = 256
debug = false # mümkünse
```

```toml
[profile.release.build-override]
opt-level = 0
codegen-units = 256
```

---

## 🎨 Özel profiller (custom profiles)

Yerleşik profillere ek olarak özel profiller de tanımlanabilir.

Örnek:

```toml
[profile.release-lto]
inherits = "release"
lto = true
```

Kullanım:

```bash
cargo build --profile release-lto
```

---

## 📌 Profil seçimi (profile selection)

Komuta göre varsayılan profiller:

| Komut                          | Varsayılan Profil |
| ------------------------------ | ----------------- |
| cargo run, build, check, rustc | dev               |
| cargo test                     | test              |
| cargo bench                    | bench             |
| cargo install                  | release           |

---

## 🎯 Geçersiz kılmalar (overrides)

Belirli paketler için ayarlar geçersiz kılınabilir.

Örnek:

```toml
[profile.dev.package.foo]
opt-level = 3
```

Tüm bağımlılıklar için:

```toml
[profile.dev.package."*"]
opt-level = 2
```

Yapı bağımlılıkları için:

```toml
[profile.dev.build-override]
opt-level = 3
```

Öncelik sırası:

1. `[profile.dev.package.name]`
2. `[profile.dev.package."*"]`
3. `[profile.dev.build-override]`
4. `[profile.dev]`
5. Cargo’nun varsayılan değerleri

---

## 🧮 Generikler ve geçersiz kılmalar

Generic kodun nerede somutlaştırıldığı, optimizasyon ayarlarını etkiler. Bu nedenle, bir bağımlılığın optimizasyon seviyesini yükseltmek her zaman beklenen performans artışını sağlamayabilir.

---

İstersen bunu daha görsel bir tablo haline getirebilirim (örn. `opt-level`, `debug`, `lto` gibi ayarların yan yana karşılaştırması). İstiyor musun?
