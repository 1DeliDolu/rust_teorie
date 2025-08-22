## 🔡 String’lere Dönüştürme ve String’lerden Dönüştürme (to and from Strings)

### 🔄 String’e Dönüştürme

Herhangi bir türü `String`’e dönüştürmek için `ToString` trait’i uygulanabilir. Ancak doğrudan `ToString` yerine `fmt::Display` trait’ini uygulamak tercih edilir. Çünkü `Display` uygulanırsa, `ToString` otomatik olarak sağlanır ve ayrıca `print!` bölümünde açıklandığı gibi türün yazdırılabilmesini de sağlar.

```rust
use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
```

### 🔍 String’den Ayrıştırma (Parsing a String)

String’leri farklı türlere dönüştürmek faydalıdır. En yaygın işlemlerden biri, bir string’i sayıya dönüştürmektir. Bunun için idiomatik yaklaşım `parse` fonksiyonunu kullanmaktır. Tür çıkarımı (type inference) yapılabilir veya `'turbofish'` sözdizimi ile tür açıkça belirtilebilir.

Aşağıdaki örnekte her iki yöntem de gösterilmiştir:

```rust
fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
```

Bu dönüşüm, ilgili tür için `FromStr` trait’i uygulanmışsa mümkündür. Standart kütüphanedeki birçok tür için bu trait zaten uygulanmıştır.

### 🏗️ Kullanıcı Tanımlı Türlerde FromStr Kullanımı

Kullanıcı tanımlı bir türde string’den dönüştürme yapmak için `FromStr` trait’i uygulanabilir:

```rust
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle{ radius: num }),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let radius = "    3 ";
    let circle: Circle = radius.parse().unwrap();
    println!("{:?}", circle);
}
```
