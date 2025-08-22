## 📋 cargo-rustc(1)

### 🏷️ İSİM (NAME)

cargo-rustc — Geçerli paketi derler ve derleyiciye (compiler) ek seçenekler iletir

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo rustc [options] [-- args]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Geçerli paket (veya `-p` ile belirtilen paket) tüm bağımlılıklarıyla birlikte derlenir. Belirtilen `args` yalnızca son derleyici çağrısına iletilir, bağımlılıklara değil. Derleyici, `-L`, `--extern` ve `--crate-type` gibi argümanları yine de alır; verilen `args` sadece bunlara eklenir.

`rustc` bayrakları (flags) hakkında bilgi için bakınız:
[https://doc.rust-lang.org/rustc/index.html](https://doc.rust-lang.org/rustc/index.html)

Bu komut, yalnızca tek bir hedef derlenirken ek argüman sağlanmasına izin verir. Birden fazla hedef mevcutsa, `--lib`, `--bin` gibi filtreler kullanılmalıdır.

Tüm derleyici işlemlerine bayrak geçirmek için `RUSTFLAGS` ortam değişkeni veya `build.rustflags` yapılandırma değeri kullanılabilir.

---

### ⚙️ SEÇENEKLER (OPTIONS)

#### 📦 Paket Seçimi (Package Selection)

Varsayılan olarak geçerli çalışma dizinindeki paket seçilir. `-p` bayrağı farklı bir paketi seçmek için kullanılabilir.

`-p spec`
`--package spec`
Derlenecek paketi belirtir. SPEC formatı için `cargo-pkgid(1)`’e bakınız.

#### 🎯 Hedef Seçimi (Target Selection)

Hedef seçimi yapılmazsa, seçilen paketin tüm ikili ve kütüphane hedefleri derlenir.
Entegrasyon testi veya benchmark seçilmişse ikili hedefler otomatik olarak derlenir.

`--lib` → Kütüphaneyi derler
`--bin name…` → Belirtilen ikiliyi derler (birden çok kez kullanılabilir, glob desenlerini destekler)
`--bins` → Tüm ikili hedefleri derler
`--example name…` → Belirtilen örneği derler
`--examples` → Tüm örnekleri derler
`--test name…` → Belirtilen entegrasyon testini derler
`--tests` → Tüm test hedeflerini derler
`--bench name…` → Belirtilen benchmark’ı derler
`--benches` → Tüm benchmark hedeflerini derler
`--all-targets` → `--lib --bins --tests --benches --examples` ile eşdeğerdir

> Not: `--bin`, `--example`, `--test`, `--bench` bayrakları Unix glob desenlerini (`*`, `?`, `[]`) destekler. Shell’in bunları Cargo’dan önce genişletmemesi için tek veya çift tırnak kullanılmalıdır.

#### 🔑 Özellik Seçimi (Feature Selection)

`-F features` / `--features features` → Etkinleştirilecek özellikleri listeler
`--all-features` → Tüm özellikleri etkinleştirir
`--no-default-features` → Varsayılan özellikleri devre dışı bırakır

#### 🛠️ Derleme Seçenekleri (Compilation Options)

`--target triple` → Belirtilen mimari için derler
`-r` / `--release` → `release` profili ile optimize edilmiş çıktıyı üretir
`--profile name` → Belirtilen profil ile derler (`check`, `test`, `bench` özel davranışlara sahiptir)
`--timings=fmts` → Derleme süreleri ve eşzamanlılık bilgilerini raporlar (`html`, `json`)
`--crate-type crate-type` → Belirtilen crate türüyle derler (`lib`, `cdylib` vb.)

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
`--frozen` → `--locked` ve `--offline` ile aynı anda eşdeğer
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
`--future-incompat-report` → Gelecekteki uyumsuzluk uyarıları için rapor gösterir (bkz. `cargo-report(1)`)

---

### 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

### 🚪 ÇIKIŞ DURUMU (EXIT STATUS)

`0`: Cargo başarıyla tamamlandı
`101`: Cargo başarısız oldu

---

### 📚 ÖRNEKLER (EXAMPLES)

Paketinizin (bağımlılıklar hariç) `unsafe` kod kullanıp kullanmadığını kontrol edin:

```
cargo rustc --lib -- -D unsafe-code
```

👉 Bu komut, `unsafe` kod kullanımını hata (`deny`) olarak değerlendirir.

Nightly derleyicide deneysel bir bayrak kullanın (örneğin tüm türlerin boyutunu yazdırır):

```
cargo rustc --lib -- -Z print-type-sizes
```

👉 Bu komut, türlerin bellek boyutlarını gösterir.

`Cargo.toml` içindeki `crate-type` alanını komut satırından geçersiz kılın:

```
cargo rustc --lib --crate-type lib,cdylib
```

👉 Bu komut, kütüphaneyi hem `lib` hem de `cdylib` olarak derler.

---

### 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-build(1)`, `rustc(1)`
