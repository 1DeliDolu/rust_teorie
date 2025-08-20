## 🔗 Rust Borçlanma ve Referanslar (Rust Borrowing and References)

## 📌 Borçlanma ve Referanslar (Borrowing and References)

Bazen bir değeri **sahiplenmeden** (ownership almadan) kullanmak istersiniz.

Rust bunu **referans (reference)** ile yapmanıza izin verir. Buna **borçlanma (borrowing)** denir.

---

## ❓ Referans Nedir? (What is a Reference?)

Referans, bir değere sahip olmadan ona bakmanıza izin verir.
Referans oluşturmak için `&` sembolü kullanılır:

### Örnek (Example)

```rust
let a = String::from("Hello");
let b = &a;

println!("a = {}", a);
println!("b = {}", b);
```

👉 Burada `b` yalnızca değeri **ödünç alır (borrow)**, hala `a` sahip olmaya devam eder.

---

## ✏️ Değiştirilebilir Referanslar (Mutable References)

Bir değeri referans üzerinden değiştirmek istiyorsanız, referansı **mutable (değiştirilebilir)** yapmanız gerekir:

### Örnek (Example)

```rust
let mut name = String::from("John");
let name_ref = &mut name;
name_ref.push_str(" Doe");

println!("{}", name_ref); // John Doe
```

⚠️ Not: Aynı anda yalnızca **bir tane mutable referans** oluşturabilirsiniz!

---

## 🎯 Neden Borçlanma Önemlidir? (Why Borrowing is Important)

Borçlanma sayesinde değerleri güvenli bir şekilde tekrar kullanabilirsiniz, sahipliklerini vermeden.

* Değerleri sahiplenmeden kullanmanızı sağlar
* Büyük verilerde **klonlama (cloning)** yapmadan çalışır, bu da daha hızlıdır
* Programlarınızı daha güvenli ve daha performanslı hale getirir
