## 🔄 Rust Döngüler (Rust Loops)

## 📌 Döngüler (Loops)

Döngüler (loops), belirli bir koşul sağlandığı sürece bir kod bloğunu çalıştırabilir.

Döngüler zaman kazandırır, hataları azaltır ve kodu daha okunabilir hale getirir. Örneğin, aynı satırı 10 kez yazmak yerine bir döngü kullanarak tekrar ettirebilirsiniz.

Rust’ta üç tip döngü vardır:

* `loop`
* `while`
* `for`

---

## ♾️ loop

`loop`, Rust’taki en basit döngü türüdür.

Durmasını söylemezseniz sonsuza kadar çalışır:

```rust
loop {
  println!("This will repeat forever!");
}
```

⚠️ Uyarı: Bu döngü asla durmaz! Programı sonlandırmak için `Ctrl + C` tuşlarına basmanız gerekir.

Bir döngüyü durdurmak için `break` anahtar kelimesini kullanın:

### Örnek (Example)

```rust
let mut count = 1;

loop {
  println!("Hello World!");

  if count == 3 {
    break;
  }

  count += 1;
}
```

### Örneğin Açıklaması (Example explained):

* `"Hello World!"` 3 kez yazdırılır
* Kaç kez tekrarlandığını takip etmek için sayaç (`count`) kullanılır
* Sayaç 1’den başlar (`let mut count = 1;`)
* Her döngü çalıştığında sayaç 1 artar (`count += 1;`)
* Sayaç 3 olduğunda döngü durur

---

## ↩️ Değer Döndürme (Return a Value)

Bir döngüden `break` ile birlikte bir değer döndürebilirsiniz.

Bu sayede döngünün sonucunu bir değişkende saklayabilirsiniz:

### Örnek (Example)

```rust
let mut count = 1;

let result = loop {
  println!("Hello!");

  if count == 3 {
    break count; // Döngüyü durdur ve 3 değerini döndür
  }

  count += 1;
};

println!("The loop stopped at: {}", result);
```

### Örneğin Açıklaması:

* `"Hello!"` yazdırılır
* Sayaç 3 olduğunda döngü durur ve o sayı (`3`) döndürülür
* Dönen değer `result` değişkenine atanır

ℹ️ Not: Bir döngü sonucunu bir değişkene kaydederseniz, satırın sonuna mutlaka noktalı virgül (`;`) koymanız gerekir.

---

👉 Sıradaki adım: Koşul doğru olduğu sürece kodu tekrarlamak için `while` döngülerini öğrenmek.
