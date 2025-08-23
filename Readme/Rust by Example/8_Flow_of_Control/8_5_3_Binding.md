## 🔗 Bağlama (binding)

Bir değişkene dolaylı erişim yapmak, onu yeniden bağlamadan (re-binding) kullanmayı ve dallandırmayı imkansız hale getirir. `match`, değerleri isimlere bağlamak için `@` işaretini sağlar:

```rust
// `u32` döndüren bir `age` fonksiyonu.
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // Doğrudan `1 ..= 12` eşleştirilebilirdi ama o zaman çocuk kaç yaşında olurdu?
        // Bunun yerine `n` olarak bağlanır. Böylece yaş raporlanabilir.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Hiçbir şey bağlanmaz. Sonuç döndürülür.
        n             => println!("I'm an old person of age {:?}", n),
    }
}
```

👉 Burada `@` kullanılarak eşleşen değeri hem kontrol etmek hem de bir değişkene bağlamak mümkündür.

---

Enum varyantları da `binding` ile parçalanabilir (destructure), örneğin `Option`:

```rust
fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    match some_number() {
        // `Some` varyantı geldi, değeri `n`'e bağlanır ve 42 olup olmadığı kontrol edilir.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Başka herhangi bir sayı eşleşirse.
        Some(n)      => println!("Not interesting... {}", n),
        // Diğer her şey (`None` varyantı) için.
        _            => (),
    }
}
```

👉 Bu örnekte `@` ile `Option` içindeki değer hem filtrelenir hem de bir değişkene atanarak kullanılabilir.
