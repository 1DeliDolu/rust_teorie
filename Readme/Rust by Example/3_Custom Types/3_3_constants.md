## ⚓ Sabitler (constants)

Rust’ta **her kapsamda** (global dahil) tanımlanabilen iki farklı sabit türü vardır. Her ikisi de açık tür açıklaması gerektirir:

* `const`: Değiştirilemeyen değer (en yaygın kullanım).
* `static`: `'static` ömrüne sahip (lifetime) bir değişken. Değiştirilebilir (`mutable`) olabilir. Statik ömür derleyici tarafından çıkarılır ve belirtilmesine gerek yoktur. Ancak değiştirilebilir (`mutable static`) bir değişkene erişmek veya onu değiştirmek **unsafe** kabul edilir.

```rust
// Global sabitler tüm diğer kapsamların dışında tanımlanır.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Bir fonksiyon içinde sabite erişim
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Ana thread içinde sabitlere erişim
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Hata! Bir `const` değiştirilemez.
    THRESHOLD = 5;
    // FIXME ^ Bu satırı yorum satırı haline getirin
}
```

👉 Bu örnekte `static` ve `const` sabitlerinin nasıl tanımlanıp kullanıldığı gösterilmektedir.
