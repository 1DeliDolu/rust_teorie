## ❓ `?` Operatörünün Diğer Kullanımları (other uses of ?)

Önceki örnekte fark ederseniz, `parse` çağrısına verdiğimiz ilk tepki, kütüphane hatasını bir **boxed error**’a dönüştürmek olmuştu:

```rust
.and_then(|s| s.parse::<i32>())
    .map_err(|e| e.into())
```

Bu basit ve sık yapılan bir işlem olduğundan, bunun daha kısa bir yolu olsa kullanışlı olurdu. Ne yazık ki `and_then` yeterince esnek olmadığı için bunu doğrudan yapamayız. Ancak bunun yerine `?` kullanabiliriz.

Daha önce `?`, ya `unwrap` ya da `return Err(err)` anlamına geliyor diye açıklanmıştı. Bu yalnızca **büyük ölçüde** doğrudur. Aslında `?`, `unwrap` veya `return Err(From::from(err))` demektir. `From::from`, farklı türler arasında dönüştürme aracı olduğundan, `?` kullanıldığında hata dönüş türüne dönüştürülebiliyorsa otomatik olarak dönüşüm yapılır.

Aşağıda önceki örneği `?` kullanarak yeniden yazıyoruz. Bunun sonucunda `map_err` gereksiz hale gelir çünkü `From::from` hata türümüz için uygulanmıştır:

```rust
use std::error;
use std::fmt;

// Alias'ı `Box<dyn error::Error>` kullanacak şekilde değiştiriyoruz.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

// Daha önceki yapının aynısı, fakat tüm `Result` ve `Option` zincirlemek yerine
// `?` kullanarak içteki değeri hemen çıkarıyoruz.
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?; 
    let parsed = first.parse::<i32>()?;      
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
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

Bu hali artık oldukça temiz. İlk `panic` ile karşılaştırıldığında, bu yaklaşım `unwrap` çağrılarını `?` ile değiştirmeye çok benzer; tek fark dönüş türünün `Result` olmasıdır. Dolayısıyla, en üst seviyede bu `Result` yapısının **destructure edilmesi** gerekir.
