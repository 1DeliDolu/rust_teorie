## 🔗 Entegrasyon Testleri (integration testing)

Birim testleri (unit tests) tek bir modülü izole şekilde test eder: küçüktürler ve özel (private) kodu da test edebilirler. Entegrasyon testleri (integration tests) ise crate’inizin dışında bulunur ve yalnızca halka açık (public) arayüzünü kullanır, tıpkı başka herhangi bir kodun kullanacağı gibi. Amaç, kütüphanenizin birçok parçasının birlikte doğru çalıştığını test etmektir.

Cargo, entegrasyon testlerini `src` klasörünün yanında bulunan `tests` dizininde arar.

---

### 📂 Örnek Dosya Yapısı

`src/lib.rs` dosyası:

```rust
// Bunu `adder` adlı bir crate içinde tanımlayın.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

`tests/integration_test.rs` dosyası:

```rust
#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
```

---

### ▶️ Testleri Çalıştırma

```bash
$ cargo test
```

Çıktı:

```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-bcd60824f5fbfe19

running 1 test
test test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 📂 Ortak Kod Paylaşımı (sharing code between integration tests)

`tests` dizini altındaki her Rust kaynak dosyası ayrı bir crate olarak derlenir. Entegrasyon testleri arasında bazı kodları paylaşmak için, ortak fonksiyonları içeren bir modül oluşturabilir ve bunu testlerde kullanabilirsiniz.

`tests/common/mod.rs` dosyası:

```rust
pub fn setup() {
    // bazı hazırlık kodları, örn. gerekli dosya/dizin oluşturma, 
    // sunucu başlatma vb.
}
```

`tests/integration_test.rs` dosyası:

```rust
// ortak modülü içe aktarma
mod common;

#[test]
fn test_add() {
    // ortak kodu kullanma
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}
```

⚠️ Modülü `tests/common.rs` olarak oluşturmak da mümkündür, fakat önerilmez. Çünkü test çalıştırıcısı bu dosyayı ayrı bir test crate’i olarak görecek ve içindeki testleri çalıştırmaya çalışacaktır.
