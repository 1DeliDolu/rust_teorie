## 📦 `dyn` ile Özellik Döndürme (returning traits with dyn)

Rust **derleyicisi (compiler)**, her fonksiyonun **dönüş türü (return type)** için ne kadar alan gerektiğini bilmek zorundadır. Bu, tüm fonksiyonlarınızın somut bir tür (**concrete type**) döndürmesi gerektiği anlamına gelir. Diğer dillerin aksine, `Animal` gibi bir **özellik (trait)** varsa, `Animal` döndüren bir fonksiyon yazamazsınız; çünkü farklı uygulamalar farklı miktarda belleğe ihtiyaç duyar.

Ancak, kolay bir geçici çözüm vardır. Bir **özellik nesnesi (trait object)** doğrudan döndürmek yerine, fonksiyonlarımız bir `Box` döndürür ve bunun içinde bir `Animal` bulunur. Bir `Box`, yığında (**heap**) bir belleğe referans (**reference**) olmaktan ibarettir. Bir referansın boyutu derleme zamanında statik olarak bilindiğinden ve derleyici bunun yığında ayrılmış bir `Animal`’ı işaret ettiğini garanti edebildiğinden, fonksiyonumuzdan bir trait döndürebiliriz!

Rust, yığında bellek ayırdığında mümkün olduğunca açık olmaya çalışır. Bu nedenle, fonksiyonunuz bu şekilde yığın üzerindeki bir özelliğe işaretçi döndürüyorsa, dönüş türünü `dyn` **anahtar sözcüğü (keyword)** ile yazmanız gerekir; örneğin `Box<dyn Animal>`.

```rust
struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
```
