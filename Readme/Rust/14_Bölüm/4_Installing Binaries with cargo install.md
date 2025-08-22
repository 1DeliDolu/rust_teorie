## 💾 cargo install ile İkili Dosyalar Yükleme (Installing Binaries with cargo install)

`cargo install` komutu, ikili crate’leri (binary crates) yerel olarak yüklemenizi ve kullanmanızı sağlar. Bu, sistem paketlerinin yerine geçmesi için değil; Rust geliştiricilerinin `crates.io` üzerinde paylaşılan araçları kolayca yükleyip kullanabilmeleri için tasarlanmıştır.

👉 Yalnızca ikili hedefleri (binary targets) olan paketleri yükleyebilirsiniz.

* **Binary target**: Çalıştırılabilir bir programdır; crate’in `src/main.rs` dosyası varsa veya başka bir dosya ikili olarak tanımlanmışsa oluşturulur.
* **Library target**: Tek başına çalıştırılamaz, ancak başka programlara dahil edilmeye uygundur.

Genellikle, bir crate’in README dosyasında bunun bir kütüphane mi, ikili hedefe mi yoksa her ikisine de sahip olup olmadığı hakkında bilgi bulunur.

---

## 📂 Yükleme Dizini ve PATH (Installation Directory and PATH)

`cargo install` ile yüklenen tüm ikili dosyalar, kurulum kök dizininin `bin` klasöründe saklanır. Eğer Rust’ı `rustup.rs` ile kurduysanız ve özel bir yapılandırmanız yoksa bu dizin şu olacaktır:

```
$HOME/.cargo/bin
```

👉 Bu dizinin `$PATH` içinde olduğundan emin olun, böylece `cargo install` ile yüklediğiniz programları doğrudan terminalden çalıştırabilirsiniz.

---

## 🔍 Örnek: ripgrep Yükleme (Example: Installing ripgrep)

Bölüm 12’de, dosyaları aramak için kullanılan `grep` aracının Rust implementasyonu olan `ripgrep`’ten bahsetmiştik. `ripgrep`’i yüklemek için şu komutu çalıştırabiliriz:

```
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

Çıktının sondan bir önceki satırı, yüklenen ikilinin konumunu ve adını gösterir. `ripgrep` için bu ad `rg`’dir.

👉 `$PATH` ayarınızda bu dizin varsa, artık şu komutu çalıştırabilirsiniz:

```
rg --help
```

Böylece dosya arama işlemleri için daha hızlı, Rust tabanlı bir araç olan **ripgrep**’i kullanmaya başlayabilirsiniz!
