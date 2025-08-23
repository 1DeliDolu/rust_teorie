## 🧩 Özellikler (Traits)

Elbette **trait**’ler de jenerik (generic) olabilir. Aşağıdaki örnekte, `Drop` trait’ini yeniden uygulayarak hem kendisini hem de bir girdiyi serbest bırakan jenerik bir metot tanımlıyoruz.

```rust
// Kopyalanamayan türler
struct Empty;
struct Null;

// `T` üzerinde jenerik bir trait.
trait DoubleDrop<T> {
    // Çağıran tür üzerinde bir metot tanımlar.
    // Ekstra bir `T` parametresi alır ve onunla hiçbir şey yapmaz.
    fn double_drop(self, _: T);
}

// Herhangi bir `T` jenerik parametresi ve
// herhangi bir çağıran `U` için `DoubleDrop<T>` uygulanır.
impl<T, U> DoubleDrop<T> for U {
    // Bu metot, verilen her iki argümanın da sahipliğini alır
    // ve ikisini de serbest bırakır.
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null  = Null;

    // `empty` ve `null` serbest bırakılır.
    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO: Bu satırların yorumunu kaldırmayı deneyin.
}
```

Bu örnekte `empty.double_drop(null)` çağrısı hem `empty`’nin hem de `null`’un sahipliğini alır ve ikisini de bellekten düşürür.
