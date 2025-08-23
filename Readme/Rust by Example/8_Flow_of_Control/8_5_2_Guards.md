## 🛡️ Korumalar (guards)

Bir `match guard`, `arm` (kol) için filtreleme yapmak amacıyla eklenebilir:

```rust
#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);
    // ^ TODO: `temperature` için farklı değerler deneyin

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // Buradaki `if condition` bir guard’dır
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }
}
```

👉 Bu örnekte `if` koşulu ile ek filtreleme yapılarak eşleşme kontrolü hassaslaştırılır.

---

⚠️ Derleyici, `guard` koşullarını, `match` ifadesinde tüm kalıpların (patterns) kapsanıp kapsanmadığını kontrol ederken dikkate almaz:

```rust
fn main() {
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        // _ => unreachable!("Should never happen."),
        // TODO ^ derleme hatasını düzeltmek için yorum satırını kaldırın
    }
}
```

👉 Burada tüm olasılıkların kapsanması için `_` ile bir default case eklenmelidir, çünkü `guard` koşulları bu kapsama hesaplamasına dahil edilmez.
