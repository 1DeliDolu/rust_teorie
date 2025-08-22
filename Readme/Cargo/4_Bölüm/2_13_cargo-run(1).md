## 📋 cargo-run(1)

### 🏷️ İSİM (NAME)

cargo-run — Geçerli paketi çalıştırır

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo run [options] [-- args]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Yerel paketin bir ikili dosyasını (binary) veya örneğini (example) çalıştırır.

İki tire (`--`) sonrasındaki tüm argümanlar çalıştırılacak ikili dosyaya aktarılır. Hem Cargo’ya hem de ikili dosyaya argüman gönderiyorsanız, `--` sonrasındakiler ikili dosyaya, öncekiler Cargo’ya gider.

`cargo-test(1)` ve `cargo-bench(1)`’ten farklı olarak, `cargo run` çalıştırılan ikilinin çalışma dizinini (working directory) mevcut çalışma dizini olarak ayarlar; yani doğrudan shell üzerinden çalıştırılmış gibi davranır.

---

### ⚙️ SEÇENEKLER (OPTIONS)

#### 📦 Paket Seçimi (Package Selection)

Varsayılan olarak, geçerli çalışma dizinindeki paket seçilir. `-p` bayrağı bir çalışma alanındaki (workspace) farklı bir paketi seçmek için kullanılabilir.

`-p spec`
`--package spec`
Çalıştırılacak paketi belirtir. SPEC formatı için `cargo-pkgid(1)`’e bakınız.

#### 🎯 Hedef Seçimi (Target Selection)

Hedef seçimi yapılmazsa, `cargo run` ikili hedefi çalıştırır. Birden fazla ikili hedef varsa, bir hedef bayrağı verilmelidir. Alternatif olarak, `Cargo.toml` içindeki `[package]` bölümünde `default-run` alanı belirtilerek varsayılan çalıştırılacak ikilinin adı seçilebilir.

`--bin name`
Belirtilen ikiliyi çalıştırır.

`--example name`
Belirtilen örneği çalıştırır.

#### 🔑 Özellik Seçimi (Feature Selection)

Özellik bayrakları hangi özelliklerin etkinleştirileceğini kontrol etmenizi sağlar. Hiçbir özellik seçeneği verilmezse, her seçilen paket için varsayılan özellik etkinleştirilir.

`-F features`
`--features features`
Boşluk veya virgülle ayrılmış etkinleştirilecek özellik listesi. Çalışma alanı üyelerinin özellikleri `package-name/feature-name` sözdizimi ile etkinleştirilebilir. Bu bayrak birden çok kez belirtilebilir.

`--all-features`
Tüm seçili paketlerin tüm mevcut özelliklerini etkinleştirir.

`--no-default-features`
Seçili paketlerin varsayılan özelliklerini etkinleştirmez.

#### 🛠️ Derleme Seçenekleri (Compilation Options)

`--target triple`
Belirtilen mimari için çalıştırır. Varsayılan, host mimarisidir. Desteklenen hedefler için `rustc --print target-list` çalıştırılabilir.
Ayrıca `build.target` yapılandırma değeri ile belirtilebilir.

`-r` / `--release`
`release` profili ile optimize edilmiş çıktıyı çalıştırır.

`--profile name`
Belirtilen profil ile çalıştırır.

`--timings=fmts`
Her derlemenin ne kadar sürdüğünü ve eşzamanlılık bilgisini gösterir. Geçerli formatlar: `html` (insan okunabilir), `json` (makine tarafından okunabilir).

#### 📂 Çıktı Seçenekleri (Output Options)

`--target-dir directory`
Tüm üretilen çıktılar ve ara dosyalar için dizin.

#### 👁️ Görüntüleme Seçenekleri (Display Options)

`-v`, `--verbose`
Ayrıntılı çıktı.

`-q`, `--quiet`
Cargo log mesajlarını bastırır.

`--color when`
Renkli çıktı kontrolü: `auto`, `always`, `never`.

`--message-format fmt`
Teşhis (diagnostic) mesajlarının formatı: `human`, `short`, `json`, vb.

#### 📜 Manifest Seçenekleri (Manifest Options)

`--manifest-path path`
`Cargo.toml` dosya yolu.

`--ignore-rust-version`
`rust-version` belirtimini yok say.

`--locked`
Mevcut `Cargo.lock` dosyasındaki bağımlılıkların tam sürümlerini kullanmayı garanti eder.

`--offline`
Ağa erişmeden çalışır.

`--frozen`
`--locked` ve `--offline` ile aynı anda eşdeğer.

`--lockfile-path PATH`
Varsayılan `Cargo.lock` yolunu değiştirir (yalnızca nightly).

#### ⚙️ Ortak Seçenekler (Common Options)

`+toolchain`
Rustup ile belirli bir toolchain seçmek için.

`--config KEY=VALUE or PATH`
Cargo yapılandırmasını geçersiz kıl.

`-C PATH`
Belirtilen dizine geçip oradan çalıştırır (yalnızca nightly).

`-h`, `--help`
Yardım bilgisini yazdırır.

`-Z flag`
Deneysel bayraklar (yalnızca nightly).

#### 🔄 Çeşitli Seçenekler (Miscellaneous Options)

`-j N`, `--jobs N`
Paralel iş sayısı.

`--keep-going`
Başarısız olsa bile mümkün olan tüm bağımlılıkları derlemeye devam et.

---

### 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

### 🚪 ÇIKIŞ DURUMU (EXIT STATUS)

`0`: Cargo başarıyla tamamlandı.
`101`: Cargo başarısız oldu.

---

### 📚 ÖRNEKLER (EXAMPLES)

Yerel paketi derleyip ana hedefini çalıştır:

```
cargo run
```

👉 Bu komut, paketin ana ikili dosyasını çalıştırır.

Ek argümanlarla bir örnek çalıştır:

```
cargo run --example exname -- --exoption exarg1 exarg2
```

👉 Bu komut, `exname` örneğini çalıştırır ve `exoption exarg1 exarg2` argümanlarını ona aktarır.

---

### 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-build(1)`
