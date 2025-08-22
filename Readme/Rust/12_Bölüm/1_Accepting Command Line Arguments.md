## 🖥️ Komut Satırı Argümanlarını Kabul Etme (Accepting Command Line Arguments)

Her zamanki gibi `cargo new` ile yeni bir proje oluşturalım. Projemizi, sisteminizde zaten yüklü olabilecek `grep` aracından ayırmak için `minigrep` olarak adlandıracağız.

```bash
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

İlk görevimiz, `minigrep`’in iki komut satırı argümanını kabul etmesini sağlamaktır: dosya yolu ve aranacak dize (string). Yani programımızı şu şekilde çalıştırabilmeyi istiyoruz: `cargo run`, ardından iki tire (`--`) ile Cargo’ya değil programımıza ait argümanları belirttiğimizi göstermek, sonra aranacak dize ve içinde arama yapılacak dosya yolu:

```bash
$ cargo run -- searchstring example-filename.txt
```

Şu anda, `cargo new` ile üretilen program kendisine verilen argümanları işleyemez. `crates.io` üzerindeki bazı mevcut kütüphaneler, komut satırı argümanlarını kabul eden bir program yazmaya yardımcı olabilir, ancak bu kavramı yeni öğreniyor olduğunuz için, bu özelliği kendimiz uygulayalım.

---

## 📖 Argüman Değerlerini Okuma (Reading the Argument Values)

`minigrep`’in kendisine iletilen komut satırı argümanlarını okuyabilmesi için Rust standart kütüphanesindeki `std::env::args` fonksiyonuna ihtiyacımız var. Bu fonksiyon, `minigrep`’e iletilen komut satırı argümanlarının bir yineleyicisini (iterator) döndürür.

Yineleyiciler (iterators) konusunu ayrıntılı olarak Chapter 13’te inceleyeceğiz. Şimdilik yalnızca iki detayı bilmeniz yeterlidir:

* Yineleyiciler bir dizi değer üretir.
* Bir yineleyici üzerinde `collect` metodunu çağırarak, üretilen tüm elemanları içeren bir koleksiyona (örneğin `vector`) dönüştürebiliriz.

Aşağıdaki 12-1 numaralı kod parçası, `minigrep` programınızın kendisine iletilen tüm komut satırı argümanlarını okuyup bunları bir `vector` içine toplamasını sağlar.

---

### 📄 Dosya Adı: `src/main.rs`

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

12-1: Komut satırı argümanlarını bir `vector` içine toplama ve yazdırma

---

İlk olarak, `std::env` modülünü `use` ifadesiyle kapsama alanına dahil ediyoruz, böylece `args` fonksiyonunu kullanabiliyoruz. Dikkat edin, `std::env::args` fonksiyonu iki seviye modülün içinde tanımlıdır. Chapter 7’de tartıştığımız gibi, istenen fonksiyon birden fazla modül içinde yer aldığında, sadece fonksiyonu değil, ebeveyn modülü kapsama almak daha uygundur.

Bunu yaparak, `std::env` içindeki diğer fonksiyonları da kolayca kullanabiliriz. Ayrıca, yalnızca `use std::env::args` ekleyip ardından `args` fonksiyonunu çağırmak yerine `env::args` şeklinde kullanmak, daha az belirsizdir. Çünkü `args` ismi, mevcut modülde tanımlanmış olabilecek başka bir fonksiyonla kolayca karıştırılabilir.


## ⚙️ `args` Fonksiyonu ve Geçersiz Unicode (The args Function and Invalid Unicode)

Dikkat edin: `std::env::args`, herhangi bir argüman geçersiz Unicode içerirse panic oluşturur. Eğer programınız geçersiz Unicode içeren argümanları kabul etmek zorundaysa, bunun yerine `std::env::args_os` kullanmalısınız. Bu fonksiyon, `String` yerine `OsString` değerleri üreten bir yineleyici döndürür. Biz burada basitlik adına `std::env::args` kullanmayı tercih ettik çünkü `OsString` değerleri platforma göre farklılık gösterir ve `String`’e kıyasla daha karmaşıktır.

---

`main` fonksiyonunun ilk satırında `env::args` çağırıyoruz ve hemen ardından `collect` ile bu yineleyiciyi, ürettiği tüm değerleri içeren bir `vector`’e dönüştürüyoruz. `collect` fonksiyonunu birçok farklı koleksiyon türü oluşturmak için kullanabiliriz, bu yüzden `args` değişkeninin türünü açıkça belirtiyoruz: bir `string` vektörü istediğimizi söylüyoruz.

Rust’ta türleri açıkça belirtme ihtiyacı nadiren ortaya çıkar, ancak `collect` fonksiyonu bunun istisnalarından biridir çünkü Rust, hangi koleksiyon türünü istediğinizi kendi başına çıkaramaz.

Son olarak, `dbg!` makrosunu kullanarak vektörü yazdırıyoruz. Önce kodu argüman vermeden çalıştıralım, ardından iki argüman ile deneyelim:

```bash
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/minigrep`
[src/main.rs:5:5] args = [
    "target/debug/minigrep",
]
```

