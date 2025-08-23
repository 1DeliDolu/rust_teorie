## 🏷️ Belirleyiciler (Designators)

Bir makronun argümanları `\$` işareti ile başlar ve bir **belirleyici (designator)** ile tür ek açıklaması yapılır:

```rust
macro_rules! create_function {
    // Bu makro, `ident` belirleyicisine sahip bir argüman alır
    // ve `$func_name` adında bir fonksiyon oluşturur.
    // `ident` belirleyicisi değişken/fonksiyon isimleri için kullanılır.
    ($func_name:ident) => {
        fn $func_name() {
            // `stringify!` makrosu bir `ident`'i string'e dönüştürür.
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

// Yukarıdaki makro ile `foo` ve `bar` adlı fonksiyonlar oluşturulur.
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // Bu makro, `expr` türünde bir ifade alır ve
    // hem ifadenin kendisini hem de sonucunu ekrana yazdırır.
    // `expr` belirleyicisi ifadeler (expressions) için kullanılır.
    ($expression:expr) => {
        // `stringify!` ifadeyi *olduğu gibi* string'e dönüştürür.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // Hatırlayın: bloklar da bir ifadedir!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
```

## 📋 Kullanılabilir bazı belirleyiciler

* `block`
* `expr` → ifadeler (expressions) için kullanılır
* `ident` → değişken/fonksiyon adları için kullanılır
* `item`
* `literal` → sabit değerler (literal constants) için kullanılır
* `pat` → desenler (patterns) için kullanılır
* `path`
* `stmt` → ifadeler (statements) için kullanılır
* `tt` → token ağacı (token tree) için kullanılır
* `ty` → türler (types) için kullanılır
* `vis` → görünürlük niteleyicisi (visibility qualifier) için kullanılır

Tam liste için Rust Reference’a bakınız.
