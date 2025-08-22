## ➗ Rust Operatörler (Rust Operators)

## 📌 Operatörler (Operators)

Operatörler (operators), değerler (values) ve değişkenler (variables) üzerinde işlemler yapmak için kullanılır.

Rust birçok yaygın operatörü destekler, örneğin:

* Aritmetik Operatörler (Arithmetic Operators)
* Atama Operatörleri (Assignment Operators)
* Karşılaştırma Operatörleri (Comparison Operators)
* Mantıksal Operatörler (Logical Operators)

---

## ➕ Aritmetik Operatörler (Arithmetic Operators)

Aritmetik operatörler temel matematik işlemleri için kullanılır:

| Operatör | Adı (Name)              | Örnek (Example) | Sonuç (Result) |
| -------- | ----------------------- | --------------- | -------------- |
| `+`      | Toplama (Addition)      | `5 + 3`         | `8`            |
| `-`      | Çıkarma (Subtraction)   | `5 - 3`         | `2`            |
| `*`      | Çarpma (Multiplication) | `5 * 3`         | `15`           |
| `/`      | Bölme (Division)        | `10 / 2`        | `5`            |
| `%`      | Kalan (Remainder)       | `10 % 3`        | `1`            |

### Örnek (Example)

```rust
fn main() {
  let add = 5 + 3;
  let sub = 10 - 4;
  let mul = 6 * 2;
  let div = 12 / 3;
  let rem = 10 % 3;

  println!("Add: {}", add);
  println!("Sub: {}", sub);
  println!("Mul: {}", mul);
  println!("Div: {}", div);
  println!("Rem: {}", rem);
}
```

---

## 📝 Atama Operatörleri (Assignment Operators)

Atama operatörleri, değerleri atamak ve güncellemek için kullanılır:

| Operatör | Örnek (Example) | Aynısı (Same As) |
| -------- | --------------- | ---------------- |
| `=`      | `x = 5`         | `x`’e 5 ata      |
| `+=`     | `x += 3`        | `x = x + 3`      |
| `-=`     | `x -= 2`        | `x = x - 2`      |
| `*=`     | `x *= 4`        | `x = x * 4`      |
| `/=`     | `x /= 2`        | `x = x / 2`      |
| `%=`     | `x %= 2`        | `x = x % 2`      |

### Örnek (Example)

```rust
fn main() {
  let mut x = 10;
  println!("Start: {}", x);

  x += 5;
  println!("After += 5: {}", x);

  x -= 2;
  println!("After -= 2: {}", x);

  x *= 2;
  println!("After *= 2: {}", x);

  x /= 3;
  println!("After /= 3: {}", x);

  x %= 4;
  println!("After %= 4: {}", x);
}
```

---

## ⚖️ Karşılaştırma Operatörleri (Comparison Operators)

Karşılaştırma operatörleri, değerleri karşılaştırır ve `true` ya da `false` döndürür:

| Operatör | Anlamı (Meaning)                 | Örnek (Example)   |
| -------- | -------------------------------- | ----------------- |
| `==`     | Eşittir (Equal to)               | `5 == 5` → `true` |
| `!=`     | Eşit değildir (Not equal to)     | `5 != 3` → `true` |
| `>`      | Büyüktür (Greater than)          | `7 > 3` → `true`  |
| `<`      | Küçüktür (Less than)             | `2 < 5` → `true`  |
| `>=`     | Büyük eşittir (Greater or equal) | `5 >= 5` → `true` |
| `<=`     | Küçük eşittir (Less or equal)    | `3 <= 4` → `true` |

### Örnek (Example)

```rust
fn main() {
  let a = 5;
  let b = 10;

  println!("5 == 10: {}", a == b);
  println!("5 != 10: {}", a != b);
  println!("5 < 10: {}", a < b);
  println!("5 >= 10: {}", a >= b);
}
```

---

## 🔐 Mantıksal Operatörler (Logical Operators)

Mantıksal operatörler, boolean değerlerle çalışmak için kullanılır:

| Operatör | Adı (Name) | Açıklama (Description)               |    |                                         |
| -------- | ---------- | ------------------------------------ | -- | --------------------------------------- |
| `&&`     | AND        | İki değer de `true` ise sonuç `true` |    |                                         |
| \`       |            | \`                                   | OR | En az bir değer `true` ise sonuç `true` |
| `!`      | NOT        | Boolean değerini tersine çevirir     |    |                                         |

### Örnek (Example)

```rust
fn main() {
  let logged_in = true;
  let is_admin = false;

  println!("Is regular user: {}", logged_in && !is_admin);
  println!("Has any access: {}", logged_in || is_admin);
  println!("Not logged in: {}", !logged_in);
}
```
