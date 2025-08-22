## 🔄 Enum’lar (enums)

`enum` anahtar kelimesi, birkaç farklı varyanttan (variant) biri olabilen bir türün oluşturulmasına izin verir. Bir struct için geçerli olan her varyant aynı zamanda bir enum içinde de geçerlidir.

```rust
// Bir web olayını sınıflandırmak için `enum` oluşturma.
// İsimler ve tür bilgileri birlikte varyantı belirler:
// `PageLoad != PageUnload` ve `KeyPress(char) != Paste(String)`.
// Her biri farklı ve bağımsızdır.
enum WebEvent {
    // `enum` varyantı `unit-like` olabilir,
    PageLoad,
    PageUnload,
    // tuple struct gibi olabilir,
    KeyPress(char),
    Paste(String),
    // veya C-benzeri struct olabilir.
    Click { x: i64, y: i64 },
}

// Bir `WebEvent` enum’unu argüman olarak alan
// ve hiçbir şey döndürmeyen bir fonksiyon.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Enum varyantının içindeki `c` değişkenini destructuring yapma.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // `Click` varyantını `x` ve `y` değerlerine açma.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` bir string slice’tan `String` oluşturur.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
```

👉 Bu örnek, enum’ların farklı türlerde varyantlar içerebildiğini ve `match` ile nasıl ayrıştırıldığını göstermektedir.

---

## 🏷️ Tür Takma Adları (type aliases)

Bir **tür takma adı** (type alias) kullanarak, enum varyantlarına bu takma ad üzerinden erişebilirsiniz. Bu, enum adı çok uzun veya çok genel olduğunda, onu yeniden adlandırmak için yararlı olabilir.

```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Tür takma adı oluşturma
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // Artık varyantlara uzun ve kullanışsız ad yerine
    // takma ad üzerinden erişebiliriz.
    let x = Operations::Add;
}
```

---

### 🔁 `Self` ile Enum Kullanımı

En yaygın kullanım yerlerinden biri `impl` bloklarıdır. Burada `Self` alias’ı kullanılır.

```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
```

👉 Bu kullanımda `Self`, enum adının yerine geçer ve kodu daha kısa ve okunabilir hale getirir.

---

## 📚 Ayrıca bakınız

* `match`
* `fn`
* `String`
* **"Type alias enum variants" RFC** (enum varyantları için tür takma adları)
