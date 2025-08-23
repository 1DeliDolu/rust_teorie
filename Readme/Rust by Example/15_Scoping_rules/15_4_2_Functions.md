## ⚙️ Fonksiyonlar (functions)

Elision (örtük yaşam süresi çıkarımı) göz ardı edildiğinde, yaşam süresi (lifetime) içeren fonksiyon imzalarının birkaç kısıtlaması vardır:

* Her referansın açıkça anotasyonlanmış bir yaşam süresine sahip olması gerekir.
* Döndürülen herhangi bir referans, bir girdi ile aynı yaşam süresine sahip olmalı veya `'static` olmalıdır.

Ayrıca, geçersiz verilere referans döndürülmesine neden oluyorsa girdisiz referans döndürmek yasaktır. Aşağıdaki örnek, yaşam sürelerine sahip bazı geçerli fonksiyon biçimlerini göstermektedir:

```rust
// `'a` yaşam süresine sahip tek bir girdi referansı, 
// fonksiyon süresi boyunca en az o kadar uzun yaşamalıdır.
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// Yaşam süreleri ile birlikte değiştirilebilir (mutable) referanslar da mümkündür.
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Farklı yaşam sürelerine sahip birden fazla öğe.
// Bu durumda her ikisinin de aynı `'a` yaşam süresine sahip olması sorun olmaz,
// ancak daha karmaşık durumlarda farklı yaşam süreleri gerekebilir.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// Fonksiyona girilen referansları döndürmek kabul edilebilir.
// Ancak doğru yaşam süresi döndürülmelidir.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// Yukarıdaki geçersizdir: `'a`, fonksiyondan daha uzun yaşamalıdır.
// Burada, `&String::from("foo")` önce bir `String` oluşturur, ardından bir referans üretir.
// Fonksiyon kapsamı bitince veri düşer (drop edilir), bu da geçersiz bir referansın döndürülmesine yol açar.

fn main() {
    let x = 7;
    let y = 9;
    
    print_one(&x);
    print_multi(&x, &y);
    
    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
```
