## 📦 Rust Vektörler (Rust Vectors)

## 📌 Vektörler (Vectors)

**Vector**, yeniden boyutlandırılabilir bir dizidir.
Normal dizilerden farklı olarak, vektörler büyüyebilir veya küçülebilir.

---

## 🛠️ Vektör Oluşturma (Creating a Vector)

Bir vektör oluşturmak için `vec!` makrosu kullanılır:

### Örnek (Example)

```rust
let fruits = vec!["apple", "banana", "orange"];
```

👉 Bu, üç string elemanlı bir vektör oluşturur.

---

## 🎯 Vektör Elemanlarına Erişim (Access Vector Elements)

Vektör elemanlarına **indeks numarası** ile erişilir (tıpkı dizilerde olduğu gibi):

### Örnek (Example)

```rust
let fruits = vec!["apple", "banana", "orange"];
println!("First fruit: {}", fruits[0]);
```

---

## ✏️ Vektör Elemanlarını Değiştirme (Change Vector Values)

Vektördeki bir değeri değiştirmek için indeks numarasına yeni bir değer atanır.

⚠️ Vektörün değiştirilebilir (`mut`) olarak tanımlanması gerekir.

### Örnek (Example)

```rust
let mut fruits = vec!["apple", "banana", "orange"];
fruits[0] = "grape";
println!("New first fruit: {}", fruits[0]);
```

---

## ➕ Vektöre Eleman Ekleme (Add Elements to a Vector)

Vektörün sonuna yeni eleman eklemek için `push()` metodu kullanılır:

### Örnek (Example)

```rust
let mut fruits = vec!["apple", "banana"];
fruits.push("cherry");
println!("{:?}", fruits); // ["apple", "banana", "cherry"]
```

---

## ➖ Vektörden Eleman Silme (Remove Elements from a Vector)

Son elemanı silmek için `pop()` metodu kullanılır:

### Örnek (Example)

```rust
let mut fruits = vec!["apple", "banana", "cherry"];
fruits.pop();
println!("{:?}", fruits); // ["apple", "banana"]
```

---

## 📍 Belirli Bir İndekste Eleman Ekleme veya Silme (Add or Remove Elements at a Specified Index)

Vektörler genellikle sonuna eleman ekleyip silmek için tasarlanmıştır.
Ancak `insert()` ile belirli bir indekse eleman ekleyebilir, `remove()` ile eleman silebilirsiniz.

### Örnek – Başa Eleman Ekleme

```rust
let mut fruits = vec!["banana", "orange"];
fruits.insert(0, "apple");
println!("{:?}", fruits); // ["apple", "banana", "orange"]
```

### Örnek – Ortaya Eleman Ekleme

```rust
let mut fruits = vec!["banana", "orange"];
fruits.insert(1, "apple");
println!("{:?}", fruits); // ["banana", "apple", "orange"]
```

### Örnek – İlk Elemanı Silme

```rust
let mut fruits = vec!["apple", "banana", "orange"];
fruits.remove(0);
println!("{:?}", fruits); // ["banana", "orange"]
```

⚠️ Not: Vektörün başından eleman eklemek veya silmek, sondan işlem yapmaya göre daha yavaştır çünkü diğer elemanların pozisyonları kaydırılır.

---

## 📏 Vektör Uzunluğu (Vector Length)

Bir vektörde kaç eleman olduğunu `.len()` metodu ile öğrenebilirsiniz:

### Örnek (Example)

```rust
let fruits = vec!["apple", "banana", "cherry"];
println!("There are {} fruits.", fruits.len());
```

---

## 🔄 Vektör Üzerinde Döngü (Loop Through a Vector)

Tıpkı dizilerde olduğu gibi, `for` döngüsü ile vektör elemanlarının üzerinden geçebilirsiniz.

### Örnek (Example)

```rust
let fruits = vec!["apple", "banana", "orange"];
for fruit in &fruits {
  println!("I like {}.", fruit);
}
```

⚠️ Not: `&fruits` kullanarak vektörü **ödünç alırsınız (borrow)**.

* Eğer `&` kullanmazsanız, değerler **taşınır (moved)** ve vektör artık kullanılamaz.
* `&` ile sadece referans alınır, böylece vektör döngüden sonra da kullanılabilir.

---

👉 Sahiplik (ownership) ve borçlanma (borrowing) hakkında daha fazla bilgiyi önceki bölümlerde öğrenmiştiniz.
Vektörlerde bu kurallar özellikle önemlidir.
