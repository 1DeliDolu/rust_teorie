## 📚 Bağımlılıklar (Dependencies)

Çoğu program bazı kütüphanelere bağımlıdır. Eğer daha önce bağımlılıkları manuel olarak yönetmek zorunda kaldıysanız, bunun ne kadar zahmetli olabileceğini bilirsiniz. Neyse ki Rust ekosistemi `cargo` ile birlikte gelir! `cargo`, bir proje için bağımlılıkları yönetebilir.

Yeni bir Rust projesi oluşturmak için:

```bash
# Bir ikili (binary)
cargo new foo

# Bir kütüphane (library)
cargo new --lib bar
```

Bu bölümün geri kalanında bir ikili (binary) yaptığımızı varsayacağız, ancak tüm kavramlar bir kütüphane (library) için de aynıdır.

Yukarıdaki komutlardan sonra şöyle bir dosya hiyerarşisi görmelisiniz:

```
.
├── bar
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── foo
    ├── Cargo.toml
    └── src
        └── main.rs
```

`main.rs`, yeni `foo` projenizin kök kaynak dosyasıdır — burada yeni bir şey yok. `Cargo.toml` ise bu proje için `cargo` yapılandırma dosyasıdır. İçine bakarsanız şöyle bir şey görürsünüz:

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
```

* `[package]` altındaki `name` alanı, projenin adını belirler. Eğer `crate`’i yayınlarsanız (daha sonra anlatılacak), bu `crates.io` tarafından kullanılır. Ayrıca derleme sırasında çıktı ikilisinin adıdır.
* `version` alanı, Semantik Versiyonlama (Semantic Versioning) kullanan bir sürüm numarasıdır.
* `authors` alanı, crate yayınlanırken kullanılan yazarların listesidir.
* `[dependencies]` bölümü, projeniz için bağımlılıkları eklemenizi sağlar.

Örneğin, programımız için güçlü bir CLI yapmak istediğimizi varsayalım. `crates.io` (resmi Rust paket kayıt merkezi) üzerinde birçok harika paket bulabilirsiniz. Popüler seçeneklerden biri `clap`’tir. Bu yazı yazıldığı sırada `clap`’in en güncel sürümü `2.27.1` idi. Programımıza bir bağımlılık eklemek için, `Cargo.toml` dosyamızda `[dependencies]` altına şunu eklememiz yeterlidir:

```toml
clap = "2.27.1"
```

Hepsi bu kadar! Artık programınızda `clap` kullanmaya başlayabilirsiniz.

`cargo`, başka bağımlılık türlerini de destekler. İşte küçük bir örnek:

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # crates.io üzerinden
rand = { git = "https://github.com/rust-lang-nursery/rand" } # çevrimiçi repo’dan
bar = { path = "../bar" } # yerel dosya sisteminde bir path’ten
```

`cargo`, sadece bir bağımlılık yöneticisi değildir. Tüm yapılandırma seçenekleri `Cargo.toml` dosyasının format tanımında listelenmiştir.

Projemizi derlemek için, proje dizini içinde (alt dizinler dahil) şu komutu çalıştırabiliriz:

```bash
cargo build
```

Ayrıca derleyip çalıştırmak için şu komutu da kullanabiliriz:

```bash
cargo run
```

Bu komutlar tüm bağımlılıkları çözer, gerekirse `crates` indirir ve her şeyi derler, sizin `crate`’iniz dahil. (Not: `make`’e benzer şekilde yalnızca daha önce derlenmemiş olanları yeniden derler.)

İşte bu kadar! 🚀
