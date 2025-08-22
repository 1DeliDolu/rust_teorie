## 🔀 Rust If .. Else Koşulları (Rust If .. Else Conditions)

## 📌 Koşullar ve If..Else (Conditions and If..Else)

Rust, matematikteki tanıdık karşılaştırma koşullarını destekler:

* Küçüktür: `a < b`
* Küçük eşittir: `a <= b`
* Büyüktür: `a > b`
* Büyük eşittir: `a >= b`
* Eşittir: `a == b`
* Eşit değildir: `a != b`

Bu koşulları kullanarak farklı durumlara göre farklı işlemler yapabilirsiniz.

Rust aşağıdaki koşul ifadelerini (conditional statements) destekler:

* `if`: Belirtilen koşul doğruysa çalıştırılacak kod bloğunu belirtir
* `else`: Koşul yanlışsa çalıştırılacak kod bloğunu belirtir
* `else if`: İlk koşul yanlışsa test edilecek yeni bir koşul belirtir
* `switch`: Birçok alternatif kod bloğu belirtmek için kullanılır

ℹ️ Not: Çoğu programlama dilinden farklı olarak Rust’ta `if..else` hem bir ifade (statement) hem de bir ifade değeri (expression) olarak kullanılabilir. Yani bir değişkene değer atamak için kullanılabilir.

---

## 🟢 if

`if`, koşul doğruysa çalıştırılacak kod bloğunu belirtmek için kullanılır.

### Örnek (Example)

```rust
if 7 > 5 {
  println!("7 is greater than 5.");
}
```

Değişkenleri de test edebilirsiniz:

```rust
let x = 7;
let y = 5;

if x > y {
  println!("x is greater than y.");
}
```

---

## 🔄 if...else

Koşul doğru değilse, `else` ile farklı bir kod çalıştırabilirsiniz:

### Örnek (Example)

```rust
let age = 16;

if age >= 18 {
  println!("You can vote.");
} else {
  println!("You are too young to vote.");
}
```

---

## 🔁 else if

Birden fazla koşulu `else if` ile kontrol edebilirsiniz:

### Örnek (Example)

```rust
let score = 85;

if score >= 90 {
  println!("Grade: A");
} else if score >= 80 {
  println!("Grade: B");
} else if score >= 70 {
  println!("Grade: C");
} else {
  println!("Grade: F");
}
```

---

## ✨ If’in Bir İfade Olarak Kullanımı (Using if as an Expression)

Rust’ta `if..else` bir ifade (expression) olarak da kullanılabilir.

Bu, bir `if` ifadesinin sonucunu bir değişkene atayabileceğiniz anlamına gelir:

### Örnek (Example)

```rust
let time = 20;
let greeting = if time < 18 {
  "Good day."
} else {
  "Good evening."
};
println!("{}", greeting);
```

👉 If bir ifade olarak kullanıldığında `else` mutlaka yazılmalıdır. Bu, her durumda bir değer dönmesini garanti eder.

---

## 📝 Basitleştirilmiş Sözdizimi (Simplified Syntax)

Eğer her blok yalnızca tek satırdan oluşuyorsa, süslü parantezler `{}` kaldırılarak daha kısa yazılabilir:

### Örnek (Example)

```rust
let time = 20;
let greeting = if time < 18 { "Good day." } else { "Good evening." };
println!("{}", greeting);
```

ℹ️ Not: `if` ve `else` bloklarının değerleri aynı türde (type) olmalıdır, örneğin ikisi de metin (string) ya da ikisi de sayı (integer).

💡 İpucu: Bu örnek, Java veya C dillerindeki üçlü (ternary) operatöre benzer şekilde çalışır.
