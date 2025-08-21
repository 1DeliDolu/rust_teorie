## ✍️ Test Yazma (how to write tests)

Testler (tests), test olmayan kodun (non-test code) beklenen şekilde çalıştığını doğrulayan Rust fonksiyonlarıdır (functions). Test fonksiyonlarının gövdeleri (bodies) genellikle şu üç adımı gerçekleştirir:

* Gerekli veri veya durumu (data or state) hazırla.
* Test etmek istediğiniz kodu çalıştır.
* Sonuçların beklendiğiniz gibi olduğunu doğrula (assert).

Şimdi bu adımları gerçekleştirmek için Rust’ın test yazmaya özel sağladığı özelliklere bakalım. Bunlara `test` özniteliği (attribute), birkaç makro (macros) ve `should_panic` özniteliği dahildir.

## 🧩 Bir Test Fonksiyonunun Anatomisi (the anatomy of a test function)

En basit haliyle, Rust’taki bir test (test), `test` özniteliği (attribute) ile işaretlenmiş bir fonksiyondur (function). Öznitelikler (attributes), Rust kodunun parçaları hakkında meta veridir (metadata); bir örnek, Bölüm 5’te yapılar (structs) ile kullandığımız `derive` özniteliğidir. Bir fonksiyonu test fonksiyonuna dönüştürmek için, `fn` satırının öncesine `#[test]` ekleyin. `cargo test` komutu ile testlerinizi çalıştırdığınızda, Rust test fonksiyonlarını çalıştıran ve her test fonksiyonunun başarılı mı yoksa başarısız mı olduğunu raporlayan bir test çalıştırıcısı (test runner) ikili dosyası oluşturur.

Cargo ile yeni bir kütüphane projesi (library project) oluşturduğumuzda, içerisinde bir test fonksiyonu bulunan bir test modülü (test module) otomatik olarak oluşturulur. Bu modül, testlerinizi yazmak için bir şablon (template) sunar; böylece her yeni projeye başladığınızda doğru yapı ve sözdizimini (syntax) aramak zorunda kalmazsınız. İstediğiniz kadar ek test fonksiyonu ve test modülü ekleyebilirsiniz!

Kodunuzu gerçekten test etmeden önce, şablon test üzerinde deneyler yaparak testlerin nasıl çalıştığının bazı yönlerini inceleyeceğiz. Ardından yazdığımız bazı kodları çağıran ve davranışının doğru olduğunu doğrulayan gerçek dünyadan testler yazacağız.

Şimdi iki sayıyı toplayan `adder` adında yeni bir kütüphane projesi oluşturalım:

```bash
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

`adder` kütüphanenizdeki `src/lib.rs` dosyasının içeriği, Listeleme 11-1’deki gibi olmalıdır:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

👉 Bu kod `cargo new` tarafından otomatik oluşturulur ve `add` fonksiyonunu test edecek örnek bir test içerir.

Şimdilik yalnızca `it_works` fonksiyonuna odaklanalım. Buradaki `#[test]` özniteliğine dikkat edin: bu öznitelik, bunun bir test fonksiyonu olduğunu belirtir, böylece test çalıştırıcısı bu fonksiyonu test olarak değerlendirir. Test modülünde ortak senaryoları kurmaya veya ortak işlemler yapmaya yardımcı olacak test olmayan fonksiyonlarımız da olabilir; bu yüzden hangi fonksiyonların test olduğunu her zaman belirtmeliyiz.

Örnek fonksiyon gövdesi, `assert_eq!` makrosunu (macro) kullanarak `add(2, 2)` çağrısının sonucunun `4` olduğunu doğrular. Bu doğrulama, tipik bir test için biçim örneği niteliğindedir. Bunu çalıştırarak testin geçtiğini görelim.

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running unittests src/lib.rs (target/debug/deps/adder-01ad14159ff659ab)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

👉 Bu çıktı, Cargo’nun testi derleyip çalıştırdığını ve sonucun başarılı olduğunu gösterir.

Özet satırındaki `test result: ok.` tüm testlerin geçtiğini belirtir. Ayrıca `1 passed; 0 failed` gibi rakamlar, kaç testin geçtiğini veya başarısız olduğunu gösterir. Henüz herhangi bir testi görmezden gelmediğimiz için (`ignored`), `0 ignored` görünüyor. Benzer şekilde, herhangi bir filtreleme (`filtered out`) uygulanmadığından `0 filtered out` yazıyor. `0 measured` ise yalnızca nightly Rust’ta kullanılabilen performans ölçüm testleri (benchmark tests) içindir. `Doc-tests` bölümü, API belgelerinde bulunan kod örneklerinin derlenip çalıştırılma sonuçlarını gösterir.

