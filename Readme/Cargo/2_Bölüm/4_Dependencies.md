## 🔗 Bağımlılıklar (Dependencies)

**crates.io**, Rust topluluğunun merkezi paket kayıt deposudur. Paketleri keşfetmek ve indirmek için kullanılır. `cargo` varsayılan olarak bağımlılıkları bulmak için bu kayıt deposunu kullanacak şekilde yapılandırılmıştır.

Bir kütüphaneye (library) `crates.io` üzerinden bağımlılık eklemek için onu `Cargo.toml` dosyanıza eklemeniz gerekir.

---

### ➕ Bağımlılık Ekleme (Adding a dependency)

Eğer `Cargo.toml` dosyanızda henüz `[dependencies]` bölümü yoksa, onu ekleyin ve ardından kullanmak istediğiniz crate’in adını ve sürümünü yazın. Bu örnekte `time` crate’ine bağımlılık eklenmektedir:

```
[dependencies]
time = "0.1.12"
```

👉 Buradaki sürüm bilgisi **SemVer** sürüm kuralına göre yazılmıştır. Bağımlılık belirtme seçenekleri hakkında daha fazla bilgi için ilgili belgelere göz atabilirsiniz.

Ayrıca `regex` crate’ini de eklemek istersek, her crate için ayrı bir `[dependencies]` eklememize gerek yoktur. İşte hem `time` hem de `regex` bağımlılıklarını içeren tam `Cargo.toml` örneği:

```
[package]
name = "hello_world"
version = "0.1.0"
edition = "2024"

[dependencies]
time = "0.1.12"
regex = "0.1.41"
```

---

### ⚙️ Bağımlılıkları İndirip Derlemek

```
$ cargo build
      Updating crates.io index
   Downloading memchr v0.1.5
   Downloading libc v0.1.10
   Downloading regex-syntax v0.2.1
   Downloading memchr v0.1.5
   Downloading aho-corasick v0.3.0
   Downloading regex v0.1.41
     Compiling memchr v0.1.5
     Compiling libc v0.1.10
     Compiling regex-syntax v0.2.1
     Compiling memchr v0.1.5
     Compiling aho-corasick v0.3.0
     Compiling regex v0.1.41
     Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
```

👉 `Cargo.lock` dosyası tüm bu bağımlılıkların hangi sürümlerinin kullanıldığını kesin olarak kaydeder.

Eğer `regex` güncellense bile, siz `cargo update` çalıştırana kadar aynı sürümle derleme yapılmaya devam eder.

---

### 🧑‍💻 Bağımlılığı Kullanmak

Artık `regex` kütüphanesini `main.rs` dosyanızda kullanabilirsiniz:

```rust
use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
```

Çalıştırdığınızda şu sonucu göreceksiniz:

```
$ cargo run
   Running `target/hello_world`
Did our date match? true
```
