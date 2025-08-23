## 🗂️ HashMap

Vektörler (vectors) değerleri tamsayı indeksleriyle saklarken, **HashMap**’ler değerleri anahtar (key) ile saklar.
HashMap anahtarları `Eq` ve `Hash` trait’lerini uygulayan her tür olabilir: boolean, tamsayı, string vb.

Vektörler gibi HashMap’ler de büyüyebilir, ancak ayrıca fazla alanları olduğunda kendilerini küçültebilirler.
Belirli bir başlangıç kapasitesiyle bir HashMap oluşturmak için `HashMap::with_capacity(uint)` kullanılabilir, ya da `HashMap::new()` ile varsayılan kapasiteye sahip bir HashMap oluşturulabilir (önerilen yöntem).

```rust
use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}

fn main() { 
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Bir referans alır ve Option<&V> döndürür
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` eğer değer yeni ise `None`,
    // zaten varsa `Some(value)` döndürür
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley"); 

    // `HashMap::iter()`, rastgele sırada (&'a key, &'a value) çiftleri döndüren
    // bir yineleyici (iterator) döndürür.
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number)); 
    }
}
```

👉 HashMap’lerin (ya da hash tabloların) nasıl çalıştığı hakkında daha fazla bilgi için [Hash Table - Wikipedia](https://en.wikipedia.org/wiki/Hash_table) sayfasına bakabilirsiniz.
