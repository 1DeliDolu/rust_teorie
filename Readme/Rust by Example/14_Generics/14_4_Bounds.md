## 📏 Sınırlar (bounds)

Genel türlerle (generics) çalışırken, tür parametrelerinin (type parameters) hangi işlevselliği uyguladığını belirtmek için sıklıkla trait sınırları (trait bounds) kullanmak gerekir. Örneğin aşağıdaki örnek, yazdırma yapmak için `Display` trait’ini (Display) kullanır; dolayısıyla `T`, `Display` ile sınırlandırılmalıdır (yani `T`, `Display`’i uygulamalıdır).

```rust
// Genel tür `T` alan ve `Display` trait'ini zorunlu kılan `printer` fonksiyonunu tanımla.
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

Sınır koyma (bounding), jenereği yalnızca bu sınırlara uyan türlerle kısıtlar. Yani:

```rust
struct S<T: Display>(T);

// Hata! `Vec<T>` `Display`'i uygulamaz. Bu uzmanlaştırma başarısız olur.
let s = S(vec![1]);
```

Sınırların bir diğer etkisi, belirtilen trait’lerin (traits) metotlarına erişimi sağlamasıdır. Örneğin:

```rust
// `{:?}` işaretleyicisiyle yazdırma yapan bir trait.
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// Genel `T`, `Debug`'u uygulamalıdır.
// Tür ne olursa olsun bu doğru çalışır.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T`, `HasArea`'ı uygulamalıdır.
// Sınırı karşılayan herhangi bir tür, `HasArea`'ın `area` fonksiyonuna erişebilir.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ TODO: Bu satırların yorumunu kaldırmayı deneyin.
    // | Hata: `_triangle` ne `Debug`'u ne de `HasArea`'ı uygular.
}
```

Ek bir not olarak, bazı durumlarda sınırları daha ifade gücü yüksek biçimde uygulamak için `where` hükümleri (where clauses) de kullanılabilir.
