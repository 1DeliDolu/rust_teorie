## 🔁 Tekrar (Repetition)

Makrolar, argüman listesinde `+` kullanarak bir argümanın **en az bir kez** tekrar edebileceğini, `*` kullanarak ise **sıfır veya daha fazla kez** tekrar edebileceğini belirtebilir.

Aşağıdaki örnekte, eşleştiriciyi `$(...),+` ile sarmak, **virgüllerle ayrılmış bir veya daha fazla ifade (expression)** ile eşleşir. Ayrıca son durumda noktalı virgülün opsiyonel olduğunu unutmayın.

```rust
// `find_min!` herhangi sayıda argümanın minimumunu hesaplar.
macro_rules! find_min {
    // Temel durum:
    ($x:expr) => ($x);
    // `$x` ifadesini, ardından en az bir `$y,` ile eşleştirir
    ($x:expr, $($y:expr),+) => (
        // Kuyruk `$y` üzerinde `find_min!` çağırılır
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
```
