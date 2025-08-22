## 🎯 Rust Match (Rust Match)

## 📌 Match

Birden fazla seçeneğiniz olduğunda, birçok `if...else` yazmak yerine `match` kullanmak daha kolaydır.

`match`, çalıştırılacak kod bloklarından birini seçmek için kullanılır:

### Örnek (Example)

```rust
fn main() {
  let day = 4;

  match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    3 => println!("Wednesday"),
    4 => println!("Thursday"),
    5 => println!("Friday"),
    6 => println!("Saturday"),
    7 => println!("Sunday"),
    _ => println!("Invalid day."),
  }
}
```

### Örneğin Açıklaması (Example explained):

* `match` değişkeni (`day`) bir kez değerlendirilir.
* `day` değişkeninin değeri, her “branch” (dal) ile karşılaştırılır.
* Her dal bir değerle başlar, ardından `=>` ve sonuç gelir.
* Eşleşme varsa ilgili kod bloğu çalıştırılır.
* `_`, eşleşme olmazsa çalıştırılacak kodu belirtmek için kullanılır (diğer dillerdeki `default` gibi).
* Yukarıdaki örnekte `day = 4` olduğu için `"Thursday"` yazdırılır.

---

## 🔗 Birden Fazla Eşleşme (Multiple Matches)

Birden fazla değeri aynı anda eşleştirmek için `|` (OR) operatörünü kullanabilirsiniz:

### Örnek (Example)

```rust
fn main() {
  let day = 6;

  match day {
    1 | 2 | 3 | 4 | 5 => println!("Weekday"),
    6 | 7 => println!("Weekend"),
    _ => println!("Invalid day"),
  }
}
```

---

## 🔄 match ile Değer Döndürme (match with a Return Value)

`if` gibi, `match` de bir değer döndürebilir.

Bu, `match` sonucunu bir değişkende saklayabileceğiniz anlamına gelir:

### Örnek (Example)

```rust
fn main() {
  let day = 4;

  let result = match day {
    1 => "Monday",
    2 => "Tuesday",
    3 => "Wednesday",
    4 => "Thursday",
    5 => "Friday",
    6 => "Saturday",
    7 => "Sunday",
    _ => "Invalid day.",
  };

  println!("{}", result);
}
```

ℹ️ Not: `match`’in her dalındaki (branch) değerler aynı türde (type) olmalıdır — tıpkı `if...else` ifadelerinde olduğu gibi.
