## 🧪 Test Odaklı Geliştirme ile Kütüphane İşlevselliğini Geliştirme (Developing the Library’s Functionality with Test-Driven Development)

Artık arama mantığını `src/lib.rs` dosyasında `main` fonksiyonundan ayrı tuttuğumuza göre, kodumuzun temel işlevselliği için testler yazmak çok daha kolay. Fonksiyonları doğrudan çeşitli argümanlarla çağırabilir ve dönüş değerlerini kontrol edebiliriz; bunun için binary’i komut satırından çağırmamıza gerek yok.

Bu bölümde, test odaklı geliştirme (Test-Driven Development, **TDD**) süreciyle `minigrep` programına arama mantığını ekleyeceğiz. Adımlar şunlardır:

1. Başarısız olacak bir test yazın ve beklediğiniz nedenle başarısız olduğundan emin olun.
2. Yeni testin geçmesini sağlamak için yeterli miktarda kod yazın veya değiştirin.
3. Az önce eklediğiniz veya değiştirdiğiniz kodu yeniden düzenleyin (refactor) ve testlerin geçmeye devam ettiğinden emin olun.
4. 1. adıma geri dönün!

Bu, yazılım geliştirmek için birçok yoldan yalnızca biridir; ancak TDD, kod tasarımını yönlendirmeye yardımcı olabilir. Testi, onu geçirecek kodu yazmadan önce yazmak, süreç boyunca yüksek test kapsamını (test coverage) korumaya yardımcı olur.

Şimdi, dosya içeriğinde arama dizesini (query string) gerçekten arayacak ve eşleşen satırların bir listesini üretecek işlevselliğin implementasyonunu test odaklı olarak geliştireceğiz. Bu işlevselliği `search` adlı bir fonksiyona ekleyeceğiz.
## 🧪 Başarısız Olan Bir Test Yazma (Writing a Failing Test)

`src/lib.rs` dosyasında, Chapter 11’de yaptığımız gibi bir `tests` modülü ekleyeceğiz. Bu test fonksiyonu, `search` fonksiyonundan beklediğimiz davranışı belirler: bir arama sorgusu (**query**) ve aranacak metni alacak, ardından yalnızca sorguyu içeren satırları döndürecektir. Aşağıdaki Liste 12-15 bunu göstermektedir.

---

### 📄 Dosya Adı: `src/lib.rs`

*Bu kod derlenmez!*

```rust
// --snip--

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

**Liste 12-15**: `search` fonksiyonu için istediğimiz işlevselliği test eden başarısız bir test oluşturma

---

Bu test, `"duct"` dizesini arıyor. Aradığımız metin üç satırdan oluşuyor ve yalnızca bir satır `"duct"` içeriyor. (Açılış çift tırnağından sonra gelen ters eğik çizgi (`\`), Rust’a bu `string literal`’ın başına bir satır sonu eklememesini söylüyor.) Test, `search` fonksiyonunun döndürdüğü değerin yalnızca beklediğimiz satırı içerip içermediğini doğruluyor.

Bu testi çalıştırırsak şu anda başarısız olur çünkü `unimplemented!` makrosu `"not implemented"` mesajıyla panikler. TDD ilkelerine uygun olarak küçük bir adım atacağız: paniklememesi için `search` fonksiyonunu, her zaman boş bir vektör döndürecek şekilde tanımlayacağız. Böylece test derlenecek, ancak boş bir vektör ile `"safe, fast, productive."` satırını içeren vektör eşleşmediği için başarısız olacaktır.

---

### 📄 Dosya Adı: `src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

**Liste 12-16**: `search` fonksiyonunu çağrıldığında paniklememesi için yeterli kodu tanımlama

---

Şimdi, neden `search` fonksiyonunun imzasında açıkça bir yaşam süresi (**lifetime**) `'a` belirtmemiz gerektiğini tartışalım.

Chapter 10’da hatırlayacağınız gibi, yaşam süresi parametreleri, hangi argüman yaşam süresinin dönüş değerinin yaşam süresine bağlı olduğunu belirtir. Burada, dönen vektörün, `query` değil `contents` argümanındaki string dilimlerini (`string slices`) referans alacağını söylüyoruz.

Başka bir deyişle, `search` fonksiyonundan dönen verinin, `contents` argümanına verilen veri kadar yaşayacağını Rust’a bildiriyoruz. Bu önemlidir! Bir slice’ın (dilim) referans verdiği verinin geçerli olması gerekir; eğer derleyici, `query` yerine `contents`’ten dilimler aldığımızı yanlış varsayarsa, güvenlik denetimlerini doğru yapamaz.

Eğer yaşam süresi ek açıklamalarını (lifetime annotations) unutursak ve bu fonksiyonu derlemeye çalışırsak, şu hatayı alırız:

```bash
$ cargo build
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:51
  |
1 | pub fn search(query: &str, contents: &str) -> Vec<&str> {
  |                      ----            ----         ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `query` or `contents`
help: consider introducing a named lifetime parameter
  |
1 | pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  |              ++++         ++                 ++              ++
```

Rust, çıktının hangi parametreye bağlı olduğunu bilemez, bu yüzden bunu açıkça belirtmemiz gerekir. Yardım metninde, her iki parametre için de aynı lifetime kullanmamızı öneriyor, ancak bu yanlış olur! Çünkü aramak istediğimiz tüm metin `contents` içinde yer alıyor ve dönen değerler de `contents`’tan alınan satırlar olacak. Bu yüzden yalnızca `contents` parametresi, lifetime sözdizimiyle dönüş değeriyle ilişkilendirilmelidir.

