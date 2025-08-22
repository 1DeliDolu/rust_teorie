## 💬 Yorumlar (comments)

Tüm programcılar, kodlarını anlaşılır kılmaya çalışır; ancak bazen ekstra açıklama yapmak gerekir. Bu durumlarda, programcılar kaynak koda yorumlar (comments) ekler. Derleyici bu yorumları yok sayar, fakat kaynak kodu okuyan kişiler için faydalı olabilir.

İşte basit bir yorum örneği:

```rust
// hello, world
```

Rust’ta alışılmış (idiomatic) yorum biçimi, `//` ile başlar ve yorum satırın sonuna kadar devam eder. Birden fazla satıra yayılan yorumlar için her satırın başına `//` eklemeniz gerekir:

```rust
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.
```

Yorumlar, kod içeren satırların sonuna da eklenebilir:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let lucky_number = 7; // I'm feeling lucky today
}
```

Ancak, yorumlar daha çok açıklama yaptığı kodun üstünde ayrı bir satırda kullanılır:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    // I'm feeling lucky today
    let lucky_number = 7;
}
```

Rust ayrıca başka bir yorum türüne de sahiptir: **dokümantasyon yorumları (documentation comments)**. Bunları, Bölüm 14’teki “Publishing a Crate to Crates.io” kısmında ele alacağız.
