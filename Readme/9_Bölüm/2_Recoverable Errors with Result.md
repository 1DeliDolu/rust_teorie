## ✅ `Result` ile Kurtarılabilir Hatalar (recoverable errors)

Çoğu hata, programın tamamen durmasını gerektirecek kadar ciddi değildir. Bazen bir fonksiyonun başarısız olmasının nedeni kolayca yorumlanabilir ve uygun şekilde yanıt verilebilir. Örneğin, bir dosyayı açmaya çalıştığınızda bu işlem dosya mevcut olmadığı için başarısız olursa, işlemi sonlandırmak yerine dosyayı oluşturmak isteyebilirsiniz.

Bölüm 2’deki **“Handling Potential Failure with Result”** kısmından hatırlayacağınız üzere, `Result` enum’u şu iki varyanta sahiptir: `Ok` ve `Err`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Buradaki `T` ve `E` generic tip parametreleridir. Bu konuyu Bölüm 10’da daha ayrıntılı inceleyeceğiz. Şimdilik bilmeniz gereken, `T` tipinin başarılı bir durumda `Ok` varyantının içinde döndürülecek değeri temsil ettiği, `E` tipinin ise başarısızlık durumunda `Err` varyantı ile döndürülecek hata tipini temsil ettiğidir. `Result` bu generic parametrelere sahip olduğundan, farklı başarı ve hata tiplerine ihtiyaç duyduğumuz pek çok durumda kullanılabilir.

### 📂 Dosya Açma Örneği (Listing 9-3)

Bir fonksiyonun başarısız olabileceği durumda, `Result` döndüren bir fonksiyon çağıralım.

**Dosya adı:** `src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

`File::open` fonksiyonunun dönüş tipi `Result<T, E>`’dir. Burada:

* `T`, başarılı durumda dönen değer tipidir: `std::fs::File` (bir dosya tutamacı / file handle).
* `E`, hata durumunda dönen değer tipidir: `std::io::Error`.

Bu dönüş tipi bize `File::open` çağrısının ya başarılı olup bir dosya tutamacı döndüreceğini ya da başarısız olup bir hata bilgisi sağlayacağını anlatır.

Başarı durumunda, `greeting_file_result` değişkeninde bir `Ok` varyantı ve onun içinde dosya tutamacı bulunur. Başarısızlık durumunda ise `greeting_file_result`, bir `Err` varyantı ve hata bilgisi içerir.

### 🔀 `match` ile Result Kullanımı (Listing 9-4)

Listing 9-3’teki koda, `File::open` çağrısının sonucuna göre farklı eylemler ekleyelim. Bunu yapmanın temel yolu, Bölüm 6’da incelediğimiz `match` ifadesidir.

**Dosya adı:** `src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

Burada dikkat edelim: `Option` enum’u gibi `Result` enum’u ve varyantları da *prelude* aracılığıyla scope’a alınmıştır. Yani `Ok` ve `Err` varyantları için `Result::` yazmamıza gerek yoktur.

* Eğer sonuç `Ok` ise, `Ok` içindeki dosya değeri dışarı çıkarılır ve `greeting_file` değişkenine atanır.
* Eğer sonuç `Err` ise, bu durumda `panic!` makrosu çağrılır.

Eğer geçerli dizinde `hello.txt` dosyası yoksa ve bu kodu çalıştırırsak, şu çıktıyı görürüz:

```bash
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/error-handling`

thread 'main' panicked at src/main.rs:8:23:
Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Her zamanki gibi, bu çıktı bize tam olarak neyin yanlış gittiğini gösterir.
## 🔀 Farklı Hataları Eşleştirme (matching on different errors)

Listing 9-4’teki kod, `File::open` neden başarısız olursa olsun `panic!` oluşturur. Ancak biz farklı hata nedenleri için farklı işlemler yapmak isteriz. Örneğin:

* Eğer `File::open` dosya mevcut olmadığı için başarısız olduysa, dosyayı oluşturmak ve yeni dosya tutamacını (handle) döndürmek isteriz.
* Eğer `File::open` başka bir nedenle (örneğin dosyayı açmaya iznimiz olmadığı için) başarısız olduysa, Listing 9-4’teki gibi yine `panic!` çağırmak isteriz.

Bunun için iç içe bir `match` ifadesi ekleriz.

### 📂 İç içe `match` ile hata işleme (Listing 9-5)

