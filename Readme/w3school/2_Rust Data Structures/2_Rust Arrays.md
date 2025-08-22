## 🔢 Rust Diziler (Rust Arrays)

## 📌 Diziler (Arrays)

Diziler, her değer için ayrı değişken tanımlamak yerine, birden fazla değeri tek bir değişkende saklamak için kullanılır.

---

## 🛠️ Dizi Oluşturma (Create an Array)

Bir dizi oluşturmak için köşeli parantezler `[ ]` kullanılır ve değerler virgülle ayrılır.

⚠️ Not: Tüm değerler aynı veri türünde olmalıdır (aşağıdaki örnekte tamsayılar).

### Örnek (Example)

```rust
let numbers = [1, 2, 3, 4, 5];
```

👉 Bu, beş tamsayıdan oluşan bir dizi oluşturur.

---

## 🎯 Dizi Elemanlarına Erişim (Access Array Elements)

Bir dizi elemanına erişmek için **indeks numarası** kullanılır.

İndeksler `0`’dan başlar: `[0]` → ilk eleman, `[1]` → ikinci eleman, vb.

### Örnek (Example)

```rust
let numbers = [1, 2, 3, 4, 5];
println!("The first number is: {}", numbers[0]);
```

---

## ✏️ Dizi Elemanlarını Değiştirme (Change Array Values)

Belirli bir elemanın değerini değiştirmek için indeks numarasına yeni bir değer atanır.

⚠️ Diziyi değiştirmek için `mut` anahtar kelimesi kullanılmalıdır.

### Örnek (Example)

```rust
let mut numbers = [1, 2, 3, 4, 5];
numbers[0] = 10;
println!("The new first number is: {}", numbers[0]);
```

---

## 📏 Dizi Uzunluğu (Array Length)

Bir dizideki eleman sayısını öğrenmek için `.len()` metodu kullanılır:

### Örnek (Example)

```rust
let numbers = [1, 2, 3, 4, 5];
println!("This array has {} elements.", numbers.len());
```

---

## 🔄 Dizi Üzerinde Döngü (Loop Through an Array)

`for` döngüsü ile dizi elemanlarının üzerinden geçebilirsiniz:

### Örnek (Example)

```rust
let fruits = ["apple", "banana", "orange"];
for fruit in fruits {
  println!("I like {}.", fruit);
}
```

---

## 🖨️ Tüm Diziyi Yazdırma (Print the Entire Array)

Tüm diziyi yazdırırken `println!` içinde `{:?}` kullanılmalıdır:

### Örnek (Example)

```rust
let numbers = [1, 2, 3, 4, 5];
println!("{:?}", numbers);
```

👉 Tek bir elemanı yazdırırken `{}` kullanılabilir:

```rust
let numbers = [1, 2, 3, 4, 5];
println!("{}", numbers[0]);
```

### Kural:

* Tek bir eleman yazdırırken → `{}`
* Tüm diziyi yazdırırken → `{:?}`

💡 Basit türler (`string`, `number`, `boolean`) için `{}`, tüm veri yapıları (`array`, `vector`) için `{:?}` kullanmak iyi bir alışkanlıktır.

---

## 📌 Sabit Boyutlu (Array) vs Dinamik Boyutlu (Vector)

### 🔒 Sabit Boyut (Fixed Size - Arrays)

Rust’ta dizilerin boyutu sabittir. Yani bir dizi oluşturulduktan sonra eleman eklenemez veya silinemez:

```rust
// 3 elemanlı bir dizi
let mut cars = ["Volvo", "BMW", "Ford"];

// Dördüncü bir eleman eklemeye çalışmak hata verir
cars[3] = "Mazda";   // Error: index out of bounds
```

---

### 🔓 Dinamik Boyut (Dynamic Size - Vectors)

Eleman ekleyip çıkarmak için **Vector (Vec)** kullanılır.

Vector’lerin boyutu dinamiktir; gerektiğinde büyüyebilir veya küçülebilir.

Vector oluşturmak için `vec!` makrosu kullanılır:

### Örnek (Example)

```rust
// 3 elemanlı bir vektör
let mut cars = vec!["Volvo", "BMW", "Ford"];

// Yeni bir eleman ekle
cars.push("Mazda");

println!("{:?}", cars); // ["Volvo", "BMW", "Ford", "Mazda"]
```

---

👉 Bu, vektörlere kısa bir girişti.
Bir sonraki bölümde vektörleri daha ayrıntılı inceleyeceksiniz.
