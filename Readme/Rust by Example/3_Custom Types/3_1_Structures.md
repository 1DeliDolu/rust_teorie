## 🏗️ Yapılar (structures)

`struct` anahtar kelimesi ile üç tür yapı (struct) oluşturulabilir:

* **Tuple struct’lar**: Temel olarak adlandırılmış tuple’lardır.
* **Klasik C tarzı struct’lar**: Alanlara sahip yapılardır.
* **Unit struct’lar**: Alansız yapılardır, özellikle generic yapılar için kullanışlıdır.

```rust
// Kullanılmayan kodlar için uyarıları gizlemek üzere bir öznitelik
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Bir unit struct
struct Unit;

// Bir tuple struct
struct Pair(i32, f32);

// İki alanlı bir struct
struct Point {
    x: f32,
    y: f32,
}

// Struct’lar başka bir struct içinde alan olarak yeniden kullanılabilir
struct Rectangle {
    // Bir dikdörtgen, sol üst ve sağ alt köşeleri ile tanımlanabilir.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Alan başlatma kısayolu ile struct oluşturma
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Debug formatında struct yazdırma
    println!("{:?}", peter);

    // Bir `Point` örneği oluşturma
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Point’in alanlarına erişim
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Struct update sözdizimi ile yeni bir nokta oluşturma
    let bottom_right = Point { x: 10.3, ..another_point };

    // `bottom_right.y`, `another_point.y` ile aynı olacak
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // `let` bağlaması ile destructuring yapma
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct oluşturma da bir ifadedir
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Bir unit struct örneği oluşturma
    let _unit = Unit;

    // Bir tuple struct örneği oluşturma
    let pair = Pair(1, 0.1);

    // Tuple struct alanlarına erişim
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Tuple struct destructuring
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
```

👉 Bu örnek, Rust’ta farklı struct türlerinin nasıl tanımlanıp kullanılabileceğini göstermektedir.

---

## 📝 Aktivite

* Bir `rect_area` fonksiyonu ekleyin. Bu fonksiyon, bir `Rectangle`’ın alanını hesaplasın (iç içe destructuring kullanmayı deneyin).
* Bir `square` fonksiyonu ekleyin. Bu fonksiyon bir `Point` ve `f32` argümanları alacak, verilen noktayı sol üst köşe kabul ederek genişlik ve yüksekliği `f32` olan bir `Rectangle` döndürecek.

---

## 📚 Ayrıca bakınız

* `attributes`
* `raw identifiers`
* `destructuring`
