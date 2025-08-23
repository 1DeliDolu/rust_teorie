## 🗑️ Drop

**`Drop` trait’i**, yalnızca bir metoda sahiptir: `drop`. Bu yöntem, bir nesne kapsam (scope) dışına çıktığında otomatik olarak çağrılır. `Drop` trait’inin temel kullanım amacı, uygulayıcı (implementor) örneğinin sahip olduğu kaynakları serbest bırakmaktır.

`Box`, `Vec`, `String`, `File` ve `Process`, kaynakları serbest bırakmak için `Drop` trait’ini uygulayan türlere örnektir. `Drop` trait’i ayrıca özel (custom) veri türleri için de manuel olarak uygulanabilir.

Aşağıdaki örnek, `drop` fonksiyonuna konsola yazdırma ekleyerek ne zaman çağrıldığını gösterecek şekilde düzenlenmiştir:

```rust
struct Droppable {
    name: &'static str,
}

// Bu basit `drop` uygulaması konsola yazdırma ekler.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Değişken, `drop` fonksiyonu kullanılarak manuel olarak bırakılabilir.
    drop(_a);
    // TODO ^ Bu satırı yorum satırı haline getirmeyi deneyin.

    println!("end of the main function");

    // `_a` burada tekrar `drop` edilmeyecek, çünkü daha önce manuel olarak bırakıldı.
}
```

### 📂 Daha Pratik Bir Örnek

Aşağıda, `Drop` trait’inin artık ihtiyaç duyulmadığında geçici dosyaları otomatik olarak temizlemek için nasıl kullanılabileceğini görebilirsiniz:

```rust
use std::fs::File;
use std::path::PathBuf;

struct TempFile {
    file: File,
    path: PathBuf,
}

impl TempFile {
    fn new(path: PathBuf) -> std::io::Result<Self> {
        // Not: File::create() mevcut dosyaların üzerine yazar
        let file = File::create(&path)?;
        
        Ok(Self { file, path })
    }
}

// TempFile bırakıldığında:
// 1. Önce `File` otomatik olarak kapanır (`Drop` for File).
// 2. Ardından bizim `drop` uygulamamız dosyayı siler.
impl Drop for TempFile {
    fn drop(&mut self) {
        // Not: Bu noktada dosya zaten kapatılmıştır.
        if let Err(e) = std::fs::remove_file(&self.path) {
            eprintln!("Failed to remove temporary file: {}", e);
        }
        println!("> Dropped temporary file: {:?}", self.path);
    }
}

fn main() -> std::io::Result<()> {
    // Drop davranışını göstermek için yeni bir scope oluşturuyoruz
    {
        let temp = TempFile::new("test.txt".into())?;
        println!("Temporary file created");
        // `temp` kapsam dışına çıktığında dosya otomatik olarak temizlenecek
    }
    println!("End of scope - file should be cleaned up");

    // Manuel olarak da bırakabiliriz
    let temp2 = TempFile::new("another_test.txt".into())?;
    drop(temp2); // Dosyayı açıkça bırak
    println!("Manually dropped file");
    
    Ok(())
}
```
