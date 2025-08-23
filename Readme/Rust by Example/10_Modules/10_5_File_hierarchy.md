## 📂 Dosya Hiyerarşisi (file hierarchy)

Modüller, dosya/dizin hiyerarşisine eşlenebilir. Hadi görünürlük (visibility) örneğini dosyalara ayıralım:

```bash
$ tree .
.
├── my
│   ├── inaccessible.rs
│   └── nested.rs
├── my.rs
└── split.rs
```

---

### 📄 split.rs

```rust
// Bu bildirim `my.rs` adında bir dosya arar
// ve içeriğini, bu kapsam altında `my` adında bir modüle ekler
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
```

---

### 📄 my.rs

```rust
// Benzer şekilde, `mod inaccessible` ve `mod nested` ifadeleri
// `inaccessible.rs` ve `nested.rs` dosyalarını bulur
// ve içeriklerini kendi modülleri altında buraya ekler
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}
```

---

### 📄 my/nested.rs

```rust
pub fn function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my::nested::private_function()`");
}
```

---

### 📄 my/inaccessible.rs

```rust
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inaccessible::public_function()`");
}
```

---

### ▶️ Çalıştırma Sonucu

```bash
$ rustc split.rs && ./split
called `my::function()`
called `function()`
called `my::indirect_access()`, that
> called `my::private_function()`
called `my::nested::function()`
```
