## 🔗 Test Durumu: Bağlı Liste (linked-list)

Bağlı listeyi (linked-list) uygulamanın yaygın bir yolu `enum` (enum) kullanmaktır:

```rust
use crate::List::*;

enum List {
    // Cons: Bir elemanı ve bir sonraki düğüme işaretçiyi saran tuple struct (tuple struct)
    Cons(u32, Box<List>),
    // Nil: Bağlı listenin sonunu belirten düğüm
    Nil,
}

// Enum’a metotlar eklenebilir
impl List {
    // Boş bir liste oluştur
    fn new() -> List {
        // `Nil` türü `List`tir
        Nil
    }

    // Bir listeyi tüketir ve listenin başına yeni bir eleman eklenmiş halini döndürür
    fn prepend(self, elem: u32) -> List {
        // `Cons` da `List` türündedir
        Cons(elem, Box::new(self))
    }

    // Listenin uzunluğunu döndür
    fn len(&self) -> u32 {
        // Bu metodun davranışı `self`in varyantına bağlı olduğundan `self` eşleştirilmelidir
        // `self`in türü `&List`tir ve `*self`in türü `List`tir; somut bir tür `T` üzerinde
        // eşleştirme, başvuru `&T` üzerinde eşleştirmeye tercih edilir
        // Rust 2018 sonrası burada doğrudan self de kullanılabilir ve aşağıda tail (ref olmadan)
        // yazılabilir; Rust `&` ve `ref tail`i çıkarır.
        // Bkz. https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Kuyruğun (tail) sahipliğini alamayız çünkü `self` ödünç alınmıştır;
            // bunun yerine kuyruğa bir başvuru alın
            // Ayrıca bu, uzun listelerde yığıt taşmasına (stack overflow) yol açabilecek kuyruk-dışı (non-tail) özyinelemeli bir çağrıdır.
            Cons(_, ref tail) => 1 + tail.len(),
            // Taban durum: Boş bir listenin uzunluğu sıfırdır
            Nil => 0
        }
    }

    // Listenin gösterimini (heap üzerinde ayrılmış) bir dizge (string) olarak döndür
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!`, `print!`e benzer, ancak konsola yazdırmak yerine
                // heap üzerinde ayrılmış bir dizge döndürür
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Boş bir bağlı liste oluştur
    let mut list = List::new();

    // Bazı elemanlar ekle (listenin başına)
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Listenin son durumunu göster
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
```
