## ⚠️ Result

`Option` enum’unun, başarısız olabilecek fonksiyonlardan dönüş değeri olarak kullanılabileceğini görmüştük; burada `None` başarısızlığı ifade eder. Ancak bazen **neden** bir işlemin başarısız olduğunu belirtmek önemlidir. Bunun için `Result` enum’u vardır.

`Result<T, E>` enum’unun iki varyantı vardır:

* `Ok(value)`: işlemin başarılı olduğunu gösterir ve döndürülen değeri sarar (`value` türü `T`).
* `Err(why)`: işlemin başarısız olduğunu gösterir ve sebebini sarar (`why` türü `E`).

```rust
mod checked {
    // Yakalamak istediğimiz matematiksel "hatalar"
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // Bu işlem `fail` olurdu; bunun yerine sebebi `Err` ile döndür
            Err(MathError::DivisionByZero)
        } else {
            // Geçerli işlem, sonucu `Ok` ile döndür
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> f64 {
    // Üç katmanlı bir match yapısı!
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    // Bu başarısız olacak mı?
    println!("{}", op(1.0, 10.0));
}
```

👉 Bu örnek, `Result` enum’unun hem başarılı (`Ok`) hem de hatalı (`Err`) sonuçları nasıl ifade ettiğini göstermektedir.
