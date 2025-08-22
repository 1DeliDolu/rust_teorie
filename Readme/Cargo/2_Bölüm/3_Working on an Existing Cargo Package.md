## 🛠️ Mevcut Bir Cargo Paketi Üzerinde Çalışma (Working on an Existing Cargo Package)

Cargo kullanan mevcut bir paketi indirirseniz, işe başlamak gerçekten çok kolaydır.

Öncelikle paketi bir yerden edinin. Bu örnekte GitHub’daki deposundan klonladığımız **regex** paketini kullanıyoruz:

```
$ git clone https://github.com/rust-lang/regex.git
$ cd regex
```

👉 Derlemek için `cargo build` komutunu kullanın:

```
$ cargo build
   Compiling regex v1.5.0 (file:///path/to/package/regex)
```

👉 Bu işlem tüm bağımlılıkları indirip derleyecek ve ardından paketi de birlikte derleyecektir.
