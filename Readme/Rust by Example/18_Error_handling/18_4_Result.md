## 🛠️ Result (Result)

`Result`, olası **yokluk** durumunu tanımlayan `Option`’ın daha zengin bir versiyonudur; burada odak, **olası hata** üzerindedir.

`Result<T, E>` iki farklı sonuçtan birini dönebilir:

* `Ok(T)`: T türünden bir değer bulundu
* `Err(E)`: E türünden bir hata bulundu

Rust’ta konvansiyon olarak, beklenen sonuç `Ok`, beklenmeyen sonuç ise `Err`’dir.

---

### 🔎 unwrap() ve hata durumları

`Option` gibi `Result` da birçok metoda sahiptir. Örneğin, `unwrap()` ya değeri (`T`) döner ya da `panic` oluşturur. Rust’ta, `Result` ve `Option` arasında örtüşen birçok birleştirici (combinator) vardır.

Rust ile çalışırken, `Result` döndüren metotlara sık sık rastlarsınız. Örneğin `parse()` metodu:
Bir string’i başka bir türe çevirmek her zaman mümkün olmayabilir, bu yüzden `parse()` olası bir başarısızlığı belirtmek için `Result` döndürür.

Başarılı ve başarısız `parse()` denemelerine bakalım:

```rust
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // `unwrap()` ile değeri almayı deneyelim. Bizi zor durumda bırakır mı?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}
```

👉 Başarısız durumda, `parse()` hata döndürür ve `unwrap()` bu hata üzerinde `panic` üretir. Bu da programımızın kapanmasına ve tatsız bir hata mesajına neden olur.

Daha kaliteli bir hata mesajı için, dönüş türünü daha açık belirtmek ve hatayı açıkça ele almak gerekir.

---

### 🏷️ `main` içinde `Result` kullanımı

`Result` türü, `main` fonksiyonunun dönüş türü olarak da açıkça belirtilebilir. Normalde `main` şu formdadır:

```rust
fn main() {
    println!("Hello World!");
}
```

Ancak `main`, dönüş türü olarak `Result` alabilir. Eğer `main` içinde bir hata oluşursa, program bir **hata kodu** ile döner ve hatanın **Debug** gösterimini ekrana yazdırır.

Aşağıdaki örnek, böyle bir senaryoyu göstermektedir:

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```

👉 Bu şekilde, `main` fonksiyonu başarısız olduğunda daha kontrollü bir hata yönetimi sağlanır.

---

İstiyor musunuz ki ben size `Option` ve `Result` arasındaki farkları bir tabloyla özetleyeyim?
