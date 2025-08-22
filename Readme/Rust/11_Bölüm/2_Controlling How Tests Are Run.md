## 🎛️ Testlerin Nasıl Çalıştırılacağını Kontrol Etme (controlling how tests are run)

`cargo run` kodunuzu derleyip ortaya çıkan ikili dosyayı (binary) çalıştırdığı gibi, `cargo test` de kodunuzu test modunda derler ve ortaya çıkan test ikili dosyasını çalıştırır. `cargo test` tarafından üretilen ikili dosyanın varsayılan davranışı, tüm testleri paralel olarak çalıştırmak ve test çalışmaları sırasında üretilen çıktıyı yakalamaktır. Bu, çıktının görüntülenmesini engeller ve test sonuçlarıyla ilgili çıktıyı okumayı kolaylaştırır. Ancak, bu varsayılan davranışı değiştirmek için komut satırı seçeneklerini belirtebilirsiniz.

Bazı komut satırı seçenekleri `cargo test` için geçerlidir, bazıları ise ortaya çıkan test ikili dosyası için geçerlidir. Bu iki tür argümanı ayırmak için önce `cargo test` için olan argümanları, ardından ayırıcı `--` işaretini, ardından test ikili dosyasına giden argümanları yazarsınız. `cargo test --help` çalıştırıldığında `cargo test` ile kullanabileceğiniz seçenekler gösterilir, `cargo test -- --help` çalıştırıldığında ise ayırıcıdan sonraki seçenekler gösterilir. Bu seçenekler ayrıca *rustc kitabının* “Tests” bölümünde de belgelenmiştir.

## 🧵 Testleri Paralel veya Ardışık Çalıştırma (running tests in parallel or consecutively)

Birden fazla test çalıştırdığınızda, varsayılan olarak bunlar **iş parçacıkları (threads)** kullanılarak paralel çalıştırılır; bu, testlerin daha hızlı tamamlanması ve daha çabuk geri bildirim almanız anlamına gelir. Ancak testler aynı anda çalıştığından, testlerinizin birbirine veya paylaşılan herhangi bir duruma (örneğin mevcut çalışma dizini veya ortam değişkenleri gibi paylaşılan bir ortama) bağlı olmadığından emin olmanız gerekir.

Örneğin, her testinizin `test-output.txt` adında bir dosya oluşturup bu dosyaya bazı veriler yazan bir kod çalıştırdığını varsayalım. Daha sonra her test bu dosyadaki verileri okur ve dosyanın belirli bir değeri içerdiğini doğrular, ancak bu değer her testte farklıdır. Testler aynı anda çalıştığından, bir test dosyaya yazıp okumadan önce başka bir test dosyayı üzerine yazabilir. Bu durumda ikinci test başarısız olur; sebep kodun hatalı olması değil, testlerin paralel çalışırken birbirine müdahale etmesidir. Bunun bir çözümü, her testin farklı bir dosyaya yazmasını sağlamak; bir diğer çözüm ise testleri teker teker çalıştırmaktır.

Testleri paralel çalıştırmak istemezseniz veya kullanılan iş parçacığı sayısı üzerinde daha hassas bir kontrol sağlamak isterseniz, `--test-threads` bayrağını ve istediğiniz iş parçacığı sayısını test ikili dosyasına gönderebilirsiniz. Aşağıdaki örneğe bakın:

```bash
$ cargo test -- --test-threads=1
```

Burada test iş parçacığı sayısını `1` olarak ayarladık, yani programa paralellik kullanmamasını söyledik. Testleri tek bir iş parçacığıyla çalıştırmak, paralel çalıştırmaya göre daha uzun sürecektir, ancak testler ortak durumu paylaşıyorsa birbirlerine müdahale etmeyeceklerdir.
## 📤 Fonksiyon Çıktısını Gösterme (showing function output)

Varsayılan olarak, bir test başarılı olursa Rust’ın test kütüphanesi standart çıktıya (standard output) yazdırılan her şeyi yakalar. Örneğin, bir testte `println!` çağırırsak ve test başarılı olursa, terminalde `println!` çıktısını görmeyiz; yalnızca testin geçtiğini belirten satırı görürüz. Bir test başarısız olursa, hata mesajıyla birlikte standart çıktıya yazdırılan her şeyi görürüz.

Aşağıdaki **Listeleme 11-10**, parametresinin değerini yazdırıp `10` döndüren basit bir fonksiyon ve biri başarılı, diğeri başarısız olan iki test içermektedir.

**Dosya adı: src/lib.rs**

```rust
// Bu kod panic oluşturur!
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}
```

