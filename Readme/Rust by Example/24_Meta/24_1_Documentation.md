## 📖 Dokümantasyon (documentation)

`cargo doc` komutunu kullanarak `target/doc` dizininde dokümantasyon oluşturabilirsiniz.
`cargo doc --open` komutu dokümantasyonu otomatik olarak web tarayıcınızda açacaktır.

`cargo test` komutu tüm testleri (dokümantasyon testleri dahil) çalıştırır,
`cargo test --doc` ise yalnızca dokümantasyon testlerini çalıştırır.

Bu komutlar gerektiğinde `rustdoc` (ve `rustc`) araçlarını çağırır.

---

## 📝 Dokümantasyon Yorumları (doc comments)

Dokümantasyon yorumları, büyük projeler için oldukça faydalıdır. `rustdoc` çalıştırıldığında, bu yorumlar derlenip dokümantasyona eklenir.
`///` ile başlarlar ve **Markdown** sözdizimini desteklerler.

````rust
#![crate_name = "doc"]

/// Burada bir insan temsil edilmektedir
pub struct Person {
    /// Bir kişinin mutlaka bir adı olmalıdır, Juliet bundan ne kadar nefret ederse etsin
    name: String,
}

impl Person {
    /// Verilen isimle yeni bir kişi oluşturur.
    ///
    /// # Örnekler
    ///
    /// ```
    /// // Yorumlar içindeki çitler arasında Rust kodu yazabilirsiniz
    /// // Eğer `rustdoc`'a --test parametresini verirseniz, bu kodu test edecektir!
    /// use doc::Person;
    /// let person = Person::new("isim");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Dostça bir merhaba döndürür!
    ///
    /// Çağrıldığı `Person` için "Hello, [name](Person::name)" ifadesini söyler.
    pub fn hello(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}
````

Testleri çalıştırmak için önce kodu bir kütüphane olarak derlemeli, ardından `rustdoc`'a bu kütüphaneyi nerede bulacağını söylemelisiniz:

```bash
$ rustc doc.rs --crate-type lib
$ rustdoc --test --extern doc="libdoc.rlib" doc.rs
```

---

## ⚙️ Dokümantasyon Öznitelikleri (doc attributes)

Aşağıda `rustdoc` ile en sık kullanılan `#[doc]` özniteliklerinden bazı örnekler bulunmaktadır:

### `inline`

Dokümantasyonu ayrı sayfaya yönlendirmek yerine satır içine dahil eder.

```rust
#[doc(inline)]
pub use bar::Bar;

/// bar dokümantasyonu
pub mod bar {
    /// Bar için dokümantasyon
    pub struct Bar;
}
```

### `no_inline`

Ayrı sayfaya yönlendirmeyi veya herhangi bir bağlantıyı engeller.

```rust
// libcore/prelude örneği
#[doc(no_inline)]
pub use crate::mem::drop;
```

### `hidden`

Bu öznitelik kullanıldığında öğe dokümantasyona dahil edilmez.

```rust
// futures-rs kütüphanesinden örnek
#[doc(hidden)]
pub use self::async_await::*;
```

---

Rust topluluğunda `rustdoc` yaygın olarak kullanılmaktadır. `std` kütüphane dokümantasyonu da bu araçla üretilmektedir.

---

## 📚 Ayrıca bakınız

* Rust Kitabı: Yararlı Dokümantasyon Yorumları Yazmak
* The rustdoc Book
* The Reference: Doc comments
* RFC 1574: API Documentation Conventions
* RFC 1946: Relative links to other items from doc comments (intra-rustdoc links)
* *Is there any documentation style guide for comments?* (reddit)
