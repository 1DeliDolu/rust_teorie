## 🗂️ Test Organizasyonu (test organization)

Bölümün başında da belirtildiği gibi, test etme karmaşık bir disiplindir ve farklı kişiler farklı terimler ve organizasyon biçimleri kullanır. Rust topluluğu testleri iki ana kategoriye ayırır: **birim testleri (unit tests)** ve **entegrasyon testleri (integration tests)**.

* **Birim testleri (unit tests)** küçük ve daha odaklıdır; her seferinde bir modülü izole ederek test eder ve özel (private) arayüzleri test edebilir.
* **Entegrasyon testleri (integration tests)** kütüphanenizin tamamen dışındadır ve kodunuzu başka herhangi bir dış kodun kullanacağı şekilde kullanır; yalnızca **herkese açık arayüzü (public interface)** kullanır ve her testte birden fazla modülü birlikte çalıştırabilir.

Her iki tür testi de yazmak önemlidir; böylece kütüphanenizin parçalarının hem ayrı ayrı hem de birlikte beklendiği gibi çalıştığından emin olabilirsiniz.

---

## 🧩 Birim Testleri (unit tests)

Birim testlerinin amacı, kodun her bir birimini geri kalan koddan izole ederek test etmektir; böylece kodun nerede beklendiği gibi çalıştığını veya çalışmadığını hızlıca tespit edebilirsiniz.

Birim testlerini, test ettikleri kodla aynı dosyanın içinde, `src` dizininde tutarsınız. Geleneksel olarak, her dosyada test fonksiyonlarını içeren bir `tests` modülü oluşturulur ve bu modül `cfg(test)` ile işaretlenir.

---

## 🏷️ Tests Modülü ve `#[cfg(test)]` (the tests module and #\[cfg(test)])

`tests` modülündeki `#[cfg(test)]` özniteliği, Rust’a test kodunu yalnızca `cargo test` çalıştırıldığında derlemesini ve çalıştırmasını, fakat `cargo build` sırasında derlememesini söyler.

Bu yaklaşım:

* Kütüphaneyi yalnızca derlemek istediğinizde derleme süresinden tasarruf sağlar.
* Ortaya çıkan derlenmiş çıktıda testler dahil edilmediği için yerden tasarruf sağlar.

Entegrasyon testleri farklı bir dizinde bulunduğundan `#[cfg(test)]` özniteliğine ihtiyaç duymaz. Ancak birim testleri aynı dosyada bulunduğundan, derlenmiş sonuca dahil edilmemeleri için `#[cfg(test)]` kullanılmalıdır.

Bu bölümün ilk kısmında yeni bir `adder` projesi oluşturduğumuzda, Cargo bizim için şu kodu üretmişti:

**Dosya adı: src/lib.rs**

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

👉 Burada otomatik oluşturulan `tests` modülünde `cfg` özniteliği, **configuration (yapılandırma)** anlamına gelir ve Rust’a sonraki öğenin yalnızca belirli bir yapılandırma seçeneği verildiğinde dahil edilmesi gerektiğini söyler. Bu durumda, seçenek `test`tir ve Rust tarafından testleri derleyip çalıştırmak için sağlanır.

`cfg` özniteliğini kullanarak, Cargo test kodunu yalnızca `cargo test` ile testleri çalıştırdığımızda derler. Bu, `#[test]` ile işaretlenmiş fonksiyonların yanı sıra bu modül içindeki yardımcı fonksiyonları da kapsar.


## 🔒 Özel Fonksiyonları Test Etme (testing private functions)

Test topluluğunda, özel (private) fonksiyonların doğrudan test edilip edilmemesi gerektiği konusunda bir tartışma vardır. Bazı dillerde özel fonksiyonları test etmek zor veya imkânsızdır. Ancak hangi test yaklaşımını benimserseniz benimseyin, Rust’ın gizlilik kuralları özel fonksiyonları test etmenize izin verir.

Aşağıdaki **Listeleme 11-12**, `internal_adder` adlı özel bir fonksiyonla yazılmış koda bir örnektir:

**Dosya adı: src/lib.rs**

```rust
pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
```

👉 Bu örnekte `internal_adder` fonksiyonu `pub` olarak işaretlenmemiştir. Yani dışarıya açık değildir. Ancak testler de sadece Rust kodudur ve `tests` modülü yalnızca başka bir modüldür.

“Modül Ağacındaki Bir Öğeye Başvuru Yolları” bölümünde tartışıldığı gibi, alt modüllerdeki öğeler üst modüllerdeki öğeleri kullanabilir. Bu testte, `use super::*` ile test modülünün üst modülündeki tüm öğeleri kapsam içine alıyoruz ve bu sayede `internal_adder` fonksiyonu çağrılabiliyor.

