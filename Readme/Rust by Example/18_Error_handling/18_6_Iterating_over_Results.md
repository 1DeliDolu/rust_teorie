## 🔁 `Result` Üzerinde Yineleme (iterating over Results)

Bir `Iter::map` işlemi başarısız olabilir, örneğin:

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
```

Şimdi bunu ele almanın stratejilerine bakalım.

---

### 🚫 Başarısız öğeleri yok saymak: `filter_map()`

`filter_map`, bir fonksiyon çağırır ve sonucu `None` olan öğeleri filtreler.

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);
}
```

---

### 📥 Başarısız öğeleri toplamak: `map_err()` ve `filter_map()`

`map_err`, hata ile bir fonksiyon çağırır. Bunu önceki `filter_map` çözümüne ekleyerek, yineleme sırasında hataları kenarda saklayabiliriz.

```rust
fn main() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```

---

### ❌ Tüm işlemi başarısız kılmak: `collect()`

`Result`, `FromIterator` uygular. Bu sayede bir `Result` vektörü (`Vec<Result<T, E>>`), bir `Result<Vec<T>, E>`’ye dönüştürülebilir. İlk `Err` bulunduğunda yineleme sonlanır.

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}
```

Aynı teknik `Option` ile de kullanılabilir.

---

### ↔️ Başarılı ve başarısız değerleri ayırmak: `partition()`

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```

Sonuçlara baktığınızda, her şeyin hâlâ `Result` içine sarılı olduğunu göreceksiniz. Bunun için biraz daha **boilerplate** gerekir:

```rust
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
```
