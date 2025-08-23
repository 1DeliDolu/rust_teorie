## ➕ Çoklu Sınırlar (Multiple bounds)

Bir tür için birden fazla sınır (bound) `+` operatörü ile uygulanabilir. Normalde olduğu gibi, farklı türler `,` ile ayrılır.

```rust
use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // TODO ^ Bu satırın yorumunu kaldırmayı deneyin.

    compare_types(&array, &vec);
}
```

Bu örnekte:

* `compare_prints`, yalnızca hem `Debug` hem de `Display` implementasyonu olan türlerde çalışır.
* `compare_types`, farklı türlerde (`T`, `U`) ama ikisi de `Debug` implementasyonu olan nesnelerde çalışır.
