## ⚠️ Birden Fazla Hata Türü (multiple error types)

Önceki örnekler her zaman oldukça kullanışlıydı; `Result` tipleri diğer `Result` tipleriyle ve `Option` tipleri diğer `Option` tipleriyle etkileşime giriyordu.

Bazen bir `Option` bir `Result` ile etkileşime girmek zorunda kalır veya bir `Result<T, Error1>` bir `Result<T, Error2>` ile etkileşime girmek zorunda kalır. Bu durumlarda, farklı hata türlerimizi **birleştirilebilir** (composable) ve **kolay etkileşilebilir** olacak şekilde yönetmek isteriz.

Aşağıdaki kodda, iki farklı `unwrap` çağrısı farklı hata türleri üretmektedir. `Vec::first` bir `Option` döndürürken, `parse::<i32>` bir `Result<i32, ParseIntError>` döndürmektedir:

```rust
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Hata 1 üretir
    2 * first.parse::<i32>().unwrap() // Hata 2 üretir
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    println!("The first doubled is {}", double_first(empty));
    // Hata 1: giriş vektörü boştur

    println!("The first doubled is {}", double_first(strings));
    // Hata 2: eleman sayıya dönüştürülemez
}
```

Sonraki bölümlerde, bu tür problemleri ele almak için birkaç strateji göreceğiz.
