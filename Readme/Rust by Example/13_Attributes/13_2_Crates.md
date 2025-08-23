## 📦 Crates

`crate_type` özniteliği (attribute), derleyiciye bir crate’in ikili (binary) mi yoksa kütüphane (library) mi olduğunu (ve hatta hangi tür kütüphane olduğunu) belirtmek için kullanılabilir.
`crate_name` özniteliği ise crate’in adını ayarlamak için kullanılır.

Ancak önemli bir nokta şudur: **`crate_type` ve `crate_name` özniteliklerinin Cargo kullanıldığında hiçbir etkisi yoktur.** Rust projelerinin çoğunda Cargo kullanıldığı için, bu özniteliklerin gerçek dünyadaki kullanım alanı oldukça sınırlıdır.

```rust
// Bu crate bir kütüphanedir
#![crate_type = "lib"]
// Kütüphanenin adı "rary" olarak belirlenmiştir
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

`crate_type` özniteliği kullanıldığında artık `rustc`’ye `--crate-type` bayrağını vermemize gerek yoktur:

```bash
$ rustc lib.rs
$ ls lib*
library.rlib
```
