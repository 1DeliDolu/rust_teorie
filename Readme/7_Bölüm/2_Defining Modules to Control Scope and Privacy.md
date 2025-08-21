## 📂 Kapsamı ve Gizliliği Kontrol Etmek için Modülleri Tanımlama (defining modules to control scope and privacy)

Bu bölümde, modüller (modules) ve modül sisteminin diğer parçaları hakkında konuşacağız. Yani, öğeleri adlandırmanıza izin veren yollar (paths), bir yolu kapsama (scope) getiren `use` anahtar kelimesi, öğeleri herkese açık (public) hale getiren `pub` anahtar kelimesi. Ayrıca `as` anahtar kelimesini, harici paketleri (external packages) ve yıldızlı işlemciyi (glob operator) tartışacağız.

---

## 📋 Modüller Hızlı Başvuru (modules cheat sheet)

Modüller (modules) ve yollar (paths) ile ilgili ayrıntılara girmeden önce, modüllerin, yolların, `use` anahtar kelimesinin ve `pub` anahtar kelimesinin derleyicide nasıl çalıştığına ve çoğu geliştiricinin kodunu nasıl organize ettiğine dair hızlı bir referans sunuyoruz. Bu bölüm boyunca her bir kural için örnekler üzerinden geçeceğiz, ancak modüllerin nasıl çalıştığını hatırlamak için burası iyi bir başlangıç noktasıdır.

* Crate kökünden başla (crate root): Bir crate derlenirken, derleyici önce crate kök dosyasında (genellikle kütüphane crate’i için `src/lib.rs` veya ikili crate için `src/main.rs`) derlenecek kodu arar.
* Modül tanımlama (declaring modules): Crate kök dosyasında yeni modüller tanımlayabilirsiniz; örneğin, `mod garden;` ile bir “garden” modülü tanımlarsanız, derleyici modülün kodunu şu yerlerde arar:

  * Süslü parantezler içinde, `mod garden;` ifadesini noktalı virgül yerine süslü parantezler takip ediyorsa
  * `src/garden.rs` dosyasında
  * `src/garden/mod.rs` dosyasında
* Alt modül tanımlama (declaring submodules): Crate kök dosyası dışındaki herhangi bir dosyada alt modüller tanımlayabilirsiniz. Örneğin, `src/garden.rs` içinde `mod vegetables;` tanımlayabilirsiniz. Derleyici alt modülün kodunu, ebeveyn modülün adını taşıyan dizin içinde şu yerlerde arar:

  * `mod vegetables;` ifadesini noktalı virgül yerine süslü parantezler takip ediyorsa
  * `src/garden/vegetables.rs` dosyasında
  * `src/garden/vegetables/mod.rs` dosyasında
* Modüllerdeki koda yollarla erişim (paths to code in modules): Bir modül crate’inizin parçası olduğunda, gizlilik kuralları izin verdiği sürece o modüldeki koda aynı crate’in başka herhangi bir yerinden, kodun yolunu (path) kullanarak erişebilirsiniz. Örneğin, `garden::vegetables` modülündeki `Asparagus` tipi `crate::garden::vegetables::Asparagus` yolunda bulunur.
* Özel vs. herkese açık (private vs. public): Bir modül içindeki kod varsayılan olarak üst modüllerine karşı özeldir (private). Bir modülü herkese açık yapmak için `mod` yerine `pub mod` ile tanımlayın. Herkese açık bir modül içindeki öğeleri de herkese açık yapmak için bildirimlerinden önce `pub` yazın.
* `use` anahtar kelimesi: Bir kapsam (scope) içinde `use` anahtar kelimesi, uzun yolların tekrarını azaltmak için kısayollar oluşturur. Örneğin, `crate::garden::vegetables::Asparagus`’a atıfta bulunabilen herhangi bir kapsamda `use crate::garden::vegetables::Asparagus;` yazabilir ve o kapsamda yalnızca `Asparagus` yazarak bu tipi kullanabilirsiniz.

---

## 🌱 Örnek Crate: backyard

Burada, bu kuralları göstermek için `backyard` adında bir ikili crate (binary crate) oluşturuyoruz. Crate’in dizini, `backyard` olarak adlandırılmıştır ve şu dosya ve dizinleri içerir:

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

Crate kök dosyası bu durumda `src/main.rs`’dir ve şu içeriğe sahiptir:

**Dosya Adı: src/main.rs**

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

👉 Bu kodda `pub mod garden;` satırı derleyiciye `src/garden.rs` içindeki kodu eklemesini söyler.

---

**Dosya Adı: src/garden.rs**

```rust
pub mod vegetables;
```

👉 Burada `pub mod vegetables;` ifadesi, `src/garden/vegetables.rs` içindeki kodun da eklenmesi gerektiği anlamına gelir.

---

**Dosya Adı: src/garden/vegetables.rs**

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

👉 Burada `Asparagus` adında bir `struct` tanımlanmıştır ve `Debug` özelliği ile birlikte herkese açık (`pub`) yapılmıştır.

