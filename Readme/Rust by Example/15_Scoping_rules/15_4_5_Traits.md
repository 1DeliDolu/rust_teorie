## 🔗 Özellikler (traits)

Trait metotlarında yaşam süresi (lifetime) anotasyonları temelde fonksiyonlardaki ile aynıdır. Ayrıca `impl` tanımlarında da yaşam süresi anotasyonları bulunabilir.

```rust
// Yaşam süresi anotasyonu olan bir struct.
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// `impl` üzerinde yaşam süresi anotasyonu.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}
```
