## 📋 cargo-bench(1)

### 🏷️ İSİM (NAME)

cargo-bench — Bir paketin kıyaslama (benchmark) testlerini çalıştırır

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo bench [options] [benchname] [-- bench-options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Kıyaslama testlerini derler ve çalıştırır.

`benchname` bağımsız değişkeni ve `--` (çift tire) sonrasındaki tüm bağımsız değişkenler kıyaslama ikili dosyalarına (benchmark binaries) ve dolayısıyla `libtest`’e (Rustc’nin yerleşik birim testi ve mikro-kıyaslama çerçevesi) aktarılır. Eğer hem Cargo’ya hem de ikili dosyaya argüman geçiriyorsanız, `--` öncesindekiler Cargo’ya, sonrasındakiler ikili dosyaya gider.

`libtest` argümanları hakkında ayrıntılı bilgi için:

```
cargo bench -- --help
```

ayrıca Rustc kitabındaki [testlerin nasıl çalıştığı](https://doc.rust-lang.org/rustc/tests/index.html) bölümüne bakınız.

Örneğin, yalnızca `foo` adlı kıyaslamayı çalıştırmak için:

```
cargo bench -- foo --exact
```

Kıyaslamalar, `rustc`’ye `--test` seçeneği ile inşa edilir. Bu, kodunuzu `libtest` ile bağlayarak özel bir çalıştırılabilir dosya oluşturur. Çalıştırılabilir dosya otomatik olarak `#[bench]` özniteliği ile işaretlenmiş tüm fonksiyonları çalıştırır. Cargo, yalnızca kıyaslamaların çalıştırılması gerektiğini belirtmek için test harness’e `--bench` bayrağını geçirir.

`libtest` harness’i, hedef manifest ayarlarında `harness = false` ile devre dışı bırakılabilir. Bu durumda kıyaslamaları çalıştırmak için kendi `main` fonksiyonunuzu sağlamanız gerekir.

Not: `#[bench]` özniteliği şu anda kararsızdır (unstable) ve yalnızca `nightly` kanalında kullanılabilir. `Criterion` gibi crates.io’da bulunan bazı paketler, `stable` kanalında kıyaslamaları çalıştırmaya yardımcı olabilir.

Varsayılan olarak, `cargo bench` `bench` profilini kullanır. Bu profil optimizasyonları etkinleştirir ve hata ayıklama bilgisini devre dışı bırakır. Eğer hata ayıklama gerekiyorsa:

```
cargo bench --profile=dev
```

ile `dev` profilinde kıyaslama çalıştırabilirsiniz.

### 📂 Kıyaslamaların Çalışma Dizini (Working directory of benchmarks)

Her kıyaslamanın çalışma dizini, ait olduğu paketin kök dizinidir. Bu, kıyaslamaların paket dosyalarına göreceli yollarla güvenilir şekilde erişebilmesini sağlar.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🔧 Kıyaslama Seçenekleri (Benchmark Options)

* `--no-run` → Derle ama çalıştırma.
* `--no-fail-fast` → Bir hata sonrası durma, tüm kıyaslamaları çalıştır.

### 📦 Paket Seçimi (Package Selection)

* `-p`, `--package` → Belirli paketler için kıyaslama çalıştır.
* `--workspace` → Çalışma alanındaki tüm üyeleri kıyasla.
* `--all` → `--workspace` için eski (deprecated) takma ad.
* `--exclude` → Belirli paketleri hariç tut.

### 🎯 Hedef Seçimi (Target Selection)

Varsayılan hedefler: `lib`, `bins`, `bench` hedefleri.
Seçenekler:

* `--lib` → Paketin kütüphanesini kıyasla.
* `--bin name` → Belirtilen ikili dosyayı kıyasla.
* `--bins` → Tüm ikili hedefleri kıyasla.
* `--example name` → Belirtilen örneği kıyasla.
* `--examples` → Tüm örnek hedefleri kıyasla.
* `--test name` → Belirtilen entegrasyon testini kıyasla.
* `--tests` → Tüm `test = true` hedeflerini kıyasla.
* `--bench name` → Belirtilen kıyaslamayı çalıştır.
* `--benches` → Tüm `bench = true` hedeflerini çalıştır.
* `--all-targets` → Tüm hedefleri kıyasla.

### 🔑 Özellik Seçimi (Feature Selection)

* `--features` → Belirtilen özellikleri etkinleştir.
* `--all-features` → Tüm özellikleri etkinleştir.
* `--no-default-features` → Varsayılan özelliği devre dışı bırak.

### 🏗️ Derleme Seçenekleri (Compilation Options)

* `--target triple` → Belirli bir mimari için derleme.
* `--profile name` → Belirtilen profil ile kıyaslama (`dev`, `release`, `bench`).
* `--timings` → Derleme süreleri raporu oluşturur (`html`, `json`).

### 📤 Çıktı Seçenekleri (Output Options)

* `--target-dir` → Üretilen çıktılar için dizin.

### 👀 Görüntüleme Seçenekleri (Display Options)

* `--nocapture` → Kıyaslama çıktısını göster.
* `-v, --verbose` → Ayrıntılı çıktı.
* `-q, --quiet` → Sessiz mod.
* `--color when` → Renkli çıktıyı ayarla (`auto`, `always`, `never`).
* `--message-format` → Çıktı formatı (`human`, `short`, `json`).

### 📄 Manifest Seçenekleri (Manifest Options)

* `--manifest-path` → Kullanılacak `Cargo.toml` dosyası.
* `--locked` → `Cargo.lock` dosyasındaki sürümleri değişmeden kullan.
* `--offline` → Ağ erişimini kapat.
* `--frozen` → Hem `--locked` hem `--offline`.

### ⚡ Genel Seçenekler (Common Options)

* `+toolchain` → Belirli bir Rust sürüm zinciri (ör. `+nightly`).
* `--config KEY=VALUE` → Konfigürasyon değerini geçersiz kıl.
* `-C PATH` → Çalışma dizinini değiştir.
* `-h, --help` → Yardım göster.

### 🔄 Diğer Seçenekler (Miscellaneous Options)

* `-j N, --jobs N` → Paralel derleme iş sayısı.

---

## 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🚪 ÇIKIŞ DURUMU (EXIT STATUS)

* `0` → Başarılı.
* `101` → Başarısız.

---

## 📚 ÖRNEKLER (EXAMPLES)

Tüm kıyaslamaları derle ve çalıştır:

```
cargo bench
```

Belirli bir kıyaslamayı çalıştır:

```
cargo bench --bench bench_name -- modname::some_benchmark
```

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-test(1)`
