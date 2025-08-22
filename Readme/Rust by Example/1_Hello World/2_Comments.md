## 🖨️ Formatted print / Biçimlendirilmiş yazdırma

Yazdırma işlemi `std::fmt` içinde tanımlı bir dizi makro (macro) ile gerçekleştirilir. Bunlardan bazıları şunlardır:

* `format!`: Biçimlendirilmiş metni `String` içine yazar.
* `print!`: `format!` ile aynı fakat metin konsola (`io::stdout`) yazdırılır.
* `println!`: `print!` ile aynı fakat satır sonuna yeni satır eklenir.
* `eprint!`: `print!` ile aynı fakat metin standart hata çıkışına (`io::stderr`) yazdırılır.
* `eprintln!`: `eprint!` ile aynı fakat satır sonuna yeni satır eklenir.

Tüm bu makrolar metni aynı şekilde çözümler. Ek olarak, Rust biçimlendirme doğruluğunu derleme zamanında kontrol eder.

---

```rust
fn main() {
    // Genel olarak, `{}` otomatik olarak herhangi bir argümanla değiştirilir.
    // Bu argümanlar string’e dönüştürülür.
    println!("{} days", 31);

    // Konumsal argümanlar (positional arguments) kullanılabilir.
    // İçindeki tam sayı, hangi ek argümanın yerleştirileceğini belirtir.
    // Argümanlar format string’inden sonra 0’dan başlar.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // İsimlendirilmiş argümanlar da kullanılabilir.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Farklı biçimlendirmeler `:` sonrası belirtilir.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // Sağdan hizalama (right-justify) genişlik belirtilerek yapılabilir.
    // Çıktı "    1" olacaktır. (Toplam genişlik 5, boşluklar + "1")
    println!("{number:>5}", number=1);

    // Sayılar sıfırlarla doldurulabilir.
    println!("{number:0>5}", number=1); // 00001
    // Sola yaslama (left-adjust) işareti ters çevirerek yapılır.
    println!("{number:0<5}", number=1); // 10000

    // Format belirleyicide isimlendirilmiş argümanlar `$` eklenerek kullanılabilir.
    println!("{number:0>width$}", number=1, width=5);

    // Rust ayrıca argüman sayısının doğru olmasını da kontrol eder.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Sadece `fmt::Display` uygulayan türler `{}` ile biçimlendirilebilir.
    // Kullanıcı tanımlı türler varsayılan olarak `fmt::Display` uygulamaz.

    #[allow(dead_code)] // kullanılmayan modül için `dead_code` uyarısını kapatır
    struct Structure(i32);

    // Bu derlenmez çünkü `Structure` fmt::Display uygulamaz.
    // println!("This struct `{}` won't print...", Structure(3));

    // Rust 1.58 ve sonrası, değişkenleri doğrudan format string’den yakalayabilir.
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
```

---

## 📊 std::fmt özellikleri

`std::fmt` birçok trait içerir ve bunlar metnin nasıl gösterileceğini belirler. İki önemli temel trait şunlardır:

* `fmt::Debug`: `{:?}` işaretleyicisini kullanır. Metni hata ayıklama (debugging) amacıyla biçimlendirir.
* `fmt::Display`: `{}` işaretleyicisini kullanır. Metni daha şık ve kullanıcı dostu bir şekilde biçimlendirir.

Burada `fmt::Display` kullandık çünkü standart kütüphane bu türler için uygulamalar sağlar. Özel türleri yazdırmak için ek adımlar gerekir.

`fmt::Display` trait’i uygulanınca `ToString` trait’i de otomatik olarak uygulanır ve türün `String`’e dönüştürülmesine izin verir.

Satır 43’teki `#[allow(dead_code)]`, yalnızca takip eden modül için geçerli bir özelliktir (attribute).

---

## 🏋️ Aktiviteler (Activities)

1. Yukarıdaki koddaki hatayı (FIXME) düzeltin, yani eksik argümanı ekleyin.
2. `Structure` struct’ını biçimlendirmeye çalışan satırı (TODO) yorumdan çıkarıp deneyin.
3. Aşağıdaki çıktıyı üretecek şekilde bir `println!` ekleyin:

   ```
   Pi is roughly 3.142
   ```

   Bunun için `let pi = 3.141592;` değerini kullanın ve gösterilen ondalık basamak sayısını kontrol edin.

---

## 📚 Ayrıca bakınız (See also)

* `std::fmt`
* `macros`
* `struct`
* `traits`
* `dead_code`
