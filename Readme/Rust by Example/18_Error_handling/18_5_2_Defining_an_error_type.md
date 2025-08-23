## 🛠️ Hata Türü Tanımlama (defining an error type)

Bazen tüm farklı hata türlerini tek bir hata türü altında **maskelemek** kodu basitleştirir. Bunu özel bir hata (custom error) ile göstereceğiz.

Rust bize kendi hata türlerimizi tanımlama imkânı verir. Genel olarak, “iyi” bir hata türü şunları sağlar:

* Farklı hataları aynı tür altında temsil eder
* Kullanıcıya anlaşılır hata mesajları sunar
* Diğer türlerle kolayca karşılaştırılabilir

  * İyi: `Err(EmptyVec)`
  * Kötü: `Err("Please use a vector with at least one element".to_owned())`
* Hata hakkında bilgi taşıyabilir

  * İyi: `Err(BadChar(c, position))`
  * Kötü: `Err("+ cannot be used here".to_owned())`
* Diğer hatalarla iyi şekilde birleşebilir

```rust
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// Hata türlerimizi tanımlıyoruz. Bunlar hata yönetimi durumlarımıza göre özelleştirilebilir.
// Artık kendi hatalarımızı yazabilir, altta yatan bir hata implementasyonuna yönlendirebilir,
// veya ikisinin ortasında bir şey yapabiliriz.
#[derive(Debug, Clone)]
struct DoubleError;

// Hatanın üretilmesi ile ekranda gösterilmesi tamamen ayrıdır.
// Yani karmaşık mantık içinde gösterim tarzını düşünmemize gerek yok.
//
// Burada hatalar hakkında ekstra bilgi saklamıyoruz.
// Bu nedenle başarısız olan string'in hangisi olduğunu söyleyemeyiz; 
// bunu yapmak için türlerimizi bu bilgiyi taşıyacak şekilde değiştirmemiz gerekir.
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // Hata türünü yeni tipimize dönüştürüyoruz.
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // Burada da hata türünü yeni tipimize güncelliyoruz.
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```
