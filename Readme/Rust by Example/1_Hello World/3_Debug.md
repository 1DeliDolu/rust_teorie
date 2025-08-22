## 🐞 Debug / Hata Ayıklama

`std::fmt` biçimlendirme (formatting) trait’lerini kullanmak isteyen tüm türler (types), yazdırılabilir (printable) olabilmek için bir uygulamaya (implementation) ihtiyaç duyar. Otomatik uygulamalar yalnızca standart kütüphane (std library) türleri gibi belirli türler için sağlanır. Diğer tüm türler manuel olarak uygulanmak zorundadır.

`fmt::Debug` trait’i bu işi oldukça kolaylaştırır. Tüm türler `fmt::Debug` uygulamasını türetebilir (derive, yani otomatik oluşturabilir). Bu durum `fmt::Display` için geçerli değildir, çünkü `fmt::Display` manuel olarak uygulanmalıdır.

---

```rust
// Bu yapı (structure), ne `fmt::Display` ile
// ne de `fmt::Debug` ile yazdırılabilir.
struct UnPrintable(i32);

// `derive` özelliği, `fmt::Debug` ile yazdırılabilir olmak için
// gereken uygulamayı otomatik olarak oluşturur.
#[derive(Debug)]
struct DebugPrintable(i32);
```

---

Tüm standart kütüphane (std library) türleri de `{:?}` ile otomatik olarak yazdırılabilir:

```rust
// `Structure` için `fmt::Debug` uygulamasını türet (derive).
// `Structure` bir `i32` içeren bir yapıdır.
#[derive(Debug)]
struct Structure(i32);

// `Structure`’ı `Deep` adlı başka bir yapının içine koy.
// Bunu da yazdırılabilir yap.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // `{:?}` ile yazdırmak, `{}` ile yazdırmaya benzer.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` yazdırılabilir!
    println!("Now {:?} will print!", Structure(3));

    // `derive` ile ilgili sorun, sonucun nasıl görüneceği üzerinde
    // hiçbir kontrol olmamasıdır. Ya sadece `7` görünsün istersem?
    println!("Now {:?} will print!", Deep(Structure(7)));
}
```

---

## 🎨 Pretty Printing

`fmt::Debug` kesinlikle yazdırılabilirlik sağlar, ancak biraz zarafetten ödün verir. Rust ayrıca `{:#?}` ile “pretty printing” (güzel biçimlendirilmiş yazdırma) olanağı sağlar:

```rust
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
```

---

Bir türün nasıl gösterileceğini kontrol etmek için `fmt::Display` manuel olarak uygulanabilir.

---

## 📚 Ayrıca bakınız (See also)

* `attributes`
* `derive`
* `std::fmt`
* `struct`
