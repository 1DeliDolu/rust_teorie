## 🔁 loop

Rust, sonsuz döngüyü belirtmek için `loop` anahtar kelimesini sağlar.

`break` ifadesi (statement) herhangi bir zamanda döngüden çıkmak için kullanılabilirken, `continue` ifadesi yinelemenin (iteration) geri kalan kısmını atlayıp yeni bir yinelemeye başlamak için kullanılır.

```rust
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}
```

👉 Bu örnekte `loop` sonsuz döngü başlatıyor, `continue` belirli bir adımı atlıyor, `break` ise döngüyü sonlandırıyor.
