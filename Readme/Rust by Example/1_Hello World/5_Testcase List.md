## 🧪 Test Durumu: Liste (test case: list)

Öğelerin her birinin sırasıyla işlenmesi gereken bir yapı (structure) için `fmt::Display` (fmt::Display) uygulamak zordur. Sorun, her `write!` (write!) çağrısının bir `fmt::Result` (fmt::Result) üretmesidir. Bunu doğru şekilde ele almak, tüm sonuçlarla ilgilenmeyi gerektirir. Rust tam da bu amaçla `?` işleçini (operator `?`) sağlar.

`write!` üzerinde `?` kullanımı şu şekildedir:

```rust
// Try `write!` to see if it errors. If it errors, return
// the error. Otherwise continue.
write!(f, "{}", value)?;
```

```rust
use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // index in `index`.
        for (index, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```

## 🏋️ Aktivite (activity)

Programı vektördeki her öğenin dizinini (index) de yazdıracak şekilde değiştirmeyi deneyin. Yeni çıktı aşağıdaki gibi olmalıdır:

```
[0: 1, 1: 2, 2: 3]
```

## 📚 Ayrıca bakınız (see also)

* `for` (for)
* `ref` (ref)
* `Result` (Result)
* `struct` (struct)
* `?` (operator `?`)
* `vec!` (vec! macro)
