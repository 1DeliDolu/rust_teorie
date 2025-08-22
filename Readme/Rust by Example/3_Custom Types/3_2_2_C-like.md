## 🎨 C-benzeri Enum’lar (C-like enums)

`enum`, C-benzeri enum’lar şeklinde de kullanılabilir.

```rust
// Kullanılmayan kodlar için uyarıları gizlemek üzere bir öznitelik
#![allow(dead_code)]

// Örtük ayıraç (discriminator) ile enum (0’dan başlar)
enum Number {
    Zero,
    One,
    Two,
}

// Açık ayıraç (discriminator) ile enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enum` değerleri tamsayıya dönüştürülebilir (cast).
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
```

👉 Bu örnekte, Rust `enum`’larının hem otomatik (örtük) hem de açıkça belirtilmiş ayıraçlarla (discriminator) kullanılabileceği ve tamsayıya dönüştürülebileceği gösterilmektedir.
