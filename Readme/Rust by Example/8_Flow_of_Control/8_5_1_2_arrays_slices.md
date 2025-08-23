## 📐 Dizilerin ve Dilimlerin Yapıbozumu (destructuring arrays and slices)

Demetler (`tuple`) gibi, diziler (`array`) ve dilimler (`slice`) de bu şekilde yapıbozuma uğratılabilir:

```rust
fn main() {
    // Try changing the values in the array, or make it a slice!
    let array = [1, -2, 6];

    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}
```

👉 Bu örnekte:

* `[0, second, third]` → İlk eleman `0`, diğerleri değişkenlere atanıyor.
* `[1, _, third]` → Ortadaki eleman `_` ile yok sayılıyor.
* `[-1, second, ..]` → İlk iki eleman bağlanıyor, geri kalanlar yok sayılıyor.
* `[3, second, tail @ ..]` → İlk iki eleman bağlanıyor, kalanlar `tail` adlı dilime atanıyor.
* `[first, middle @ .., last]` → İlk ve son eleman bağlanıyor, aradakiler `middle` adlı dilime atanıyor.
