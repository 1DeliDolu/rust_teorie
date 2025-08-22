## 🔄 Değiştirilebilirlik (mutability)

Değişken bağlamaları (variable bindings) varsayılan olarak değiştirilemezdir (immutable), ancak `mut` değiştiricisi (mut modifier) kullanılarak bu durum geçersiz kılınabilir.

```rust
fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Tamam
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Hata! Değiştirilemez bir değişkene yeni bir değer atanamaz
    _immutable_binding += 1;
}
```

Derleyici, değiştirilebilirlik (mutability) hataları hakkında ayrıntılı bir tanılama mesajı verir.
