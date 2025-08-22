## 🛠️ Rust Fonksiyonlar (Rust Functions)

## 📌 Fonksiyonlar (Functions)

Fonksiyon (function), yalnızca çağrıldığında çalışan bir kod bloğudur.

Fonksiyonlar kodunuzu düzenlemek, tekrar eden kodları önlemek ve programınızı daha anlaşılır hale getirmek için kullanılır.

---

## 📝 Fonksiyon Oluşturma (Creating a Function)

Bir fonksiyon oluşturmak için `fn` anahtar kelimesi, ardından fonksiyon adı, parantez `()` ve süslü parantez `{}` kullanılır:

### Örnek (Example)

```rust
fn function_name() {
  // code to be executed
}
```

---

## ▶️ Fonksiyon Çağırma (Calling a Function)

Bir fonksiyon oluşturduktan sonra, onu çağırarak çalıştırabilirsiniz.

Bir fonksiyonu çağırmak için fonksiyon adını yazıp ardından `()` ekleyin:

### Örnek (Example)

```rust
// Create a function
fn say_hello() {
  println!("Hello from a function!");
}

say_hello(); // Call the function
```

---

## 📩 Parametreli Fonksiyonlar (Functions with Parameters)

Fonksiyonlara parametre (parameter) göndererek bilgi aktarabilirsiniz. Parametreler parantez `()` içine yazılır.

### Örnek (Example)

```rust
fn greet(name: &str) {
  println!("Hello, {}!", name);
}

greet("John");
```

👉 Bu örnekte fonksiyon, `name` adlı bir string parametresi alır ve selamlama mesajında yazdırır.

---

## 🔄 Değer Döndüren Fonksiyonlar (Functions with Return Values)

Fonksiyonlar bir değer de döndürebilir.

Fonksiyonun döndüreceği türü belirtmek için fonksiyon başlığında `->` sembolü kullanılır.
Fonksiyon içinde değeri geri göndermek için `return` kullanılır:

### Örnek (Example)

```rust
fn add(a: i32, b: i32) -> i32 {
  return a + b;
}

let sum = add(3, 4);
println!("Sum is: {}", sum);
```

👉 Bu fonksiyon iki sayıyı toplar ve sonucu döndürür.

Rust’ta `return` yazmak zorunlu değildir. Fonksiyonun son satırında noktalı virgül (`;`) olmadan değer yazarsanız, otomatik olarak döndürülür:

### Örnek (Example)

```rust
fn add(a: i32, b: i32) -> i32 {
  a + b
}

let sum = add(3, 4);
println!("Sum is: {}", sum);
```

👉 Burada `a + b` otomatik olarak döndürülür.

Her iki yöntem de aynı işi yapar, hangisini kullanacağınız size kalmıştır.

---

## 🎯 Neden Fonksiyon Kullanılır? (Why Use Functions?)

* Kodunuzu düzenlemek için
* Aynı kodu tekrar tekrar yazmamak için
* Programlarınızı daha okunabilir ve değiştirilebilir yapmak için
