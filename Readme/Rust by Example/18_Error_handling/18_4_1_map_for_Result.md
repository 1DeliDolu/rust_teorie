## 🔗 Result için map (map for Result)

Önceki örnekteki `multiply` fonksiyonunun `panic` etmesi, **güçlü (robust)** bir kod oluşturmaz. Genel olarak, hatayı çağırana geri döndürmek daha iyidir, böylece çağıran hataya nasıl yanıt vereceğine karar verebilir.

İlk olarak, hangi tür hatayla uğraştığımızı bilmemiz gerekir. `Err` türünü belirlemek için `parse()` metoduna bakarız. `parse()`, `i32` için `FromStr` trait’i ile uygulanmıştır. Sonuç olarak, `Err` türü `ParseIntError` olarak belirtilmiştir.

---

### 📝 `match` ile uzun çözüm

Aşağıdaki örnekte, basit `match` ifadeleriyle hata yönetimi yapılmaktadır. Ancak kod biraz hantaldır:

```rust
use std::num::ParseIntError;

// Dönüş türü Result olarak yeniden yazıldı, `unwrap()` yerine match kullanılıyor.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number)  => {
            match second_number_str.parse::<i32>() {
                Ok(second_number)  => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // Hâlâ doğru sonuç veriyor.
    let twenty = multiply("10", "2");
    print(twenty);

    // Bu ise daha anlamlı bir hata mesajı üretiyor.
    let tt = multiply("t", "2");
    print(tt);
}
```

---

### ⚡ Combinators ile daha kısa çözüm

Şanslıyız ki, `Option` için geçerli olan `map()`, `and_then()` ve birçok diğer combinator aynı zamanda `Result` için de uygulanmıştır.

Bu sayede kod çok daha **kompakt** ve okunabilir hale gelir:

```rust
use std::num::ParseIntError;

// `Option` gibi, burada da `map()` ve `and_then()` kullanabiliriz.
// Açıklaması: Eğer her iki değer de parse edilebilirse çarp, aksi halde hatayı ilet.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // Doğru durum
    let twenty = multiply("10", "2");
    print(twenty);

    // Hatalı durum, daha okunabilir hata mesajı döner
    let tt = multiply("t", "2");
    print(tt);
}
```

👉 Bu yaklaşım, hem hataları çağırana düzgün şekilde iletir hem de `match` ile uzun uzun yazmaya gerek bırakmadan daha temiz bir kod sağlar.

İsterseniz bir sonraki adımda size `?` operatörünü `Result` ile nasıl kullanacağımızı göstereyim mi?
