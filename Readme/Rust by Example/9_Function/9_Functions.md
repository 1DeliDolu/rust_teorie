## 🛠️ Fonksiyonlar (functions)

Fonksiyonlar `fn` anahtar sözcüğü kullanılarak tanımlanır. Argümanları, tıpkı değişkenlerde olduğu gibi tür açıklamalı (type annotated) olmalıdır ve eğer fonksiyon bir değer döndürecekse, dönüş türü bir `->` okundan sonra belirtilmelidir.

Fonksiyondaki son ifade dönüş değeri olarak kullanılır. Alternatif olarak, `return` ifadesi kullanılarak fonksiyon içinden, hatta döngülerden veya `if` ifadelerinden bile, daha erken dönüş yapılabilir.

Hadi, FizzBuzz örneğini fonksiyonlar kullanarak yeniden yazalım!

```rust
// C/C++'dan farklı olarak, fonksiyon tanımlarının sırası konusunda kısıtlama yoktur
fn main() {
    // Bu fonksiyonu burada kullanabiliriz, tanımı daha sonra yapılabilir
    fizzbuzz_to(100);
}

// Boolean değer döndüren fonksiyon
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Köşe durumu, erken dönüş
    if rhs == 0 {
        return false;
    }

    // Bu bir ifadedir, burada `return` anahtar sözcüğü gerekli değildir
    lhs % rhs == 0
}

// "Değer döndürmeyen" fonksiyonlar aslında birim türünü `()` döndürür
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// Bir fonksiyon `()` döndürdüğünde, dönüş türü imzadan çıkarılabilir
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
```
