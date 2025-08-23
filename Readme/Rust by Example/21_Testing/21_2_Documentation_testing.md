## 📖 Dokümantasyon Testleri (documentation testing)

Bir Rust projesini belgelemenin (documentation) temel yolu, kaynak kodu üzerinde açıklama (annotation) yapmaktır. Dokümantasyon yorumları (documentation comments), CommonMark Markdown formatında yazılır ve içinde kod bloklarını destekler. Rust doğruluğa önem verdiği için, bu kod blokları derlenir ve dokümantasyon testleri (documentation tests) olarak kullanılır.

````rust
/// İlk satır, fonksiyonu tanımlayan kısa bir özet içerir.
///
/// Sonraki satırlar detaylı açıklama içerir. Kod blokları üç ters tırnakla
/// başlar ve içerisinde örtük olarak `fn main()`
/// ve `extern crate <cratename>` bulunur. Burada `playground`
/// adında bir kütüphane crate test ediliyor varsayalım:
///
/// ```
/// let result = playground::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

````rust
/// Genellikle dokümantasyon yorumları "Examples", "Panics" ve "Failures"
/// bölümlerini içerebilir.
///
/// Bu fonksiyon iki sayıyı böler.
///
/// # Examples
///
/// ```
/// let result = playground::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// İkinci argüman sıfır olduğunda fonksiyon panic eder.
///
/// ```rust,should_panic
/// // sıfıra bölmede panic oluşur
/// playground::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}
````

Dokümantasyon içindeki kod blokları, normal `cargo test` komutu çalıştırıldığında otomatik olarak test edilir:

```bash
$ cargo test
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests playground

running 3 tests
test src/lib.rs - add (line 7) ... ok
test src/lib.rs - div (line 21) ... ok
test src/lib.rs - div (line 31) ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 🎯 Dokümantasyon Testlerinin Amacı (motivation behind documentation tests)

Dokümantasyon testlerinin temel amacı, fonksiyonelliği çalıştıran örnekler sunmaktır. Bu, en önemli yönergelerden biridir çünkü belgelerdeki örneklerin tam kod parçacıkları (code snippets) olarak kullanılmasını sağlar.

Ancak `?` kullanmak, `main` fonksiyonunun `()` döndürmesi sebebiyle derleme hatasına yol açar. Bu noktada bazı kaynak satırlarını dokümantasyondan gizleme (hide) özelliği kurtarıcı olur: Örneğin `fn try_main() -> Result<(), ErrorType>` şeklinde yazılır, bu fonksiyon gizlenir ve `main` içerisinde unwrap edilerek çağrılır.

Bir örnek:

````rust
/// Doküman testlerinde gizli `try_main` kullanımı.
///
/// ```
/// # // gizli satırlar `#` ile başlar, ama yine de derlenirler!
/// # fn try_main() -> Result<(), String> { // doc'ta gösterilen gövdeyi saran satır
/// let res = playground::try_div(10, 2)?; 
/// # Ok(()) // try_main’den dönüş
/// # }
/// # fn main() { // unwrap eden main fonksiyonu
/// #    try_main().unwrap(); // try_main’i çağırır ve unwrap yapar
/// #                         // hata durumunda test panic edecektir
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}
````

---

## 📚 Ayrıca Bakınız

* Dokümantasyon stili üzerine *RFC505*
* API yönergeleri: dokümantasyon kılavuzları
