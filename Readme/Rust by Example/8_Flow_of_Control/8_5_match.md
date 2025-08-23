## 🎭 match

Rust, `match` anahtar kelimesi ile desen eşleştirme (pattern matching) sağlar. Bu yapı C dilindeki `switch` ifadesine benzer. İlk eşleşen dal (arm) çalıştırılır ve tüm olası değerler kapsanmak zorundadır.

```rust
fn main() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}
```

👉 Bu örnekte `match` tek değer, birden fazla değer, aralık (range) ve tüm diğer durumları (`_`) kapsayacak şekilde kullanılıyor. Ayrıca `match` bir ifade (expression) olduğu için değer döndürebiliyor (`binary` değişkenine atanması gibi).
