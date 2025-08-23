## 🕵️ Tür Anonimliği (type anonymity)

Kapanışlar (closures) dıştaki kapsamdan değişkenleri kısa yoldan yakalar. Bunun bir sonucu var mı? Elbette var. Bir kapanışı fonksiyon parametresi olarak kullanmak, tanımlanma şekli nedeniyle generics (genel tür parametreleri) gerektirir:

```rust
// `F` generic olmalıdır.
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}
```

Bir kapanış tanımlandığında, derleyici yakalanan değişkenleri içinde saklamak için yeni, anonim bir yapı (structure) oluşturur. Ardından bu bilinmeyen tür için `Fn`, `FnMut` veya `FnOnce` trait’lerinden birini uygulayarak işlevselliği sağlar. Bu tür, çağrılana kadar bir değişkende saklanır.

Bu yeni tür bilinmeyen (unknown type) olduğundan, fonksiyon içinde kullanılabilmesi için generics gereklidir. Ancak sınırsız bir `<T>` tür parametresi belirsiz olacağı için izin verilmez. Bu nedenle, kapanışın uyguladığı `Fn`, `FnMut` veya `FnOnce` trait’iyle sınırlandırmak yeterlidir.

```rust
// `F`, herhangi bir girdi almayan ve hiçbir şey döndürmeyen
// bir kapanış için `Fn` implement etmelidir.
// Bu tam olarak `print` için gereken şeydir.
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // `x` anonim bir türe yakalanır ve bu tür için `Fn` uygulanır.
    // Daha sonra `print` içinde saklanır.
    let print = || println!("{}", x);

    apply(print);
}
```
