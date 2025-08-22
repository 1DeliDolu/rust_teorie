## 🎨 Formatting / Biçimlendirme

Biçimlendirme (formatting), bir format dizgesi (format string) aracılığıyla belirtilir:

```rust
format!("{}", foo)       -> "3735928559"
format!("0x{:X}", foo)   -> "0xDEADBEEF"
format!("0o{:o}", foo)   -> "0o33653337357"
```

Aynı değişken (`foo`), kullanılan argüman türüne bağlı olarak farklı şekilde biçimlendirilebilir: `X` – `o` – veya belirtilmemiş (`{}`).

Bu biçimlendirme işlevselliği trait’ler aracılığıyla uygulanır ve her argüman türü için bir trait vardır. En yaygın biçimlendirme trait’i `Display`’dir. Bu, argüman türünün belirtilmediği durumları ele alır, örneğin `{}`.

---

```rust
use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` bir arabellek (buffer)’tir, ve bu metot
    // biçimlendirilmiş string’i bu arabelleğe yazmalıdır.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!`, `format!` gibidir fakat çıktıyı bir arabelleğe yazar.
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Bunu {} kullanacak şekilde değiştirin
        // Color için fmt::Display uygulamasını ekledikten sonra.
        println!("{:?}", color);
    }
}
```

---

## 🏋️ Aktivite (Activity)

Yukarıdaki `Color` struct’ı için `fmt::Display` trait’ini uygulayın. Çıktı şu şekilde olmalıdır:

```
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```

### 🔑 İpuçları

1. RGB renk uzayında (color space) bir rengin hesaplama formülü şudur:

   ```
   RGB = (R * 65536) + (G * 256) + B
   ```

   (R = Kırmızı, G = Yeşil, B = Mavi).
   Daha fazla bilgi için: RGB color format & calculation.

2. Her rengi birden fazla kez listelemeniz gerekebilir.

3. Sıfırlarla doldurmak için `:0>2` biçimlendiricisini kullanabilirsiniz.

---

## 📚 Ayrıca bakınız (See also)

* `std::fmt`
