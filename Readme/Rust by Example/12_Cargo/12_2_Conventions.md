## 📐 Kurallar (Conventions)

Önceki bölümde şu dizin hiyerarşisini görmüştük:

```
foo
├── Cargo.toml
└── src
    └── main.rs
```

Peki aynı projede iki ikili (binary) dosya bulundurmak istersek ne olur?

Aslında `cargo` bunu destekler. Varsayılan ikili adı `main`’dir, ancak `bin/` dizini içine ek ikililer koyarak yeni ikililer oluşturabilirsiniz:

```
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs
```

`cargo`ya sadece bu ikiliyi derlemesini veya çalıştırmasını söylemek için, şu bayrağı kullanırız:

```bash
cargo run --bin my_other_bin
```

Burada `my_other_bin`, çalışmak istediğimiz ikilinin adıdır.

Ek ikililere (binaries) ek olarak `cargo`, kıyaslamalar (benchmarks), testler (tests) ve örnekler (examples) gibi daha fazla özelliği de destekler.

Bir sonraki bölümde testlere daha yakından bakacağız.