```bash
$ cargo run -- needle haystack
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/minigrep needle haystack`
[src/main.rs:5:5] args = [
    "target/debug/minigrep",
    "needle",
    "haystack",
]
```

---

Vektördeki ilk değerin `"target/debug/minigrep"` olduğuna dikkat edin; bu bizim ikili dosyamızın (binary) adıdır. Bu davranış, C dilindeki argüman listesiyle aynıdır ve programların çalıştırıldıkları isimle erişim sağlamasına olanak tanır.

Program adını erişilebilir kılmak genellikle kullanışlıdır: mesajlarda yazdırmak veya programın çağrıldığı komut satırı takma adına (alias) göre davranışını değiştirmek isteyebilirsiniz. Ancak bu bölüm için biz bu değeri yok sayacağız ve yalnızca ihtiyaç duyduğumuz iki argümanı saklayacağız.

## ⚙️ `args` Fonksiyonu ve Geçersiz Unicode (The args Function and Invalid Unicode)

Dikkat edin: `std::env::args`, herhangi bir argüman geçersiz Unicode içerirse panic oluşturur. Eğer programınız geçersiz Unicode içeren argümanları kabul etmek zorundaysa, bunun yerine `std::env::args_os` kullanmalısınız. Bu fonksiyon, `String` yerine `OsString` değerleri üreten bir yineleyici döndürür. Biz burada basitlik adına `std::env::args` kullanmayı tercih ettik çünkü `OsString` değerleri platforma göre farklılık gösterir ve `String`’e kıyasla daha karmaşıktır.

---

`main` fonksiyonunun ilk satırında `env::args` çağırıyoruz ve hemen ardından `collect` ile bu yineleyiciyi, ürettiği tüm değerleri içeren bir `vector`’e dönüştürüyoruz. `collect` fonksiyonunu birçok farklı koleksiyon türü oluşturmak için kullanabiliriz, bu yüzden `args` değişkeninin türünü açıkça belirtiyoruz: bir `string` vektörü istediğimizi söylüyoruz.

Rust’ta türleri açıkça belirtme ihtiyacı nadiren ortaya çıkar, ancak `collect` fonksiyonu bunun istisnalarından biridir çünkü Rust, hangi koleksiyon türünü istediğinizi kendi başına çıkaramaz.

Son olarak, `dbg!` makrosunu kullanarak vektörü yazdırıyoruz. Önce kodu argüman vermeden çalıştıralım, ardından iki argüman ile deneyelim:

```bash
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/minigrep`
[src/main.rs:5:5] args = [
    "target/debug/minigrep",
]
```

```bash
$ cargo run -- needle haystack
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/minigrep needle haystack`
[src/main.rs:5:5] args = [
    "target/debug/minigrep",
    "needle",
    "haystack",
]
```

---

Vektördeki ilk değerin `"target/debug/minigrep"` olduğuna dikkat edin; bu bizim ikili dosyamızın (binary) adıdır. Bu davranış, C dilindeki argüman listesiyle aynıdır ve programların çalıştırıldıkları isimle erişim sağlamasına olanak tanır.

Program adını erişilebilir kılmak genellikle kullanışlıdır: mesajlarda yazdırmak veya programın çağrıldığı komut satırı takma adına (alias) göre davranışını değiştirmek isteyebilirsiniz. Ancak bu bölüm için biz bu değeri yok sayacağız ve yalnızca ihtiyaç duyduğumuz iki argümanı saklayacağız.
