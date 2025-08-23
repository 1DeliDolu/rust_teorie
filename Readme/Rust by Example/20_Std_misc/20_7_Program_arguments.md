## 💻 Program Argümanları (Program arguments)

### 📚 Standart Kütüphane (Standard Library)

Komut satırı argümanlarına `std::env::args` kullanılarak erişilebilir. Bu fonksiyon, her argüman için bir `String` döndüren bir yineleyici (iterator) sağlar:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // İlk argüman, programı çağırmak için kullanılan yoldur.
    println!("Benim yolum {}.", args[0]);

    // Geri kalan argümanlar, komut satırında verilen parametrelerdir.
    // Programı şu şekilde çağırın:
    //   $ ./args arg1 arg2
    println!("{} argüman aldım: {:?}.", args.len() - 1, &args[1..]);
}
```

### ✅ Örnek çıktı

```bash
$ ./args 1 2 3
My path is ./args.
I got 3 arguments: ["1", "2", "3"].
```

---

### 📦 Crates

Alternatif olarak, komut satırı uygulamaları geliştirirken ek işlevsellik sağlayan birçok crate vardır. Bunlardan en popüler olanlardan biri `clap` crate’idir.
