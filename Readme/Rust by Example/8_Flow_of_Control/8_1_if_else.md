## 🔀 if/else

`if-else` ile dallanma (branching) diğer dillere benzerdir. Ancak çoğundan farklı olarak, boolean (mantıksal) koşul parantez içine alınmak zorunda değildir ve her koşul bir blok ile takip edilir. `if-else` koşulları birer ifadedir (expression) ve tüm dallar aynı türü döndürmek zorundadır.

```rust
fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}
```

👉 Bu örnekte `if-else` blokları ifadeler olarak kullanılıyor ve her iki dal da `i32` döndürmek zorunda.