Şimdi testi kendimize göre özelleştirelim. Önce `it_works` fonksiyonunun adını `exploration` olarak değiştirelim:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

👉 Bu sefer çıktı, testin adı `exploration` olarak görünmektedir.

Şimdi başka bir test ekleyelim, ancak bu kez testi başarısız yapacağız! Testler, test fonksiyonu içerisinde bir `panic` oluştuğunda başarısız olur. Her test yeni bir iş parçacığında (thread) çalıştırılır ve ana iş parçacığı, test iş parçacığının çöktüğünü gördüğünde testi başarısız olarak işaretler. Bölüm 9’da en basit panik yolunun `panic!` makrosunu çağırmak olduğunu öğrenmiştik. Yeni testi `another` adıyla ekleyelim:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

👉 Bu test, `panic!` çağrısı nedeniyle kesin olarak başarısız olacaktır.

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::another ... FAILED
test tests::exploration ... ok

failures:

---- tests::another stdout ----

thread 'tests::another' panicked at src/lib.rs:17:9:
Make this test fail
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

👉 Bu sefer çıktı, `tests::another` testinin **FAILED** olduğunu gösteriyor. Ayrıntılı hata mesajında, testin `panic!` nedeniyle başarısız olduğu belirtiliyor. Ayrıca özet kısmında `1 passed; 1 failed` bilgisi yer alıyor.

Artık farklı senaryolarda test çıktılarının nasıl göründüğünü gördünüz. Şimdi testlerde `panic!` dışında faydalı olan diğer makrolara (macros) bakalım.
## ✔️ Sonuçları `assert!` Makrosu ile Kontrol Etme (checking results with the assert! macro)

Standart kütüphane (standard library) tarafından sağlanan `assert!` makrosu (macro), bir testte (test) belirli bir koşulun doğruya (true) değerlendiğinden emin olmak istediğinizde kullanışlıdır. `assert!` makrosuna, Boole (Boolean) değeri döndüren bir ifade veririz. Eğer değer `true` ise hiçbir şey olmaz ve test başarılı olur. Eğer değer `false` ise, `assert!` makrosu `panic!` çağırır ve testin başarısız olmasına neden olur. `assert!` kullanmak, kodumuzun istediğimiz şekilde çalıştığını doğrulamamıza yardımcı olur.

Bölüm 5’te, Listeleme 5-15’te bir `Rectangle` yapısı (struct) ve bir `can_hold` metodu (method) kullanmıştık; bunlar Listeleme 11-5’te tekrar gösterilmektedir. Bu kodu `src/lib.rs` dosyasına yerleştirelim ve ardından `assert!` makrosunu kullanarak bunun için bazı testler yazalım.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

👉 Bu yapı `Rectangle` tanımlar ve `can_hold` metodu bir Boole döndürür.

`can_hold` metodu Boole döndürdüğü için, `assert!` makrosu için mükemmel bir kullanım durumudur. Listeleme 11-6’da, `width` değeri 8 ve `height` değeri 7 olan bir `Rectangle` örneği oluşturup, onun `width` değeri 5 ve `height` değeri 1 olan başka bir `Rectangle` örneğini tutabildiğini doğrulayan bir test yazıyoruz.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

👉 Bu test, büyük dikdörtgenin küçük olanı tutabildiğini doğrular.

Buradaki `use super::*;` satırına dikkat edin. `tests` modülü normal bir modüldür (module) ve Bölüm 7’de ele aldığımız görünürlük kurallarına (visibility rules) uyar. `tests` iç modül (inner module) olduğu için, dış modüldeki (outer module) test edilen kodu içeri almak gerekir. Burada glob (`*`) kullandığımız için dış modülde tanımlanan her şey bu `tests` modülü içinde kullanılabilir.

Testimizin adı `larger_can_hold_smaller` ve ihtiyacımız olan iki `Rectangle` örneğini oluşturduk. Daha sonra `assert!` makrosunu çağırıp `larger.can_hold(&smaller)` ifadesini ona ilettik. Bu ifade `true` döndürmelidir, dolayısıyla test geçmelidir.

```bash
$ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/lib.rs (target/debug/deps/rectangle-6584c4561e48942e)

running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

👉 Test başarıyla geçti.

Şimdi başka bir test ekleyelim, bu sefer küçük bir dikdörtgenin büyük bir dikdörtgeni tutamayacağını doğrulayalım:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
```