---

Şimdi bu kuralların ayrıntılarına inelim ve uygulamalarını görelim!

## 📦 İlgili Kodu Modüllerde Gruplama (grouping related code in modules)

Modüller (modules), bir crate içindeki kodu okunabilirlik ve kolay yeniden kullanım için organize etmemizi sağlar. Modüller ayrıca öğelerin gizliliğini (privacy) kontrol etmemize olanak tanır, çünkü bir modül içindeki kod varsayılan olarak özeldir (private). Özel öğeler, dışarıdan kullanıma açık olmayan dahili uygulama ayrıntılarıdır. Modülleri ve içlerindeki öğeleri herkese açık (public) yapmayı seçebiliriz; bu da harici kodların onları kullanmasına ve onlara bağımlı olmasına izin verir.

Örneğin, bir restoranın işlevselliğini sağlayan bir kütüphane crate (library crate) yazalım. Restoranın uygulama ayrıntılarından çok kodun organizasyonuna odaklanmak için fonksiyonların gövdelerini boş bırakıp yalnızca imzalarını tanımlayacağız.

Restoran endüstrisinde, restoranın bazı bölümleri ön alan (front of house) olarak, bazıları ise arka alan (back of house) olarak adlandırılır. Ön alan (front of house), müşterilerin bulunduğu yerdir; bu bölümde resepsiyon görevlileri müşterileri masalara oturtur, garsonlar sipariş ve ödemeleri alır, barmenler içecekleri hazırlar. Arka alan (back of house) ise şeflerin ve aşçıların mutfakta çalıştığı, bulaşıkçıların temizlik yaptığı ve yöneticilerin idari işleri yürüttüğü bölümdür.

Crate’imizi bu şekilde yapılandırmak için fonksiyonlarını iç içe modüller (nested modules) halinde organize edebiliriz. Önce `cargo new restaurant --lib` komutunu çalıştırarak `restaurant` adında yeni bir kütüphane (library) oluşturun. Ardından, bazı modülleri ve fonksiyon imzalarını tanımlamak için A Listing 7-1’deki kodu `src/lib.rs` dosyasına girin; bu kod, ön alan (front of house) bölümünü temsil eder.

---

**Dosya Adı: src/lib.rs**

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

👉 Listing 7-1: Diğer modülleri ve fonksiyonları içeren `front_of_house` modülü

---

Bir modül, `mod` anahtar kelimesi ve ardından modül adı ile tanımlanır (bu durumda `front_of_house`). Modülün gövdesi süslü parantezler içine yazılır. Modüllerin içine başka modüller de yerleştirebiliriz; bu örnekte `hosting` ve `serving` modülleri gibi. Modüller ayrıca `struct`, `enum`, `const`, `trait` gibi diğer öğelerin tanımlarını ve Listing 7-1’de olduğu gibi fonksiyon tanımlarını da içerebilir.

Modülleri kullanarak ilgili tanımları bir arada gruplandırabilir ve neden ilişkili olduklarını isimlendirebilirsiniz. Bu kodu kullanan programcılar, tüm tanımları tek tek okumak zorunda kalmadan gruplara göre gezinerek kendileri için ilgili tanımları daha kolay bulabilirler. Koda yeni işlevsellik ekleyen programcılar da programın düzenli kalması için kodu nereye yerleştireceklerini bilirler.

Daha önce, `src/main.rs` ve `src/lib.rs` dosyalarının crate kökleri (crate roots) olarak adlandırıldığını belirtmiştik. Bunun nedeni, bu iki dosyanın içeriklerinin crate’in modül yapısının kökünde `crate` adında bir modül oluşturmasıdır. Bu yapı modül ağacı (module tree) olarak bilinir.

A Listing 7-2, Listing 7-1’deki yapının modül ağacını göstermektedir.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

👉 Listing 7-2: Listing 7-1’deki kod için modül ağacı

---

Bu ağaç, bazı modüllerin diğer modüllerin içinde nasıl iç içe geçtiğini gösterir; örneğin, `hosting`, `front_of_house` içinde yer alır. Ağaç ayrıca bazı modüllerin kardeş (siblings) olduğunu da gösterir; `hosting` ve `serving`, `front_of_house` içinde tanımlandıkları için kardeş modüllerdir. Eğer A modülü B modülünün içinde tanımlıysa, A modülünün B modülünün çocuğu (child), B modülünün de A modülünün ebeveyni (parent) olduğunu söyleriz. Dikkat edin, tüm modül ağacı, örtük (implicit) `crate` adlı modülün altında köklenmiştir.

Modül ağacı size bilgisayarınızdaki dosya sisteminin dizin ağacını hatırlatabilir; bu benzetme oldukça uygundur! Dosya sistemindeki dizinler gibi, kodunuzu organize etmek için modüller kullanırsınız. Ve tıpkı dizindeki dosyalar gibi, modüllerimizi bulmanın da bir yoluna ihtiyacımız vardır.
