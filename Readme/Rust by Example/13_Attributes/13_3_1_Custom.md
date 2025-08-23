## 🛠️ Özel Koşullar (Custom)

`target_os` gibi bazı koşullar `rustc` tarafından örtük (implicit) olarak sağlanır, ancak **özel koşullar (custom conditionals)** `rustc`’ye `--cfg` bayrağı ile verilmelidir.

```rust
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}
```

Bu programı özel `cfg` bayrağı olmadan çalıştırmayı deneyin ve ne olduğunu görün.

Özel `cfg` bayrağı ile:

```bash
$ rustc --cfg some_condition custom.rs && ./custom
condition met!
```
