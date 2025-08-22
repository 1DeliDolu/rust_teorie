## 🖨️ Biçimlendirilmiş Yazdırma (formatted print)

Yazdırma işlemi `std::fmt` içinde tanımlanan bir dizi makrodan (macro) oluşur; bunlardan bazıları şunlardır:

* `format!`: Biçimlendirilmiş metni `String` içine yazar.
* `print!`: `format!` ile aynıdır ancak metin konsola (`io::stdout`) yazdırılır.
* `println!`: `print!` ile aynıdır ancak sonuna yeni satır eklenir.
* `eprint!`: `print!` ile aynıdır ancak metin standart hata çıkışına (`io::stderr`) yazdırılır.
* `eprintln!`: `eprint!` ile aynıdır ancak sonuna yeni satır eklenir.

Hepsi metni aynı şekilde ayrıştırır (parse). Ek olarak, Rust biçimlendirme doğruluğunu derleme zamanında (compile time) kontrol eder.

```rust
fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
```

`std::fmt` metnin nasıl gösterileceğini yöneten birçok özellik (trait) içerir. Önemli iki tanesinin temel biçimi aşağıdaki gibidir:

* `fmt::Debug`: `{:?}` işaretleyicisini (marker) kullanır. Metni hata ayıklama (debugging) amacıyla biçimlendirir.
* `fmt::Display`: `{}` işaretleyicisini kullanır. Metni daha zarif ve kullanıcı dostu şekilde biçimlendirir.

Burada `fmt::Display` kullandık çünkü standart kütüphane (std library) bu türler için uygulamalar sağlar. Özel türleri yazdırmak için ek adımlar gerekir.

`fmt::Display` özelliğini (trait) uygulamak, `ToString` özelliğini de otomatik olarak uygular; bu da türü `String`e dönüştürmemizi sağlar.

43. satırdaki `#[allow(dead_code)]`, yalnızca ardından gelen modüle (module) uygulanan bir özniteliktir (attribute).

## 🧪 Etkinlikler (activities)

* Yukarıdaki koddaki sorunu (FIXME) düzeltin ki hata olmadan çalışsın.
* `Structure` yapısını biçimlendirmeye çalışan satırı (TODO) yorumdan çıkarıp deneyin.
* Şu çıktıyı üreten bir `println!` makrosu (macro) ekleyin: `Pi is roughly 3.142`. Bu alıştırma için `let pi = 3.141592` değerini pi’nin tahmini olarak kullanın. (İpucu: gösterilecek ondalık basamak sayısını ayarlamak için `std::fmt` belgelendirmesine bakmanız gerekebilir.)

## 📚 Ayrıca bakınız (see also)

`std::fmt`, `macros`, `struct`, `traits`, ve `dead_code`
