## 🔄 Tür Dönüştürme (casting)

Rust, ilkel türler (primitive types) arasında örtük tür dönüşümüne (implicit type conversion, coercion) izin vermez. Ancak, açık tür dönüşümü (explicit type conversion, casting) `as` anahtar kelimesi ile yapılabilir.

Tamsayı türleri (integral types) arasındaki dönüştürme kuralları genellikle C dilindeki kuralları takip eder, ancak C’de tanımsız davranışa (undefined behavior) yol açan durumlar hariç tutulur. Rust’ta tamsayı türleri arasındaki tüm dönüşümlerin davranışı açıkça tanımlanmıştır.

```rust
// Taşmaya neden olan tüm cast uyarılarını bastır.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Hata! Örtük dönüşüm yok
    let integer: u8 = decimal;
    // FIXME ^ Bu satırı yorum satırı yapın

    // Açık dönüşüm
    let integer = decimal as u8;
    let character = integer as char;

    // Hata! Dönüşüm kurallarında sınırlamalar vardır.
    // Bir float doğrudan char’a dönüştürülemez.
    let character = decimal as char;
    // FIXME ^ Bu satırı yorum satırı yapın

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // Bir değeri işaretsiz türe (unsigned type) dönüştürürken,
    // değer yeni türe sığana kadar T::MAX + 1 eklenir veya çıkarılır

    // 1000 zaten bir u16 içine sığar
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // İçeride, en düşük 8 bit (LSB) tutulur,
    // geri kalan en yüksek bitlere (MSB) doğru olanlar kesilir.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // Pozitif sayılar için bu, modulus işlemine eşdeğerdir
    println!("1000 mod 256 is : {}", 1000 % 256);

    // İşaretli türe dönüştürmede, (bit seviyesinde) sonuç,
    // önce karşılık gelen işaretsiz türe dönüştürülmüş gibidir.
    // Eğer bu değerin en anlamlı biti (MSB) 1 ise, değer negatiftir.

    // Tabii ki, değer zaten sığıyorsa farklı değildir.
    println!(" 128 as a i16 is: {}", 128 as i16);

    // Sınır durumda, 128’in 8-bit iki’nin tümleyeni gösteriminde değeri -128’dir
    println!(" 128 as a i8 is : {}", 128 as i8);

    // Yukarıdaki örneğin tekrarı
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // ve 232’nin 8-bit iki’nin tümleyeni gösterimindeki değeri -24’tür
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Rust 1.45’ten itibaren `as` anahtar kelimesi,
    // float → int dönüşümünde *doyurucu dönüşüm* (saturating cast) yapar.
    // Eğer kayan nokta değeri üst sınırı aşarsa veya alt sınırın altına düşerse,
    // dönen değer aşılan sınıra eşit olur.

    // 300.0 as u8 -> 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 -> 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 -> 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // Bu davranış küçük bir çalışma zamanı maliyetine sahiptir
    // ve unsafe yöntemlerle önlenebilir,
    // ancak sonuçlar taşabilir ve **geçersiz değerler** döndürebilir.
    // Bu yöntemleri dikkatli kullanın:
    unsafe {
        // 300.0 as u8 -> 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 -> 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 -> 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
```
