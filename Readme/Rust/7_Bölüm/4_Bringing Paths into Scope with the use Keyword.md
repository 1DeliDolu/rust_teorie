## 📂 use Anahtar Kelimesi ile Yolları (paths) Scope’a Dahil Etme

Fonksiyonları çağırmak için yolları (paths) tekrar tekrar yazmak zahmetli ve tekrarlı gelebilir. Örneğin Listeleme 7-7’de, `add_to_waitlist` fonksiyonunu çağırmak için ister mutlak (absolute) ister göreceli (relative) yolu seçelim, her seferinde `front_of_house` ve `hosting` belirtmek zorunda kaldık. Neyse ki bu süreci basitleştirmek için bir yol vardır: `use` anahtar kelimesini bir defa kullanarak bir yola kısayol oluşturabiliriz ve daha sonra bu scope içerisinde kısaltılmış ismi kullanabiliriz.

Listeleme 7-11’de, `crate::front_of_house::hosting` modülünü `eat_at_restaurant` fonksiyonunun scope’una getiriyoruz, böylece artık `hosting::add_to_waitlist` yazarak fonksiyonu çağırabiliyoruz.

### 📄 Dosya Adı: src/lib.rs

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

👉 Listeleme 7-11: `use` ile bir modülün scope’a dahil edilmesi

Bir scope’a `use` ve yol eklemek, dosya sisteminde sembolik bağlantı (symbolic link) oluşturmaya benzer. `use crate::front_of_house::hosting` satırını crate köküne ekleyerek `hosting` artık bu scope’ta geçerli bir isim olur; sanki `hosting` modülü doğrudan crate kökünde tanımlanmış gibi. `use` ile scope’a getirilen yollar, diğer tüm yollar gibi gizlilik (privacy) kurallarına tabidir.

Dikkat edilmesi gereken nokta şudur: `use` sadece bulunduğu scope için kısayol oluşturur. Listeleme 7-12’de, `eat_at_restaurant` fonksiyonu `customer` adlı yeni bir alt modüle taşınmıştır. Bu modül, `use` ifadesinin bulunduğu scope’tan farklı bir scope olduğundan, fonksiyon gövdesi derlenmeyecektir.

### 📄 Dosya Adı: src/lib.rs

Bu kod derlenmez!

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

👉 Listeleme 7-12: `use` ifadesi yalnızca bulunduğu scope’ta geçerlidir.

Derleyici hatası, kısayolun artık `customer` modülü içinde geçerli olmadığını gösterir:

```bash
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`
   |
help: consider importing this module through its public re-export
   |
10 +     use crate::hosting;
   |

warning: unused import: `crate::front_of_house::hosting`
 --> src/lib.rs:7:5
  |
7 | use crate::front_of_house::hosting;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` (lib) due to 1 previous error; 1 warning emitted
```

Dikkat ederseniz ayrıca `use` ifadesinin artık bulunduğu scope’ta kullanılmadığına dair bir uyarı da vardır! Bu problemi çözmek için `use` ifadesini `customer` modülünün içine taşıyabilir veya alt modül `customer` içerisinden `super::hosting` kullanarak üst modüldeki kısayola erişebilirsiniz.

## 🧭 İdiomatik use Yolları (idiomatic use paths) Oluşturma

Listeleme 7-11’de, neden `use crate::front_of_house::hosting` yazıp `eat_at_restaurant` içinde `hosting::add_to_waitlist` çağırdığımızı, bunun yerine doğrudan `add_to_waitlist` fonksiyonunu scope’a getirmediğimizi merak etmiş olabilirsiniz. Bunun alternatifi Listeleme 7-13’te gösterilmiştir.

### 📄 Dosya Adı: src/lib.rs

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

👉 Listeleme 7-13: `add_to_waitlist` fonksiyonunu doğrudan `use` ile scope’a getirmek (idiomatik olmayan kullanım)

Her iki yaklaşım da aynı işi yapar, ancak `use` ile bir fonksiyonu scope’a getirmenin idiomatik yolu Listeleme 7-11’deki gibidir. Fonksiyonun bağlı olduğu üst modülü scope’a `use` ile getirmek, fonksiyonu çağırırken üst modülün adını da belirtmeyi gerektirir. Bu, fonksiyonun yerel olarak tanımlanmadığını açıkça gösterir ve tam yolun tekrarını azaltır. Listeleme 7-13’teki kod ise `add_to_waitlist`’in nerede tanımlandığını belirsiz kılar.

Öte yandan, `struct`, `enum` ve diğer öğeleri `use` ile scope’a getirirken idiomatik olan tam yolu belirtmektir. Listeleme 7-14, standart kütüphanedeki `HashMap` yapısının (struct) binary crate’in scope’una idiomatik olarak nasıl getirileceğini göstermektedir.

### 📄 Dosya Adı: src/main.rs

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

👉 Listeleme 7-14: `HashMap`’i scope’a idiomatik şekilde getirme

