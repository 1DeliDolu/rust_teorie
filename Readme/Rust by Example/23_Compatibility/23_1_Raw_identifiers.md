## 🏷️ Ham Tanımlayıcılar (raw identifiers)

Rust, birçok programlama dili gibi, "anahtar kelime" (keywords) kavramına sahiptir. Bu tanımlayıcıların dil için özel bir anlamı vardır, bu nedenle değişken adları, fonksiyon adları veya diğer benzer yerlerde kullanılamazlar. **Ham tanımlayıcılar (raw identifiers)**, normalde izin verilmeyen yerlerde anahtar kelimelerin kullanılmasına olanak tanır.

Bu özellikle, Rust yeni bir anahtar kelime tanıttığında ve eski bir sürümle yazılmış bir kütüphanede bu kelimeyle aynı ada sahip bir değişken ya da fonksiyon bulunduğunda faydalıdır.

Örneğin, Rust’un 2015 sürümüyle derlenmiş ve `try` adında bir fonksiyon dışa aktaran bir `foo` crate’ini düşünelim. Ancak `try`, 2018 sürümünde yeni bir özellik için ayrılmış bir anahtar kelimedir. Ham tanımlayıcılar olmadan bu fonksiyona erişmek mümkün olmazdı.

```rust
extern crate foo;

fn main() {
    foo::try();
}
```

Bu durumda şu hata alınır:

```
error: expected identifier, found keyword `try`
 --> src/main.rs:4:4
  |
4 | foo::try();
  |      ^^^ expected identifier, found keyword
```

Ham tanımlayıcı kullanarak bu sorun çözülebilir:

```rust
extern crate foo;

fn main() {
    foo::r#try();
}
```
