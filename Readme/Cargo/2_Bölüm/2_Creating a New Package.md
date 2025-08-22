## 📦 Yeni Bir Paket Oluşturma (Creating a New Package)

Yeni bir paket başlatmak için şu komutu kullanın:

```
$ cargo new hello_world --bin
```

👉 Burada `--bin` parametresini geçiyoruz çünkü ikili (binary) bir program oluşturuyoruz. Eğer bir kütüphane (library) yapıyor olsaydık bunun yerine `--lib` kullanırdık. Bu komut ayrıca varsayılan olarak yeni bir Git deposu başlatır. Bunu istemezseniz `--vcs none` parametresini ekleyebilirsiniz.

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

---

### 📑 Cargo.toml

```
[package]
name = "hello_world"
version = "0.1.0"
edition = "2024"

[dependencies]
```

👉 Bu dosya **manifest** olarak adlandırılır ve Cargo’nun paketinizi derlemek için ihtiyaç duyduğu tüm metadata’yı içerir. Dosya **TOML** biçiminde yazılmıştır (okunuşu: /tɑməl/).

---

### 📂 src/main.rs

```
fn main() {
    println!("Hello, world!");
}
```

👉 Cargo bizim için bir “hello world” programı üretti. Bu, ikili (binary) bir crate’tir.

---

### ⚙️ Derleme ve Çalıştırma

```
$ cargo build
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
```

👉 Programı çalıştırmak için:

```
$ ./target/debug/hello_world
Hello, world!
```

👉 Tek adımda derleyip çalıştırmak için:

```
$ cargo run
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
     Running `target/debug/hello_world`
Hello, world!
```

Burada ayrıca yeni bir dosya oluştuğunu fark edeceksiniz: `Cargo.lock`. Bu dosya bağımlılıklarınız hakkında bilgi içerir. Henüz bağımlılığımız olmadığı için çok ilginç değildir.

---

### 🚀 Release Modunda Derleme

Hazır olduğunuzda optimizasyonlarla derlemek için:

```
$ cargo build --release
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
```

👉 `cargo build --release` komutu çıktıyı `target/debug` yerine `target/release` klasörüne koyar.

* **Debug modu** geliştirme için varsayılandır. Derleme süresi kısadır çünkü derleyici optimizasyon yapmaz, ancak kod daha yavaş çalışır.
* **Release modu** daha uzun sürede derlenir ama kodunuz daha hızlı çalışır.
