## 🧪 Test Durumu: Birim Açıklaması (testcase: unit clarification)

Birim dönüşümleri (unit conversions) için faydalı bir yöntem, hayalet tür parametresi (phantom type parameter) kullanarak `Add` özelliğini (trait) uygulamayı içerir. `Add` özelliği (trait) aşağıda incelenmiştir:

```rust
// Bu kurgu şu koşulu dayatır: `Self + RHS = Output`
// RHS, implementasyonda belirtilmezse varsayılan olarak Self kabul edilir.
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// `Output`, `T<U>` olmalıdır; böylece `T<U> + T<U> = T<U>` olur.
impl<U> Add for T<U> {
    type Output = T<U>;
    ...
}
```

Tüm uygulama (implementation) şöyledir:

```rust
use std::ops::Add;
use std::marker::PhantomData;

/// Birim türlerini tanımlamak için boş (void) numaralandırmalar oluşturun.
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length`, hayalet tür parametresi (phantom type parameter) `Unit` olan bir türdür
/// ve uzunluk değerinin türünde (yani `f64`) generic değildir.
///
/// `f64`, zaten `Clone` ve `Copy` özelliklerini (traits) uygular.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// `Add` özelliği (trait), `+` operatörünün davranışını tanımlar.
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // add(), toplamı içeren yeni bir `Length` yapısı döndürür.
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+`, `f64` için `Add` implementasyonunu çağırır.
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    // `one_foot`, hayalet tür parametresi `Inch` olacak şekilde belirtilir.
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    // `one_meter`, hayalet tür parametresi `Mm`'dir.
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // `+`, `Length<Unit>` için implement ettiğimiz `add()` metodunu çağırır.
    //
    // `Length`, `Copy` uyguladığından `add()`, `one_foot` ve `one_meter`'i
    // tüketmez; bunları `self` ve `rhs` içine kopyalar.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Toplama çalışır.
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // Anlamsız işlemler olması gerektiği gibi başarısız olur:
    // Derleme zamanı hatası: tür uyumsuzluğu.
    //let one_feter = one_foot + one_meter;
}
```
