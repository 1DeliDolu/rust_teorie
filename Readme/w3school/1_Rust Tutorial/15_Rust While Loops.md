## 🔁 Rust While Döngüleri (Rust While Loops)

## 📌 while Döngüsü (The while Loop)

`while` döngüsü, koşul doğru olduğu sürece çalışır.

### Örnek (Example)

```rust
let mut count = 1;

while count <= 5 {
  println!("Count: {}", count);
  count += 1;
}
```

Yukarıdaki örnekte döngü, sayaç (`count`) 5’ten küçük veya eşit olduğu sürece çalışır.
Sonuç olarak 1’den 5’e kadar olan sayılar satır satır yazdırılır.

---

## ❌ Yanlış Koşul (False Condition)

`while` döngüsü her tekrar öncesinde koşulu kontrol eder. Eğer koşul en başta yanlışsa, döngü hiç çalışmaz:

### Örnek (Example)

```rust
let count = 10;

while count <= 5 {
  println!("This won't be printed.");
}
```

👉 Bu örnekte koşul en baştan yanlış olduğu için ekrana hiçbir şey yazdırılmaz.

---

## 🛑 while Döngüsünü Durdurma (Stop a While Loop)

Bir `while` döngüsünü istediğiniz noktada `break` ifadesiyle durdurabilirsiniz:

### Örnek (Example)

```rust
let mut num = 1;

while num <= 10 {
  if num == 6 {
    break;
  }
  println!("Number: {}", num);
  num += 1;
}
```

Bu kod, `num` değişkeni 6 olduğunda döngüyü durdurur.
Sonuç olarak **1’den 5’e kadar** olan sayılar yazdırılır.

---

## ⏭️ Bir Değeri Atlamak (Skip a Value)

Bir değeri atlamak için `continue` ifadesi kullanılabilir:

### Örnek (Example)

```rust
let mut num = 1;

while num <= 10 {
  if num == 6 {
    num += 1;
    continue;
  }

  println!("Number: {}", num);
  num += 1;
}
```

Bu kod, 1’den 10’a kadar olan sayıları yazdırır, ancak **6’yı atlar**.

---

👉 Sıradaki adım: Belirli bir değer aralığında dönmek için `for` döngüsünü öğrenmek.
