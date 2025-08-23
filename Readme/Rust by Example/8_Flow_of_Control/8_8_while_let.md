## 🔁 while let

`if let`’e benzer şekilde, `while let` de hantal `match` döngülerini daha kullanışlı hale getirebilir.
Aşağıdaki örneği inceleyelim; burada `i` artırılmaktadır:

```rust
// `optional` tipini `Option<i32>` yap
let mut optional = Some(0);

// Bu testi tekrar tekrar dene.
loop {
    match optional {
        // Eğer `optional` parçalanırsa, bloğu çalıştır.
        Some(i) => {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ 3 seviye girinti gerektiriyor!
        },
        // Parçalama başarısız olursa döngüyü bitir:
        _ => { break; }
        // ^ Gerçekten gerekli mi? Daha iyi bir yol olmalı!
    }
}
```

---

`while let` kullanmak bu diziyi çok daha temiz hale getirir:

```rust
fn main() {
    // `optional` tipini `Option<i32>` yap
    let mut optional = Some(0);
    
    // Bu şu anlama gelir: "`let`, `optional`’ı `Some(i)` olarak parçalayabildiği sürece
    // bloğu (`{}`) çalıştır. Aksi halde `break`."
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Daha az girinti ve başarısız durumu açıkça
        // ele almak gerekmez.
    }
    // ^ `if let`, ek olarak `else` / `else if` koşulları da alabiliyordu.
    // `while let` bunlara sahip değildir.
}
```

👉 `while let`, döngü içinde `Option` veya benzeri enum’ların durumunu kolayca kontrol etmek için pratik bir çözümdür.
