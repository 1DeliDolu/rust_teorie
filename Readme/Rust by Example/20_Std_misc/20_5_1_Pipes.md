## 🔗 Borular (Pipes)

`std::process::Child` yapısı, bir alt süreci (child process) temsil eder ve altta yatan süreçle etkileşim için `stdin`, `stdout` ve `stderr` tanıtıcılarını (handle) borular (pipes) aracılığıyla açığa çıkarır.

```rust
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
"the quick brown fox jumps over the lazy dog\n";

fn main() {
    // `wc` komutunu başlat
    let mut cmd = if cfg!(target_family = "windows") {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command").arg("$input | Measure-Object -Line -Word -Character");
        cmd
    } else {
        Command::new("wc")
    };
    let process = match cmd
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("wc başlatılamadı: {}", why),
        Ok(process) => process,
    };

    // `wc`’nin `stdin`’ine bir string yaz.
    //
    // `stdin` türü `Option<ChildStdin>`’dir, fakat bu örnekte kesinlikle
    // mevcut olduğundan doğrudan `unwrap` edebiliriz.
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("wc stdin'e yazılamadı: {}", why),
        Ok(_) => println!("pangram wc’ye gönderildi"),
    }

    // Yukarıdaki çağrılardan sonra `stdin` artık yaşamaz, bu yüzden `drop` edilir
    // ve boru kapatılır.
    //
    // Bu çok önemlidir, aksi takdirde `wc` az önce gönderdiğimiz girdiyi
    // işlemeye başlamazdı.

    // `stdout` alanı da `Option<ChildStdout>` türündedir, bu yüzden unwrap edilmelidir.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("wc stdout okunamadı: {}", why),
        Ok(_) => print!("wc şu cevabı verdi:\n{}", s),
    }
}
```
