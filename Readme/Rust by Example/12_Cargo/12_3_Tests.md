## 🧪 Test Etme (Testing)

Bildiğimiz gibi test etme, her yazılımın ayrılmaz bir parçasıdır! Rust, birim testler (unit tests) ve entegrasyon testleri (integration tests) için birinci sınıf destek sunar (bkz. *The Rust Programming Language* kitabındaki ilgili bölüm).

Yukarıdaki test bölümlerinde, birim testlerin ve entegrasyon testlerinin nasıl yazıldığını görürüz. Organizasyon açısından, birim testleri test ettikleri modüllerin içine, entegrasyon testlerini ise kendi `tests/` dizinlerine yerleştirebiliriz:

```
foo
├── Cargo.toml
├── src
│   └── main.rs
│   └── lib.rs
└── tests
    ├── my_test.rs
    └── my_other_test.rs
```

`tests` dizinindeki her dosya ayrı bir entegrasyon testidir; yani, kütüphanenizi sanki ona bağımlı bir `crate` tarafından çağrılıyormuş gibi test eden birimlerdir.

**Testing** bölümü, üç farklı test stilini detaylıca açıklar:

* Birim testler (Unit tests)
* Dokümantasyon testleri (Doc tests)
* Entegrasyon testleri (Integration tests)

`cargo` doğal olarak tüm testlerinizi çalıştırmanın kolay bir yolunu sağlar:

```bash
cargo test
```

Şöyle bir çıktı görmelisiniz:

```bash
$ cargo test
   Compiling blah v0.1.0 (file:///nobackup/blah)
    Finished dev [unoptimized + debuginfo] target(s) in 0.89 secs
     Running target/debug/deps/blah-d3b32b97275ec472

running 4 tests
test test_bar ... ok
test test_baz ... ok
test test_foo_bar ... ok
test test_foo ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Belirli bir isimle eşleşen testleri de çalıştırabilirsiniz:

```bash
cargo test test_foo
```

Çıktı:

```bash
$ cargo test test_foo
   Compiling blah v0.1.0 (file:///nobackup/blah)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35 secs
     Running target/debug/deps/blah-d3b32b97275ec472

running 2 tests
test test_foo ... ok
test test_foo_bar ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

⚠️ Bir uyarı: Cargo testleri eşzamanlı (concurrently) çalıştırabilir, bu yüzden testlerinizin birbirleriyle çakışmamasına dikkat edin.

Bu eşzamanlı çalıştırmanın sorun yaratabileceği bir örnek, iki testin aynı dosyaya çıktı vermesidir:

```rust
#[cfg(test)]
mod tests {
    // Gerekli modülleri içe aktar
    use std::fs::OpenOptions;
    use std::io::Write;

    // Bu test bir dosyaya yazar
    #[test]
    fn test_file() {
        // ferris.txt dosyasını açar veya yoksa oluşturur.
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        // "Ferris" 5 kez yazdırır.
        for _ in 0..5 {
            file.write_all("Ferris\n".as_bytes())
                .expect("Could not write to ferris.txt");
        }
    }

    // Bu test aynı dosyaya yazmaya çalışır
    #[test]
    fn test_file_also() {
        // ferris.txt dosyasını açar veya yoksa oluşturur.
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ferris.txt")
            .expect("Failed to open ferris.txt");

        // "Corro" 5 kez yazdırır.
        for _ in 0..5 {
            file.write_all("Corro\n".as_bytes())
                .expect("Could not write to ferris.txt");
        }
    }
}
```

Beklenen çıktı şöyleydi:

```bash
$ cat ferris.txt
Ferris
Ferris
Ferris
Ferris
Ferris
Corro
Corro
Corro
Corro
Corro
```

Ama aslında `ferris.txt` içine şunlar yazılır:

```bash
$ cargo test test_file && cat ferris.txt
Corro
Ferris
Corro
Ferris
Corro
Ferris
Corro
Ferris
Corro
Ferris
```

Yani testler eşzamanlı çalıştığında dosya çıktısı karışabilir.
