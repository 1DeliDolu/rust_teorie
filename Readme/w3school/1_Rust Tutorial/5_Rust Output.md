## 🖨️ Rust Çıktı (Metin Yazdırma) (Rust Output - Print Text)

## 📌 Çıktı (Output - Print Text)

Rust’ta metin yazdırmak için `println!()` makrosunu (macro) kullanabilirsiniz:

### Örnek (Example)

```rust
fn main() {
  println!("Hello World!");
}
```

👉 Bu kod ekrana `"Hello World!"` yazdırır.

## ➕ Birden Fazla println!() Kullanımı

İstediğiniz kadar `println!()` makrosu kullanabilirsiniz. Her `println!()` yeni bir satır ekler:

### Örnek (Example)

```rust
fn main() {
  println!("Hello World!");
  println!("I am learning Rust.");
  println!("It is awesome!");
}
```

👉 Bu kod, her satıra ayrı ayrı metin yazdırır.

## 🔄 Print Makrosu (The Print Macro)

`print!()` adında bir makro da vardır ve `println!()` ile benzerdir.

Tek fark, çıktı sonunda yeni satır eklememesidir:

### Örnek (Example)

```rust
fn main() {
  print!("Hello World! ");
  print!("I will print on the same line.");
}
```

👉 Çıktı aynı satırda yazılır. Okunabilirliği artırmak için gerektiğinde boşluk eklenir (örnekte `"Hello World!"` sonrasında olduğu gibi).

Bu eğitimde, çıktıları daha kolay okumak için yalnızca `println!()` kullanılacaktır.

## ↩️ Elle Yeni Satır Ekleme (Add New Lines Manually)

Eğer `print!()` içinde gerçekten yeni satır eklemek isterseniz `\n` karakterini kullanabilirsiniz:

### Örnek (Example)

```rust
fn main() {
  print!("Hello World!\n");
  print!("I will print on a new line.");
}
```

👉 `\n` özel bir kaçış dizisidir (escape sequence). İmleci bir sonraki satırın başına taşır ve yeni satır oluşturur.

Bir satırı ortadan da bölebilirsiniz. Bu hem `print!()` hem de `println!()` için geçerlidir:

### Örnek (Example)

```rust
fn main() {
  println!("Hello World!\nThis line was broken up!");
}
```

👉 Çıktı iki satıra bölünmüş şekilde yazdırılır.
