## 🔀 Değişken Sayıda Argüman Alan Arayüzler (Variadic Interfaces)

Bir **variadic arayüz (interface)**, keyfi sayıda argüman alır. Örneğin, `println!` biçimlendirme dizesine (format string) bağlı olarak keyfi sayıda argüman kabul edebilir.

Önceki bölümdeki `calculate!` makromuzu variadic olacak şekilde genişletebiliriz:

```rust
macro_rules! calculate {
    // Tek bir `eval` için desen
    (eval $e:expr) => {
        {
            let val: usize = $e; // Türleri tamsayıya zorla
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    // Birden fazla `eval`’ı özyinelemeli (recursive) olarak çözümle
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    calculate! { // Bakın! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
```

### 📤 Çıktı:

```
1 + 2 = 3
3 + 4 = 7
(2 * 3) + 1 = 7
```
