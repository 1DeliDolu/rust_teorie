## 🔄 Option İçinden Result Çekmek (pulling Results out of Options)

**Karışık hata türlerini** ele almanın en temel yolu, onları birbirinin içinde gömmektir.

```rust
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));

    println!("The first doubled is {:?}", double_first(empty));
    // Hata 1: giriş vektörü boştur

    println!("The first doubled is {:?}", double_first(strings));
    // Hata 2: eleman sayıya dönüştürülemez
}
```

Bazen hatalar üzerinde `?` ile olduğu gibi işlemi **durdurmak**, ancak `Option` `None` olduğunda **devam etmek** isteriz. Bu durumda `transpose` fonksiyonu, `Result` ve `Option` tiplerini **yer değiştirmek** için kullanışlıdır.

```rust
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.transpose()
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));
}
```
