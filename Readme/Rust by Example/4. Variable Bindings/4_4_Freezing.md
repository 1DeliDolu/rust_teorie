## ❄️ Donma (freezing)

Bir veri aynı adla değiştirilemez (immutable) olarak bağlandığında, donmuş (frozen) hale gelir. Donmuş veriler, değiştirilemez bağlama kapsam dışına çıkana kadar değiştirilemez:

```rust
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // `_mutable_integer` değiştirilemez olarak gölgeleniyor
        let _mutable_integer = _mutable_integer;

        // Hata! `_mutable_integer` bu kapsamda donmuştur
        _mutable_integer = 50;
        // FIXME ^ Bu satırı yorum satırı yapın

        // `_mutable_integer` kapsam dışına çıkar
    }

    // Tamam! `_mutable_integer` bu kapsamda donmuş değildir
    _mutable_integer = 3;
}
```
