## 🌍 Ortam Değişkenleriyle Çalışma (Working with Environment Variables)

`minigrep` ikilisini (binary) ekstra bir özellikle geliştireceğiz: kullanıcıların bir ortam değişkeni (environment variable) aracılığıyla açıp kapatabileceği **büyük/küçük harf duyarsız arama** (case-insensitive searching) seçeneği. Bu özelliği bir komut satırı seçeneği (command line option) yapsak, kullanıcıların her seferinde bunu girmesi gerekirdi; ancak ortam değişkeni yaparak, kullanıcıların bir kez ayarlayıp o terminal oturumundaki tüm aramaları büyük/küçük harfe duyarsız şekilde yapmalarını sağlamış oluruz.

## 🧪 Büyük/Küçük Harf Duyarsız `search` Fonksiyonu İçin Başarısız Test Yazma (Writing a Failing Test for the Case-Insensitive search Function)

Önce, ortam değişkeninin bir değer aldığı durumda çağrılacak yeni bir `search_case_insensitive` fonksiyonunu `minigrep` kütüphanesine ekleyeceğiz. TDD (test-driven development) sürecine uymaya devam edeceğiz; bu yüzden ilk adım yine başarısız bir test yazmaktır. Yeni `search_case_insensitive` fonksiyonu için yeni bir test ekleyeceğiz ve iki test arasındaki farkları netleştirmek için eski testimizin adını `one_result`’tan `case_sensitive` olarak değiştireceğiz; 12-20 numaralı listede gösterildiği gibi.

### 📄 Dosya Adı: `src/lib.rs`

*Bu kod derlenmez!*

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

**Liste 12-20**: Eklemek üzere olduğumuz büyük/küçük harf duyarsız fonksiyon için yeni başarısız test ekleme

Eski testin içeriğini de güncellediğimize dikkat edin. Aramayı büyük/küçük harfe duyarlı yaptığımızda `"duct"` sorgusuyla eşleşmemesi için baş harfi büyük `D` olan `"Duct tape."` satırını ekledik. Eski testi bu şekilde değiştirmek, hâlihazırda uyguladığımız büyük/küçük harfe duyarlı arama işlevini yanlışlıkla bozmamamıza yardımcı olur. Bu test şu anda geçmelidir ve büyük/küçük harfe duyarsız arama üzerinde çalışırken de geçmeye devam etmelidir.

Büyük/küçük harfe duyarsız arama için yeni test, sorgu olarak `"rUsT"` kullanır. Birazdan ekleyeceğimiz `search_case_insensitive` fonksiyonunda `"rUsT"` sorgusu, baş harfi büyük `R` içeren `"Rust:"` satırıyla ve `"Trust me."` satırıyla (ikisinin de harf büyüklüğü sorgudan farklı olsa bile) eşleşmelidir. Bu bizim başarısız testimizdir ve şu anda derlenmeyecektir çünkü `search_case_insensitive` fonksiyonunu henüz tanımlamadık. 12-16’daki `search` fonksiyonuna benzer şekilde, her zaman boş bir vektör döndüren iskelet bir uygulama ekleyip testin derlenip başarısız olduğunu görmeyi tercih edebilirsiniz.

## 🏗️ `search_case_insensitive` Fonksiyonunu Uygulama (Implementing the search\_case\_insensitive Function)

12-21 numaralı listede gösterilen `search_case_insensitive` fonksiyonu, neredeyse `search` ile aynı olacaktır. Tek fark, sorguyu ve her satırı küçük harfe çevirmemizdir; böylece girdilerin harf büyüklüğü ne olursa olsun, karşılaştırma yaparken aynı büyüklükte olacaklardır.

### 📄 Dosya Adı: `src/lib.rs`

