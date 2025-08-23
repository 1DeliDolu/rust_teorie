## 🔗 Girdi Fonksiyonları (input functions)

Kapanışlar (closures) argüman olarak kullanılabildiğinden, aynı şeyin fonksiyonlar için de geçerli olup olmadığını merak edebilirsiniz. Ve evet, mümkündür! Eğer bir fonksiyon, parametre olarak bir kapanış alacak şekilde tanımlanırsa, o kapanışın trait sınırını karşılayan herhangi bir fonksiyon da parametre olarak aktarılabilir.

```rust
// `Fn` ile sınırlandırılmış generic `F` argümanını alan
// ve onu çağıran bir fonksiyon tanımla
fn call_me<F: Fn()>(f: F) {
    f();
}

// `Fn` sınırını karşılayan bir sarmalayıcı fonksiyon tanımla
fn function() {
    println!("I'm a function!");
}

fn main() {
    // `Fn` sınırını karşılayan bir kapanış tanımla
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
```

Ek bir not olarak, `Fn`, `FnMut` ve `FnOnce` trait’leri, bir kapanışın dıştaki kapsamdan değişkenleri nasıl yakalayacağını belirler.
