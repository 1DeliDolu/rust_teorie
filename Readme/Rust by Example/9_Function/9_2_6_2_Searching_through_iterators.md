## 🔍 Yineleyicilerde Arama (searching through iterators)

`Iterator::find`, bir yineleyici üzerinde yineleme yaparak, verilen koşulu sağlayan ilk değeri arayan bir fonksiyondur. Eğer hiçbir değer koşulu sağlamazsa `None` döner. İmzası (signature) şu şekildedir:

```rust
pub trait Iterator {
    // Yineleme yapılan tür.
    type Item;

    // `find`, `&mut self` alır, yani çağıran borçlanabilir (borrowed)
    // ve değiştirilebilir (modified), fakat tüketilemez (not consumed).
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` demek, yakalanan değişkenler en fazla değiştirilebilir,
        // fakat tüketilemez. `&Self::Item`, kapanışın (closure) 
        // argümanları referansla aldığı anlamına gelir.
        P: FnMut(&Self::Item) -> bool;
}
```

```rust
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` vektörler için `&i32` döner.
    let mut iter = vec1.iter();
    // `into_iter()` vektörler için `i32` döner.
    let mut into_iter = vec2.into_iter();

    // `iter()` için sonuç `&i32`, elemanı referanslamak istediğimizde
    // `&&i32` → `i32` yapıbozum yapılmalı.
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter()` için sonuç `i32`, elemanı referanslamak istediğimizde
    // `&i32` → `i32` yapıbozum yapılmalı.
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // Diziler için `iter()` `&&i32` döner.
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // Diziler için `into_iter()` `&i32` döner.
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));
}
```

`Iterator::find`, öğeye bir referans döndürür. Ancak, öğenin **indeksini** (index) almak isterseniz `Iterator::position` kullanılmalıdır:

```rust
fn main() {
    let vec = vec![1, 9, 3, 3, 13, 2];

    // `iter()` vektörler için `&i32` döner ve `position()` referans almaz,
    // bu yüzden `&i32` → `i32` yapıbozum yapılmalı.
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));
    
    // `into_iter()` vektörler için `i32` döner ve `position()` referans almaz,
    // bu yüzden yapıbozum gerekmez.
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}
```
