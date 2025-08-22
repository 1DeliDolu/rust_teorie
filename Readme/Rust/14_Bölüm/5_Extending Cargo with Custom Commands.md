## 🛠️ Cargo’yu Özel Komutlarla Genişletme (Extending Cargo with Custom Commands)

Cargo, herhangi bir değişiklik yapmadan yeni alt komutlarla genişletilebilecek şekilde tasarlanmıştır. Eğer `$PATH` içinde `cargo-something` adında bir ikili (binary) varsa, onu şu şekilde çalıştırabilirsiniz:

```
cargo something
```

👉 Yani `cargo-something`, Cargo’nun yerleşik bir alt komutuymuş gibi çalıştırılabilir.

Ayrıca, `cargo --list` komutunu çalıştırdığınızda bu özel komutlar da listelenir. `cargo install` ile bu tür uzantıları yükleyebilmek ve ardından yerleşik Cargo araçları gibi çalıştırabilmek, Cargo’nun tasarımının oldukça kullanışlı bir avantajıdır.

---

## 📖 Özet (Summary)

Cargo ve `crates.io` ile kod paylaşmak, Rust ekosistemini birçok farklı görev için faydalı kılan şeylerden biridir. Rust’ın standart kütüphanesi küçük ve kararlı tutulmuştur; ancak crate’ler kolayca paylaşılabilir, kullanılabilir ve dilin kendisinden farklı bir zaman çizelgesinde geliştirilebilir.

👉 Sizin için faydalı olan bir kodu `crates.io` üzerinde paylaşmaktan çekinmeyin; büyük ihtimalle başkaları için de faydalı olacaktır!