Diğer dillerde imzalarında (signatures) argümanlarla dönüş değerlerini bu şekilde ilişkilendirmeniz gerekmez, ancak Rust’ta bu pratik zamanla kolaylaşacaktır. Bu örneği, Chapter 10’daki “Yaşam Süreleriyle Başvuruları Doğrulama (Validating References with Lifetimes)” bölümündeki örneklerle karşılaştırmak faydalı olabilir.

## 🧩 Testi Geçmek İçin Kod Yazma (Writing Code to Pass the Test)

Şu anda testimiz her zaman boş bir vektör (vector) döndürdüğümüz için başarısız oluyor. Bunu düzeltmek ve `search`’ü uygulamak için programımızın şu adımları izlemesi gerekiyor:

* `contents`’ın her satırını yinelemek (iterate).
* Satırın arama dizgemizi (query string) içerip içermediğini kontrol etmek.
* İçeriyorsa, döndüreceğimiz değerler listesine eklemek.
* İçermiyorsa, hiçbir şey yapmamak.
* Eşleşen sonuçların listesini döndürmek.

Her adımı, satırlar arasında gezinmekle (iterating through lines) başlayarak ele alalım.

---

## 🔡 `lines` Yöntemiyle Satırlar Arasında Gezinme (Iterating Through Lines with the lines Method)

Rust, `lines` adında, `string`’lerde satır-satır yinelemeyi kolayca yapan yardımcı bir yöntem (method) sağlar; 12-17 numaralı listede gösterildiği gibi çalışır. Bunun henüz derlenmeyeceğine dikkat edin.

### 📄 Dosya Adı: `src/lib.rs`

*Bu kod derlenmez!*

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

**Liste 12-17**: `contents` içindeki her satırda yineleme yapmak

`lines` yöntemi bir yineleyici (iterator) döndürür. Yineleyicileri Chapter 13’te ayrıntılı olarak ele alacağız; ancak Chapter 3-5’te bir koleksiyondaki her öğe üzerinde kod çalıştırmak için yineleyiciyle birlikte `for` döngüsü kullandığımız yaklaşımı görmüştünüz.

---

## 🔍 Her Satırda Sorguyu Arama (Searching Each Line for the Query)

Sırada, mevcut satırın arama dizgemizi içerip içermediğini kontrol etmek var. Neyse ki, `string`’lerde bunu yapan yararlı bir yöntem olan `contains` (contains) bulunuyor! 12-18 numaralı listede gösterildiği gibi `search` fonksiyonuna `contains` çağrısı ekleyin. Bunun da henüz derlenmeyeceğini unutmayın.

### 📄 Dosya Adı: `src/lib.rs`

*Bu kod derlenmez!*

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
        }
    }
}
```

**Liste 12-18**: Satırın `query` içindeki dizeyi içerip içermediğini görme işlevini ekleme

Şu anda işlevselliği adım adım oluşturuyoruz. Kodu derlemek için, imzada belirttiğimiz gibi gövdeden bir değer döndürmemiz gerekir.

---

## 🧺 Eşleşen Satırları Depolama (Storing Matching Lines)

Bu fonksiyonu tamamlamak için, döndürmek istediğimiz eşleşen satırları depolayacak bir yola ihtiyacımız var. Bunun için, `for` döngüsünden önce değiştirilebilir (mutable) bir vektör oluşturabilir ve satırı bu vektöre eklemek için `push` (push) metodunu çağırabiliriz. `for` döngüsünden sonra vektörü döndürürüz; 12-19 numaralı listede olduğu gibi.

### 📄 Dosya Adı: `src/lib.rs`

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

**Liste 12-19**: Döndürebilmek için eşleşen satırları depolama

Artık `search` fonksiyonu yalnızca `query`’yi içeren satırları döndürmelidir ve testimiz geçmelidir. Testi çalıştıralım:

```bash
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.22s
     Running unittests src/lib.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 1 test
test tests::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Testimiz geçti, yani çalıştığını biliyoruz!

Bu noktada, testler geçmeye devam ederken aynı işlevselliği koruyarak `search` fonksiyonunun implementasyonunda refaktör fırsatlarını düşünebiliriz. `search` fonksiyonu fena olmasa da, yineleyicilerin (iterators) bazı yararlı özelliklerinden yararlanmıyor. Bu örneğe Chapter 13’te geri dönecek, yineleyicileri ayrıntılı inceleyecek ve nasıl iyileştirebileceğimize bakacağız.

Artık tüm program çalışmalıdır! Önce Emily Dickinson şiirinden tam olarak bir satır döndürmesi gereken bir kelimeyle deneyelim: `frog`.

```bash
$ cargo run -- frog poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/minigrep frog poem.txt`
How public, like a frog
```

Harika! Şimdi de birden fazla satırla eşleşecek bir kelime deneyelim, örneğin `body`:

```bash
$ cargo run -- body poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep body poem.txt`
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

Son olarak, şiirde hiç geçmeyen bir sözcükle arama yaptığımızda satır gelmediğinden emin olalım: `monomorphization`:

```bash
$ cargo run -- monomorphization poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep monomorphization poem.txt`
```

Mükemmel! Klasik bir aracın mini sürümünü oluşturduk ve uygulamaları nasıl yapılandıracağımız konusunda çok şey öğrendik. Ayrıca dosya girdi/çıktısı (file input/output), ömürler (lifetimes), test etme (testing) ve komut satırı ayrıştırma (command line parsing) hakkında da biraz bilgi edindik.

Bu projeyi tamamlamak için, komut satırı programları yazarken yararlı olan ortam değişkenleri (environment variables) ile çalışma ve standart hataya (standard error) yazdırmayı kısaca göstereceğiz.
