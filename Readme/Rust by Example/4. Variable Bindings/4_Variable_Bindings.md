## 🔗 Değişken Bağlamaları (variable bindings)

Rust, statik tip denetimi (static typing) sayesinde tür güvenliği sağlar. Değişken bağlamaları (variable bindings) tanımlanırken tür belirtilebilir. Ancak çoğu durumda derleyici, değişkenin türünü bağlamdan çıkarabildiği için tür belirtme yükünü büyük ölçüde azaltır.

Değerler (values) (örneğin sabitler (literals)) `let` bağlaması (let binding) kullanılarak değişkenlere bağlanabilir.

```rust
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // `an_integer` değerini `copied_integer` içine kopyala
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // Derleyici, kullanılmayan değişken bağlamaları için uyarı verir;
    // bu uyarılar değişken adının başına alt çizgi (_) eklenerek susturulabilir
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ Uyarıyı bastırmak için değişken adının başına alt çizgi ekleyin
    // Lütfen dikkat edin: uyarılar tarayıcıda gösterilmeyebilir
}
```