**Dosya adı:** `src/main.rs`

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
```

Listing 9-5: Farklı hata türlerini farklı yollarla ele almak

`File::open` çağrısı `Err` döndürdüğünde, bu `Err` içinde `std::io::Error` tipinde bir değer bulunur. Bu struct, `kind` adında bir metoda sahiptir; bu metod bize `io::ErrorKind` değerini döndürür.

`io::ErrorKind`, standart kütüphanede tanımlı bir enum’dur ve olası I/O hatalarının türlerini belirtir. Burada kullanmak istediğimiz varyant `ErrorKind::NotFound`’dur, bu da açmaya çalıştığımız dosyanın henüz mevcut olmadığını gösterir.

Dolayısıyla, önce `greeting_file_result` üzerinde eşleştirme yaparız, ardından içte `error.kind()` üzerinde bir eşleştirme daha yaparız:

* Eğer hata `NotFound` ise, `File::create` ile dosyayı oluşturmaya çalışırız.
* Eğer `File::create` de başarısız olursa, bu kez farklı bir hata mesajı veririz.
* Eğer hata `NotFound` dışındaki bir nedenden kaynaklanıyorsa, program `panic!` ile sonlanır.

---

## ✨ `match` Yerine Alternatifler

Çok fazla `match` kullanmak kodu karmaşık hale getirebilir. `match` ifadesi çok faydalı olsa da oldukça temel bir yapıdır. Bölüm 13’te closure’lar ve `Result<T, E>` üzerinde tanımlı pek çok metod göreceksiniz. Bu metodlar, hata işleme sırasında `match` kullanmaya kıyasla daha kısa ve okunaklı çözümler sağlar.

### 🧩 `unwrap_or_else` ile hata işleme

Aşağıda, Listing 9-5’teki mantığı `unwrap_or_else` ve closure’larla yazılmış haliyle görebilirsiniz:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

Bu kod Listing 9-5 ile aynı davranışa sahiptir, ancak hiç `match` ifadesi içermez ve daha okunaklıdır.

Bölüm 13’ü okuduktan sonra bu örneğe geri dönün ve standart kütüphane belgelerinde `unwrap_or_else` metoduna bakın. Bunun gibi pek çok metod, hatalarla uğraşırken iç içe geçmiş büyük `match` ifadelerini sadeleştirmeye yardımcı olur.

## ⏩ Hata Durumunda Panic için Kısayollar: `unwrap` ve `expect`

`match` kullanmak gayet iyi çalışır, fakat biraz uzun olabilir ve her zaman niyeti açıkça göstermeyebilir. `Result<T, E>` tipi üzerinde, daha özel görevler için tanımlanmış birçok yardımcı metod vardır.

### 🔓 `unwrap` Metodu

`unwrap` metodu, Listing 9-4’te yazdığımız `match` ifadesine benzer şekilde uygulanmış bir kısayoldur.

* Eğer `Result` değeri `Ok` varyantı ise, `unwrap` içindeki değeri döndürür.
* Eğer `Result` değeri `Err` varyantı ise, `unwrap` bizim için `panic!` makrosunu çağırır.

**Dosya adı:** `src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

Eğer `hello.txt` dosyası yoksa bu kodu çalıştırdığımızda, `unwrap` metodunun çağırdığı `panic!`’ten gelen hata mesajını görürüz:

```text
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

---

### 📝 `expect` Metodu

`expect` metodu da `unwrap` gibi çalışır, ancak `panic!` hata mesajını bizim seçmemize imkân tanır. `unwrap` yerine `expect` kullanmak ve açıklayıcı hata mesajları vermek, niyetinizi daha net iletir ve panik kaynağını bulmayı kolaylaştırır.

**Dosya adı:** `src/main.rs`

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

Burada `expect`, dosya tutamacını döndürür veya `panic!` çağırır. Ancak `unwrap`’ın varsayılan hata mesajı yerine, bizim `expect` metoduna parametre olarak verdiğimiz mesaj kullanılır.

Çıktı şu şekilde olur:

```text
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

---

### 💡 Neden `expect` Tercih Edilir?

Üretim kalitesinde (production-quality) kodlarda, Rust geliştiricileri genellikle `unwrap` yerine `expect` kullanır ve işlemin neden her zaman başarılı olmasının beklendiğine dair daha fazla bağlam verir. Böylece, eğer varsayımlarınız yanlış çıkarsa, hata ayıklamak için daha faydalı bilgilere sahip olursunuz.

