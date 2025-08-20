## ✅ Rust Boolean’lar (Rust Booleans)

## 📌 Boolean’lar (Booleans)

Programlamada çok sık olarak yalnızca iki değerden birine sahip olabilen bir veri türüne ihtiyaç duyarsınız, örneğin:

* **EVET / HAYIR (YES / NO)**
* **AÇIK / KAPALI (ON / OFF)**
* **DOĞRU / YANLIŞ (TRUE / FALSE)**

Bunun için Rust’ta `bool` veri türü vardır, buna **boolean** denir.

Boolean’lar yalnızca `true` veya `false` değerlerini temsil eder.

---

## 🛠️ Boolean Değişkenleri Oluşturma (Creating Boolean Variables)

Bir değişkende boolean değeri saklamak için `bool` türünü kullanabilirsiniz:

### Örnek (Example)

```rust
let is_programming_fun: bool = true;
let is_fish_tasty: bool = false;

println!("Is Programming Fun? {}", is_programming_fun);
println!("Is Fish Tasty? {}", is_fish_tasty);
```

Rust, `true` ve `false` değerlerinin zaten boolean olduğunu anlar, bu yüzden `bool` yazmanız gerekmez:

### Örnek (Example)

```rust
let is_programming_fun = true;
let is_fish_tasty = false;

println!("Is Programming Fun? {}", is_programming_fun);
println!("Is Fish Tasty? {}", is_fish_tasty);
```

---

## ⚖️ Karşılaştırmadan Boolean Üretme (Boolean from Comparison)

Çoğu zaman `true` veya `false` değerlerini kendiniz yazmanıza gerek yoktur. Bunun yerine boolean değerleri genellikle karşılaştırmalardan gelir (`==`, `>` gibi operatörlerle):

### Örnek (Example)

```rust
let age = 20;
let can_vote = age >= 18;

println!("Can vote? {}", can_vote);
```

Burada `age >= 18` ifadesi, `age` 18 veya daha büyük olduğu sürece `true` döner.

---

## 🔀 if İfadelerinde Boolean Kullanımı (Using Booleans in if Statements)

Boolean değerleri genellikle hangi kodun çalışacağını belirlemek için `if` ifadelerinde kullanılır:

### Örnek (Example)

```rust
let is_logged_in = true;

if is_logged_in {
  println!("Welcome back!");
} else {
  println!("Please log in.");
}
```

Harika değil mi? Boolean’lar Rust’taki tüm karşılaştırmaların ve koşulların temelini oluşturur.
Bir sonraki bölümde `if` ve `else` ifadeleri hakkında daha fazla öğreneceksiniz.
