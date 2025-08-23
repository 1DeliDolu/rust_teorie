## ❓ Option

Bazen bir programın bazı bölümlerindeki hataları `panic!` çağırmak yerine yakalamak istenir; bu `Option` enum’u kullanılarak gerçekleştirilebilir.

`Option<T>` enum’unun iki varyantı vardır:

* `None`: başarısızlığı veya değer yokluğunu ifade eder
* `Some(value)`: `T` türünden bir değeri saran tuple struct

```rust
// `panic!` etmeyen bir tamsayı bölme
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Başarısızlık `None` varyantıyla ifade edilir
        None
    } else {
        // Sonuç `Some` varyantı içinde sarılır
        Some(dividend / divisor)
    }
}

// Başarısız olabilecek bir bölme işlemini ele alan fonksiyon
fn try_division(dividend: i32, divisor: i32) {
    // `Option` değerleri diğer enum’lar gibi desen eşleştirme ile işlenebilir
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // `None` bir değişkene bağlanırken tür açıklaması gerekir
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Bir `Some` varyantını açmak (unwrap) değeri çıkarır
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Bir `None` varyantını açmak `panic!` oluşturur
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}
```

👉 Bu örnek, `Option` enum’unun değer var/yok durumlarını güvenli şekilde nasıl temsil ettiğini göstermektedir.
