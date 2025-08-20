## 🗂️ Rust Veri Yapıları (Rust Data Structures)

## 📌 Veri Yapıları (Data Structures)

Rust’ta veri yapıları (data structures), değerleri saklamak ve düzenlemek için kullanılır.

Rust, farklı türde verileri yönetmek için birçok **yerleşik veri yapısı (built-in data structure)** sağlar.

En yaygın kullanılanlar şunlardır:

* **Array (Dizi)**
* **Vector (Vec)**
* **Tuple (Demet)**
* **HashMap**

Bu bölümde her birini kısaca tanıyacağız.

---

## 🔢 Diziler (Arrays)

Rust’ta **array**, aynı türden değerleri saklayan sabit uzunlukta (fixed-size) bir listedir.

Bir array oluşturulduktan sonra boyutu değiştirilemez.

Array elemanlarına **indeks numarası** ile erişilir. İndeksler `0`’dan başlar (`[0]` → ilk eleman, `[1]` → ikinci eleman, vb.).

### Örnek (Example)

```rust
let fruits = ["apple", "banana", "orange"];
println!("Last fruit: {}", fruits[2]);
```

👉 Çıktı: `orange`

---

## 📈 Vektörler (Vectors)

**Vector (Vec)**, yeniden boyutlandırılabilir (growable) bir dizidir.

Dizilerden farklı olarak, vektörlerin boyutu artırılabilir veya azaltılabilir.

### Örnek (Example)

```rust
let mut fruits = vec!["apple", "banana"];
fruits.push("cherry");

println!("Last fruit: {}", fruits[2]);
```

👉 Çıktı: `cherry`

---

## 🎭 Demetler (Tuples)

**Tuple**, farklı türlerde birden fazla değeri bir arada tutabilir.
Farklı türdeki verileri gruplamak için kullanışlıdır.

Tuple elemanlarına nokta (`.`) ve indeks numarası ile erişilir (`person.0`, `person.1` vb.).

### Örnek (Example)

```rust
let person = ("John", 30, true);
println!("Name: {}", person.0);
println!("Age: {}", person.1);
println!("Is active: {}", person.2);
```

👉 Çıktı:

```
Name: John
Age: 30
Is active: true
```

---

## 🗝️ HashMap’ler (HashMaps)

**HashMap**, anahtar-değer (key-value) çiftlerini saklar.
Bir değeri **anahtarı** kullanarak aramanıza izin verir.

HashMap kullanmak için standart kütüphaneden (`std::collections`) içe aktarılmalıdır.

### Örnek (Example)

```rust
// Import HashMap
use std::collections::HashMap;

fn main() {
  let mut capitalCities = HashMap::new();
  capitalCities.insert("France", "Paris");
  capitalCities.insert("Japan", "Tokyo");

  println!("Capital of Japan is {}", capitalCities["Japan"]);
}
```

👉 Çıktı: `Capital of Japan is Tokyo`

---

## 📋 Veri Yapıları Genel Bakış (Data Structures Overview)

| Veri Yapısı (Data Structure) | Kullanım Durumu (Use Case)           | Büyüyebilir mi? (Can Grow?) |
| ---------------------------- | ------------------------------------ | --------------------------- |
| **Array**                    | Sabit uzunluklu, aynı türde değerler | ❌ Hayır                     |
| **Vector (Vec)**             | Büyüyebilir, aynı türde değerler     | ✅ Evet                      |
| **Tuple**                    | Farklı türleri bir arada gruplamak   | ❌ Hayır                     |
| **HashMap**                  | Anahtar-değer arama                  | ✅ Evet                      |

---

👉 Sıradaki adım: Her bir veri yapısını daha detaylı incelemek.
