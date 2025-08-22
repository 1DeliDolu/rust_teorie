## 📖 Bir Dosya Okuma (Reading a File)

Şimdi `file_path` argümanında belirtilen dosyayı okuyacak işlevselliği ekleyeceğiz. Öncelikle test etmek için örnek bir dosyaya ihtiyacımız var: birkaç satırlık, tekrarlanan kelimeler içeren küçük bir metin dosyası. 12-3 numaralı listede Emily Dickinson’a ait kısa bir şiir var; bu bizim için iyi bir test örneği olacak!

Projenizin kök dizinine `poem.txt` adında bir dosya oluşturun ve içine şu şiiri yazın:

---

### 📄 Dosya Adı: `poem.txt`

```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

12-3: Emily Dickinson’ın şiiri — iyi bir test durumu (test case).

---

Metni hazırladıktan sonra `src/main.rs` dosyasını düzenleyin ve 12-4 numaralı listede gösterildiği gibi dosyayı okumak için kod ekleyin.

---

### 📄 Dosya Adı: `src/main.rs`

```rust
use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
```

12-4: İkinci argümanla belirtilen dosyanın içeriğini okuma

---

Öncelikle, dosya işlemlerini yapabilmek için standart kütüphaneden ilgili kısmı `use` ifadesiyle dahil ediyoruz: `std::fs`.

`main` içinde yeni eklediğimiz `fs::read_to_string` ifadesi, `file_path` ile belirtilen dosyayı açar ve `std::io::Result<String>` tipinde bir değer döndürür. Bu değerde dosyanın içeriği bulunur.

Ardından, dosya okunduktan sonra `contents` değerini ekrana yazdırmak için geçici bir `println!` ekliyoruz. Böylece programın şu ana kadar doğru çalışıp çalışmadığını görebiliyoruz.

---

Aşağıdaki komutla programı çalıştıralım; ilk argüman için herhangi bir dize verebilirsiniz (çünkü henüz arama kısmını uygulamadık), ikinci argüman ise `poem.txt` olacak:

```bash
$ cargo run -- the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

Harika! Kod dosyayı okudu ve içeriğini yazdırdı.

Ancak kodda bazı eksiklikler var:

* Şu anda `main` fonksiyonu birden fazla sorumluluğa sahip. Genel olarak, her fonksiyon yalnızca tek bir işle ilgilendiğinde daha açık ve sürdürülebilir olur.
* Ayrıca, hataları olması gerektiği kadar iyi yönetmiyoruz.

Program hala küçük olduğu için bu eksikler büyük bir sorun değil; fakat program büyüdükçe bunları temiz bir şekilde düzeltmek zorlaşır. Bu yüzden yazılım geliştirirken erken aşamalarda refaktör (refactor) yapmak iyi bir pratiktir. Bunu bir sonraki adımda yapacağız.

