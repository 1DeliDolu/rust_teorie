## 📦 Anahtarları İlişkili Değerlerle Saklamak (Storing Keys with Associated Values in Hash Maps)

En yaygın koleksiyonlarımızdan (collection) sonuncusu `hash map`’tir. `HashMap<K, V>` tipi, `K` türündeki anahtarları (key) `V` türündeki değerlerle (value) eşleştirerek saklar. Bunu yaparken bir `hashing function` (karma fonksiyonu) kullanır; bu fonksiyon, bu anahtarların ve değerlerin belleğe nasıl yerleştirileceğini belirler. Birçok programlama dili bu tür veri yapısını destekler, ancak genellikle farklı isimler kullanır: `hash`, `map`, `object`, `hash table`, `dictionary` veya `associative array` gibi.

Hash map’ler, verileri bir `indeks` (index) ile değil, herhangi bir türden olabilecek bir `anahtar` (key) ile aramak istediğinizde kullanışlıdır. Örneğin bir oyunda, her takımın skorunu takip etmek için bir hash map kullanabilirsiniz; burada her `anahtar` takımın adı, `değer` ise takımın skorudur. Belirli bir takım adı verildiğinde, o takımın skorunu alabilirsiniz.

Bu bölümde hash map’lerin temel API’sini ele alacağız, ancak standart kütüphanedeki `HashMap<K, V>` üzerinde tanımlanan fonksiyonlarda birçok ek özellik bulunmaktadır. Her zamanki gibi, daha fazla bilgi için standart kütüphane dokümantasyonuna göz atınız.

---

## 🆕 Yeni Bir Hash Map Oluşturmak (Creating a New Hash Map)

Boş bir hash map oluşturmanın bir yolu `new` kullanmak ve `insert` ile öğeler eklemektir. Liste 8-20’de, isimleri Blue ve Yellow olan iki takımın skorlarını takip ediyoruz. Blue takımı 10 puanla, Yellow takımı ise 50 puanla başlıyor.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Liste 8-20: Yeni bir hash map oluşturma ve bazı anahtarlar ile değerler ekleme

Burada dikkat edilmesi gereken nokta, önce standart kütüphanenin `collections` bölümünden `HashMap`’i kullanmamız gerektiğidir. Üç yaygın koleksiyondan bu en az kullanılanıdır, bu nedenle `prelude` içinde otomatik olarak kapsamımıza alınmaz. Ayrıca hash map’lerin standart kütüphane desteği daha azdır; örneğin onları oluşturmak için yerleşik bir `macro` bulunmaz.

Tıpkı `vector` (vektörler) gibi, hash map’ler de verilerini `heap` üzerinde saklar. Bu `HashMap`’in anahtarları `String` türünde, değerleri ise `i32` türündedir. Vektörlerde olduğu gibi hash map’ler de homojendir: tüm anahtarlar aynı türden, tüm değerler de aynı türden olmalıdır.

---

## 🔑 Hash Map İçinde Değerlere Erişim (Accessing Values in a Hash Map)

Bir hash map içinden değer almak için ilgili `anahtarı` `get` metoduna veririz. Bu, Liste 8-21’de gösterilmektedir:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

Liste 8-21: Hash map’te saklanan Blue takımının skoruna erişmek

Burada `score`, Blue takımına karşılık gelen değeri alacak ve sonuç 10 olacaktır. `get` metodu bir `Option<&V>` döndürür; eğer hash map’te bu anahtar için bir değer yoksa `get` `None` döndürür. Bu program `Option`’ı şu şekilde ele alır: önce `copied` çağrılarak `Option<&i32>` yerine `Option<i32>` elde edilir, ardından `unwrap_or` ile eğer `scores` içinde bu anahtar yoksa `score` değeri 0 yapılır.

Hash map üzerinde, tıpkı vektörlerde olduğu gibi, her bir `anahtar-değer çifti` (key-value pair) üzerinde bir `for` döngüsü ile yineleme (iteration) yapabiliriz:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

Bu kod, her bir çifti rastgele bir sırayla yazdırır:

```
Yellow: 50
Blue: 10
```
## 📂 Hash Map’ler ve Sahiplik (Hash Maps and Ownership)

