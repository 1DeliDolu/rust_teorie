## 🏭 Alt süreçler (Child processes)

`process::Output` yapısı, tamamlanmış bir alt sürecin çıktısını temsil eder; `process::Command` yapısı ise bir süreç oluşturucudur (process builder).

```rust
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("süreç çalıştırılamadı: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc başarılı oldu ve stdout şuydu:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc başarısız oldu ve stderr şuydu:\n{}", s);
    }
}
```

(Bu örneği, `rustc`’ye yanlış bir bayrak geçirerek denemeniz tavsiye edilir.)