Eğer özel fonksiyonların test edilmemesi gerektiğini düşünüyorsanız, Rust sizi bunu yapmaya zorlamaz.


## 🔗 Entegrasyon Testleri (integration tests)

Rust’ta **entegrasyon testleri**, kütüphanenizin tamamen dışında bulunur. Kodunuzu başka herhangi bir dış kodun kullanacağı şekilde kullanırlar; bu da yalnızca kütüphanenizin **herkese açık API’sindeki (public API)** fonksiyonları çağırabilecekleri anlamına gelir. Amaç, kütüphanenizin birçok bölümünün birlikte doğru çalışıp çalışmadığını test etmektir.

Tek başına doğru çalışan kod parçaları, entegre edildiklerinde sorun çıkarabilir. Bu nedenle, entegre kodun test kapsamı da önemlidir. Entegrasyon testleri oluşturmak için önce bir `tests` dizinine ihtiyaç vardır.

---

## 📂 Tests Dizini (the tests directory)

`tests` dizini, proje kök dizininde (`src` klasörünün yanında) oluşturulur. Cargo, entegrasyon testlerini bu dizinde arar. Burada istediğimiz kadar test dosyası oluşturabiliriz ve Cargo her bir dosyayı bağımsız bir crate olarak derler.

Örneğin, **Listeleme 11-12**’deki kod hâlâ `src/lib.rs` dosyasında olsun. Şimdi bir `tests` dizini oluşturalım ve `tests/integration_test.rs` adında yeni bir dosya ekleyelim. Dizinin yapısı şöyle olacaktır:

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

Şimdi, **Listeleme 11-13**’teki kodu `tests/integration_test.rs` dosyasına ekleyelim:

**Dosya adı: tests/integration\_test.rs**

```rust
use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```

👉 Her `tests` dosyası ayrı bir crate olduğundan, kütüphanemizi her test dosyasına ayrıca dahil etmemiz gerekir. Bu nedenle en üste `use adder::add_two;` ekliyoruz.

Burada `#[cfg(test)]` kullanmamıza gerek yoktur. Cargo, `tests` dizinini özel olarak ele alır ve buradaki dosyaları yalnızca `cargo test` çalıştırıldığında derler.

---

## ▶️ Entegrasyon Testlerini Çalıştırma

Şimdi `cargo test` komutunu çalıştıralım:

```bash
$ cargo test
```

Çıktı şu üç bölümü içerir:

1. **Birim testleri (unit tests)** – Daha önce gördüğümüz gibi, her bir test için satır ve özet.
2. **Entegrasyon testleri (integration tests)** – `tests/integration_test.rs` içindeki testlerin sonuçları.
3. **Dokümantasyon testleri (doc tests)** – Dokümanlardaki örnek kodların testleri.

Önemli not: Eğer herhangi bir bölümdeki test başarısız olursa, sonraki bölümler çalıştırılmaz. Örneğin birim test başarısız olursa, entegrasyon ve dokümantasyon testleri çalıştırılmaz.

Her entegrasyon test dosyası ayrı bir bölümde gösterilir. `tests` dizinine daha fazla dosya eklersek, her dosya için ayrı entegrasyon test bölümleri oluşur.

---

## 🎯 Belirli Entegrasyon Testlerini Çalıştırma

Belirli bir **test fonksiyonunu** çalıştırmak için fonksiyon adını `cargo test` komutuna verebiliriz.

Belirli bir **entegrasyon test dosyasındaki tüm testleri** çalıştırmak için `--test` argümanı kullanılır. Örneğin, yalnızca `tests/integration_test.rs` dosyasındaki testleri çalıştırmak için:

```bash
$ cargo test --test integration_test
```

Bu komut yalnızca `tests/integration_test.rs` dosyasındaki testleri çalıştırır.


## 🧩 Entegrasyon Testlerinde Alt Modüller (submodules in integration tests)

Daha fazla entegrasyon testi ekledikçe, bunları `tests` dizininde birden fazla dosya halinde organize etmek isteyebilirsiniz. Örneğin, test fonksiyonlarını test ettikleri işlevselliğe göre gruplayabilirsiniz. Daha önce belirtildiği gibi, `tests` dizinindeki her dosya ayrı bir crate olarak derlenir. Bu, son kullanıcıların kütüphanenizi nasıl kullanacağını daha yakından taklit etmek için faydalıdır. Ancak bu, `tests` dizinindeki dosyaların, `src` dizinindeki dosyalarla aynı davranışı paylaşmadığı anlamına gelir (bkz. Bölüm 7: kodu modüllere ve dosyalara ayırma).

Bu farklılık özellikle birden fazla entegrasyon test dosyasında kullanmak istediğiniz yardımcı fonksiyonlar olduğunda belirginleşir. Bölüm 7’deki “Modülleri Farklı Dosyalara Ayırma” adımlarını izleyerek bunları ortak bir modüle taşımaya çalışırsanız, `tests/common.rs` oluşturup `setup` adında bir fonksiyon eklerseniz, şöyle olur:

**Dosya adı: tests/common.rs**

```rust
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```

Testleri tekrar çalıştırdığınızda, `common.rs` için test çıktısında ayrı bir bölüm görürsünüz, üstelik bu dosya test fonksiyonu içermemesine ve `setup` fonksiyonu hiçbir yerde çağrılmamış olmasına rağmen:

```bash
$ cargo test

Running tests/common.rs (target/debug/deps/common-...)
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored
```

👉 `common` dosyasının “0 test çalıştı” şeklinde görünmesi istediğimiz bir durum değildir. Biz yalnızca diğer entegrasyon test dosyalarıyla kod paylaşmak istemiştik.

---

## 📂 Doğru Yol: `tests/common/mod.rs`

`tests/common.rs` yerine `tests/common/mod.rs` oluşturmalıyız. Proje dizini şöyle görünür:

```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

Bu, Bölüm 7’de “Alternate File Paths” başlığı altında bahsedilen eski adlandırma kuralıdır. Dosyayı bu şekilde adlandırmak, Rust’a `common` modülünü bir entegrasyon test dosyası olarak değil, bir modül olarak değerlendirmesini söyler.

Böylece `tests/common.rs` silinir, `setup` fonksiyonu `tests/common/mod.rs` içine taşınır ve test çıktısında artık `common` bölümü görünmez. Çünkü `tests` dizinindeki **alt dizinlerdeki dosyalar**, bağımsız crate olarak derlenmez ve test çıktısında kendi bölümlerine sahip olmazlar.

---

## ▶️ Ortak Modülün Kullanılması

Artık `tests/common/mod.rs` içindeki fonksiyonları herhangi bir entegrasyon test dosyasında kullanabiliriz. Örneğin `tests/integration_test.rs` dosyasında `setup` fonksiyonunu çağırmak için:

**Dosya adı: tests/integration\_test.rs**

```rust
use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
```

👉 Buradaki `mod common;` bildirimi, **Listeleme 7-21**’de gösterdiğimiz modül bildirimine eşdeğerdir. Daha sonra test fonksiyonu içinde `common::setup()` çağrısı yapabiliriz.

## 📦 İkili Crate’ler için Entegrasyon Testleri (integration tests for binary crates)

Eğer projemiz yalnızca `src/main.rs` dosyası içeren bir **ikili crate (binary crate)** ise ve `src/lib.rs` dosyası yoksa, `tests` dizininde entegrasyon testleri oluşturup `src/main.rs` dosyasında tanımlı fonksiyonları `use` ifadesiyle kapsam içine alamayız. Çünkü yalnızca **kütüphane crate’leri (library crates)** diğer crate’lerin kullanabileceği fonksiyonları dışa açar; ikili crate’ler ise tek başına çalıştırılmak üzere tasarlanmıştır.

Bu nedenle Rust projelerinde ikili bir çıktı sağlayacaksa genellikle şöyle bir yapı izlenir:

* `src/main.rs` dosyası basit tutulur,
* asıl mantık `src/lib.rs` dosyasında yer alır.

Bu yapı sayesinde entegrasyon testleri, `use` ile kütüphaneyi kullanabilir ve önemli işlevselliği test edebilir. Eğer kütüphanedeki önemli işlevsellik çalışıyorsa, `src/main.rs` dosyasındaki küçük miktardaki kod da doğru çalışacaktır. Dolayısıyla `src/main.rs`’deki bu küçük kod parçasının ayrıca test edilmesine gerek yoktur.

---

## 📝 Özet (summary)

Rust’ın test özellikleri, kodun nasıl çalışması gerektiğini belirtmenin ve kodda değişiklikler yapsanız bile beklendiği gibi çalışmaya devam ettiğinden emin olmanın bir yolunu sunar.

* **Birim testleri (unit tests)** kütüphanenin farklı bölümlerini ayrı ayrı test eder ve özel (private) uygulama ayrıntılarını da sınayabilir.
* **Entegrasyon testleri (integration tests)** ise kütüphanenin birçok bölümünün birlikte doğru çalışıp çalışmadığını kontrol eder ve kütüphanenin herkese açık API’sini kullanarak kodu dışarıdan gelen bir kod gibi test eder.

Rust’ın **tip sistemi (type system)** ve **sahiplik kuralları (ownership rules)** bazı hata türlerini engellese de, testler hâlâ mantıksal hataları azaltmak ve kodunuzun beklenen şekilde davrandığından emin olmak için çok önemlidir.

Şimdi, bu bölümde ve önceki bölümlerde öğrendiğiniz bilgileri birleştirerek bir projede uygulayalım!
