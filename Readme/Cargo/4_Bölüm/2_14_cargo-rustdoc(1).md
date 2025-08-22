## 📋 cargo-rustdoc(1)

### 🏷️ İSİM (NAME)

cargo-rustdoc — Belirtilen özel bayrakları (custom flags) kullanarak bir paketin dokümantasyonunu oluşturur

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo rustdoc [options] [-- args]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Geçerli paket (veya `-p` ile belirtilen paket) belirtilen `args` parametreleri `rustdoc` çağrısına eklenerek belgelenir. Bağımlılıklar bu komut kapsamında belgelenmez. `rustdoc` yine de koşulsuz olarak `-L`, `--extern`, `--crate-type` gibi argümanları alır; verilen `args` bunlara eklenir.

`rustdoc` bayrakları hakkında bilgi için bakınız:
[https://doc.rust-lang.org/rustdoc/index.html](https://doc.rust-lang.org/rustdoc/index.html)

Bu komut, yalnızca tek bir hedef derlenirken ek argüman verilmesine izin verir. Birden fazla hedef mevcutsa, `--lib`, `--bin` gibi filtreler kullanılmalıdır.

Tüm `rustdoc` işlemlerine bayrak geçirmek için `RUSTDOCFLAGS` ortam değişkeni veya `build.rustdocflags` yapılandırma değeri kullanılabilir.

---

### ⚙️ SEÇENEKLER (OPTIONS)

#### 📖 Dokümantasyon Seçenekleri (Documentation Options)

`--open`
Derleme sonrası belgeleri varsayılan tarayıcıda açar. Tarayıcı `BROWSER` ortam değişkeni veya `doc.browser` yapılandırma seçeneği ile değiştirilebilir.

#### 📦 Paket Seçimi (Package Selection)

Varsayılan olarak, geçerli çalışma dizinindeki paket seçilir. `-p` bayrağı başka bir paketi seçmek için kullanılabilir.

`-p spec`
`--package spec`
Belgelenecek paketi belirtir. SPEC formatı için `cargo-pkgid(1)`’e bakınız.

#### 🎯 Hedef Seçimi (Target Selection)

Hiçbir hedef seçilmezse, paketin tüm ikili ve kütüphane hedefleri belgelenir.
Aynı ada sahip olan kütüphane ile ikililer atlanır. Eksik `required-features` olan ikililer de atlanır.

`--lib` → Paketin kütüphanesini belgeler
`--bin name…` → Belirtilen ikiliyi belgeler
`--bins` → Tüm ikili hedefleri belgeler
`--example name…` → Belirtilen örneği belgeler
`--examples` → Tüm örnekleri belgeler
`--test name…` → Belirtilen entegrasyon testini belgeler
`--tests` → Tüm test hedeflerini belgeler
`--bench name…` → Belirtilen benchmark’ı belgeler
`--benches` → Tüm benchmark hedeflerini belgeler
`--all-targets` → `--lib --bins --tests --benches --examples` ile eşdeğerdir

> Not: `--bin`, `--example`, `--test`, `--bench` bayrakları Unix glob desenlerini (`*`, `?`, `[]`) destekler. Shell’in bunları Cargo’dan önce genişletmemesi için tek veya çift tırnak kullanılmalıdır.

#### 🔑 Özellik Seçimi (Feature Selection)

`-F features` / `--features features` → Etkinleştirilecek özellikleri listeler
`--all-features` → Tüm özellikleri etkinleştirir
`--no-default-features` → Varsayılan özellikleri devre dışı bırakır

#### 🛠️ Derleme Seçenekleri (Compilation Options)

`--target triple` → Belirtilen mimari için belgeler
`-r` / `--release` → `release` profili ile belgeler
`--profile name` → Belirtilen profil ile belgeler
`--timings=fmts` → Derleme süreleri ve eşzamanlılık bilgilerini raporlar (`html`, `json`)

#### 📂 Çıktı Seçenekleri (Output Options)

`--target-dir directory` → Tüm çıktılar için dizin belirtir

#### 👁️ Görüntüleme Seçenekleri (Display Options)

`-v`, `--verbose` → Ayrıntılı çıktı
`-q`, `--quiet` → Sessiz mod
`--color when` → Renk kontrolü: `auto`, `always`, `never`
`--message-format fmt` → Teşhis mesaj formatı (`human`, `short`, `json` vb.)

#### 📜 Manifest Seçenekleri (Manifest Options)

`--manifest-path path` → `Cargo.toml` yolunu belirtir
`--ignore-rust-version` → `rust-version` belirtimini yok sayar
`--locked` → `Cargo.lock` dosyasındaki tam sürümleri zorunlu kılar
`--offline` → Ağa erişimi engeller
`--frozen` → `--locked` ve `--offline` ile aynı anda eşdeğerdir
`--lockfile-path PATH` → Kilit dosyası yolunu değiştirir (yalnızca nightly)

#### ⚙️ Ortak Seçenekler (Common Options)

`+toolchain` → Rustup ile belirli toolchain seçer
`--config KEY=VALUE or PATH` → Yapılandırma değerlerini geçersiz kılar
`-C PATH` → Belirtilen dizine geçerek çalıştırır (yalnızca nightly)
`-h`, `--help` → Yardım bilgisini yazdırır
`-Z flag` → Deneysel bayraklar (yalnızca nightly)

#### 🔄 Çeşitli Seçenekler (Miscellaneous Options)

`-j N`, `--jobs N` → Paralel iş sayısını ayarlar
`--keep-going` → Başarısız olsa bile mümkün olan tüm bağımlılıkları derlemeye devam eder
`--output-format` → Belgelerin çıktı türü:

* `html` (varsayılan)
* `json` (deneysel, yalnızca nightly)

---

### 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

### 🚪 ÇIKIŞ DURUMU (EXIT STATUS)

`0`: Cargo başarıyla tamamlandı
`101`: Cargo başarısız oldu

---

### 📚 ÖRNEKLER (EXAMPLES)

Özel bir CSS dosyasıyla dokümantasyon oluşturun:

```
cargo rustdoc --lib -- --extend-css extra.css
```

👉 Bu komut, `extra.css` dosyasını ekleyerek belgeleri oluşturur.

---

### 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-doc(1)`, `rustdoc(1)`
