## 🛠️ Ek D: Faydalı Geliştirme Araçları (useful development tools)

Bu ekte, Rust projesinin sağladığı bazı faydalı geliştirme araçlarını ele alıyoruz. Otomatik biçimlendirme, uyarı düzeltmelerini hızlıca uygulama yolları, bir linter ve IDE entegrasyonuna bakacağız.

---

## ✨ rustfmt ile Otomatik Biçimlendirme (automatic formatting with rustfmt)

`rustfmt` aracı, kodunuzu topluluk kod stili kurallarına göre yeniden biçimlendirir. Birçok ortak projede, hangi stilin kullanılacağına dair tartışmaları önlemek için `rustfmt` kullanılır: herkes kodunu bu araçla biçimlendirir.

Rust kurulumları `rustfmt` ile birlikte gelir, bu nedenle sisteminizde `rustfmt` ve `cargo-fmt` komutları zaten bulunur. Bu iki komut, `rustc` ve `cargo`’ya benzer şekilde çalışır: `rustfmt` daha ince ayar kontrol sağlar, `cargo-fmt` ise Cargo kullanan bir projenin kurallarını anlar.

Herhangi bir Cargo projesini biçimlendirmek için şu komutu çalıştırın:

```bash
$ cargo fmt
```

Bu komut, geçerli crate içindeki tüm Rust kodunu yeniden biçimlendirir. Bu işlem yalnızca kod stilini değiştirir, kodun anlamını (semantics) değiştirmez.

Daha fazla bilgi için `rustfmt` belgelerine bakınız.

---

## 🔧 rustfix ile Kodunuzu Düzeltmek (fix your code with rustfix)

`rustfix`, Rust kurulumlarına dahildir ve derleyici uyarılarını otomatik olarak düzeltebilir. Bu araç, problemin nasıl düzeltileceği açıksa ve muhtemelen sizin de isteyeceğiniz şekildeyse çalışır.

Örneğin:

**Dosya Adı: src/main.rs**

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

Burada `x` değişkenini `mut` olarak tanımlıyoruz ama aslında değiştirmiyoruz. Derleyici şu uyarıyı verir:

```bash
$ cargo build
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
```

`mut` anahtar kelimesinin kaldırılması gerektiği önerilmektedir. Bunu otomatik olarak uygulamak için şu komutu çalıştırabiliriz:

```bash
$ cargo fix
```

Komut tamamlandıktan sonra kod şu hale gelir:

**Dosya Adı: src/main.rs**

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

Artık değişken `immutable` hale gelmiş ve uyarı ortadan kalkmıştır.

Ayrıca `cargo fix` komutunu, kodunuzu farklı Rust sürümleri (editions) arasında taşımak için de kullanabilirsiniz (Bkz. Ek E).

---

## 🔍 Clippy ile Daha Fazla Lint (more lints with Clippy)

`Clippy`, kodunuzu analiz etmek için kullanılan lint koleksiyonudur. Bu sayede yaygın hataları yakalayabilir ve Rust kodunuzu geliştirebilirsiniz. Clippy, standart Rust kurulumlarıyla birlikte gelir.

Herhangi bir Cargo projesinde Clippy çalıştırmak için:

```bash
$ cargo clippy
```

Örneğin, aşağıdaki programda pi sayısının yaklaşık bir değeri kullanılmaktadır:

**Dosya Adı: src/main.rs**

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

Clippy bu programı çalıştırdığınızda şu hatayı verir:

```
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  = help: consider using the constant directly
```

Bu hata, Rust’ın zaten daha hassas bir `PI` sabiti tanımladığını belirtir. Kodunuzu şu şekilde değiştirmelisiniz:

**Dosya Adı: src/main.rs**

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

Artık Clippy herhangi bir hata veya uyarı vermez. Daha fazla bilgi için Clippy belgelerine bakabilirsiniz.

---

## 🖥️ rust-analyzer ile IDE Entegrasyonu (IDE integration using rust-analyzer)

IDE entegrasyonuna yardımcı olmak için Rust topluluğu `rust-analyzer` kullanılmasını önerir. Bu araç, Derleyici Merkezli Yardımcı Araçlar (compiler-centric utilities) kümesidir ve IDE’ler ile programlama dillerinin birbirleriyle iletişim kurmasını sağlayan **Language Server Protocol (LSP)** spesifikasyonunu kullanır.

Farklı istemciler `rust-analyzer` kullanabilir, örneğin Visual Studio Code için `rust-analyzer` eklentisi.

Kurulum talimatları için `rust-analyzer` projesinin ana sayfasını ziyaret edin, ardından IDE’nize uygun dil sunucusu desteğini yükleyin. IDE’niz şu yeteneklere sahip olacaktır:

* Otomatik tamamlama (autocompletion)
* Tanıma gitme (jump to definition)
* Satır içi hata gösterimi (inline errors)
