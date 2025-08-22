## 🔢 Sabitler (literals)

Sayısal sabitler (numeric literals), tür eklenerek (suffix) tür açıklamalı hale getirilebilir. Örneğin, `42` sabitinin türünün `i32` olmasını belirtmek için `42i32` yazılır.

Tür eklenmemiş sayısal sabitlerin türü ise kullanım biçimlerine bağlıdır. Eğer hiçbir kısıtlama yoksa, derleyici tamsayılar için `i32`, kayan noktalı sayılar için `f64` türünü kullanır.

```rust
fn main() {
    // Ekli sabitler, türleri başlatma sırasında bellidir
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Tür eklenmemiş sabitler, türleri kullanım biçimlerine bağlıdır
    let i = 1;
    let f = 1.0;

    // `size_of_val`, bir değişkenin bayt cinsinden boyutunu döner
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
```

Yukarıdaki kodda kullanılan fakat henüz açıklanmamış bazı kavramlar:

`std::mem::size_of_val` bir fonksiyondur, ancak tam yoluyla (full path) çağrılmıştır. Kod, modül (module) adı verilen mantıksal birimlere ayrılabilir. Bu durumda `size_of_val` fonksiyonu `mem` modülü içinde tanımlıdır ve `mem` modülü `std` crate’i içinde yer alır. Daha fazla bilgi için modüller (modules) ve crate’lere bakınız.
