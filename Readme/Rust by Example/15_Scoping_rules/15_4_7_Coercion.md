## 🔄 Dönüştürme (coercion)

Daha uzun bir yaşam süresi (lifetime), normalde çalışamayacağı bir kapsam (scope) içinde çalışabilmesi için daha kısa birine dönüştürülebilir. Bu, Rust derleyicisi tarafından çıkarımla (inferred coercion) yapılabileceği gibi, yaşam süresi farkı açıkça belirtilerek de yapılabilir:

```rust
// Burada Rust, mümkün olan en kısa yaşam süresini çıkarır.
// İki referans daha sonra o yaşam süresine dönüştürülür.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` şu şekilde okunur: `'a` yaşam süresi, en az `'b` kadar uzundur.
// Burada `&'a i32` alınır ve dönüştürme sonucunda `&'b i32` döndürülür.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Daha uzun yaşam süresi
    
    {
        let second = 3; // Daha kısa yaşam süresi
        
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}
```
