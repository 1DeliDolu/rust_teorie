## 🔙 Erken Dönüşler (early returns)

Önceki örnekte, hataları birleştiriciler (combinators) kullanarak açıkça ele aldık. Bu tür durum analizleriyle başa çıkmanın bir diğer yolu, `match` ifadeleri (match statements) ile **erken dönüşler** (early returns) kombinasyonunu kullanmaktır.

Yani, bir hata meydana gelirse, fonksiyonu yürütmeyi basitçe durdurup hatayı döndürebiliriz. Bazıları için bu kod biçimi hem okumayı hem de yazmayı kolaylaştırabilir. Aşağıda, önceki örneğin erken dönüşler kullanılarak yeniden yazılmış hâlini görebilirsiniz:

```rust
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
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

Bu noktada, hataları birleştiriciler (combinators) ve erken dönüşler (early returns) kullanarak açıkça ele almayı öğrendik. Genellikle `panic` (panic) etmekten kaçınmak isteriz; ancak tüm hataları açıkça ele almak zahmetli olabilir.

Bir sonraki bölümde, `?` operatörünü (operator) tanıtacağız; bu, `panic` oluşturmadan basitçe `unwrap` (unwrap) etmemiz gereken durumlar için uygundur.
