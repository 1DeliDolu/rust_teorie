## 📋 cargo-build(1)

### 🏷️ İSİM (NAME)

cargo-build — Geçerli paketi derler

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo build [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Yerel paketleri ve tüm bağımlılıklarını derler.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 📦 Paket Seçimi (Package Selection)

Varsayılan olarak, paket seçimi belirtilmediğinde seçilen paketler, kullanılan manifest dosyasına göre belirlenir.

* Eğer manifest bir çalışma alanının (workspace) kökü ise, çalışma alanının varsayılan üyeleri seçilir.
* Aksi halde yalnızca manifest’te tanımlı paket seçilir.

Çalışma alanının varsayılan üyeleri `workspace.default-members` anahtarı ile kök manifest’te açıkça tanımlanabilir.

* `-p spec`, `--package spec` → Yalnızca belirtilen paketleri derle.
* `--workspace` → Çalışma alanındaki tüm paketleri derle.
* `--all` → `--workspace` için kullanımdan kalkmış (deprecated) takma ad.
* `--exclude SPEC` → Belirtilen paketleri hariç tut (yalnızca `--workspace` ile birlikte).

---

### 🎯 Hedef Seçimi (Target Selection)

Hedef seçilmediğinde tüm ikili (binary) ve kütüphane (library) hedefleri derlenir.

* `--lib` → Paketin kütüphanesini derle.
* `--bin name` → Belirtilen ikiliyi derle.
* `--bins` → Tüm ikili hedefleri derle.
* `--example name` → Belirtilen örneği derle.
* `--examples` → Tüm örnekleri derle.
* `--test name` → Belirtilen entegrasyon testini derle.
* `--tests` → Tüm `test = true` hedeflerini derle.
* `--bench name` → Belirtilen benchmark’ı derle.
* `--benches` → Tüm `bench = true` hedeflerini derle.
* `--all-targets` → Tüm hedefleri derle (`--lib --bins --tests --benches --examples` eşdeğeridir).

---

### 🔑 Özellik Seçimi (Feature Selection)

* `--features` → Belirtilen özellikleri etkinleştir.
* `--all-features` → Tüm özellikleri etkinleştir.
* `--no-default-features` → Varsayılan özelliği devre dışı bırak.

---

### 🏗️ Derleme Seçenekleri (Compilation Options)

* `--target triple` → Belirtilen mimari için derle.
* `-r, --release` → `release` profili ile optimize edilmiş derleme.
* `--profile name` → Belirtilen profil ile derle.
* `--timings=fmts` → Derleme süresi raporları üret (`html`, `json`).

---

### 📤 Çıktı Seçenekleri (Output Options)

* `--target-dir directory` → Derleme çıktıları için dizin.
* `--artifact-dir directory` → Nihai çıktıları belirtilen dizine kopyala (yalnızca `nightly`).

---

### 👀 Görüntüleme Seçenekleri (Display Options)

* `-v, --verbose` → Ayrıntılı çıktı.
* `-q, --quiet` → Sessiz mod.
* `--color when` → Renkli çıktı ayarı (`auto`, `always`, `never`).
* `--message-format fmt` → Tanılama çıktısı formatı (`human`, `short`, `json`).
* `--build-plan` → JSON biçiminde derleme planı çıktısı (yalnızca `nightly`).

---

### 📄 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path` → Kullanılacak `Cargo.toml` yolu.
* `--ignore-rust-version` → `rust-version` kontrolünü yok say.
* `--locked` → `Cargo.lock` dosyasını değiştirmeden kullan.
* `--offline` → Ağ erişimi olmadan çalış.
* `--frozen` → Hem `--locked` hem `--offline`.
* `--lockfile-path PATH` → Kilit dosyası yolunu değiştir (yalnızca `nightly`).

---

### ⚡ Genel Seçenekler (Common Options)

* `+toolchain` → Belirli bir Rust sürüm zinciri (örn. `+stable`, `+nightly`).
* `--config KEY=VALUE` → Belirli bir konfigürasyon değerini geçersiz kıl.
* `-C PATH` → Çalışma dizinini değiştir.
* `-h, --help` → Yardım bilgisini yazdır.
* `-Z` → Kararsız (nightly) bayrakları.

---

### 🔄 Diğer Seçenekler (Miscellaneous Options)

* `-j N, --jobs N` → Paralel iş sayısı.
* `--keep-going` → Hata olsa bile mümkün olduğunca çok crate derle.
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

Yerel paketi ve bağımlılıklarını derle:

```
cargo build
```

Optimizasyonlu derleme:

```
cargo build --release
```

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-rustc(1)`
