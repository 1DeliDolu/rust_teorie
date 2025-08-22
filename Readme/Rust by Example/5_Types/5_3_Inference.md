## 🧠 Tür Çıkarımı (inference)

Tür çıkarım motoru (type inference engine) oldukça akıllıdır. Bir değişken başlatılırken yalnızca değer ifadesinin türüne bakmakla kalmaz, aynı zamanda değişkenin sonradan nasıl kullanıldığına da bakarak türünü çıkarır. İşte tür çıkarımının gelişmiş bir örneği:

```rust
fn main() {
    // Açıklama sayesinde derleyici `elem` değişkeninin türünün u8 olduğunu bilir.
    let elem = 5u8;

    // Boş bir vektör (büyüyebilen dizi) oluştur.
    let mut vec = Vec::new();
    // Bu noktada derleyici `vec`’in tam türünü bilmez,
    // yalnızca bunun bir şeylerin vektörü (`Vec<_>`) olduğunu bilir.

    // `elem` değişkenini vektöre ekle.
    vec.push(elem);
    // İşte! Artık derleyici `vec`’in u8 türünden bir vektör (`Vec<u8>`) olduğunu bilir
    // TODO ^ `vec.push(elem)` satırını yorum satırı yapmayı deneyin

    println!("{:?}", vec);
}
```

Hiçbir değişken için tür açıklaması gerekmedi, derleyici mutlu, programcı da mutlu!
