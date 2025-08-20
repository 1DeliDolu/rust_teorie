## 🎭 Rust Demetler (Rust Tuples)

## 📌 Demetler (Tuples)

**Tuple (demet)**, farklı türlerdeki değerlerin tek bir değişkende saklanabildiği bir yapıdır.

Birden fazla değeri birlikte döndürmek veya işlemek istediğinizde çok kullanışlıdır.

---

## 🛠️ Demet Oluşturma (Create a Tuple)

Demetler, parantez `()` içine yazılır ve değerler virgülle ayrılır:

### Örnek (Example)

```rust
let person = ("John", 30, true);
```

👉 Bu demet bir `&str`, bir `i32` ve bir `bool` içerir.

---

## 🎯 Demet Elemanlarına Erişim (Access Tuple Values)

Demet elemanlarına nokta (`.`) ve indeks numarası ile erişilir:

### Örnek (Example)

```rust
let person = ("John", 30, true);
println!("Name: {}", person.0);
println!("Age: {}", person.1);
println!("Is active: {}", person.2);
```

---

## 📦 Demeti Açma (Unpack a Tuple)

Bir demet oluşturulduğunda genellikle değerler ona atanır. Buna **packing** denir:

```rust
let person = ("Jenny", 45, false);
```

Rust’ta ayrıca değerleri ayrı değişkenlere çıkartabilirsiniz. Buna **unpacking** denir:

### Örnek (Example)

```rust
let person = ("Jenny", 45, false);
let (name, age, active) = person;

println!("Name: {}", name);
println!("Age: {}", age);
println!("Active: {}", active);
```

---

## 🔄 Fonksiyondan Demet Döndürme (Return a Tuple from a Function)

Demetler, fonksiyonlardan birden fazla değer döndürmek için sıkça kullanılır:

### Örnek (Example)

```rust
fn get_user() -> (String, i32) {
  (String::from("Liam"), 25)
}

fn main() {
  let user = get_user();
  println!("User: {} ({} years old)", user.0, user.1);
}
```

👉 Burada `get_user` fonksiyonu bir `String` ve bir `i32` döndürür.
Fonksiyon çağrıldığında, demet kullanılarak değerler birlikte alınır.
