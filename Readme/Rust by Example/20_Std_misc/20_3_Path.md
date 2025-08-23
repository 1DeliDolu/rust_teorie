## 📂 Yol (Path)

`Path` yapısı, altta yatan dosya sistemindeki dosya yollarını temsil eder. `Path`’in iki türü vardır: UNIX benzeri sistemler için `posix::Path` ve Windows için `windows::Path`. `Prelude`, uygun platforma özgü `Path` çeşidini dışa aktarır.

Bir `Path`, bir `OsStr`’den oluşturulabilir ve yolun işaret ettiği dosya/dizin hakkında bilgi edinmek için çeşitli yöntemler sağlar.

Bir `Path` değiştirilemez (immutable). `Path`’in sahipli sürümü `PathBuf`’dur. `Path` ile `PathBuf` arasındaki ilişki, `str` ile `String` arasındaki ilişkiye benzer: bir `PathBuf` yerinde değiştirilebilir (mutate) ve bir `Path`’e dereference edilebilir.

Bir `Path` dahili olarak bir UTF-8 dizesi (string) olarak temsil edilmez, bunun yerine bir `OsString` olarak saklanır. Bu nedenle, bir `Path`’i `&str`’e dönüştürmek ücretsiz değildir ve başarısız olabilir (bir `Option` döner). Ancak, bir `Path` `into_os_string` ve `as_os_str` kullanılarak serbestçe `OsString` veya `&OsStr`’ye dönüştürülebilir.

```rust
use std::path::Path;

fn main() {
    // Bir `Path`, `&'static str`'den oluşturulur
    let path = Path::new(".");

    // `display` metodu, `Display` edilebilir bir yapı döndürür
    let _display = path.display();

    // `join`, bir yolu OS'e özgü ayırıcı kullanarak bayt kapsayıcısı ile birleştirir
    // ve bir `PathBuf` döndürür
    let mut new_path = path.join("a").join("b");

    // `push`, `PathBuf`'u bir `&Path` ile genişletir
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name`, `PathBuf`'un dosya adını günceller
    new_path.set_file_name("package.tgz");

    // `PathBuf`'u bir string slice'a dönüştür
    match new_path.to_str() {
        None => panic!("yeni yol geçerli bir UTF-8 dizisi değil"),
        Some(s) => println!("yeni yol: {}", s),
    }
}
```

Diğer `Path` metodlarını (`posix::Path` veya `windows::Path`) ve `Metadata` yapısını da incelemeyi unutmayın.

### 🔗 Ayrıca bakınız

* `OsStr`
* `Metadata`
