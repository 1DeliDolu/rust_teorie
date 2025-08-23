## 📌 İşaretçiler / Referanslar (pointers / ref)

İşaretçiler (pointers) için **destructuring (parçalama)** ve **dereferencing (çözme)** arasında bir ayrım yapılmalıdır. Bunlar C/C++ gibi dillerden farklı şekilde kullanılır.

* Dereferencing `*` kullanır.
* Destructuring `&`, `ref`, ve `ref mut` kullanır.

```rust
fn main() {
    // `i32` tipinde bir referans ata. `&`, bir referans atandığını belirtir.
    let reference = &4;

    match reference {
        // Eğer `reference`, `&val` ile eşleştirilirse şu karşılaştırma yapılır:
        // `&i32`
        // `&val`
        // ^ Eğer eşleşen `&` işaretleri atılırsa, `i32` değeri `val`’e atanır.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // `&` kullanmamak için, eşleştirmeden önce dereference işlemi yapılır.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // Peki ya başlangıçta referans olmazsa?
    // `reference` bir `&` idi çünkü sağ taraf zaten bir referanstı.
    // Bu durumda sağ taraf referans olmadığı için bu da referans değildir.
    let _not_a_reference = 3;

    // Rust bu amaç için `ref` sağlar.
    // Bu, atamayı değiştirerek eleman için bir referans oluşturur.
    let ref _is_a_reference = 3;

    // Buna göre, referans olmadan tanımlanan iki değere
    // `ref` ve `ref mut` ile referans alınabilir.
    let value = 5;
    let mut mut_value = 6;

    // Referans oluşturmak için `ref` anahtar kelimesini kullan.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Benzer şekilde `ref mut` kullanılır.
    match mut_value {
        ref mut m => {
            // Bir referans alındı. Üzerine ekleme yapmadan önce dereference gerekir.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
```

👉 Bu örnekte `&`, `ref` ve `ref mut` kullanılarak referanslar nasıl alınır ve `*` operatörüyle nasıl dereference yapılır gösterilmektedir.
