## 📦 Crate test (crate test)

Öğe yolunu kopyala (Copy item path) · Ayarlar (Settings) · Yardım (Help)

---

## 🧾 Özet (Summary)

**Kaynak (Source)**

🔬 Bu yalnızca nightly’de kullanılabilen deneysel bir API’dir (`test`).

Bu crate, **rustc’nin yerleşik birim test (unit-test) ve mikro-benchmarking çerçevesi** (framework) için destek kodlarını içerir.

Kullanıcı kodlarının neredeyse tamamı yalnızca `Bencher` ve `black_box` ile ilgilenir. Diğer tüm etkileşimler (örneğin test ve benchmark yazmak) doğrudan `#[test]` ve `#[bench]` öznitelikleri aracılığıyla yapılmalıdır.

Daha fazla ayrıntı için kitabın **Testing Chapter** bölümüne bakınız.

---

## 🔄 Yeniden dışa aktarımlar (Re-exports)

* `pub use self::bench::Bencher;` Deneysel (Experimental)
* `pub use self::bench::black_box;` Deneysel (Experimental)
* `pub use self::ColorConfig::*;` Deneysel (Experimental)
* `pub use self::types::TestName::*;` Deneysel (Experimental)
* `pub use NamePadding::*;` Deneysel (Experimental)
* `pub use TestFn::*;` Deneysel (Experimental)
* `pub use TestName::*;` Deneysel (Experimental)

---

## 🧩 Modüller (Modules)

* `bench` Deneysel (Experimental)
  Benchmarking modülü.

* `stats` Deneysel (Experimental)

* `test` Deneysel (Experimental)

---

## 🏗️ Yapılar (Structs)

* `Options` Deneysel (Experimental)
  Komut satırı argümanları yerine çağıran tarafından tanımlanan test çalıştırma seçenekleri. Yeni seçenekler eklemek gerekirse bu yapıya dahil edilir.

* `TestDesc` Deneysel (Experimental)

* `TestDescAndFn` Deneysel (Experimental)

* `TestId` Deneysel (Experimental)

* `TestOpts` Deneysel (Experimental)

---

## 🔄 Numaralandırmalar (Enums)

* `ColorConfig` Deneysel (Experimental)
  Konsol çıktısının renkli olup olmayacağını belirtir.

* `NamePadding` Deneysel (Experimental)

* `OutputFormat` Deneysel (Experimental)
  Test sonuçları çıktısının biçimi.

* `RunIgnored` Deneysel (Experimental)
  Yoksayılan (ignored) testlerin çalıştırılıp çalıştırılmayacağını belirtir.

* `ShouldPanic` Deneysel (Experimental)
  Testin panik (panic) beklenip beklenmediğini belirtir.

* `TestFn` Deneysel (Experimental)

* `TestName` Deneysel (Experimental)

* `TestType` Deneysel (Experimental)
  Testin türü (Rust kitabındaki test türleri uyarınca).

---

## ⚙️ Fonksiyonlar (Functions)

* `assert_test_result` Deneysel (Experimental)
  Birim testler bittiğinde çağrılır. Test başarısız kabul edilirse `Result::Err` döner. Varsayılan olarak `report()` çağrılır ve 0 sonucu kontrol edilir.

* `convert_benchmarks_to_tests` Deneysel (Experimental)

* `filter_tests` Deneysel (Experimental)

* `run_test` Deneysel (Experimental)

* `run_tests` Deneysel (Experimental)

* `run_tests_console` Deneysel (Experimental)
  Basit bir konsol test çalıştırıcısı. Sağlanan testleri çalıştırır, süreci ve sonuçları `stdout`’a raporlar.

* `test_main` Deneysel (Experimental)

* `test_main_static` Deneysel (Experimental)
  Statik test vektörüyle çağrı için optimize edilmiş varyant. Dinamik test verildiğinde kasıtlı olarak panikler.

* `test_main_static_abort` Deneysel (Experimental)
  Statik test vektörüyle çağrı için optimize edilmiş varyant. Dinamik test verildiğinde kasıtlı olarak panikler.

* `test_main_with_exit_callback` Deneysel (Experimental)
