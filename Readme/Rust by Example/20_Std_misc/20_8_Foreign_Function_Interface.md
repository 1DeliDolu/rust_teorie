## 🌐 Yabancı Fonksiyon Arayüzü (Foreign Function Interface)

Rust, C kütüphanelerine erişim için bir **Yabancı Fonksiyon Arayüzü (FFI)** sağlar.
Yabancı fonksiyonlar, yabancı kütüphanenin adını içeren bir `#[link]` özniteliğiyle işaretlenmiş bir `extern` bloğu içinde bildirilmelidir.

```rust
use std::fmt;

// bu extern bloğu libm kütüphanesine bağlanır
#[cfg(target_family = "windows")]
#[link(name = "msvcrt")]
extern {
    // bu, tek duyarlıklı (single precision) karmaşık sayının karekökünü
    // hesaplayan bir yabancı fonksiyondur
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}
#[cfg(target_family = "unix")]
#[link(name = "m")]
extern {
    // bu, tek duyarlıklı (single precision) karmaşık sayının karekökünü
    // hesaplayan bir yabancı fonksiyondur
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}

// Yabancı fonksiyonları çağırmak unsafe kabul edilir,
// bu yüzden genellikle onların etrafında güvenli sarmalayıcılar yazılır.
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // yabancı fonksiyonu çağırmak unsafe bir işlemdir
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // unsafe işlemi sarmalayan güvenli API’yi çağırmak
    println!("cos({:?}) = {:?}", z, cos(z));
}

// Tek duyarlıklı karmaşık sayıların minimal implementasyonu
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
```
