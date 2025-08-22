## 📋 cargo-fix(1)

### 🏷️ İSİM (NAME)

cargo-fix — `rustc` tarafından raporlanan lint uyarılarını otomatik olarak düzeltir

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo fix [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu Cargo alt komutu, `rustc` tanılamalarında (diagnostics) verilen uyarıların önerilerini alır ve kaynak kodunuza uygular. Amaç, `rustc`’nin size nasıl düzelteceğini söylediği hataları otomatikleştirmektir.

`cargo fix`, arka planda `cargo-check(1)` çalıştırır. Uygulanabilir tüm düzeltmeler otomatik olarak yapılır, geri kalan uyarılar işlem sonunda gösterilir.

Örneğin, mevcut pakete tüm düzeltmeleri uygulamak için:

```
cargo fix
```

Bu, `cargo check --all-targets` ile aynı şekilde davranır.

`cargo fix`, yalnızca `cargo check` tarafından derlenen kodu düzeltebilir. Opsiyonel özellikler (features) ile etkinleştirilen kod için ilgili özellikleri belirtmeniz gerekir:

```
cargo fix --features foo
```

Platforma özgü kodlar için `--target` parametresi ile derleme hedefi seçebilirsiniz:

```
cargo fix --target x86_64-pc-windows-gnu
```

Sorun yaşarsanız veya yeni özellik istekleriniz varsa, [Cargo GitHub](https://github.com/rust-lang/cargo) üzerinden bildirim yapabilirsiniz.

---

### 🌀 Sürüm Geçişi (Edition migration)

`cargo fix`, bir paketi bir sürümden (edition) diğerine taşımak için de kullanılabilir:

1. `cargo fix --edition` çalıştırın. Projede birden fazla özellik varsa `--all-features` kullanmanız tavsiye edilir. Platforma özgü kod için farklı `--target` bayraklarıyla tekrarlayabilirsiniz.
2. `Cargo.toml` dosyasındaki `edition` alanını güncelleyin.
3. Proje testlerini çalıştırarak doğrulayın. Yeni uyarılar varsa, `cargo fix` komutunu (bu sefer `--edition` bayrağı olmadan) tekrar çalıştırın.

Not: `cargo fix`, etkin olmayan özellikler veya `cfg` ifadeleri için kodu güncellemez. Ayrıca bazı durumlarda derleyici tüm kodu otomatik dönüştüremeyebilir, manuel düzenleme gerekebilir.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🔧 Düzeltme Seçenekleri (Fix options)

* `--broken-code` → Kod bozuk olsa bile düzeltmeleri uygular.
* `--edition` → Kodu bir sonraki sürüme (edition) taşımak için değişiklikleri uygular.
* `--edition-idioms` → Mevcut sürüm için önerilen stil düzeltmelerini uygular.
* `--allow-no-vcs` → VCS algılanmasa bile düzeltmeleri uygular.
* `--allow-dirty` → Çalışma dizininde değişiklikler olsa bile düzeltir.
* `--allow-staged` → Staged değişiklikler olsa bile düzeltir.

---

### 📦 Paket Seçimi (Package Selection)

* `-p spec`, `--package spec` → Belirtilen paketleri düzelt.
* `--workspace` → Çalışma alanındaki tüm paketleri düzelt.
* `--all` → `--workspace` için kullanımdan kalkmış takma ad.
* `--exclude SPEC` → Belirli paketleri hariç tut.

---

### 🎯 Hedef Seçimi (Target Selection)

Varsayılan olarak tüm hedefler düzeltilir (`--all-targets` varsayılandır).

* `--lib` → Kütüphaneyi düzelt.
* `--bin name` → Belirtilen ikiliyi düzelt.
* `--bins` → Tüm ikili hedefleri düzelt.
* `--example name` → Belirtilen örneği düzelt.
* `--examples` → Tüm örnek hedefleri düzelt.
* `--test name` → Belirtilen testi düzelt.
* `--tests` → Tüm test hedeflerini düzelt.
* `--bench name` → Belirtilen benchmark’ı düzelt.
* `--benches` → Tüm benchmark hedeflerini düzelt.
* `--all-targets` → Tüm hedefleri düzelt.

---

### 🔑 Özellik Seçimi (Feature Selection)

* `--features` → Belirtilen özellikleri etkinleştir.
* `--all-features` → Tüm özellikleri etkinleştir.
* `--no-default-features` → Varsayılan özelliği devre dışı bırak.

---

### 🏗️ Derleme Seçenekleri (Compilation Options)

* `--target triple` → Belirtilen mimari için düzelt.
* `-r, --release` → `release` profiliyle düzelt.
* `--profile name` → Belirtilen profil ile düzelt.
* `--timings=fmts` → Derleme sürelerini raporla (`html`, `json`).

---

### 📤 Çıktı Seçenekleri (Output Options)

* `--target-dir directory` → Çıktıların kaydedileceği dizin.

---

### 👀 Görüntüleme Seçenekleri (Display Options)

* `-v, --verbose` → Ayrıntılı çıktı.
* `-q, --quiet` → Sessiz mod.
* `--color when` → Renkli çıktı ayarı (`auto`, `always`, `never`).
* `--message-format fmt` → Çıktı formatı (`human`, `short`, `json`).

---

### 📄 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path` → Kullanılacak `Cargo.toml` dosyası.
* `--ignore-rust-version` → `rust-version` kontrolünü yok say.
* `--locked` → `Cargo.lock` dosyasındaki sürümleri değiştirmeden kullan.
* `--offline` → Ağ erişimini kapat.
* `--frozen` → Hem `--locked` hem `--offline`.
* `--lockfile-path PATH` → Kilit dosyası yolunu değiştir (yalnızca `nightly`).

---

### ⚡ Genel Seçenekler (Common Options)

* `+toolchain` → Belirli bir Rust sürüm zinciri (örn. `+stable`, `+nightly`).
* `--config KEY=VALUE` → Konfigürasyon değerini geçersiz kıl.
* `-C PATH` → Çalışma dizinini değiştir.
* `-h, --help` → Yardım bilgisini yazdır.
* `-Z` → Kararsız (nightly) bayraklar.

---

### 🔄 Diğer Seçenekler (Miscellaneous Options)

* `-j N, --jobs N` → Paralel iş sayısı.
* `--keep-going` → Hata olsa bile mümkün olduğunca çok crate düzelt.

---

## 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🚪 ÇIKIŞ DURUMU (EXIT STATUS)

* `0` → Başarılı.
* `101` → Başarısız.

---

## 📚 ÖRNEKLER (EXAMPLES)

Mevcut pakette düzeltmeleri uygula:

```
cargo fix
```

Paketi bir sonraki sürüme hazırla:

```
cargo fix --edition
```

Mevcut sürüm için önerilen stil değişikliklerini uygula:

```
cargo fix --edition-idioms
```

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-check(1)`
