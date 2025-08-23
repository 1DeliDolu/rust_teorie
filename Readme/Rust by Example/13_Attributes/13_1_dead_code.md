## ⚰️ dead\_code

Derleyici, **kullanılmayan fonksiyonlar** için `dead_code` adlı bir uyarı (lint) üretir. Bu uyarıyı devre dışı bırakmak için bir öznitelik (attribute) kullanılabilir.

```rust
fn used_function() {}

// `#[allow(dead_code)]`, `dead_code` uyarısını devre dışı bırakan bir özniteliktir
#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}
// FIXME ^ Uyarıyı bastırmak için bir öznitelik eklendi

fn main() {
    used_function();
}
```

Gerçek programlarda, kullanılmayan kodları ortadan kaldırmanız gerekir. Bu örneklerde ise etkileşimli doğalarından dolayı bazı yerlerde `dead_code`’a izin veriyoruz.
