## ❓ `?` operatörü

`match` ile `Result` zincirlemek (chaining) kodu oldukça karmaşık hale getirebilir; neyse ki, `?` operatörü kodu tekrar sadeleştirmek için kullanılabilir.

`?`, bir `Result` döndüren ifadenin sonunda kullanılır ve aslında bir `match` ifadesine denktir:

* `Err(err)` dalı, erken `return Err(From::from(err))` olarak açılır.
* `Ok(ok)` dalı ise sadece `ok` ifadesine açılır.

```rust
mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // Ara fonksiyon
    fn op_(x: f64, y: f64) -> MathResult {
        // Eğer `div` "başarısız olursa", `DivisionByZero` return edilir
        let ratio = div(x, y)?;

        // Eğer `ln` "başarısız olursa", `NonPositiveLogarithm` return edilir
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}", match why {
                MathError::NonPositiveLogarithm
                    => "logarithm of non-positive number",
                MathError::DivisionByZero
                    => "division by zero",
                MathError::NegativeSquareRoot
                    => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}
```

👉 `Result` ile çalışırken `map`, `and_then`, `unwrap_or` gibi birçok metodu da kullanabilirsiniz. Ayrıntılar için belgeleri inceleyin.
