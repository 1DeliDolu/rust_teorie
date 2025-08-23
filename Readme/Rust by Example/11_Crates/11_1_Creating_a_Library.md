## 🏗️ Kütüphane Oluşturma (creating a library)

Bir **kütüphane (library)** oluşturalım ve ardından onu başka bir crate’e nasıl bağlayacağımıza bakalım.

`rary.rs` dosyasında:

```rust
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```

```bash
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib
```

👉 Bu komut `rary.rs` dosyasını kütüphane (`.rlib`) olarak derler.

Kütüphaneler `"lib"` önekiyle adlandırılır ve varsayılan olarak crate dosyasının adına göre isimlendirilir. Ancak bu varsayılan ad, `rustc` derleyicisine `--crate-name` seçeneği verilerek veya `crate_name` özniteliği (attribute) kullanılarak değiştirilebilir.
