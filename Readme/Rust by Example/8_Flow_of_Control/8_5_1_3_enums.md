## 🧩 Enum’lar (enums)

Bir `enum`, benzer şekilde parçalanabilir (destructured):

```rust
// `allow`, yalnızca bir varyant kullanıldığı için uyarıları susturmak amacıyla gereklidir.
#[allow(dead_code)]
enum Color {
    // Bu 3'ü yalnızca isimleriyle belirtilir.
    Red,
    Blue,
    Green,
    // Bunlar ise `u32` tuple’larını farklı isimlere bağlar: renk modelleri.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}
```

👉 Bu kısımda farklı renk modellerini temsil eden `enum` tanımlanmıştır.

```rust
fn main() {
    let color = Color::RGB(122, 17, 40);
    // TODO ^ `color` için farklı varyantlar deneyin

    println!("What color is it?");
    // Bir `enum`, `match` kullanılarak parçalanabilir.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Başka bir kola gerek yok çünkü tüm varyantlar ele alındı
    }
}
```

👉 Bu örnekte `match` ifadesi kullanılarak `enum` içindeki farklı varyantlar (`Red`, `Blue`, `Green`, `RGB`, `HSV`, `HSL`, `CMY`, `CMYK`) ayrıştırılıp ilgili değerler ekrana yazdırılmaktadır.
