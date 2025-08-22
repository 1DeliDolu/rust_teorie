## 📦 Rust Değişkenler (Rust Variables)

## 📌 Değişkenler (Variables)

Değişkenler (variables), sayılar ve karakterler gibi veri değerlerini (data values) saklamak için kullanılan kaplardır.

Rust’ta bir değişken oluşturmak için `let` anahtar kelimesi (keyword) kullanılır ve değişkenin adı belirtilir (bu örnekte `name`):

### Örnek (Example)

```rust
let name = "John";
println!("My first name is: {}", name);
```

## ❓ {} Nedir? (What is {}?)

Rust, `println!()` içinde değişken değerlerini göstermek için `{}` kullanır.

Yukarıdaki örnekte çıktı şu şekilde olur:

```
My first name is: John
```

İstediğiniz kadar yer tutucu (placeholder) `{}` kullanabilirsiniz:

### Örnek (Example)

```rust
let name = "John";
let age = 30;
println!("{} is {} years old.", name, age);
```

## 🔄 Yer Tutucuların Sırası (Using Placeholders in Order)

Birden fazla yer tutucu kullandığınızda, gönderdiğiniz değerler aynı sırayla yerleştirilir.

Yukarıdaki örnekte:

* İlk `{}` → `name` (`"John"`) ile değiştirilir
* İkinci `{}` → `age` (`30`) ile değiştirilir

⚠️ Önemli: Sıra önemlidir. Değerleri karıştırırsanız çıktı değişir.

### Örnek (Yanlış Sıra)

```rust
let name = "John";
let age = 30;
println!("{} is {} years old.", age, name);  // Outputs 30 is John years old
```

## 🔒 Varsayılan Olarak Değişkenler Değiştirilemez (Variable Values Cannot be Changed by Default)

Rust’ta değişkenler varsayılan olarak oluşturulduktan sonra değiştirilemez:

```rust
let x = 5;
x = 10; // Error
println!("{}", x);
```

👉 Bu kod hata verir çünkü `x` değiştirilmeye çalışılıyor.

## 🔓 Değişken Değerlerini Değiştirme (Change Variable Values)

Bir değişkenin değerini değiştirmek istiyorsanız `mut` anahtar kelimesini kullanmanız gerekir (`mutable` yani değiştirilebilir anlamına gelir):

### Örnek (Example)

```rust
let mut x = 5;
println!("Before: {}", x);
x = 10;
println!("After: {}", x);
```

👉 Bu kodda `x` önce `5`, sonra `10` değerine güncellenir.