```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

**Liste 12-21**: Karşılaştırmadan önce sorguyu ve satırı küçük harfe çeviren `search_case_insensitive` fonksiyonunu tanımlama

Önce sorgu dizesini küçük harfe çevirip aynı adlı yeni bir değişkende saklıyoruz; bu, özgün `query` değişkenini gölgeler (shadowing). Sorgu üzerinde `to_lowercase` çağrısı yapmak gerekir; böylece kullanıcı sorguyu `"rust"`, `"RUST"`, `"Rust"` ya da `"rUsT"` olarak girse de biz onu `"rust"` gibi ele alır ve harf büyüklüğüne duyarsız oluruz. `to_lowercase` temel Unicode’u (Unicode) ele alır; ancak yüzde yüz doğru olmayabilir. Gerçek bir uygulama yazıyor olsaydık burada biraz daha çalışma yapmak gerekirdi; fakat bu bölümün odağı Unicode değil, ortam değişkenleridir; bu yüzden burada bırakıyoruz.

Dikkat edin: `query` artık bir `String`’dir; çünkü `to_lowercase` mevcut veriye atıf yapmak yerine yeni veri oluşturur. Örnek olarak `"rUsT"` sorgusunu düşünün: bu `string slice`, kullanabileceğimiz küçük `u` veya `t` içermez; bu yüzden `"rust"` içeren yeni bir `String` ayırmamız gerekir. Şimdi `query`’yi `contains` metoduna argüman olarak verirken, `contains` imzası bir `string slice` aldığı için önüne `&` eklememiz gerekir.

Sonraki adımda, her satır üzerindeki tüm karakterleri küçük harfe çevirmek için `to_lowercase` çağrısı ekliyoruz. Artık `line` ve `query`’yi küçük harfe dönüştürdüğümüze göre, sorgunun harf büyüklüğü ne olursa olsun eşleşmeleri bulacağız.

Bu uygulamanın testleri geçip geçmediğine bakalım:

```bash
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.33s
     Running unittests src/lib.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 2 tests
test tests::case_insensitive ... ok
test tests::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Harika! Hepsi geçti. Şimdi yeni `search_case_insensitive` fonksiyonunu `run` fonksiyonundan çağıralım. Önce, `Config` yapısına (struct) büyük/küçük harfe duyarlı ve duyarsız arama arasında geçiş yapmak için bir yapılandırma seçeneği ekleyeceğiz. Bu alanı eklemek, henüz bu alanı hiçbir yerde başlatmadığımız için derleyici hatalarına yol açacaktır:

### 📄 Dosya Adı: `src/main.rs`

*Bu kod derlenmez!*

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
```
## ⚙️ Ortam Değişkenleriyle Duyarlılığı Kontrol Etme

Artık `Config` yapımıza `ignore_case` alanını ekledik. Sıradaki adım, `run` fonksiyonunu bu değeri kontrol edecek şekilde düzenlemek. Eğer `ignore_case` **true** ise `search_case_insensitive`, değilse `search` fonksiyonu çalışacak.

---

### 📄 Dosya: `src/main.rs`

*Bu kod şu an derlenmez!*

```rust
use minigrep::{search, search_case_insensitive};

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```

👉 **Liste 12-22**: `config.ignore_case` değerine göre `search` veya `search_case_insensitive` çağırmak.

---

### 🌍 Ortam Değişkenini Kontrol Etmek

Son adımda, `IGNORE_CASE` ortam değişkeninin ayarlı olup olmadığını kontrol etmeliyiz. Bunun için Rust’ın standart kütüphanesindeki `env` modülündeki `var` fonksiyonunu kullanacağız.

### 📄 Dosya: `src/main.rs`

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

👉 **Liste 12-23**: `IGNORE_CASE` ortam değişkeninin var olup olmadığını kontrol etme.

Burada:

* `env::var("IGNORE_CASE")` çağrılır.

  * Eğer ortam değişkeni **ayarlıysa**, `Ok(value)` döner.
  * Eğer ortam değişkeni **ayarlı değilse**, `Err` döner.
* Biz `is_ok()` kullanıyoruz → sadece değişkenin ayarlı olup olmadığını umursuyoruz, değerini değil.
* Böylece `ignore_case = true` olduğunda büyük/küçük harf duyarsız arama yapılacak.

---

### 🚀 Çalıştırma Testi

Önce ortam değişkeni olmadan deneyelim:

```bash
$ cargo run -- to poem.txt
Are you nobody, too?
How dreary to be somebody!
```

👉 Çalıştı ✅

Şimdi `IGNORE_CASE` değişkenini ayarlayalım:

#### Linux / macOS:

```bash
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

#### PowerShell:

```powershell
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

Ve çıktı:

```
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

👉 Artık `To` içeren satırları da yakaladık 🎉

PowerShell’de ortam değişkenini kaldırmak için:

```powershell
PS> Remove-Item Env:IGNORE_CASE
```

---

✅ Böylece `minigrep`, ortam değişkeni sayesinde **büyük/küçük harf duyarlı veya duyarsız arama** yapabiliyor.

👉 İsteğe bağlı alıştırma: Komut satırı argümanı ile de `--ignore-case` desteği ekleyip, **hangi seçeneğin öncelikli olacağına** karar verebilirsin (argüman mı ortam değişkeni mi?).

---

İstersen ben sana şimdi `--ignore-case` argümanını ekleyerek programı genişletme adımlarını da göstereyim mi?
