## 🔤 Rust String’ler (Rust Strings)

## 📌 String’ler (Strings)

**String’ler**, metin (text) saklamak için kullanılır.

Daha önce, `&str` türünü kullanarak bir string oluşturabileceğinizi öğrenmiştiniz:

### Örnek (Example)

```rust
let greeting: &str = "Hello";
println!("{}", greeting);
```

👉 String’ler çift tırnak `"` içine yazılır (`"Hello"`).

Rust’ta iki temel string türü vardır:

* `&str` → **string slices**, sabit metinler için kullanılır (örneğin `"Hello"`)
* `String` → değiştirilebilir string’ler için kullanılır

Bu bölümde çoğunlukla **String** türü ile çalışacaksınız çünkü daha esnektir ve zamanla değiştirilebilir.

---

## 🛠️ String Oluşturma (Create a String)

Bir string literali (literal) kullanarak `to_string()` metodu veya `String::from()` fonksiyonu ile **String** oluşturabilirsiniz:

### Örnek (Example)

```rust
let text1 = "Hello World".to_string();
```

```rust
let text2 = String::from("Hello World");
```

👉 Hangisini kullanacağınız size bağlıdır — her ikisi de Rust’ta çok yaygındır.

---

## ✏️ String Değiştirme (Change a String)

String’ler değiştirilebilir (`mutable`) oldukları için `mut` ile tanımlanırlarsa güncellenebilir.

### 🔹 `push_str()` → Metin eklemek için:

```rust
let mut greeting = String::from("Hello");
greeting.push_str(" World");
println!("{}", greeting); // Hello World
```

### 🔹 `push()` → Tek bir karakter eklemek için:

```rust
let mut word = String::from("Hi");
word.push('!');
println!("{}", word); // Hi!
```

---

## 🔗 String Birleştirme (Concatenate Strings)

String’leri birleştirmek için `format!` makrosunu (macro) kullanabilirsiniz:

### Örnek (Example)

```rust
let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = String::from("What a beautiful day!");
let result = format!("{} {} {}", s1, s2, s3);
println!("{}", result);
```

Ayrıca `+` operatörünü de kullanabilirsiniz, fakat birçok değerle çalışırken biraz karmaşık olabilir:

```rust
let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = String::from("What a beautiful day!");
let result = s1 + " " + &s2 + " " + &s3;
println!("{}", result);
```

ℹ️ Not: `+` operatörüyle sadece `&str` bir `String`’e eklenebilir. Bu yüzden `&s2` ve `&s3` (string slice) burada kullanılmıştır.

💡 Bilmek güzel: String birleştirmek için genellikle `format!` kullanmak tercih edilir.

---

## 📏 String Uzunluğu (String Length)

Bir string’in uzunluğunu öğrenmek için `.len()` metodunu kullanabilirsiniz:

### Örnek (Example)

```rust
let name = String::from("John");
println!("Length: {}", name.len()); // 4
```
