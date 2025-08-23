## 🔙 Döngülerden Değer Döndürme (returning from loops)

`loop` kullanım amaçlarından biri, başarılı olana kadar bir işlemi tekrar etmektir. Ancak işlem bir değer döndürüyorsa, bu değeri geri kalan koda aktarmanız gerekebilir: değeri `break` ifadesinden sonra yazın, böylece bu değer döngü ifadesi tarafından döndürülür.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```

👉 Bu örnekte `loop` ifadesi `break counter * 2` ile sonlandırılıyor ve bu değer (`20`) döngünün sonucu olarak `result` değişkenine atanıyor.
