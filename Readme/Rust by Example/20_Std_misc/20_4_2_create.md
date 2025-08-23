## ✍️ create

`create` fonksiyonu, bir dosyayı yalnızca yazma (write-only) modunda açar. Eğer dosya zaten mevcutsa, eski içerik silinir. Aksi halde yeni bir dosya oluşturulur.

```rust
static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();

    // Bir dosyayı yalnızca yazma modunda aç, `io::Result<File>` döner
    let mut file = match File::create(&path) {
        Err(why) => panic!("{} oluşturulamadı: {}", display, why),
        Ok(file) => file,
    };

    // `LOREM_IPSUM` string'ini dosyaya yaz, `io::Result<()>` döner
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("{} dosyasına yazılamadı: {}", display, why),
        Ok(_) => println!("{} dosyasına başarıyla yazıldı", display),
    }
}
```

### ✅ Beklenen başarılı çıktı:

```bash
$ rustc create.rs && ./create
successfully wrote to lorem_ipsum.txt

$ cat lorem_ipsum.txt
Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```

(Tıpkı önceki örnekte olduğu gibi, bu örneği de başarısızlık koşullarında test etmeniz önerilir.)

### 🔧 Not

Bir dosyanın nasıl açılacağını yapılandırmak için `OpenOptions` yapısı kullanılabilir.
