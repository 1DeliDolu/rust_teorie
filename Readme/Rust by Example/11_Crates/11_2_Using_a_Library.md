## 🔗 Bir Kütüphane Kullanma (using a library)

Bir crate’i yeni oluşturduğumuz kütüphaneye bağlamak için `rustc` komutunun `--extern` bayrağı kullanılabilir. Bu durumda, kütüphanedeki tüm öğeler kütüphaneyle aynı ada sahip bir modül altında içe aktarılır. Bu modül, diğer modüller gibi davranır.

// Rust 2015 veya daha eski sürümlerinde gerekli olabilir

```rust
// extern crate rary;

fn main() {
    rary::public_function();

    // Hata! `private_function` özeldir
    // rary::private_function();

    rary::indirect_access();
}
```

```bash
# Burada library.rlib derlenmiş kütüphanenin yoludur,
# aynı dizinde olduğu varsayılmaktadır:
$ rustc executable.rs --extern rary=library.rlib && ./executable 
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`
```

👉 Bu örnekte `--extern rary=library.rlib` parametresi, `rary` adını `library.rlib` dosyasına bağlar ve program derlenip çalıştırıldığında kütüphane işlevleri kullanılabilir hale gelir.
