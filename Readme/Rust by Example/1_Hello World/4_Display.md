## 🖋️ Display / Görüntüleme

`fmt::Debug` pek kompakt ve temiz görünmez, bu yüzden çıktı görünümünü özelleştirmek genellikle avantajlıdır. Bu, `{}` yazdırma işaretleyicisini kullanan `fmt::Display` trait’ini manuel olarak uygulayarak yapılır. Uygulaması şu şekilde görünür:

```rust
// `fmt` modülünü kullanılabilir hale getirmek için içe aktar (use).
use std::fmt;

// `fmt::Display` uygulanacak bir yapı tanımla.
// Bu, bir `i32` içeren `Structure` adlı bir tuple struct’tır.
struct Structure(i32);

// `{}` işaretleyicisini kullanabilmek için
// `fmt::Display` trait’i bu tür için manuel olarak uygulanmalıdır.
impl fmt::Display for Structure {
    // Bu trait, tam olarak bu imzaya sahip bir `fmt` fonksiyonu gerektirir.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Sadece ilk elemanı çıktı akışına (`f`) yaz.
        // `fmt::Result` işlemin başarılı olup olmadığını bildirir.
        // `write!`, `println!`’a çok benzer bir sözdizimi kullanır.
        write!(f, "{}", self.0)
    }
}
```

---

`fmt::Display`, `fmt::Debug`’dan daha temiz olabilir ama bu standart kütüphane (std library) için bir problem doğurur. Belirsiz türler nasıl gösterilmelidir?

Örneğin, std kütüphanesi tüm `Vec<T>` türleri için tek bir stil uygulasaydı bu stil ne olurdu?

* `Vec<path>`: `/:/etc:/home/username:/bin` (":" ile ayrılmış)
* `Vec<number>`: `1,2,3` ("," ile ayrılmış)

Hayır, çünkü tüm türler için ideal bir stil yoktur ve std kütüphanesi bunu dayatmaz. Bu nedenle `fmt::Display`, `Vec<T>` veya diğer jenerik (generic) kapsayıcılar için uygulanmaz. Bu gibi durumlarda `fmt::Debug` kullanılmalıdır.

Fakat yeni bir kapsayıcı türü jenerik değilse, `fmt::Display` manuel olarak uygulanabilir.

---

```rust
use std::fmt; // `fmt` içe aktar

// İki sayı tutan bir yapı. `Debug` türetilecek (derive) böylece
// sonuçlar `Display` ile karşılaştırılabilir.
#[derive(Debug)]
struct MinMax(i64, i64);

// `MinMax` için `Display` uygula.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Karşılaştırma için alanları (fields) isimlendirilebilir olan bir yapı tanımla.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Benzer şekilde, `Point2D` için `Display` uygula.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Hata: Hem `Debug` hem de `Display` uygulanmış olsa da,
    // `{:b}` `fmt::Binary` uygulanmasını gerektirir.
    // println!("What does Point2D look like in binary: {:b}?", point);
}
```

---

## ⚠️ Not

`fmt::Display` uygulanmış ancak `fmt::Binary` uygulanmamıştır, bu nedenle kullanılamaz.
`std::fmt` içinde bunun gibi birçok trait vardır ve her biri kendi uygulamasını gerektirir. Ayrıntılar `std::fmt` içinde anlatılmıştır.

---

## 🏋️ Aktivite (Activity)

Yukarıdaki örneğin çıktısını gördükten sonra, `Point2D` struct’ını örnek alarak `Complex` struct’ını ekleyin. Yazdırıldığında çıktısı şu şekilde olmalıdır:

```
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```

---

## 📚 Ayrıca bakınız (See also)

* `derive`
* `std::fmt`
* `macros`
* `struct`
* `trait`
* `use`
