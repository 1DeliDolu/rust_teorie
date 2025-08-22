## 📋 cargo-test(1)

### 🏷️ İSİM (NAME)

cargo-test — Bir paketin birim (unit) ve entegrasyon (integration) testlerini çalıştırır

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo test [options] [testname] [-- test-options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Birim testler, entegrasyon testleri ve dokümantasyon testlerini derler ve çalıştırır.

Test filtreleme argümanı `TESTNAME` ve `--` sonrasındaki tüm argümanlar test ikililerine aktarılır ve dolayısıyla `libtest`’e (Rustc’nin yerleşik birim test ve mikro-benchmark çatısı) geçer. Hem Cargo’ya hem de test ikilisine argüman geçiriyorsanız, `--` sonrasındakiler ikiliye, öncekiler Cargo’ya gider. `libtest` argümanları için `cargo test -- --help` çıktısına ve Rustc kitabındaki testler bölümüne bakınız:
[https://doc.rust-lang.org/rustc/tests/index.html](https://doc.rust-lang.org/rustc/tests/index.html)

Örneğin, adı `foo` içeren testleri filtreleyip 3 iş parçacığıyla paralel çalıştırmak için:

```
cargo test foo -- --test-threads 3
```

Testler `--test` bayrağı ile `rustc` tarafından derlenir ve `libtest` ile bağlanarak özel bir yürütülebilir dosya oluşturulur. Bu yürütülebilir dosya, `#[test]` ile işaretlenmiş tüm fonksiyonları otomatik olarak paralel çalıştırır. `#[bench]` ile işaretlenmiş fonksiyonlar da bir kez çalıştırılır.

Birden fazla test hedefi varsa, her biri ayrı yürütülebilir dosya olarak derlenir ve sıralı olarak çalıştırılır.

`libtest` çatısı `harness = false` ile devre dışı bırakılabilir; bu durumda testleri çalıştıracak bir `main` fonksiyonunu sizin sağlamanız gerekir.

#### 📖 Dokümantasyon Testleri (Documentation tests)

Varsayılan olarak çalıştırılır ve `rustdoc` tarafından yürütülür. `rustdoc`, dokümantasyon yorumlarındaki kod örneklerini çıkarır ve çalıştırır. Her kod bloğu ayrı bir süreçte derlenip çalıştırılır. Bu model değişebilir, gelecekte farklı bir işleyiş olabilir.

`rustdoc` kitabında dokümantasyon testleri hakkında daha fazla bilgi bulabilirsiniz.

#### 📂 Testlerin Çalışma Dizini (Working directory of tests)

Birim ve entegrasyon testleri çalıştırılırken çalışma dizini paketin kök dizinidir. Böylece testler göreli yollarla paket dosyalarına erişebilir.
Dokümantasyon testlerinde ise `rustdoc` çağrılırken çalışma dizini çalışma alanı (workspace) köküdür.

---

### ⚙️ SEÇENEKLER (OPTIONS)

#### 🧪 Test Seçenekleri (Test Options)

`--no-run` → Derler ama testleri çalıştırmaz
`--no-fail-fast` → İlk hata sonrası durmak yerine tüm testleri çalıştırır

#### 📦 Paket Seçimi (Package Selection)

Varsayılan olarak, seçili `Cargo.toml`’a göre paket(ler) test edilir.
`-p spec…`, `--package spec…` → Yalnızca belirtilen paketi test eder (glob desenleri destekler)
`--workspace` → Çalışma alanındaki tüm üyeleri test eder
`--exclude SPEC…` → Belirtilen paketleri hariç tutar

#### 🎯 Hedef Seçimi (Target Selection)

Varsayılan olarak şu hedefler test edilir: `lib`, `bins`, `examples`, birim testler, entegrasyon testleri, dokümantasyon testleri.

`--lib` → Kütüphaneyi test et
`--bin name…` → Belirtilen ikiliyi test et
`--bins` → Tüm ikilileri test et
`--example name…` → Belirtilen örneği test et
`--examples` → Tüm örnekleri test et
`--test name…` → Belirtilen entegrasyon testini çalıştır
`--tests` → Tüm test hedeflerini çalıştır
`--bench name…` → Belirtilen benchmark’ı test et
`--benches` → Tüm benchmark hedeflerini test et
`--all-targets` → `--lib --bins --tests --benches --examples` ile eşdeğer
`--doc` → Yalnızca kütüphanenin dokümantasyon testlerini çalıştır

#### 🔑 Özellik Seçimi (Feature Selection)

`-F features` / `--features features` → Özellikleri etkinleştirir
`--all-features` → Tüm özellikleri etkinleştirir
`--no-default-features` → Varsayılan özellikleri devre dışı bırakır

#### 🛠️ Derleme Seçenekleri (Compilation Options)

`--target triple` → Belirtilen mimaride test et
`-r`, `--release` → `release` profili ile test et
`--profile name` → Belirtilen profil ile test et
`--timings=fmts` → Derleme süreleri ve eşzamanlılık bilgisini raporlar

#### 📂 Çıktı Seçenekleri (Output Options)

`--target-dir directory` → Çıktılar için dizin belirtir

#### 👁️ Görüntüleme Seçenekleri (Display Options)

Varsayılan olarak test çıktıları gizlenir. Gösterilmesi için:

```
cargo test -- --nocapture
```

`-v`, `--verbose` → Ayrıntılı çıktı
`-q`, `--quiet` → Sessiz mod
`--color when` → Renk kontrolü
`--message-format fmt` → Çıktı formatı (`human`, `short`, `json`)

#### 📜 Manifest Seçenekleri (Manifest Options)

`--manifest-path path` → `Cargo.toml` yolu
`--ignore-rust-version` → `rust-version` yok sayılır
`--locked` → Kilit dosyasındaki tam sürümleri zorunlu kılar
`--offline` → Ağ erişimi olmadan çalışır
`--frozen` → `--locked` ve `--offline` ile eşdeğer
`--lockfile-path PATH` → Kilit dosyası yolunu değiştirir (nightly)

#### ⚙️ Ortak Seçenekler (Common Options)

`+toolchain` → Belirli Rustup toolchain seçmek için
`--config KEY=VALUE or PATH` → Yapılandırma değerlerini geçersiz kılar
`-C PATH` → Belirtilen dizine geçerek çalıştırır (nightly)
`-h`, `--help` → Yardım bilgisini yazdırır
`-Z flag` → Deneysel bayraklar (yalnızca nightly)

#### 🔄 Çeşitli Seçenekler (Miscellaneous Options)

`-j N`, `--jobs N` → Paralel iş sayısını ayarlar
`--future-incompat-report` → Gelecekteki uyumsuzluk raporlarını gösterir

---

### 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

### 🚪 ÇIKIŞ DURUMU (EXIT STATUS)

`0`: Cargo başarıyla tamamlandı
`101`: Cargo başarısız oldu

---

### 📚 ÖRNEKLER (EXAMPLES)

Geçerli paketin tüm birim ve entegrasyon testlerini çalıştır:

```
cargo test
```

👉 Bu komut tüm testleri derleyip çalıştırır.

Yalnızca adı belirli bir filtreyle eşleşen testleri çalıştır:

```
cargo test name_filter
```

👉 Bu komut, adı `name_filter` ile eşleşen testleri çalıştırır.

Belirli bir entegrasyon testinde belirli bir fonksiyonu çalıştır:

```
cargo test --test int_test_name -- modname::test_name
```

👉 Bu komut, `int_test_name` entegrasyon testindeki `modname::test_name` fonksiyonunu çalıştırır.

---

### 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-bench(1)`, test türleri (types of tests), test yazma (how to write tests)
