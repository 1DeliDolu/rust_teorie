## 🐑 Özellikler (traits)

Bir **özellik (trait)**, bilinmeyen bir tür için tanımlanan yöntemler (methods) koleksiyonudur: `Self`. Bu yöntemler, aynı trait içinde tanımlanan diğer yöntemlere erişebilir.

Trait’ler herhangi bir veri türü için uygulanabilir. Aşağıdaki örnekte, `Animal` adlı bir yöntemler grubu tanımlıyoruz. Ardından `Animal` trait’i `Sheep` veri türü için uygulanır ve böylece `Sheep` üzerinde `Animal` içindeki yöntemler kullanılabilir.

```rust
struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // İlişkili fonksiyon imzası; `Self`, uygulayıcı (implementor) türünü ifade eder.
    fn new(name: &'static str) -> Self;

    // Yöntem imzaları; bunlar bir string döndürecek.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Trait’ler varsayılan yöntem tanımları sağlayabilir.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Uygulayıcı yöntemler, uygulayıcının trait yöntemlerini kullanabilir.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// `Sheep` için `Animal` trait’ini uygula.
impl Animal for Sheep {
    // `Self`, uygulayıcı türdür: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // Varsayılan trait yöntemleri ezilebilir (override).
    fn talk(&self) {
        // Örneğin, biraz sessiz düşünce ekleyebiliriz.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // Bu durumda tür açıklaması (type annotation) gereklidir.
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Tür açıklamasını kaldırmayı deneyin.

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
```
