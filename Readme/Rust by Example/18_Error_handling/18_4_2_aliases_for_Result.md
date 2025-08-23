## 🏷️ Result için Takma İsimler (Aliases for Result)

Peki aynı `Result` türünü birçok kez yeniden kullanmak istediğimizde ne olur? Hatırlarsak, Rust bize **takma isimler (aliases)** oluşturma imkânı tanır. Bu sayede belirli bir `Result` türü için kısayol tanımlayabiliriz.

Özellikle **modül düzeyinde** alias oluşturmak çok faydalıdır. Çünkü bir modüldeki hatalar genellikle aynı `Err` türünü taşır; tek bir alias ile tüm ilişkili `Result` türleri kısaca tanımlanabilir. Bu o kadar kullanışlıdır ki, standart kütüphane (`std`) bile bunu yapar: örneğin `io::Result`.

Aşağıdaki örnek, bu sözdizimini göstermektedir:

```rust
use std::num::ParseIntError;

// Hata türü `ParseIntError` olan bir `Result` için genel bir alias tanımlıyoruz.
type AliasedResult<T> = Result<T, ParseIntError>;

// Yukarıdaki alias'ı kullanarak spesifik `Result` türünü ifade ediyoruz.
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Alias sayesinde burada da yazımı kısaltabiliyoruz.
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
```

👉 Bu yöntemle, `Result<T, ParseIntError>` yerine sadece `AliasedResult<T>` yazmak yeterli olur. Kod daha okunabilir ve tekrar eden kısımlar azalır.

İsterseniz bir sonraki adımda size `io::Result` gibi standart kütüphanedeki yaygın alias örneklerini de gösterebilirim.
