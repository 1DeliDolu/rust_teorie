## ⚖️ TryFrom ve TryInto (TryFrom and TryInto)

`From` ve `Into` trait’lerine benzer şekilde, `TryFrom` ve `TryInto` da türler arası dönüşüm için genel (generic) trait’lerdir. Ancak `From/Into`’dan farklı olarak, `TryFrom/TryInto` **hata verebilecek dönüşümler** (fallible conversions) için kullanılır ve bu nedenle `Result` dönerler.

```rust
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
```

Burada `TryFrom` trait’i, yalnızca çift sayıları kabul eden bir `EvenNumber` türü tanımlar. Eğer değer çift ise `Ok`, tek ise `Err` döner.
Aynı mantık `TryInto` ile de kullanılabilir.