---

## 📤 Hataları Yaymak (propagating errors)

Bir fonksiyonun içinde başarısız olabilecek bir işlem çağırıldığında, hatayı fonksiyonun içinde işlemek yerine çağıran koda geri döndürebilirsiniz. Bu işleme **hatayı yaymak (propagate)** denir ve kontrolü çağıran koda bırakır. Çağıran kod, bağlam hakkında daha fazla bilgiye sahip olabileceğinden, hatayı nasıl ele alacağına en uygun şekilde kendisi karar verebilir.

### 📂 Dosyadan Kullanıcı Adı Okuma (Listing 9-6)

Aşağıdaki fonksiyon, bir dosyadan kullanıcı adını okur. Eğer dosya yoksa veya okunamıyorsa, hata değeri çağıran koda geri döner.

**Dosya adı:** `src/main.rs`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

Listing 9-6: `match` kullanarak hataları çağıran koda döndüren bir fonksiyon

Bu fonksiyonun dönüş tipi `Result<String, io::Error>`’dir. Yani:

* Başarılı olursa `Ok(String)` döner (dosyadan okunan kullanıcı adı).
* Başarısız olursa `Err(io::Error)` döner (hata bilgisiyle birlikte).

İlk olarak `File::open` çağrılır. Eğer hata olursa `panic!` yerine `return Err(e)` kullanarak hatayı geri döndürürüz. Eğer dosya açıldıysa, `read_to_string` ile içeriği okunur. Bu da hata verebilir, bu yüzden ikinci bir `match` ile işlenir.

Çağıran kod daha sonra bu sonucu ele alır:

* `Ok` alırsa kullanıcı adıyla devam eder.
* `Err` alırsa uygun bir işlem yapar (ör. `panic!` çağırabilir, varsayılan bir kullanıcı adı kullanabilir, ya da kullanıcı adını başka bir kaynaktan alabilir).

---

## ❓ Hataları Yaymak için Kısayol: `?` Operatörü

Hataları yaymak Rust’ta o kadar yaygındır ki, bu işlemi kolaylaştırmak için `?` operatörü sağlanmıştır.

### 🧩 `?` ile Daha Kısa Uygulama (Listing 9-7)

**Dosya adı:** `src/main.rs`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

Listing 9-7: `?` operatörünü kullanarak hataları çağıran koda döndüren bir fonksiyon

`?` operatörü, `match` ifadesiyle yaptığımız işlemin aynısını yapar:

* Eğer değer `Ok` ise, içindeki değer döner ve program devam eder.
* Eğer değer `Err` ise, fonksiyondan erken çıkar ve hatayı çağıran koda döndürür.

Aradaki fark, `?` operatörünün hata türünü `From` trait’i aracılığıyla dönüştürebilmesidir. Böylece fonksiyonunuz, farklı hataları tek bir hata tipinde toplayabilir.

---

### 🔗 Zincirleme Çağrılarla Daha Kısa Yazım (Listing 9-8)

**Dosya adı:** `src/main.rs`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

Listing 9-8: `?` operatöründen sonra metod çağrılarını zincirleme

Burada, `File::open`’ın sonucuna doğrudan `read_to_string` çağrısı ekledik. Kod aynı şekilde çalışır, fakat daha ergonomik görünür.

---

### 📖 Daha da Kısa Yazım: `fs::read_to_string` (Listing 9-9)

Rust’ta bir dosyayı string’e okumak yaygın bir işlem olduğundan, standart kütüphane bunun için kolay bir fonksiyon sunar: `fs::read_to_string`.

**Dosya adı:** `src/main.rs`

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

Listing 9-9: Dosyayı açıp okumak yerine `fs::read_to_string` kullanmak

`fs::read_to_string`, dosyayı açar, yeni bir `String` oluşturur, içeriği okur, string’e koyar ve döndürür. Daha önce uzun haliyle tüm hata yönetimini göstermek için adım adım yazdık, fakat gerçek hayatta bu kısa hali tercih edilebilir.

## ❓ `?` Operatörünün Kullanılabileceği Yerler

`?` operatörü yalnızca dönüş tipi, üzerinde `?` kullanılan değerle uyumlu olan fonksiyonlarda kullanılabilir. Bunun nedeni, `?` operatörünün fonksiyondan erken dönüş yapacak şekilde tanımlanmış olmasıdır. Listing 9-6’daki `match` ifadesinde de gördüğümüz gibi, `Err(e)` döndürülmüştü; bu da fonksiyonun dönüş tipinin `Result` olmasını gerektiriyordu.

