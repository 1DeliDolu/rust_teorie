## 🔢 Rust Veri Türleri (Rust Data Types)

## 📌 Veri Türleri (Data Types)

Birçok programlama dilinden farklı olarak Rust’ta değişkenler (variables) belirli bir tür (type) ile bildirilmek zorunda değildir (örneğin, C veya Java’daki `"String"` ya da `"Int"` gibi).

Rust’ta bir değişkenin türü, ona verdiğiniz değere göre belirlenir. Rust değere bakar ve otomatik olarak doğru türü seçer:

### Örnek (Example)

```rust
let my_num = 5;         // integer
let my_double = 5.99;   // float
let my_letter = 'D';    // character
let my_bool = true;     // boolean
let my_text = "Hello";  // string
```

Ancak, Rust’a açıkça bir değerin hangi türde olduğunu belirtmek de mümkündür:

### Örnek (Example)

```rust
let my_num: i32 = 5;          // integer
let my_double: f64 = 5.99;    // float
let my_letter: char = 'D';    // character
let my_bool: bool = true;     // boolean
let my_text: &str = "Hello";  // string
```

Bu eğitimde daha sonra hangi durumlarda tür belirtmeniz gerektiğini öğreneceksiniz. Yine de farklı türlerin ne anlama geldiğini anlamak faydalıdır.

## 📂 Rust’taki Temel Veri Türleri (Basic Data Types in Rust)

* **Sayılar (Numbers):** Tam sayılar ve ondalıklı sayılar (`i32`, `f64`)
* **Karakterler (Characters):** Tek harf veya sembol (`char`)
* **Metinler (Strings):** Karakter dizileri, yani metin (`&str`)
* **Mantıksal Değerler (Booleans):** Doğru veya yanlış (`bool`)

---

## 🔢 Sayılar (Numbers)

### 🔸 Tam Sayılar (Integer - `i32`)

`i32` türü, pozitif veya negatif tam sayıları (örneğin 123 veya -456) ondalık olmadan saklamak için kullanılır:

```rust
let age: i32 = 25;
println!("Age is: {}", age);
```

### 🔸 Ondalıklı Sayılar (Floating Point - `f64`)

`f64` türü, ondalık içeren sayıları saklamak için kullanılır:

```rust
let price: f64 = 19.99;
println!("Price is: ${}", price);
```

---

## 🔤 Karakterler (Characters - `char`)

`char` türü tek bir karakter saklamak için kullanılır. Karakter değeri tek tırnak (`' '`) içine yazılmalıdır, örneğin `'A'` veya `'c'`:

```rust
let myGrade: char = 'B';
println!("{}", myGrade);
```

---

## 📜 Metinler (Strings - `&str`)

`&str` türü bir karakter dizisini (metin) saklamak için kullanılır. Metin değerleri çift tırnak (`" "`) içine yazılmalıdır:

```rust
let name: &str = "John";
println!("Hello, {}!", name);
```

---

## ✅ Mantıksal Değerler (Booleans - `bool`)

`bool` türü yalnızca iki değer alabilir: `true` veya `false`:

```rust
let is_logged_in: bool = true;
println!("User logged in? {}", is_logged_in);
```

---

## 🔗 Veri Türlerini Birleştirme (Combining Data Types)

Aynı programda farklı türleri birlikte kullanabilirsiniz:

```rust
let name = "John";
let age = 28;
let is_admin = false;

println!("Name: {}", name);
println!("Age: {}", age);
println!("Admin: {}", is_admin);
```
