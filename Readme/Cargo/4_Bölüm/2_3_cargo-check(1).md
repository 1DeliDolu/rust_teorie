## 📋 cargo-check(1)

### 🏷️ İSİM (NAME)

cargo-check — Geçerli paketi denetler

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo check [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Yerel bir paketi ve tüm bağımlılıklarını hatalara karşı denetler. Bu işlem, paketleri neredeyse tamamen derler fakat son kod üretim (code generation) adımını atlar, bu yüzden `cargo build` çalıştırmaktan daha hızlıdır. Derleyici, gelecekteki çalışmalarda kullanılmak üzere meta veri dosyalarını diske kaydeder. Ancak, bazı tanılama ve hatalar yalnızca kod üretim aşamasında ortaya çıktığından `cargo check` sırasında rapor edilmez.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 📦 Paket Seçimi (Package Selection)

Varsayılan seçim, manifest dosyasına bağlıdır.

* Eğer manifest bir çalışma alanının kökü ise, varsayılan üyeler seçilir.

* Aksi halde yalnızca manifest’te tanımlı paket seçilir.

* `-p spec`, `--package spec` → Yalnızca belirtilen paketleri denetle.

* `--workspace` → Çalışma alanındaki tüm paketleri denetle.

* `--all` → `--workspace` için kullanımdan kalkmış (deprecated) takma ad.

* `--exclude SPEC` → Belirli paketleri hariç tut.

---

### 🎯 Hedef Seçimi (Target Selection)

Varsayılan olarak tüm ikili (binary) ve kütüphane (library) hedefleri denetlenir.

* `--lib` → Paketin kütüphanesini denetle.
* `--bin name` → Belirtilen ikiliyi denetle.
* `--bins` → Tüm ikili hedefleri denetle.
* `--example name` → Belirtilen örneği denetle.
* `--examples` → Tüm örnek hedefleri denetle.
* `--test name` → Belirtilen entegrasyon testini denetle.
* `--tests` → Tüm `test = true` hedeflerini denetle.
* `--bench name` → Belirtilen benchmark’ı denetle.
* `--benches` → Tüm `bench = true` hedeflerini denetle.
* `--all-targets` → Tüm hedefleri denetle (`--lib --bins --tests --benches --examples`).

---

### 🔑 Özellik Seçimi (Feature Selection)

* `--features` → Belirtilen özellikleri etkinleştir.
* `--all-features` → Tüm özellikleri etkinleştir.
* `--no-default-features` → Varsayılan özelliği devre dışı bırak.

---

### 🏗️ Derleme Seçenekleri (Compilation Options)

* `--target triple` → Belirtilen mimari için denetle.
* `-r, --release` → `release` profili ile denetim yap.
* `--profile name` → Belirtilen profil ile denetle (`test`, `dev`, `release`).
* `--timings=fmts` → Derleme sürelerini ve eşzamanlılık bilgisini raporla (`html`, `json`).

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
* `--locked` → `Cargo.lock` dosyasını değiştirmeden kullan.
* `--offline` → Ağ erişimini kapat.
* `--frozen` → Hem `--locked` hem `--offline`.
* `--lockfile-path PATH` → Kilit dosyası yolunu değiştir (yalnızca `nightly`).

---

### ⚡ Genel Seçenekler (Common Options)

* `+toolchain` → Belirli bir Rust sürüm zinciri (örn. `+stable`, `+nightly`).
* `--config KEY=VALUE` → Belirli bir konfigürasyonu geçersiz kıl.
* `-C PATH` → Çalışma dizinini değiştir.
* `-h, --help` → Yardım bilgisini yazdır.
* `-Z` → Kararsız (nightly) bayrakları.

---

### 🔄 Diğer Seçenekler (Miscellaneous Options)

* `-j N, --jobs N` → Paralel iş sayısı.
* `--keep-going` → Hata olsa bile mümkün olduğunca çok crate denetle.
* `--future-incompat-report` → Gelecekteki uyumsuzluk raporunu göster.

---

## 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🚪 ÇIKIŞ DURUMU (EXIT STATUS)

* `0` → Başarılı.
* `101` → Başarısız.

---

## 📚 ÖRNEKLER (EXAMPLES)

Yerel paketi hatalara karşı denetle:

```
cargo check
```

Tüm hedefleri test profiliyle denetle:

```
cargo check --all-targets --profile=test
```

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-build(1)`