### 🚫 `main` Fonksiyonunda `?` Kullanma Hatası (Listing 9-10)

Eğer `main` fonksiyonu `()` dönerken `?` kullanmaya çalışırsak, derleme hatası alırız.

**Dosya adı:** `src/main.rs`

```rust
// This code does not compile!
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

Listing 9-10: `()` döndüren bir `main` fonksiyonunda `?` kullanmaya çalışmak derlenemez.

Derleyici çıktısı:

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:48
  |
3 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
4 |     let greeting_file = File::open("hello.txt")?;
  |                                                ^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
help: consider adding return type
  |
3 ~ fn main() -> Result<(), Box<dyn std::error::Error>> {
4 |     let greeting_file = File::open("hello.txt")?;
5 +     Ok(())
```

Bu hata, `?` operatörünün yalnızca `Result`, `Option` veya `FromResidual` trait’ini implemente eden türleri döndüren fonksiyonlarda kullanılabileceğini söyler.

---

### 🔄 Hatanın Çözümü

Bu hatayı çözmek için iki seçeneğiniz vardır:

1. Fonksiyonun dönüş tipini `Result` veya `Option` gibi uyumlu bir tipe değiştirmek.
2. `match` veya `Result<T, E>` üzerindeki metodları (`unwrap_or_else`, `map_err` vb.) kullanarak hatayı manuel işlemek.

---

### 🟦 `Option<T>` ile `?` Kullanımı (Listing 9-11)

`?` operatörü `Option<T>` üzerinde de kullanılabilir, fakat bu yalnızca fonksiyonun dönüş tipi `Option` olduğunda geçerlidir.

**Örnek:** İlk satırın son karakterini bulan fonksiyon:

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

Listing 9-11: `Option<T>` değeri üzerinde `?` kullanmak

Bu fonksiyon `Option<char>` döner çünkü ilk satırda bir karakter olabilir de olmayabilir de.

* Eğer `text` boş bir string ise, `next()` çağrısı `None` döndürür, `?` da erken dönüş yaparak fonksiyondan `None` döner.
* Eğer `text` boş değilse, `Some(&str)` döner, `?` içteki slice’ı çıkarır, sonra `.chars().last()` çağrısı yapılır.

Sonuç yine bir `Option<char>` olur. Bu, uzun `match` ifadelerine gerek kalmadan, tek satırda sade bir çözüm sağlar.

---

### ⚠️ `Result` ve `Option` Arasında Karışıklık

* `Result` dönen fonksiyonlarda yalnızca `Result` üzerinde `?` kullanabilirsiniz.
* `Option` dönen fonksiyonlarda yalnızca `Option` üzerinde `?` kullanabilirsiniz.
* `?` otomatik olarak `Result` ↔ `Option` dönüşümü yapmaz. Bunun için `ok` veya `ok_or` gibi metotlar kullanılabilir.

---

### ✅ `main` Fonksiyonunda `?` Kullanmak (Listing 9-12)

`main` fonksiyonu özel bir fonksiyondur ve normalde `()` döner. Ancak Rust, `main` fonksiyonunun `Result<(), E>` döndürmesine de izin verir. Bu sayede `main` içinde `?` kullanabilirsiniz.

**Dosya adı:** `src/main.rs`

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

Listing 9-12: `main` fonksiyonunu `Result<(), E>` döndürecek şekilde tanımlamak `?` kullanımına izin verir

Buradaki `Box<dyn Error>` bir trait objesidir ve “her türlü hata” anlamına gelir. Bu sayede `main` fonksiyonunda farklı türde hatalar döndürmek mümkün olur.

* Eğer `Ok(())` dönerse, program **0 çıkış kodu** ile sonlanır (başarılı).
* Eğer `Err` dönerse, program **0 dışında bir kod** ile sonlanır (hata).

Bu davranış, C dilindeki gelenekle uyumludur.

Rust ayrıca, `main` fonksiyonunun dönüş tipi olarak `std::process::Termination` trait’ini implemente eden her türü kabul eder.

---

Bir sonraki adımda, hangi durumlarda `panic!` kullanmanın, hangi durumlarda `Result` döndürmenin daha uygun olduğuna bakacağız.