👉 Burada `can_hold` fonksiyonunun döndürmesi gereken sonuç `false` olduğu için, sonucu tersleyip (`!`) `assert!` makrosuna aktarıyoruz.

```bash
$ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/lib.rs (target/debug/deps/rectangle-6584c4561e48942e)

running 2 tests
test tests::larger_can_hold_smaller ... ok
test tests::smaller_cannot_hold_larger ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

👉 İki test de geçti.

Şimdi kodumuza bir hata (bug) sokalım. `can_hold` metodunun uygulamasında `>` işaretini `<` ile değiştirelim:

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}
```

👉 Bu kod beklenen davranışı üretmez.

```bash
$ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/lib.rs (target/debug/deps/rectangle-6584c4561e48942e)

running 2 tests
test tests::larger_can_hold_smaller ... FAILED
test tests::smaller_cannot_hold_larger ... ok

failures:

---- tests::larger_can_hold_smaller stdout ----

thread 'tests::larger_can_hold_smaller' panicked at src/lib.rs:28:9:
assertion failed: larger.can_hold(&smaller)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

👉 Bu kez `larger_can_hold_smaller` testi başarısız oldu çünkü `larger.width` (8) `smaller.width`’ten (5) küçük değildir, yani ifade `false` döner. Testlerimiz hatayı yakaladı!
## ⚖️ `assert_eq!` ve `assert_ne!` Makroları ile Eşitliği Test Etme (testing equality with the assert\_eq! and assert\_ne! macros)

Fonksiyonelliği doğrulamanın (verify functionality) yaygın bir yolu, test edilen kodun (code under test) sonucu ile beklenen (expected) değerin eşitliğini test etmektir. Bunu `assert!` makrosuna `==` operatörünü kullanan bir ifade vererek yapabilirsiniz. Ancak bu kadar yaygın bir test için standart kütüphane (standard library), bunu daha kolay yapmamızı sağlayan iki makro sunar: `assert_eq!` ve `assert_ne!`. Bu makrolar sırasıyla iki argümanı eşitlik veya eşitsizlik açısından karşılaştırır. Ayrıca, doğrulama (assertion) başarısız olursa her iki değeri de yazdırırlar; bu da testin neden başarısız olduğunu anlamayı kolaylaştırır. Öte yandan `assert!` makrosu sadece ifadenin `false` olduğunu bildirir, hangi değerlerin bu sonuca yol açtığını göstermez.

Listeleme 11-7’de, parametresine 2 ekleyen `add_two` adlı bir fonksiyon yazıyor ve bunu `assert_eq!` makrosu ile test ediyoruz.

```rust
pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
```

👉 `add_two(2)` sonucunu `result` değişkenine atıyoruz ve `assert_eq!(result, 4)` ile 4’e eşit olup olmadığını test ediyoruz.

```bash
$ cargo test
running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed
```

👉 Test başarılı geçti.

Şimdi bir hata (bug) ekleyelim: `add_two` fonksiyonunun 2 yerine 3 eklemesini sağlayalım:

```rust
pub fn add_two(a: u64) -> u64 {
    a + 3
}
```

Tekrar test çalıştıralım:

```bash
$ cargo test
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at src/lib.rs:12:9:
assertion `left == right` failed
  left: 5
 right: 4
