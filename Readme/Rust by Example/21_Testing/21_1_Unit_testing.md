## 🧪 Birim Testleri (unit testing)

Testler, Rust fonksiyonlarıdır ve test edilmeyen kodun beklenen şekilde çalıştığını doğrular. Test fonksiyonlarının gövdesi genellikle bazı hazırlıklar yapar, test etmek istediğimiz kodu çalıştırır ve ardından sonuçların beklendiği gibi olup olmadığını `assert` ifadeleriyle kontrol eder.

Çoğu birim testi `#[cfg(test)]` özniteliği ile işaretlenmiş bir `tests` modülü içine yazılır. Test fonksiyonları `#[test]` özniteliği ile işaretlenir.

Test fonksiyonu içinde bir `panic!` meydana geldiğinde test başarısız olur. Bunun için bazı yardımcı makrolar (helper macros) vardır:

* `assert!(expression)` → ifade `false` olursa panic! eder.
* `assert_eq!(left, right)` ve `assert_ne!(left, right)` → iki ifadenin eşitliğini veya eşitsizliğini test eder.

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Bu gerçekten kötü bir toplama fonksiyonudur, bu örnekte başarısız olması için yazılmıştır.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Önemli bir kullanım: dış scope’tan (mod tests için) isimleri içeri aktarma.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // Bu assert çalışır ve test başarısız olur.
        // Özel (private) fonksiyonların da test edilebildiğine dikkat edin!
        assert_eq!(bad_add(1, 2), 3);
    }
}
```

Testler `cargo test` ile çalıştırılabilir.

```bash
$ cargo test
```

---

## ❓ Testler ve `?` Operatörü

Önceki birim test örneklerinin hiçbirinde dönüş tipi yoktu. Ancak Rust 2018 ile birlikte test fonksiyonları `Result<()>` döndürebilir. Bu da `?` operatörünü testlerde kullanabilmeyi sağlar ve kodu daha özlü hale getirir.

```rust
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}
```

---

## ⚠️ Panic Testleri (testing panics)

Bazı fonksiyonların belirli koşullarda `panic!` etmesi beklenebilir. Bu durumda `#[should_panic]` özniteliği kullanılır. Bu öznitelik isteğe bağlı olarak `expected = "mesaj"` parametresini alabilir. Eğer fonksiyon farklı şekillerde panic edebiliyorsa, doğru panic mesajının test edildiğinden emin olmak için bu parametre yararlıdır.

Not: Rust ayrıca kısayol biçimini de destekler: `#[should_panic = "mesaj"]`. Bu, `#[should_panic(expected = "mesaj")]` ile aynı şekilde çalışır. İkinci biçim daha açık olduğu için daha çok tercih edilir.

```rust
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }

    #[test]
    #[should_panic = "Divide result is zero"] // Bu da çalışır
    fn test_specific_panic_shorthand() {
        divide_non_zero_result(1, 10);
    }
}
```

---

## 🎯 Belirli Testleri Çalıştırma (running specific tests)

Belirli testleri çalıştırmak için `cargo test` komutuna test adını verebilirsiniz:

```bash
$ cargo test test_any_panic
```

Birden fazla testi çalıştırmak için test adının bir kısmını yazabilirsiniz. Bu durumda o kısma uyan tüm testler çalıştırılır:

```bash
$ cargo test panic
```

---

## 🚫 Testleri Yoksayma (ignoring tests)

Bazı testler `#[ignore]` özniteliği ile işaretlenerek varsayılan çalıştırmadan hariç tutulabilir. Ancak bu testler `cargo test -- --ignored` komutu ile çalıştırılabilir.

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_hundred() {
        assert_eq!(add(100, 2), 102);
        assert_eq!(add(2, 100), 102);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }
}
```

```bash
$ cargo test
$ cargo test -- --ignored
```
