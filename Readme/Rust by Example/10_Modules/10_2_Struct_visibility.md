## 📦 Struct Görünürlüğü (struct visibility)

Struct’ların alanları (fields) için ek bir görünürlük seviyesi vardır. Varsayılan olarak görünürlük **özel (private)**’tir ve `pub` belirleyicisi ile değiştirilebilir. Bu görünürlük yalnızca bir struct, tanımlandığı modülün dışından erişildiğinde önemlidir ve amacı bilgiyi gizlemektir (kapsülleme - encapsulation).

```rust
mod my {
    // Genel (public) bir struct, genel (public) tür parametreli alan ile
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // Genel (public) bir struct, özel (private) tür parametreli alan ile
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // Genel (public) bir kurucu (constructor) metod
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}
```

```rust
fn main() {
    // Genel struct’lar, genel alanları ile normal şekilde oluşturulabilir
    let open_box = my::OpenBox { contents: "public information" };

    // ve alanlarına normal şekilde erişilebilir
    println!("The open box contains: {}", open_box.contents);

    // Genel struct’lar, özel alanlara sahip olduğunda
    // alan isimleri kullanılarak oluşturulamaz
    // Hata! `ClosedBox` özel alanlara sahiptir
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Bu satırı açmayı deneyin

    // Ancak, özel alanlara sahip struct’lar genel kurucular aracılığıyla oluşturulabilir
    let _closed_box = my::ClosedBox::new("classified information");

    // ve genel bir struct’ın özel alanlarına erişilemez
    // Hata! `contents` alanı özeldir
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Bu satırı açmayı deneyin
}
```
