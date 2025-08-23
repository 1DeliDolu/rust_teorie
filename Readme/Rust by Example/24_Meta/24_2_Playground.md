## 🏓 Playground (playground)

**Rust Playground**, Rust kodunu web arayüzü üzerinden denemek için kullanılan bir yoldur.

---

## 📘 mdbook ile Kullanımı (using it with mdbook)

`mdbook` içinde kod örneklerini **çalıştırılabilir** ve **düzenlenebilir** hale getirebilirsiniz.

Bu, okuyucunun kod örneğinizi hem çalıştırmasına hem de üzerinde değişiklik yapmasına olanak tanır.
Bunun için `codefence` bloğuna `editable` kelimesini virgülle ayırarak eklemeniz gerekir:

```rust,editable
//...buraya kodunuzu yazın
```

Ek olarak, `ignore` kullanırsanız `mdbook` bu kodu derleyip test ederken atlayacaktır:

```rust,editable,ignore
//...buraya kodunuzu yazın
```

---

## 📑 Dokümantasyon ile Kullanımı (using it with docs)

Resmi Rust dokümantasyonunda bazı örneklerin yanında görünen **"Run"** butonunu fark etmiş olabilirsiniz.
Bu buton, örneği yeni bir sekmede **Rust Playground** üzerinde açar.

Bu özellik, `#[doc]` özniteliği olan `html_playground_url` kullanıldığında etkinleşir:

````rust
#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! ```
//! println!("Hello World");
//! ```
````

---

## 📚 Ayrıca bakınız

* [The Rust Playground](https://play.rust-lang.org/)
* [The Rust Playground on GitHub](https://github.com/integer32llc/rust-playground)
* [The rustdoc Book](https://doc.rust-lang.org/rustdoc/)
