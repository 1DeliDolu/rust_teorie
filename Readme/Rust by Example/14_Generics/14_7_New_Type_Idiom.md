## 🆕 New Type Idiom

**Newtype idiomu**, bir programa doğru türde değerlerin verilmesini derleme zamanında garanti eder.

Örneğin, yaşı yıl cinsinden kontrol eden bir yaş doğrulama fonksiyonunun mutlaka `Years` türünde bir değer alması gerekir.

```rust
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    /// Yarım yılları keser
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(25);
    let age_days = age.to_days();
    println!("Is an adult? {}", is_adult(&age));
    println!("Is an adult? {}", is_adult(&age_days.to_years()));
    // println!("Is an adult? {}", is_adult(&age_days));
}
```

Yukarıdaki kodda, son `println!` satırının yorumunu kaldırırsanız, `is_adult` fonksiyonunun yalnızca `Years` türü kabul ettiğini ve `Days` türünün verilemeyeceğini görürsünüz.

### 📝 Newtype değerini temel tür olarak elde etmek

`newtype`’ın iç değerine erişmek için **tuple sözdizimi** veya **destructuring** kullanılabilir:

```rust
struct Years(i64);

fn main() {
    let years = Years(42);
    let years_as_primitive_1: i64 = years.0; // Tuple ile erişim
    let Years(years_as_primitive_2) = years; // Destructuring ile erişim
}
```
