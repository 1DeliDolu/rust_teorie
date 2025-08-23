## 🛠️ Türetme (derive)

Derleyici, `#[derive]` **özniteliği (attribute)** ile bazı **özellikler (trait)** için temel uygulamalar sağlayabilir. Daha karmaşık bir davranış gerektiğinde bu trait’ler yine manuel olarak uygulanabilir.

Aşağıda türetilebilen trait’lerin listesi yer almaktadır:

* Karşılaştırma trait’leri (comparison traits): `Eq`, `PartialEq`, `Ord`, `PartialOrd`.
* `Clone`, `&T` üzerinden kopya ile `T` oluşturmak için.
* `Copy`, bir türe **kopya semantiği (copy semantics)** kazandırıp **taşıma semantiği (move semantics)** yerine kullanılmasını sağlamak için.
* `Hash`, `&T` üzerinden karma değer (hash) hesaplamak için.
* `Default`, bir veri türünün boş/varsayılan örneğini oluşturmak için.
* `Debug`, bir değeri `{:?}` **biçimlendirici (formatter)** ile yazdırmak için.

```rust
// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct with no additional attributes
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    //println!("One second looks like: {:?}", _one_second);
    // TODO ^ Try uncommenting this line

    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    //let _this_is_true = (_one_second == _one_second);
    // TODO ^ Try uncommenting this line

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
```
