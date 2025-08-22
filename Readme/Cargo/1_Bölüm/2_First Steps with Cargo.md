## 📝 Cargo ile İlk Adımlar (First Steps with Cargo)

Bu bölümde `cargo` komut satırı aracına hızlı bir bakış sağlanmaktadır. Cargo’nun bizim için yeni bir paket oluşturma, paket içindeki crate’i (paketi) derleme ve ortaya çıkan programı çalıştırma yeteneklerini göstereceğiz.

Yeni bir paket başlatmak için şu komutu kullanın:

```
$ cargo new hello_world
```

👉 Cargo varsayılan olarak `--bin` seçeneğiyle ikili (binary) bir program oluşturur. Bir kütüphane (library) yapmak için bunun yerine `--lib` parametresini kullanırız.

Şimdi Cargo’nun bizim için ne oluşturduğuna bakalım:

```
$ cd hello_world
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

👉 Başlamak için ihtiyacımız olan her şey burada. Öncelikle `Cargo.toml` dosyasını inceleyelim:

```
[package]
name = "hello_world"
version = "0.1.0"
edition = "2024"

[dependencies]
```

👉 Bu dosya bir **manifest** olarak adlandırılır ve Cargo’nun paketinizi derlemek için ihtiyaç duyduğu tüm metadata’yı içerir.

Şimdi `src/main.rs` dosyasına bakalım:

```
fn main() {
    println!("Hello, world!");
}
```

👉 Cargo bizim için bir “hello world” programı üretti; bu, ikili (binary) bir crate’tir. Şimdi bunu derleyelim:

```
$ cargo build
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
```

👉 Ardından çalıştıralım:

```
$ ./target/debug/hello_world
Hello, world!
```

👉 Ayrıca `cargo run` komutunu kullanarak hem derleyip hem de tek adımda çalıştırabiliriz:

```
$ cargo run
     Fresh hello_world v0.1.0 (file:///path/to/package/hello_world)
   Running `target/hello_world`
Hello, world!
```

---

## 🔗 Daha İleri (Going further)

Cargo’nun kullanımıyla ilgili daha fazla ayrıntı için **Cargo Guide** bölümüne göz atabilirsiniz.
