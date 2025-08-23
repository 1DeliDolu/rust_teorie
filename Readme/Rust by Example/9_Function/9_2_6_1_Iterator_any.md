## 🔍 Iterator::any

`Iterator::any`, bir yineleyiciye (iterator) uygulandığında, herhangi bir eleman verilen koşulu (predicate) sağlıyorsa `true`, aksi halde `false` döndüren bir fonksiyondur. İmzası (signature) şu şekildedir:

```rust
pub trait Iterator {
    // Yineleme yapılan tür.
    type Item;

    // `any`, `&mut self` alır, yani çağıran borçlanabilir (borrowed) 
    // ve değiştirilebilir (modified), fakat tüketilemez (not consumed).
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` demek, yakalanan değişkenler en fazla değiştirilebilir,
        // fakat tüketilemez. `Self::Item` ifadesi, kapanışın (closure)
        // argümanları değer olarak aldığı anlamına gelir.
        F: FnMut(Self::Item) -> bool;
}
```

```rust
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` vektörler için `&i32` döner. `i32`ye açmak için yapıbozum (destructuring) gerekir.
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()` vektörler için `i32` döner. Yapıbozum gerekmez.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // `iter()` sadece `vec1` ve elemanlarını ödünç alır, bu yüzden tekrar kullanılabilirler
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);
    // `into_iter()` ise `vec2` ve elemanlarını taşır (move), bu yüzden tekrar kullanılamaz
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());
    // TODO: Yukarıdaki iki satırı açıp derleyici hatalarını görün.

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // Diziler için `iter()` `&i32` döner.
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // Diziler için `into_iter()` `i32` döner.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
}
```
