## 📦 Option’ları `?` ile Açma (Unpacking options with ?)

`Option` değerlerini `match` ifadeleriyle açabilirsiniz, ancak çoğu zaman `?` operatörünü kullanmak daha kolaydır. Eğer `x` bir `Option` ise, `x?` ifadesi:

* `Some` durumunda içteki değeri döndürür,
* `None` durumunda ise yürütülmekte olan fonksiyonu sonlandırır ve `None` döndürür.

```rust
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // Eğer `current_age` `None` ise, bu fonksiyon `None` döner.
    // Eğer `current_age` `Some` ise, içteki `u8` değeri + 1
    // `next_age` değişkenine atanır.
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}
```

👉 Bu sayede `match` kullanmadan, kısa ve okunabilir bir kod yazabilirsiniz.

Birçok `?` operatörünü zincirleme kullanarak kodunuzu çok daha okunabilir hale getirebilirsiniz:

```rust
struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // Eğer mevcutsa, kişinin iş numarasının alan kodunu alır.
    fn work_phone_area_code(&self) -> Option<u8> {
        // `?` olmadan bu işlem birçok iç içe `match` gerektirirdi.
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
```

👉 Bu örnekte, `job`, `phone_number` ve `area_code` zincirleme `?` ile kontrol edilir. Eğer herhangi biri `None` olursa, fonksiyon hemen `None` döner.
