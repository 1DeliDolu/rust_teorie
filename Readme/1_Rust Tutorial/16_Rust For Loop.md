## 🔁 Rust For Döngüsü (Rust For Loop)

## 📌 for Döngüsü (The for Loop)

Bir kod bloğunu kaç kez tekrar edeceğinizi tam olarak bildiğinizde, `while` döngüsü yerine `for` döngüsünü `in` anahtar kelimesiyle kullanabilirsiniz:

### Örnek (Example)

```rust
for i in 1..6 {
  println!("i is: {}", i);
}
```

👉 Bu kod 1’den 5’e kadar olan sayıları yazdırır.

ℹ️ Not: `1..6` ifadesi 1’den başlar ama 6’yı **dahil etmez**.
ℹ️ Not: Rust sayaç değişkenini (`i`) otomatik olarak yönetir. Yani değişkeni kendiniz tanımlayıp artırmanıza gerek yoktur.

---

## 🔢 Kapsayıcı Aralık (Inclusive Range)

Son sayıyı da dahil etmek istiyorsanız `..=` (iki nokta ve eşittir) kullanın:

### Örnek (Example)

```rust
for i in 1..=6 {
  println!("i is: {}", i);
}
```

👉 Bu kod 1’den 6’ya kadar olan sayıları (6 dahil) yazdırır.

---

## 🛑 break ve continue (Break and Continue)

Diğer döngülerde olduğu gibi, `break` ile döngüyü durdurabilir, `continue` ile bir değeri atlayabilirsiniz:

### Örnek (Example)

```rust
for i in 1..=10 {
  if i == 3 {
    continue; // 3'ü atla
  }
  if i == 5 {
    break; // 5'ten önce dur
  }
  println!("i is: {}", i);
}
```

👉 Çıktı: **1, 2 ve 4**.
(`3` atlandı, `5`’te döngü durdu.)

---

## 📋 Rust Döngüleri Özeti (Rust Loops Summary)

Rust’ta kodu tekrar tekrar çalıştırmanızı sağlayan üç tip döngü vardır. Her biri farklı durumlarda kullanılır:

### 1️⃣ loop

En basit döngü türüdür. `break` ile durdurmadığınız sürece sonsuza kadar çalışır.

```rust
loop {
  // do something
  if condition {
    break;
  }
}
```

👉 Ne kadar tekrar edeceğinizi önceden bilmediğinizde `loop` kullanın.

---

### 2️⃣ while

Koşul doğru olduğu sürece kodu tekrar eder. Her turdan önce koşulu kontrol eder.

```rust
while count <= 5 {
  println!("{}", count);
  count += 1;
}
```

👉 Bir şey olana kadar tekrar etmek istediğinizde `while` kullanın.

---

### 3️⃣ for

Belirli bir sayıda tekrar eden döngüdür.

```rust
for i in 1..=5 {
  println!("{}", i);
}
```

👉 Tam olarak neyin üzerinden geçeceğinizi bildiğinizde `for` kullanın.

---

## 🔑 Ek Anahtar Kelimeler (Extra Keywords)

Tüm döngülerde kullanılabilir:

* `break` → döngüyü durdurur
* `continue` → döngüde bir değeri atlar

---

✅ Artık döngüleri nasıl kullanacağınızı biliyorsunuz. Sırada **fonksiyonlar (functions)** ve tekrar kullanılabilir kod var!