```

👉 Bu çıktıda testin neden başarısız olduğu açıkça görülüyor: sol taraftaki (`left`) değer 5, sağ taraftaki (`right`) değer ise 4. Bu hata mesajı, özellikle çok sayıda test varken hataları ayıklamayı kolaylaştırır.

Bazı dillerde ve test çerçevelerinde parametreler `expected` ve `actual` olarak adlandırılır ve argümanların sırası önemlidir. Ancak Rust’ta isimler `left` ve `right`’tır, sıralama fark etmez. Yani aynı testi şu şekilde de yazabilirdik:

```rust
assert_eq!(4, result);
```

ve hata mesajı yine aynı olurdu.

`assert_ne!` makrosu, verilen iki değer eşit olmadığında başarılı olur; eğer eşitlerse başarısız olur. Bu makro, bir değerin ne olacağını bilmediğimiz ama ne olmayacağını bildiğimiz durumlarda faydalıdır. Örneğin, çıktısı mutlaka girişten farklı olacak bir fonksiyonu test ederken, en güvenilir doğrulama çıktının girdiyle eşit olmamasını kontrol etmektir.

Arka planda, `assert_eq!` ve `assert_ne!` makroları sırasıyla `==` ve `!=` operatörlerini kullanır. Doğrulama başarısız olduğunda, bu makrolar argümanlarını hata ayıklama biçiminde (debug formatting) yazdırır. Bu da karşılaştırılan değerlerin `PartialEq` ve `Debug` trait’lerini uygulamış (implement) olmasını gerektirir. Tüm ilkel türler (primitive types) ve çoğu standart kütüphane türü bu trait’leri uygular. Kendi tanımladığınız struct ve enum türleri için eşitliği test edebilmek adına `PartialEq`, hata durumunda değerleri yazdırabilmek için ise `Debug` trait’lerini uygulamanız gerekir. Bölüm 5’te (Listeleme 5-12) belirtildiği gibi, her iki trait de türetilebilir (derivable) olduğundan genellikle şu özniteliği eklemek yeterlidir:

```rust
#[derive(PartialEq, Debug)]
struct MyType {
    // alanlar
}
```

👉 Böylece hem `assert_eq!` hem de `assert_ne!` makrolarını kendi türleriniz üzerinde de kolayca kullanabilirsiniz.

## 📝 Özel Başarısızlık Mesajları Ekleme (adding custom failure messages)

`assert!`, `assert_eq!` ve `assert_ne!` makrolarına (macros) isteğe bağlı (optional) ek argümanlar vererek hata durumunda özel mesaj (custom message) yazdırabilirsiniz. Zorunlu argümanlardan (required arguments) sonra belirtilen tüm argümanlar, `format!` makrosuna (Bkz. Bölüm 8, “`+` Operatörü ile Birleştirme veya `format!` Makrosu”) aktarılır. Yani süslü parantezler (`{}`) içeren bir biçimlendirme (format string) dizesi ve yerleştirilecek değerler verebilirsiniz. Özel mesajlar, bir doğrulamanın (assertion) ne anlama geldiğini belgelemede faydalıdır; böylece bir test başarısız olduğunda kodda neyin yanlış gittiğini daha iyi anlayabilirsiniz.

Örneğin, insanların adlarını kullanarak selamlayan bir fonksiyonumuz (function) olduğunu ve fonksiyona ilettiğimiz adın çıktı içinde yer aldığını test etmek istediğimizi varsayalım:

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

👉 Bu test, selamlamada verilen ismin geçtiğini doğrular.

Bu programın gereksinimleri henüz kesinleşmedi ve selamlama metninin başındaki `"Hello"` kısmının değişebileceğinden eminiz. Gereksinimler değiştiğinde testi güncellemek istemiyoruz; bu nedenle, `greeting` fonksiyonunun dönüş değeriyle tam eşitlik (exact equality) kontrolü yapmak yerine, çıktının giriş parametresini içerip içermediğini doğrulamayı seçtik.

Şimdi `greeting` fonksiyonunu `name` kısmını hariç bırakacak şekilde değiştirerek bir hata (bug) ekleyelim:

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```

Testi çalıştırdığımızda şu sonucu alırız:

```bash
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at src/lib.rs:12:9:
assertion failed: result.contains("Carol")
```

👉 Bu çıktı sadece doğrulamanın başarısız olduğunu ve hangi satırda gerçekleştiğini gösteriyor. Daha faydalı bir hata mesajı, `greeting` fonksiyonundan dönen değeri de yazdırır.

Özel bir hata mesajı ekleyelim:

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`"
    );
}
```

Tekrar test çalıştıralım:

```bash
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at src/lib.rs:12:9:
Greeting did not contain name, value was `Hello!`
```

👉 Bu sefer çıktıda `greeting` fonksiyonunun döndürdüğü gerçek değeri (`Hello!`) görebiliyoruz. Bu da hatayı ayıklarken, beklediğimiz ile gerçekte olan arasındaki farkı daha net anlamamıza yardımcı olur.


## 💥 `should_panic` ile Panik Kontrolü (checking for panics with should\_panic)

Dönüş değerlerini (return values) kontrol etmenin yanı sıra, kodumuzun hata koşullarını (error conditions) beklediğimiz gibi ele aldığından da emin olmamız gerekir. Örneğin, Bölüm 9’daki Listeleme 9-13’te oluşturduğumuz `Guess` türünü (type) düşünelim. `Guess` kullanan diğer kod parçaları, `Guess` örneklerinin yalnızca 1 ile 100 arasındaki değerleri içereceği garantisine dayanır. Bu nedenle, aralık dışındaki bir değerle `Guess` oluşturmaya çalışıldığında panik (panic) yaşandığını doğrulayan bir test yazabiliriz.

