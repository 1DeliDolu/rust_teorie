## 🧾 Rust Sahiplik (Rust Ownership)

## 📌 Sahiplik (Ownership)

Rust, belleği güvenli bir şekilde yönetmek için **sahiplik (ownership)** sistemini kullanır.

Rust’ta her değerin bir sahibi vardır. Bu sahip genellikle bir değişkendir.

---

## 📜 Sahiplik Kuralları (Ownership Rules)

1. Her değerin yalnızca **bir sahibi** vardır
2. Sahip kapsamdan (scope) çıktığında, değer silinir
3. Aynı anda yalnızca **bir sahip** olabilir (borçlanma — borrowing ile istisnalar var, sonraki bölümde işlenecek)

---

## 📝 Temel Sahiplik Örneği (Basic Ownership Example)

Bu örnekte, `a` değişkeni string’in sahibidir. Sonra sahiplik `b`’ye taşınır:

```rust
let a = String::from("Hello");
let b = a;

// println!("{}", a); // Hata: a artık değere sahip değil
println!("{}", b); // Ok: b artık değerin sahibi
```

👉 `a` `b`’ye atandığında sahiplik taşınır. Yani artık sadece `b` değeri kullanabilir, `a` geçersiz olur.

---

## 🔢 Basit Türlerde Sahiplik (Simple Types)

Ancak sayılar, karakterler ve boolean gibi basit türler **taşınmaz, kopyalanır (copied)**.

Bu, değişkeni başka birine atasanız bile orijinalini kullanabileceğiniz anlamına gelir:

```rust
let a = 5;
let b = a;
println!("a = {}", a);  // Çalışır
println!("b = {}", b);  // Çalışır
```

👉 Burada `a` `b`’ye kopyalanır, taşınmaz. Dolayısıyla her ikisi de kullanılabilir.

---

## 📋 Clone

`String` gibi diğer türlerde, orijinal değeri saklamak ve aynı zamanda başka bir değişkene atamak istiyorsanız `.clone()` metodunu kullanabilirsiniz. Bu, verinin bir kopyasını oluşturur:

```rust
let a = String::from("Hello");
let b = a.clone(); // Artık ikisi de aynı değere sahip

println!("a = {}", a);  // Çalışır
println!("b = {}", b);  // Çalışır
```

👉 Ancak, bir değere iki kez sahip olmanız gerekmiyorsa, kopyalamak yerine genellikle **referans (&)** kullanmak daha iyidir. (Bunu bir sonraki bölümde "borçlanma" (borrowing) ile öğreneceksiniz.)

---

## 🎯 Neden Sahiplik Önemlidir? (Why Ownership Matters)

* Rust, sahipliği kullanarak gereksiz olduğunda belleği otomatik olarak serbest bırakır
* Daha önce silinmiş belleği kullanma gibi hataları önler
* Rust’ın güvenli (safe) ve hızlı (fast) olmasının en büyük sebeplerinden biridir

---

👉 Sıradaki adım: **Borçlanma (borrowing)** — programınızın diğer kısımlarının bir değeri sahipliği almadan nasıl kullanabileceğini öğrenmek.

