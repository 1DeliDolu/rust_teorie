## 🔄 Aşırı Yükleme (Overload)

Makrolar, farklı argüman kombinasyonlarını kabul edecek şekilde aşırı yüklenebilir. Bu açıdan `macro_rules!`, `match` bloğuna benzer şekilde çalışabilir:

```rust
// `test!` makrosu, `$left` ve `$right`
// nasıl çağrıldığına bağlı olarak farklı şekilde karşılaştırır:
macro_rules! test {
    // Argümanların virgül ile ayrılması gerekmez.
    // Herhangi bir şablon kullanılabilir!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ her kol (arm) noktalı virgül ile bitmelidir.
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
```
