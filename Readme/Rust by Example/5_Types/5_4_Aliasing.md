## 🏷️ Tür Yeniden Adlandırma (aliasing)

`type` ifadesi mevcut bir türe yeni bir ad vermek için kullanılabilir. Tür adları **UpperCamelCase** biçiminde olmalıdır, aksi halde derleyici uyarı verecektir. Bu kuralın istisnası, `usize`, `f32` gibi ilkel türlerdir (primitive types).

```rust
// `NanoSecond`, `Inch` ve `U64`, `u64` için yeni adlardır.
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    // Dikkat: Tür takma adları (type aliases) ek tür güvenliği sağlamaz,
    // çünkü bunlar yeni türler değildir
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
```

Tür takma adlarının (aliases) temel kullanım amacı tekrar eden kodu azaltmaktır.
Örneğin, `io::Result<T>` türü aslında `Result<T, io::Error>` türü için bir takma addır.
