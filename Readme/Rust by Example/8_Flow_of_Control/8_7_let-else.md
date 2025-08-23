## ✨ let-else

🛈 **Stable since:** Rust 1.65
🛈 Belirli bir sürümü hedeflemek için şu şekilde derleyebilirsiniz:
`rustc --edition=2021 main.rs`

`let-else` ile, reddedilebilir (refutable) bir pattern normal bir `let` gibi eşleşip değişkenleri çevreleyen scope’a bağlayabilir veya eşleşme başarısız olduğunda **diverge** edebilir (ör. `break`, `return`, `panic!`).

```rust
use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

fn main() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}
```

👉 Burada `let-else`, pattern eşleşmesini kolaylaştırır ve başarısızlık durumunu doğrudan `else` bloğunda yönetmeyi sağlar.

---

`let-else`’i diğerlerinden ayıran en önemli fark **isim bağlamalarının kapsamı (scope)**’dur.
Daha önce bu tür desenleri `match` veya `if let-else` ile yaklaştırmak mümkündü, ancak biraz tekrar gerekiyordu:

```rust
let (count_str, item) = match (it.next(), it.next()) {
    (Some(count_str), Some(item)) => (count_str, item),
    _ => panic!("Can't segment count item pair: '{s}'"),
};

let count = if let Ok(count) = u64::from_str(count_str) {
    count
} else {
    panic!("Can't parse integer: '{count_str}'");
};
```

👉 `let-else`, bu gereksiz tekrarları ortadan kaldırır.

---

📖 Ayrıca bakınız:

* `option`
* `match`
* `if let`
* \[let-else RFC]