`Copy` özelliğini (`trait`) uygulayan türler (örneğin `i32`) için değerler hash map içine kopyalanır. `String` gibi sahipli (owned) değerlerde ise değerler `move` edilir ve artık hash map bu değerlerin sahibi olur. Bu durum Liste 8-22’de gösterilmektedir:

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name ve field_value bu noktada artık geçersizdir,
// onları kullanmayı deneyin ve derleyici hatasını görün!
```

Liste 8-22: Anahtarlar ve değerler `insert` edildikten sonra hash map’in sahibi olur

`field_name` ve `field_value` değişkenlerini, `insert` çağrısı ile hash map’e taşındıktan (moved) sonra kullanamayız.

Eğer hash map içine değerlere ait `referanslar` (references) eklersek, değerler taşınmaz (moved) ancak hash map içindeki referansların işaret ettiği değerler, hash map geçerli olduğu sürece en az o kadar süreyle geçerli kalmalıdır. Bu konuyu 10. bölümde “Yaşam Süreleri ile Referansların Doğrulanması” (Validating References with Lifetimes) başlığında daha ayrıntılı inceleyeceğiz.

---

## 🔄 Bir Hash Map’i Güncellemek (Updating a Hash Map)

Hash map’teki anahtar-değer çiftlerinin sayısı büyüyebilir, ancak her benzersiz `anahtar` (key) yalnızca bir `değer` (value) ile ilişkilendirilebilir. Tersi durumda bir sorun yoktur; örneğin, hem Blue hem de Yellow takımı `scores` hash map’inde 10 değerine sahip olabilir.

Hash map’teki veriyi değiştirmek istediğinizde, zaten bir değeri olan bir `anahtar` ile karşılaşırsanız nasıl davranacağınıza karar vermeniz gerekir. Şunlardan birini yapabilirsiniz:

* Eski değeri yeni değerle değiştirip eskiyi tamamen yok saymak
* Eski değeri tutup yeniyi görmezden gelmek (yani sadece anahtarın henüz değeri yoksa eklemek)
* Eski değer ile yeni değeri birleştirmek

Şimdi bunların her birine bakalım!

---

## ✏️ Bir Değeri Üzerine Yazmak (Overwriting a Value)

Bir hash map içine aynı `anahtarı` iki kez farklı `değerlerle` eklersek, bu anahtar için saklanan değer yenisiyle değiştirilir. Liste 8-23’teki kod iki kez `insert` çağırsa da, hash map yalnızca bir anahtar-değer çifti içerir, çünkü Blue takımının anahtarı için değer her seferinde yeniden atanır:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}");
```

Liste 8-23: Belirli bir anahtar için saklanan değeri değiştirme

Bu kod şu çıktıyı verecektir:

```
{"Blue": 25}
```

İlk değer olan `10`, yeni değer `25` ile üzerine yazılmıştır.
## ➕ Sadece Anahtar Yoksa Ekleme (Adding a Key and Value Only If a Key Isn’t Present)

Bir hash map’te belirli bir `anahtar`ın zaten mevcut olup olmadığını kontrol etmek yaygın bir durumdur. Eğer anahtar mevcutsa, var olan değer aynı kalmalıdır; eğer anahtar mevcut değilse, o anahtar ve değer eklenmelidir.

Hash map’ler bu durum için özel bir API sağlar: `entry`. Bu metod, kontrol etmek istediğiniz `anahtarı` parametre olarak alır. `entry` metodunun dönüş değeri, değerin mevcut olup olmadığını temsil eden bir `Entry` adında `enum`dur.

Diyelim ki, `Yellow` takımının anahtarı için bir değer var mı kontrol etmek istiyoruz. Eğer yoksa, değer olarak `50` ekleyeceğiz. Aynı şeyi `Blue` takımı için de yapacağız. `entry` API’sini kullanarak kod şu şekilde olur (Liste 8-24):

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{scores:?}");
```

Liste 8-24: `entry` metodunu kullanarak sadece anahtarın değeri yoksa ekleme yapma

`Entry` üzerinde tanımlı `or_insert` metodu, eğer anahtar mevcutsa ilgili değere `mutable reference` (`&mut V`) döndürür. Eğer anahtar mevcut değilse, parametreyi yeni değer olarak ekler ve bu değere bir `mutable reference` döndürür. Bu yöntem, kendi mantığımızı yazmaktan daha temizdir ve ayrıca `borrow checker` ile daha uyumlu çalışır.

Liste 8-24’teki kod şu çıktıyı verecektir:

```
{"Yellow": 50, "Blue": 10}
```

İlk `entry` çağrısı, `Yellow` takımı için anahtar olmadığı için `50` değerini ekler. İkinci `entry` çağrısı hash map’i değiştirmez, çünkü `Blue` takımının zaten `10` değeri vardır.

---

## 🔄 Eski Değere Göre Güncelleme (Updating a Value Based on the Old Value)

Hash map’ler için başka yaygın bir kullanım senaryosu, bir `anahtarın değerini` bulup onu mevcut değerine göre güncellemektir. Örneğin, Liste 8-25’te, bir metinde her kelimenin kaç kere geçtiğini sayan bir kod bulunmaktadır. Burada `anahtar` olarak kelimeler tutulur, `değer` olarak ise o kelimenin kaç kere görüldüğü saklanır. Bir kelime ilk kez görülüyorsa önce `0` değeri eklenir.

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");
```

