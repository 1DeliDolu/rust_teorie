## 🏗️ Rust Structs (Yapılar)

## 📌 Struct Nedir?

**Struct** (structure’ın kısaltması), ilişkili değerleri bir araya toplamak için kullanılan özel bir veri yapısıdır.

Bir struct’ı, tek bir şeyin küçük bir veritabanı gibi düşünebilirsiniz. Örneğin bir kişinin **adı, yaşı ve oy kullanma durumu** bir struct içinde saklanabilir.

---

## 🛠️ Struct Oluşturma (Create a Struct)

`struct` anahtar kelimesiyle tanımlanır, alanlar (fields) süslü parantezler `{}` içine yazılır:

### Örnek (Example)

```rust
struct Person {
  name: String,
  age: u32,
  can_vote: bool,
}
```

Bir struct oluşturduktan sonra, onun **nesnesini (object)** yaratabiliriz.
Alanlara **dot syntax (.)** ile erişilir.

```rust
// Create a Struct called Person
struct Person {
  name: String,
  age: u32,
  can_vote: bool,
}

fn main() {
  // Create a Person object
  let user = Person {
    name: String::from("John"),
    age: 35,
    can_vote: true,
  };

  // Access and print the values
  println!("Name: {}", user.name);
  println!("Age: {}", user.age);
  println!("Can vote? {}", user.can_vote);
}
```

---

## ✏️ Struct İçinde Değer Güncelleme (Change a Field)

Bir alanı değiştirmek için struct nesnesini **mutable (`mut`)** yapmalısınız:

### Örnek (Example)

```rust
struct Person {
  name: String,
  age: u32,
}

fn main() {
  let mut user = Person {
    name: String::from("John"),
    age: 35,
  };

  user.age = 36; // Change value of age
  println!("Name: {}", user.name);
  println!("Updated age: {}", user.age);
}
```

---

## 🎯 Neden Struct Kullanılır? (Why Use Structs?)

* İlgili verileri **temiz bir şekilde** gruplamak için
* Kodu daha **okunabilir ve bakımı kolay** hale getirmek için
* Gerçek dünya varlıklarını (kullanıcı, kitap, araba vb.) temsil etmek için

---

👉 İstersen seninle **struct metotları (impl)** ve **tuple structs vs unit structs** konularına da girebiliriz. Onları da anlatayım mı?