Bunun arkasında güçlü bir teknik gerekçe yoktur; bu, Rust topluluğunda oluşmuş ve alışılmış bir yazım geleneğidir.

Bu idiomun bir istisnası vardır: aynı ada sahip iki öğeyi `use` ile scope’a getirmek istediğimizde Rust buna izin vermez. Listeleme 7-15, aynı ada sahip fakat farklı üst modüllerden gelen iki `Result` tipini scope’a getirmenin ve onları ayırt etmenin yolunu göstermektedir.

### 📄 Dosya Adı: src/lib.rs

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

👉 Listeleme 7-15: Aynı ada sahip iki tipi aynı scope’a getirmek için üst modüllerin kullanılması gerekir.

Gördüğünüz gibi, üst modülleri kullanmak iki farklı `Result` tipini ayırt etmeyi sağlar. Eğer `use std::fmt::Result` ve `use std::io::Result` yazsaydık, aynı scope’ta iki `Result` olurdu ve Rust hangi `Result`’un kastedildiğini anlayamazdı.

## 🏷️ as Anahtar Kelimesi ile Yeni İsimler Vermek

Aynı ada sahip iki türü `use` ile aynı scope’a getirme sorununa başka bir çözüm daha vardır: yolun (path) ardından `as` kullanarak türe yeni bir yerel isim (alias) verebiliriz. Listeleme 7-16, Listeleme 7-15’teki kodun bir alternatifini gösterir; burada iki `Result` tipinden biri `as` ile yeniden adlandırılmıştır.

### 📄 Dosya Adı: src/lib.rs

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

👉 Listeleme 7-16: Bir tipi scope’a getirirken `as` anahtar kelimesi ile yeniden adlandırma

İkinci `use` ifadesinde, `std::io::Result` için yeni bir isim olarak `IoResult` seçtik; bu, aynı zamanda scope’a getirdiğimiz `std::fmt::Result` ile çakışmayacaktır. Hem Listeleme 7-15 hem de Listeleme 7-16 idiomatik kabul edilir, dolayısıyla tercih size kalmıştır.

---

## 🔁 pub use ile İsimleri Yeniden Dışa Aktarma (re-exporting)

`use` ile bir ismi scope’a getirdiğimizde, bu isim yalnızca o scope içinde geçerli olur. Eğer bu isme, sanki o scope içinde tanımlanmış gibi dışarıdaki kodların da erişmesini istiyorsak, `pub` ve `use`’u birlikte kullanabiliriz. Bu tekniğe yeniden dışa aktarma (re-exporting) denir, çünkü öğeyi hem kendi scope’umuza alır hem de başkalarının scope’una alabilmesini sağlarız.

Listeleme 7-17, Listeleme 7-11’deki kodun, kök modüldeki `use` ifadesi `pub use` ile değiştirilmiş halini göstermektedir.

### 📄 Dosya Adı: src/lib.rs

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

👉 Listeleme 7-17: `pub use` ile bir ismi yeni bir scope’tan herkesin kullanımına açma

Bu değişiklikten önce, harici kod `add_to_waitlist` fonksiyonunu çağırmak için şu yolu kullanmak zorundaydı:
`restaurant::front_of_house::hosting::add_to_waitlist()`. Bunun için ayrıca `front_of_house` modülünün `pub` olarak işaretlenmesi gerekiyordu. Artık `pub use` ile `hosting` modülü kök modülden yeniden dışa aktarıldığı için, harici kod şu yolu kullanabilir:
`restaurant::hosting::add_to_waitlist()`.

Yeniden dışa aktarma, kodunuzun iç yapısı, kodunuzu kullanan programcıların alanı (domain) nasıl düşündüklerinden farklı olduğunda faydalıdır. Örneğin, bu restoran benzetmesinde, restoranı işleten kişiler “ön taraf” (`front of house`) ve “arka taraf” (`back of house`) olarak düşünür. Ancak restorana gelen müşteriler muhtemelen restoranın bölümlerini bu terimlerle düşünmezler. `pub use` ile kodumuzu bir yapıda yazabilir, ancak dışarıya farklı bir yapı sunabiliriz. Bu yaklaşım, kütüphane üzerinde çalışan programcılar için düzenli bir yapı sağlarken, kütüphaneyi kullanan programcılar için de anlaşılır bir API sunar.

`pub use` ile başka bir örneği ve bunun crate dokümantasyonunuza etkilerini Bölüm 14’teki “Exporting a Convenient Public API with pub use” kısmında inceleyeceğiz.

## 📦 Harici Paketleri (external packages) Kullanma

Bölüm 2’de, rastgele sayılar elde etmek için `rand` adlı harici bir paket kullanan bir tahmin oyunu projesi yazmıştık. `rand`’i projemizde kullanmak için `Cargo.toml` dosyasına şu satırı eklemiştik:

### 📄 Dosya Adı: Cargo.toml

```toml
rand = "0.8.5"
```

`Cargo.toml` dosyasına `rand` bağımlılığını eklemek, Cargo’ya `rand` paketini ve onun bağımlılıklarını **crates.io**’dan indirmesini ve projede kullanılabilir hale getirmesini söyler.

