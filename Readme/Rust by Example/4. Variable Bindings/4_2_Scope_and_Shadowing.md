## 📏 Kapsam ve Gölgeleme (scope and shadowing)

Değişken bağlamalarının (variable bindings) bir kapsamı (scope) vardır ve yalnızca bir blok (block) içinde yaşarlar. Blok, süslü parantezler `{}` ile çevrili deyimler (statements) koleksiyonudur.

```rust
fn main() {
    // Bu bağlama main fonksiyonu içinde yaşar
    let long_lived_binding = 1;

    // Bu bir bloktur ve main fonksiyonundan daha küçük bir kapsama sahiptir
    {
        // Bu bağlama yalnızca bu blokta vardır
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // Bloğun sonu

    // Hata! `short_lived_binding` bu kapsamda mevcut değil
    println!("outer short: {}", short_lived_binding);
    // FIXME ^ Bu satırı yorum satırı yapın

    println!("outer long: {}", long_lived_binding);
}
```

Ayrıca, değişken gölgelemesine (variable shadowing) de izin verilir.

```rust
fn main() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // Bu bağlama dıştakini *gölgeler*
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // Bu bağlama, önceki bağlamayı *gölgeler*
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
```
