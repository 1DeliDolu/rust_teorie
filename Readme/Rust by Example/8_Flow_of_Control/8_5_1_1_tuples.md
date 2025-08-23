## 🔗 Demetlerin Yapıbozumu (destructuring tuples)

Demetler (`tuple`) bir `match` içinde şu şekilde yapıbozuma uğratılabilir:

```rust
fn main() {
    let triple = (0, -2, 3);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
}
```

👉 Bu örnekte `match`, demetin elemanlarını tek tek açarak kullanıyor.

* `(0, y, z)` → İlk eleman `0`, diğerleri değişkenlere bağlanıyor.
* `(1, ..)` → İlk eleman `1`, geri kalanı önemli değil.
* `(.., 2)` → Son eleman `2`, diğerleri önemli değil.
* `(3, .., 4)` → İlk eleman `3`, son eleman `4`, ortadakiler önemli değil.
* `_` → Değer hiçbir değişkene bağlanmaz, tamamen yok sayılır.
