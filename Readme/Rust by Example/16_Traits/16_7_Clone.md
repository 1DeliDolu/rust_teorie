## 🧬 Klonlama (clone)

Kaynaklarla (**resources**) çalışırken, varsayılan davranış atama (**assignment**) veya fonksiyon çağrıları sırasında bu kaynakların **taşınmasıdır (move)**. Ancak bazen kaynağın **bir kopyasını (copy)** da oluşturmamız gerekir.

Bunu yapmak için **`Clone` trait’i** kullanılır. En yaygın olarak, `Clone` trait’inde tanımlanan `.clone()` yöntemi kullanılır.

```rust
// Kaynağı olmayan bir unit struct
#[derive(Debug, Clone, Copy)]
struct Unit;

// Kaynakları olan ve `Clone` trait’ini uygulayan bir tuple struct
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // `Unit` örneğini oluştur
    let unit = Unit;
    // `Unit` kopyala, taşınacak kaynak yok
    let copied_unit = unit;

    // Her iki `Unit` de bağımsız kullanılabilir
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // `Pair` örneğini oluştur
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // `pair`’i `moved_pair` içine taşı, kaynaklar taşınır
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Hata! `pair` kaynaklarını kaybetti
    //println!("original: {:?}", pair);
    // TODO ^ Bu satırı yorumdan çıkarmayı deneyin

    // `moved_pair`’i `cloned_pair` içine klonla (kaynaklar da kopyalanır)
    let cloned_pair = moved_pair.clone();
    // Orijinal taşınmış çifti std::mem::drop ile düşür
    drop(moved_pair);

    // Hata! `moved_pair` düşürüldü
    //println!("moved and dropped: {:?}", moved_pair);
    // TODO ^ Bu satırı yorumdan çıkarmayı deneyin

    // `.clone()` ile elde edilen sonuç hâlâ kullanılabilir!
    println!("clone: {:?}", cloned_pair);
}
```
