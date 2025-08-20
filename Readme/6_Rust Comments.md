## 💬 Rust Yorumlar (Rust Comments)

## 📌 Rust’ta Yorumlar (Comments in Rust)

Yorumlar (comments) kodu açıklamak ve daha okunabilir hale getirmek için kullanılır. Ayrıca alternatif kodları test ederken çalıştırmayı engellemek için de kullanılabilir.

Yorumlar tek satırlık (single-line) veya çok satırlık (multi-line) olabilir.

## 📝 Tek Satırlık Yorumlar (Single-line Comments)

Tek satırlık yorumlar `//` ile başlar.

`//` ile satır sonu arasındaki tüm metin derleyici (compiler) tarafından yok sayılır (çalıştırılmaz).

Aşağıdaki örnek, kod satırından önce tek satırlık bir yorum kullanır:

### Örnek (Example)

```rust
fn main() {
  // This is a comment
  println!("Hello World!");
}
```

Aşağıdaki örnek, kod satırının sonunda tek satırlık bir yorum kullanır:

### Örnek (Example)

```rust
println!("Hello World!"); // This is a comment
```

## 📑 Çok Satırlı Yorumlar (Multi-line Comments)

Çok satırlı yorumlar `/*` ile başlar ve `*/` ile biter.

`/*` ile `*/` arasındaki tüm metin derleyici tarafından yok sayılır:

### Örnek (Example)

```rust
/* The code below will print the words Hello World!
to the screen, and it is amazing */
println!("Hello World!");
```

## ❓ Tek Satır mı, Çok Satır mı? (Single or multi-line comments?)

Hangisini kullanacağınız size bağlıdır. Genellikle kısa açıklamalar için `//`, daha uzun açıklamalar için `/* */` tercih edilir.
