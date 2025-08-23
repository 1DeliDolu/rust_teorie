## ❓ `?` Operatörüne Giriş (Introducing ?)

Bazen `unwrap`’in sadeliğini isteriz, fakat `panic` riski olmadan. Şimdiye kadar `unwrap`, istediğimiz aslında sadece değişkeni almakken bizi giderek daha derin iç içe yazmaya zorladı. İşte `?` operatörünün tam amacı budur.

Bir `Err` bulunduğunda iki geçerli eylem vardır:

* `panic!` → mümkünse bundan kaçınmaya karar verdik
* `return` → çünkü `Err` demek, hatanın burada ele alınamayacağı anlamına gelir

`?`, neredeyse `unwrap` ile aynıdır; fakat `Err` durumunda `panic` etmek yerine fonksiyondan **erken dönüş** (`return`) yapar.

Aşağıda, birleştiricilerle yazdığımız örneği `?` kullanarak nasıl sadeleştirdiğimize bakalım:

```rust
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

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

👉 Burada `?`, hataları çağırana döndürerek kodu çok daha okunabilir hale getirir.

---

## 🕰️ `try!` Makrosu (The try! macro)

`?` gelmeden önce, aynı işlevsellik `try!` makrosu ile sağlanıyordu. Artık `?` önerilmektedir, ancak eski kodlarda `try!` hâlâ görülebilir.

Aynı `multiply` fonksiyonu, `try!` ile şu şekilde yazılırdı:

```rust
// Bu örneği hatasız derleyip çalıştırmak için Cargo kullanıyorsanız,
// `Cargo.toml` dosyasındaki `[package]` bölümünde `edition` değerini "2015" olarak değiştirin.

use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = try!(first_number_str.parse::<i32>());
    let second_number = try!(second_number_str.parse::<i32>());

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

👉 Günümüzde modern Rust kodlarında `try!` yerine `?` kullanılması tavsiye edilir.
