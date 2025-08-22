
## 🧮 İfadeler (expressions)

Bir Rust programı (çoğunlukla) bir dizi deyimden (statement) oluşur:

```rust
fn main() {
    // statement
    // statement
    // statement
}
```

Rust’ta birkaç çeşit deyim vardır. En yaygın iki tanesi, bir değişken bağlaması (variable binding) bildirmek ve bir ifadeyi noktalı virgül (`;`) ile bitirmektir:

```rust
fn main() {
    // değişken bağlaması
    let x = 5;

    // ifade;
    x;
    x + 1;
    15;
}
```

Bloklar (blocks) da ifadelerdir, bu yüzden atamalarda değer olarak kullanılabilirler. Bir bloktaki son ifade, yer ifadesine (örneğin yerel bir değişkene) atanır. Ancak, bloğun son ifadesi noktalı virgül (`;`) ile bitirilirse, dönen değer `()` olur.

```rust
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // Bu ifade `y` değişkenine atanır
        x_cube + x_squared + x
    };

    let z = {
        // Noktalı virgül bu ifadeyi bastırır ve `()` değeri `z`’ye atanır
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
```
