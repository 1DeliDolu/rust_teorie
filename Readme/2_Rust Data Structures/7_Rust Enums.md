## 🔄 Rust Enums (Numaralandırmalar)

## 📌 Enum Nedir?

**Enum** (enumeration’ın kısaltması), bir değişkenin yalnızca birkaç farklı değerden biri olabileceğini tanımlayan özel bir veri tipidir.

Her enum değerine **variant** (varyant) denir.
Enum’lar genellikle:

* Haftanın günleri,
* Yönler (sağ, sol, yukarı, aşağı),
* Başarı / hata durumları gibi seçenekler için kullanılır.

---

## 🛠️ Enum Oluşturma (Create an Enum)

`enum` anahtar kelimesiyle tanımlanır:

```rust
enum Direction {
  Up,
  Down,
  Left,
  Right,
}
```

Enum kullanmak için, değişkene `::` ile bir variant atanır:

```rust
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn main() {
  let my_direction = Direction::Up;
  println!("We are going up!");
}
```

---

## 🎭 Enum ile Match Kullanımı (Match on Enum Values)

Enum’lar `match` ifadesiyle birlikte çok güçlüdür.
Hangi varyant seçilmişse, ona uygun kod çalışır:

```rust
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn main() {
  let my_direction = Direction::Left;

  match my_direction {
    Direction::Up => println!("Going up"),
    Direction::Down => println!("Going down"),
    Direction::Left => println!("Going left"),
    Direction::Right => println!("Going right"),
  }
}
```

---

## 📦 Veri Taşıyan Enum’lar (Enums with Data)

Enum varyantları aynı zamanda **veri** de saklayabilir.
Bu, her varyantın ekstra bilgiye ihtiyaç duyduğu durumlarda faydalıdır.

```rust
enum LoginStatus {
  Success(String),
  Error(String),
}

fn main() {
  let result1 = LoginStatus::Success(String::from("Welcome, John!"));
  let result2 = LoginStatus::Error(String::from("Incorrect password"));

  match result1 {
    LoginStatus::Success(message) => println!("Success: {}", message),
    LoginStatus::Error(message) => println!("Error: {}", message),
  }
}
```

---

## 🎯 Neden Enum Kullanılır? (Why Use Enums?)

* İlgili değerleri **tek bir tip** altında toplamak için
* Kodu daha **okunabilir ve güvenli** yapmak için
* Farklı durumları `match` ile kolayca yönetmek için

---

👉 İstersen sana Rust’taki **enum + struct birleşimini (örneğin `Option<T>` veya `Result<T, E>` gibi)** gösterebilirim. Onlar Rust’ta çok sık kullanılıyor. Anlatayım mı?
