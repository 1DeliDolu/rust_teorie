## 🏗️ Yapılar (structs)

Yapılarda (structs) yaşam süresi (lifetime) anotasyonları da fonksiyonlara benzer şekilde yapılır:

```rust
// `i32` referansını barındıran bir `Borrowed` türü.
// `i32` referansı, `Borrowed`'dan daha uzun yaşamalıdır.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Benzer şekilde, buradaki her iki referans da bu yapıdan daha uzun yaşamalıdır.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// Ya `i32` ya da bir `i32` referansı olabilen bir enum.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
```