Liste 8-25: Kelime tekrarlarını saymak için kelimeleri ve sayılarını saklayan hash map kullanımı

Bu kod şu çıktıyı verecektir:

```
{"world": 2, "hello": 1, "wonderful": 1}
```

Aynı anahtar-değer çiftlerini farklı bir sırada görebilirsiniz; hatırlayın ki “Hash Map İçinde Değerlere Erişim” bölümünde belirttiğimiz gibi, hash map üzerinde yineleme (`iteration`) rastgele bir sırada gerçekleşir.

`split_whitespace` metodu, `text` içindeki değeri boşluklara göre böler ve alt dilimler (`subslices`) üzerinde bir `iterator` döndürür.

`or_insert` metodu, belirtilen anahtarın değerine bir `mutable reference` (`&mut V`) döndürür. Burada bu `mutable reference`’ı `count` değişkeninde saklıyoruz. Bu yüzden değere atama yapmak için önce `*` kullanarak `dereference` etmemiz gerekir.

`mutable reference`, `for` döngüsünün sonunda kapsamdan çıkar, bu yüzden tüm bu değişiklikler güvenlidir ve `borrowing` kurallarıyla uyumludur.

## 🔐 Hash Fonksiyonları (Hashing Functions)

Varsayılan olarak, `HashMap` bir `SipHash` adı verilen `hashing function` (karma fonksiyonu) kullanır. Bu fonksiyon, `hash table` (hash tablosu) ile ilgili hizmet reddi saldırılarına (denial-of-service / DoS) karşı direnç sağlayabilir. Bu, mevcut en hızlı `hashing algorithm` değildir; ancak performansın biraz düşmesi karşılığında daha iyi güvenlik elde edilmesi buna değerdir.

Eğer kodunuzu `profiling` yaptığınızda varsayılan `hash function`’ın amacınız için fazla yavaş olduğunu görürseniz, farklı bir `hasher` belirterek başka bir fonksiyona geçebilirsiniz. `Hasher`, `BuildHasher` özelliğini (`trait`) uygulayan bir türdür. `Trait` kavramını ve nasıl uygulanacağını 10. bölümde ele alacağız.

Kendi `hasher`ınızı sıfırdan yazmak zorunda değilsiniz; `crates.io` üzerinde, birçok yaygın `hashing algorithm`’i uygulayan `hasher` sağlayan Rust kullanıcılarının paylaştığı kütüphaneler mevcuttur.

---

## 📋 Özet (Summary)

`Vector`ler, `String`ler ve `HashMap`ler; verileri saklamak, erişmek ve değiştirmek gerektiğinde programlarda ihtiyaç duyulan geniş kapsamlı işlevsellik sağlar. Şimdi şu alıştırmaları çözebilecek donanıma sahipsiniz:

* Bir `integer` (tam sayı) listesi verildiğinde, bir `vector` kullanarak ortanca (median — sıralandığında ortadaki değer) ve tepe değerini (mode — en sık görülen değer; burada `hash map` faydalı olacaktır) döndürün.
* `String`leri Pig Latin’e dönüştürün. Her kelimenin ilk ünsüzü (consonant) kelimenin sonuna taşınır ve sonuna `ay` eklenir (örneğin `first` → `irst-fay`). Ünlü (vowel) ile başlayan kelimelere ise sonuna `hay` eklenir (`apple` → `apple-hay`). UTF-8 kodlaması ile ilgili ayrıntıları unutmayın!
* Bir `hash map` ve `vector`ler kullanarak, bir kullanıcının bir şirketteki bir departmana çalışan eklemesini sağlayan bir metin arayüzü oluşturun. Örneğin: `"Add Sally to Engineering"` veya `"Add Amir to Sales"`. Daha sonra kullanıcıya, bir departmandaki tüm kişileri veya tüm şirketteki kişileri departmanlara göre alfabetik sıralanmış şekilde listeleyin.

Standart kütüphane API dokümantasyonu, bu alıştırmalarda size yardımcı olacak `vector`, `string` ve `hash map` metodlarını açıklar!

---

## ⚠️ Sonraki Konu: Hata Yönetimi (Error Handling)

Artık işlemlerin başarısız olabileceği daha karmaşık programlara giriyoruz. Bu nedenle, hata yönetimini tartışmak için mükemmel bir zamandayız. Bunu bir sonraki bölümde ele alacağız!
