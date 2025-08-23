## 📝 Açık Anotasyon (explicit annotation)

Borrow checker, referansların ne kadar süreyle geçerli olması gerektiğini belirlemek için açık yaşam süresi anotasyonlarını (explicit lifetime annotations) kullanır. Yaşam sürelerinin elide edilmediği (elided) durumlarda, Rust bir referansın yaşam süresinin ne olacağını belirlemek için açık anotasyonlar ister. Açık yaşam süresi anotasyonu için sözdizimi, kesme işareti ile şu şekilde yapılır:

```rust
foo<'a>
// `foo` bir yaşam süresi parametresi `'a`'ya sahiptir
```

Closure’lara benzer şekilde, yaşam süreleri de generics gerektirir. Ayrıca, bu yaşam süresi sözdizimi `foo`’nun yaşam süresinin `'a`’dan uzun olamayacağını belirtir. Bir türün açık yaşam süresi anotasyonu şu biçimdedir: `&'a T` (burada `'a` önceden tanımlanmış olmalıdır).

Birden fazla yaşam süresi olduğunda sözdizimi benzerdir:

```rust
foo<'a, 'b>
// `foo`, `'a` ve `'b` yaşam süresi parametrelerine sahiptir
```

Bu durumda, `foo`’nun yaşam süresi `'a` veya `'b`’den uzun olamaz.

Aşağıdaki örnekte açık yaşam süresi anotasyonu kullanımını görebilirsiniz:

```rust
// `print_refs`, farklı yaşam sürelerine sahip `'a` ve `'b`
// olmak üzere iki `i32` referansı alır. Bu iki yaşam süresi,
// `print_refs` fonksiyonu süresinden en az o kadar uzun olmalıdır.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// Argüman almayan fakat `'a` yaşam süresi parametresine sahip bir fonksiyon.
fn failed_borrow<'a>() {
    let _x = 12;

    // HATA: `_x` yeterince uzun yaşamaz
    let _y: &'a i32 = &_x;
    // Fonksiyon içinde yaşam süresi `'a`’yı açık tip anotasyonu
    // olarak kullanmaya çalışmak başarısız olur çünkü `&_x`’in yaşam süresi,
    // `_y`’den daha kısadır. Kısa bir yaşam süresi daha uzuna dönüştürülemez.
}

fn main() {
    // Aşağıda ödünç verilecek değişkenler oluşturuluyor.
    let (four, nine) = (4, 9);
    
    // Her iki değişkenin de borçları (`&`) fonksiyona aktarılır.
    print_refs(&four, &nine);
    // Ödünç alınan her girdi, ödünç alanın yaşam süresinden uzun olmalıdır.
    // Başka bir deyişle, `four` ve `nine`’in yaşam süresi
    // `print_refs`’inkinden daha uzun olmalıdır.
    
    failed_borrow();
    // `failed_borrow`, `'a` yaşam süresini fonksiyon yaşam süresinden
    // uzun olmaya zorlayan hiçbir referans içermez, fakat `'a` daha uzundur.
    // Yaşam süresi kısıtlanmadığı için varsayılan olarak `'static` olur.
}
```

Bkz:
`generics` ve `closures`

> `elision`, yaşam sürelerini örtük (implicit) olarak anotasyonladığı için farklıdır.
