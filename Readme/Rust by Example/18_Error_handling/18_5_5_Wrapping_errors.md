## 📦 Hataları Sarmalamak (wrapping errors)

Hataları `Box` içine koymaya bir alternatif de, onları **kendi hata türünüz içinde sarmalamaktır**.

```rust
use std::error;
use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // Parse hatasının implementasyonuna yönlendiriyoruz.
    // Ekstra bilgi sağlamak için tipe daha fazla veri eklenmesi gerekir.
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            // Sarılmış hata ek bilgi içerir ve `source()` metodu ile erişilebilir.
            DoubleError::Parse(..) =>
                write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,
            // Neden, altta yatan implementasyon hata türüdür.
            // `&error::Error` trait nesnesine dönüştürülür.
            // Bu çalışır çünkü altta yatan tür zaten `Error` trait’ini uygular.
            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

// `ParseIntError` → `DoubleError` dönüşümünü implement ediyoruz.
// Eğer `?` ile bir `ParseIntError`, `DoubleError`’a dönüştürülmesi gerekirse
// bu otomatik olarak çağrılır.
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    // Burada `ParseIntError` → `DoubleError` dönüşümünü (yukarıda tanımladığımız)
    // `From` implementasyonu sayesinde otomatik kullanıyoruz.
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        },
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

Bu yöntem, hataları yönetmek için biraz daha fazla **boilerplate kod** ekler ve her uygulamada gerekli olmayabilir. Bununla ilgilenmeyi sizin yerinize yapan bazı kütüphaneler de vardır.