Bunu, test fonksiyonuna `should_panic` özniteliği (attribute) ekleyerek yaparız. Fonksiyon içindeki kod panik oluşturursa test geçer; kod panik oluşturmazsa test başarısız olur.

Listeleme 11-8, `Guess::new` fonksiyonunun hata koşullarının beklendiği gibi panik oluşturduğunu kontrol eden bir testi göstermektedir.

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

👉 Burada `#[should_panic]` özniteliği, test fonksiyonunun panik etmesi gerektiğini belirtir.

```bash
running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed
```

👉 Test başarılı geçti çünkü `Guess::new(200)` gerçekten panik üretti.

Şimdi bir hata (bug) ekleyelim: `new` fonksiyonundan `value > 100` kontrolünü kaldıralım.

```rust
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}
```

Tekrar test çalıştırıldığında:

```bash
running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected
```

👉 Bu mesaj, test fonksiyonundaki kodun beklediğimiz gibi panik etmediğini belirtir.

### 🎯 Daha Hassas `should_panic` Testleri

`should_panic` testleri bazen belirsiz olabilir. Panik, beklediğimizden farklı bir sebepten kaynaklansa bile test yine geçebilir. Daha hassas testler yazmak için `should_panic` özniteliğine `expected` parametresi ekleyebiliriz. Bu parametre, hata mesajının belirli bir alt dizgeyi (substring) içerip içermediğini kontrol eder.

Listeleme 11-9’da, `Guess::new` fonksiyonu değerin küçük ya da büyük olmasına göre farklı panik mesajları üretir:

```rust
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

👉 Bu test geçer çünkü panik mesajı `"less than or equal to 100"` ifadesini içerir.

Şimdi `if` ve `else if` bloklarının gövdelerini yanlışlıkla yer değiştirelim:

```rust
if value < 1 {
    panic!("Guess value must be less than or equal to 100, got {value}.");
} else if value > 100 {
    panic!("Guess value must be greater than or equal to 1, got {value}.");
}
```

Testi tekrar çalıştıralım:

```bash
running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

thread 'tests::greater_than_100' panicked at src/lib.rs:12:13:
Guess value must be greater than or equal to 1, got 200.
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 200."`,
 expected substring: `"less than or equal to 100"`
```

👉 Burada test gerçekten panik etti, ancak panik mesajı beklediğimiz `"less than or equal to 100"` ifadesini içermedi. Bunun yerine `"greater than or equal to 1"` içerdiği için test başarısız oldu. Bu çıktı, hatanın nerede olduğunu bulmamıza yardımcı olur.


## 🧾 Testlerde `Result<T, E>` Kullanma (using Result\<T, E> in tests)

Şimdiye kadar yazdığımız tüm testler başarısız olduklarında `panic` ettiler. Ancak testleri `Result<T, E>` döndürecek şekilde de yazabiliriz! İşte Listeleme 11-1’deki testin, `panic` yerine `Result<T, E>` kullanarak yeniden yazılmış hali:

```rust
#[test]
fn it_works() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

👉 Bu örnekte `it_works` fonksiyonu artık `Result<(), String>` döndürmektedir. Fonksiyonun gövdesinde `assert_eq!` makrosunu çağırmak yerine, test başarılı olduğunda `Ok(())`, başarısız olduğunda ise içinde hata mesajı bulunan bir `Err` döndürürüz.

Testleri `Result<T, E>` döndürecek şekilde yazmak, test gövdesinde `?` (soru işareti) operatörünü kullanabilmenizi sağlar. Bu da, içindeki herhangi bir işlem `Err` döndürdüğünde testin başarısız olması gereken durumları yazmanın uygun bir yoludur.

Ancak, `Result<T, E>` döndüren testlerde `#[should_panic]` özniteliğini kullanamazsınız. Bir işlemin `Err` döndürdüğünü doğrulamak için `?` operatörünü kullanmak yerine şu şekilde kontrol etmelisiniz:

```rust
assert!(value.is_err());
```

👉 Artık test yazmanın birkaç farklı yolunu biliyorsunuz. Şimdi testlerimizi çalıştırdığımızda neler olduğunu ve `cargo test` ile kullanabileceğimiz farklı seçenekleri inceleyelim.

