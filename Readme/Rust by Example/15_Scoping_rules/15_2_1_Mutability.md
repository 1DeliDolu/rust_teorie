## 🔄 Değiştirilebilirlik (mutability)

Verilerin değiştirilebilirliği (mutability), sahiplik (ownership) aktarıldığında değiştirilebilir.

```rust
fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Değiştirilebilirlik hatası
    //*immutable_box = 4;

    // Kutuyu *taşı*, sahipliği (ve değiştirilebilirliği) değiştir
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Kutunun içeriğini değiştir
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
```

👉 Bu örnekte `immutable_box` değiştirilemezken, sahiplik `mutable_box`’a taşındığında `mut` ile tanımlandığı için artık içeriği değiştirilebilir hale gelir.
