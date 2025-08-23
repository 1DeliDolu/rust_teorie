## 📝 Argüman Ayrıştırma (Argument parsing)

Basit argümanları ayrıştırmak için **eşleştirme (matching)** kullanılabilir:

```rust
use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // argüman verilmedi
        1 => {
            println!("Benim adım 'match_args'. Birkaç argüman deneyin!");
        },
        // bir argüman verildi
        2 => {
            match args[1].parse() {
                Ok(42) => println!("Bu cevap!"),
                _ => println!("Bu cevap değil."),
            }
        },
        // bir komut ve bir argüman verildi
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // sayıyı ayrıştır
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("hata: ikinci argüman bir tamsayı değil");
                    help();
                    return;
                },
            };
            // komutu ayrıştır
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("hata: geçersiz komut");
                    help();
                },
            }
        },
        // diğer tüm durumlar
        _ => {
            // yardım mesajını göster
            help();
        }
    }
}
```

### ✅ Örnek kullanım

Eğer programınızı `match_args.rs` olarak adlandırır ve şu şekilde derlerseniz:

```bash
rustc match_args.rs
```

Aşağıdaki gibi çalıştırabilirsiniz:

```bash
$ ./match_args Rust
This is not the answer.

$ ./match_args 42
This is the answer!

$ ./match_args do something
error: second argument not an integer
usage:
match_args <string>
    Check whether given string is the answer.
match_args {increase|decrease} <integer>
    Increase or decrease given integer by one.

$ ./match_args do 42
error: invalid command
usage:
match_args <string>
    Check whether given string is the answer.
match_args {increase|decrease} <integer>
    Increase or decrease given integer by one.

$ ./match_args increase 42
43
```