👉 Bu kodda `println!` çıktıları yalnızca test başarısız olduğunda görünür.

`cargo test` çalıştırıldığında aşağıdaki çıktı alınır:

```bash
$ cargo test
running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8

thread 'tests::this_test_will_fail' panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: 10
 right: 5
```

Burada dikkat edilirse, başarılı testte yazdırılan `I got the value 4` çıktısını görmüyoruz. Bu çıktı yakalanmıştır. Başarısız testteki `I got the value 8` ise özet kısmında görünmektedir.

Başarılı testlerin yazdırdığı çıktıları da görmek istersek, `--show-output` bayrağını kullanabiliriz:

```bash
$ cargo test -- --show-output
```

Bu şekilde çalıştırıldığında hem başarılı hem de başarısız testlerin çıktıları görünür.

---

## 🎯 Testlerin Bir Alt Kümesini İsme Göre Çalıştırma (running a subset of tests by name)

Bazen tüm testleri çalıştırmak uzun sürebilir. Belirli bir kod parçası üzerinde çalışıyorsanız, yalnızca o kodla ilgili testleri çalıştırmak isteyebilirsiniz. Bunun için `cargo test` komutuna çalıştırmak istediğiniz test(ler)in adını argüman olarak verebilirsiniz.

Bunu göstermek için, aşağıdaki **Listeleme 11-11**’de `add_two` fonksiyonu için üç farklı test oluşturulmuştur:

**Dosya adı: src/lib.rs**

```rust
pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
}
```

👉 Burada üç test üç farklı isimle tanımlanmıştır.

Eğer argüman vermeden `cargo test` çalıştırırsak, daha önce gördüğümüz gibi tüm testler paralel olarak çalıştırılır:

```bash
$ cargo test
running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

👉 Belirli bir test çalıştırmak için `cargo test test_adi` şeklinde kullanabiliriz.
## 🧪 Tekil Testleri Çalıştırma (running single tests)

Herhangi bir test fonksiyonunun adını `cargo test` komutuna vererek yalnızca o testi çalıştırabiliriz:

```bash
$ cargo test one_hundred
```

```text
running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

👉 Burada yalnızca `one_hundred` adlı test çalıştırıldı; diğer iki test bu ada uymadığı için çalıştırılmadı. Çıktının sonunda `2 filtered out` ifadesiyle kaç testin çalıştırılmadığı belirtilir.

Birden fazla test adını bu şekilde veremeyiz; yalnızca ilk değer dikkate alınır. Ancak, birden fazla testi çalıştırmanın başka bir yolu vardır.

---

## 🔎 Birden Fazla Testi Filtreleyerek Çalıştırma (filtering to run multiple tests)

Bir test adının **bir kısmını** belirterek, adı bu değere uyan tüm testleri çalıştırabiliriz. Örneğin, iki testimizin adında `add` geçtiği için şu komutla ikisini birden çalıştırabiliriz:

```bash
$ cargo test add
```

```text
running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

👉 Bu komut, isminde `add` geçen testleri çalıştırdı ve `one_hundred` testini filtreledi.
Ayrıca, testin bulunduğu **modül adı** da test adının bir parçasıdır; dolayısıyla modül adını filtreleyerek o modüldeki tüm testleri çalıştırabiliriz.

---

## 🚫 Bazı Testleri Özellikle İstendiğinde Çalıştırmak (ignoring some tests unless specifically requested)

Bazen bazı testler çok zaman alabilir. Bu testleri çoğu `cargo test` çalıştırmasında hariç tutmak isteyebilirsiniz. Bunun için ilgili testleri `#[ignore]` özniteliği ile işaretleyebiliriz:

**Dosya adı: src/lib.rs**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
```

👉 Burada `expensive_test` fonksiyonu `#[ignore]` ile işaretlenmiştir.

Normal `cargo test` çalıştırıldığında yalnızca `it_works` çalışır, `expensive_test` ise atlanır:

```bash
$ cargo test
running 2 tests
test tests::expensive_test ... ignored
test tests::it_works ... ok
```

Eğer sadece **ignore edilmiş** testleri çalıştırmak istersek:

```bash
$ cargo test -- --ignored
```

Eğer **tüm testleri** (ignore edilmiş olanlar da dahil) çalıştırmak istersek:

```bash
$ cargo test -- --include-ignored
```

👉 Bu şekilde hangi testlerin çalıştırılacağını kontrol ederek, test sonuçlarının hızlı dönmesini sağlayabilir; zamanı geldiğinde uzun süren testleri ayrıca çalıştırabilirsiniz.
