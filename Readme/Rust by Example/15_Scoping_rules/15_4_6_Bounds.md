## 📏 Sınırlar (bounds)

Nasıl ki generics türler sınırlandırılabiliyorsa, yaşam süreleri (lifetimes) de (kendileri de generic oldukları için) sınırlar kullanır. Burada `:` karakterinin anlamı biraz farklıdır, ancak `+` aynı şekilde kullanılır. Aşağıdaki ifadelerin nasıl okunduğuna dikkat edin:

* `T: 'a`: `T` içindeki tüm referanslar `'a` yaşam süresinden uzun olmalıdır.
* `T: Trait + 'a`: `T` türü `Trait` özelliğini uygulamalı ve `T` içindeki tüm referanslar `'a` yaşam süresinden uzun olmalıdır.

Aşağıdaki örnekte bu sözdiziminin `where` anahtar sözcüğü sonrasında nasıl kullanıldığı gösterilmektedir:

```rust
use std::fmt::Debug; // Sınırlandırmada kullanılacak trait.

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref`, generic tür `T`'ye bir referans içerir ve bu referansın yaşam süresi `'a`dır.
// `T`, içindeki tüm *referansların* `'a` yaşam süresinden uzun olmasını sağlayacak
// şekilde sınırlandırılmıştır. Ayrıca, `Ref` yaşam süresi `'a`'dan uzun olamaz.

// `Debug` trait’ini kullanan generic bir fonksiyon.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Burada `T`'ye bir referans alınır. `T`, `Debug` trait’ini uygulamalı
// ve içindeki tüm *referanslar* `'a`'dan uzun yaşamalıdır.
// Ayrıca `'a`, fonksiyon yaşam süresinden uzun olmalıdır.
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
```
