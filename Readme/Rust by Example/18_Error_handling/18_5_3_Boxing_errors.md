## 📦 Hataları Box’lama (boxing errors)

Orijinal hataları korurken **basit kod yazmanın** bir yolu, hataları `Box` içine koymaktır. Bunun dezavantajı, altta yatan hata türünün **sadece çalışma zamanında** bilinmesi ve **statik olarak belirlenememesidir**.

`stdlib`, `Box` türünün, `Error` trait’ini uygulayan herhangi bir türden `Box<Error>` trait nesnesine dönüşümü `From` aracılığıyla desteklemesi sayesinde hataları box’lamamıza yardımcı olur.

```rust
use std::error;
use std::fmt;

// Alias'ı `Box<dyn error::Error>` kullanacak şekilde değiştiriyoruz.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Box'a dönüştürülüyor
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Box'a dönüştürülüyor
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
