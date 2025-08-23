## ⚖️ Çakışan Özellikleri Ayırt Etme (disambiguating overlapping traits)

Bir tür, birçok farklı **trait** uygulayabilir. Peki ya iki trait aynı isimde bir fonksiyon gerektiriyorsa? Örneğin, birçok trait’in `get()` adında bir yöntemi olabilir. Hatta bunların dönüş türleri bile farklı olabilir!

İyi haber şu ki: Her trait uygulaması kendi `impl` bloğunu aldığı için, **hangi trait’in `get` metodunu** uyguladığınız açıktır.

Peki ya bu yöntemleri çağırma zamanı geldiğinde? İşte burada **Tam Nitelikli Sözdizimi (Fully Qualified Syntax)** kullanmamız gerekir.

```rust
trait Username {
    fn get(&self) -> String;
}

trait Age {
    fn get(&self) -> u8;
}

struct User {
    name: String,
    age: u8,
}

impl Username for User {
    fn get(&self) -> String {
        self.name.clone()
    }
}

impl Age for User {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };

    // Hangi trait'in `get` metodunu çağırdığımızı açıkça belirtmeliyiz:
    let username = Username::get(&user);
    let age = Age::get(&user);

    println!("Name: {}, Age: {}", username, age);
}
```

👉 Bu şekilde, **çakışan isimli metodları ayırt etmek** için `TraitName::method(&instance)` biçimindeki tam nitelikli sözdizimini kullanırız.