Daha sonra, `rand` tanımlarını paketimizin scope’una getirmek için, crate adı `rand` ile başlayan bir `use` satırı ekledik ve scope’a almak istediğimiz öğeleri listeledik. Bölüm 2’deki “Generating a Random Number” kısmını hatırlarsanız, `Rng` trait’ini scope’a alıp `rand::thread_rng` fonksiyonunu çağırmıştık:

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

Rust topluluğu, **crates.io**’da birçok paket sunmuştur ve bunları paketimize dahil etmenin yolu hep aynıdır:

* Paketleri `Cargo.toml` dosyanıza eklemek
* `use` ile ilgili crate’ten öğeleri scope’a almak

Unutmayın ki standart kütüphane (`std`) da paketimize harici olan bir crate’tir. Ancak Rust diliyle birlikte geldiği için `Cargo.toml`’a eklememiz gerekmez. Yine de oradaki öğeleri scope’a almak için `use` kullanmamız gerekir. Örneğin, `HashMap` için şu satırı ekleriz:

```rust
use std::collections::HashMap;
```

Bu, `std` ile başlayan mutlak bir yoldur (absolute path).

---

## 🧹 Büyük use Listelerini Temizlemek için İç İçe Yollar (nested paths) Kullanma

Aynı crate veya modülde tanımlanan birden fazla öğeyi kullanıyorsak, her öğeyi ayrı satırlarda yazmak dosyamızda çok yer kaplayabilir. Örneğin, Listeleme 2-4’teki tahmin oyununda `std`’dan öğeleri şu şekilde scope’a almıştık:

### 📄 Dosya Adı: src/main.rs

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

Bunun yerine, aynı öğeleri tek satırda iç içe yollar (nested paths) kullanarak scope’a alabiliriz. Bunun için yolun ortak kısmını yazıp, ardından çift iki nokta (`::`) ve süslü parantezler `{}` içine farklı kısımları yazarız. Listeleme 7-18’de gösterildiği gibi:

### 📄 Dosya Adı: src/main.rs

```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

👉 Listeleme 7-18: Aynı öneke (prefix) sahip birden fazla öğeyi tek satırda scope’a almak için iç içe yol kullanma

Daha büyük programlarda, aynı crate veya modülden birçok öğeyi scope’a almak gerektiğinde, iç içe yollar kullanmak `use` ifadelerinin sayısını ciddi ölçüde azaltabilir.

İç içe yolları yolun herhangi bir seviyesinde kullanabiliriz. Bu, ortak alt yolları paylaşan iki `use` ifadesini birleştirmek için kullanışlıdır. Örneğin, Listeleme 7-19’da biri `std::io`, diğeri `std::io::Write`’i scope’a alan iki ayrı `use` ifadesi vardır.

### 📄 Dosya Adı: src/lib.rs

```rust
use std::io;
use std::io::Write;
```

👉 Listeleme 7-19: Biri diğerinin alt yolu olan iki `use` ifadesi

Bu iki yolun ortak kısmı `std::io`’dur ve bu zaten ilk `use` ifadesinde tamamen yazılmıştır. Bunları tek bir satırda birleştirmek için iç içe yolda `self` kullanabiliriz, Listeleme 7-20’de olduğu gibi:

### 📄 Dosya Adı: src/lib.rs

```rust
use std::io::{self, Write};
```

👉 Listeleme 7-20: Listeleme 7-19’daki yolları tek `use` ifadesinde birleştirme

Bu satır hem `std::io`’yu hem de `std::io::Write`’ı scope’a alır.
## ✨ Glob Operatörü (glob operator)

Eğer bir yol (path) altında tanımlı tüm public öğeleri scope’a almak istersek, bu yolu `*` glob operatörü ile birlikte kullanabiliriz:

```rust
use std::collections::*;
```

👉 Bu `use` ifadesi, `std::collections` altında tanımlı tüm public öğeleri mevcut scope’a getirir.

---

Glob operatörünü kullanırken dikkatli olmak gerekir!

* Glob, hangi isimlerin scope’a dahil olduğunu ve programınızda kullanılan bir ismin nerede tanımlandığını anlamayı zorlaştırabilir.
* Ayrıca, bağımlılığınızın tanımları değişirse, import ettiğiniz öğeler de değişir. Bu durum, örneğin bağımlılık sizinle aynı scope’ta aynı ada sahip yeni bir tanım eklerse, bağımlılığı güncellediğinizde derleyici hatalarına yol açabilir.

---

Glob operatörü genellikle test yazarken kullanılır; test edilen her şeyi `tests` modülüne dahil etmek için. Bunun detaylarını Bölüm 11’deki **“How to Write Tests”** kısmında inceleyeceğiz.

Ayrıca, glob operatörü bazen **prelude deseni (prelude pattern)** kapsamında da kullanılır; bu desenle ilgili daha fazla bilgi için standart kütüphane dokümantasyonuna bakabilirsiniz.
