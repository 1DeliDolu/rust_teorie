## 🧪 Test Durumu: boş sınırlar (empty bounds)

Sınırların (bounds) çalışma biçiminin bir sonucu olarak, bir trait (trait) herhangi bir işlevsellik içermese bile onu bir sınır (bound) olarak kullanabilirsiniz. `Eq` ve `Copy`, `std` standart kütüphanesinden (standard library) böyle trait’lere örnektir.

```rust
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: Try uncommenting this line.
}
```
