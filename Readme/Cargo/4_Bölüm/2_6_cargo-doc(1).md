## 📋 cargo-doc(1)

### 🏷️ İSİM (NAME)

cargo-doc — Bir paketin dokümantasyonunu oluşturur

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo doc [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Yerel paket ve tüm bağımlılıklarının dokümantasyonunu oluşturur. Çıktı `target/doc` dizinine, `rustdoc`’un standart formatında yerleştirilir.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 📖 Dokümantasyon Seçenekleri (Documentation Options)

* `--open` → Dokümantasyon oluşturulduktan sonra varsayılan tarayıcıda açar.
* `--no-deps` → Bağımlılıkların dokümantasyonunu oluşturmaz.
* `--document-private-items` → Varsayılan olarak gizli (non-public) öğeleri de dahil eder (özellikle ikili hedefler belgelenirken).

---

### 📦 Paket Seçimi (Package Selection)

Varsayılan seçim manifest dosyasına bağlıdır:

* Eğer manifest bir çalışma alanı köküyse, varsayılan üyeler belgelenir.

* Aksi halde yalnızca manifest’te tanımlı paket belgelenir.

* `-p spec`, `--package spec` → Yalnızca belirtilen paketleri belgeler.

* `--workspace` → Çalışma alanındaki tüm paketleri belgeler.

* `--all` → `--workspace` için kullanımdan kalkmış (deprecated) takma ad.

* `--exclude SPEC` → Belirli paketleri hariç tutar.

---

### 🎯 Hedef Seçimi (Target Selection)

Varsayılan olarak tüm ikili (binary) ve kütüphane (library) hedefleri belgeler.

* `--lib` → Paketin kütüphanesini belgeler.
* `--bin name` → Belirtilen ikiliyi belgeler.
* `--bins` → Tüm ikili hedefleri belgeler.
* `--example name` → Belirtilen örneği belgeler.
* `--examples` → Tüm örnekleri belgeler.

---

### 🔑 Özellik Seçimi (Feature Selection)

* `--features` → Belirtilen özellikleri etkinleştir.
* `--all-features` → Tüm özellikleri etkinleştir.
* `--no-default-features` → Varsayılan özelliği devre dışı bırak.

---

### 🏗️ Derleme Seçenekleri (Compilation Options)

* `--target triple` → Belirtilen mimari için dokümantasyon oluştur.
* `-r, --release` → `release` profili ile belgeler.
* `--profile name` → Belirtilen profil ile belgeler.
* `--timings=fmts` → Derleme sürelerini raporlar (`html`, `json`).

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

* `--manifest-path path` → Kullanılacak `Cargo.toml` yolu.
* `--ignore-rust-version` → `rust-version` kontrolünü yok say.
* `--locked` → `Cargo.lock` dosyasını değiştirmeden kullan.
* `--offline` → Ağ erişimi olmadan çalış.
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
* `--keep-going` → Hata olsa bile mümkün olduğunca çok crate belgeler.

---

## 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🚪 ÇIKIŞ DURUMU (EXIT STATUS)

* `0` → Başarılı.
* `101` → Başarısız.

---

## 📚 ÖRNEKLER (EXAMPLES)

Yerel paket ve bağımlılıklarının dokümantasyonunu oluştur ve `target/doc` dizinine kaydet:

```
cargo doc
```

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-rustdoc(1)`, `rustdoc(1)`
