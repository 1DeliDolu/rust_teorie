## 🔄 Çıkış Parametreleri (as output parameters)

Kapanışların (closures) girdi parametresi olarak kullanılabilmesi mümkündür, bu nedenle kapanışların çıkış parametresi olarak döndürülmesi de mümkün olmalıdır. Ancak anonim kapanış türleri tanımı gereği bilinmeyen (unknown) türlerdir, bu yüzden bunları döndürmek için `impl Trait` kullanmamız gerekir.

Geçerli kapanış döndürme trait’leri şunlardır:

* `Fn`
* `FnMut`
* `FnOnce`

Bunun ötesinde `move` anahtar sözcüğü kullanılmalıdır. Bu, tüm yakalamaların değerle (by value) gerçekleştiğini belirtir. Bu gereklidir çünkü referansla yapılan yakalamalar fonksiyon biter bitmez düşer (drop edilir) ve kapanış içinde geçersiz referanslar bırakır.

```rust
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
```
