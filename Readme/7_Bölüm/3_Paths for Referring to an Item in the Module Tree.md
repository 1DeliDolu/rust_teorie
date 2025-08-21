## 📂 Bir Ögeden Modül Ağacında Bahsetmek için Yollar (paths for referring to an item in the module tree)

Rust’a bir ögenin modül ağacında nerede olduğunu göstermek için, dosya sisteminde gezinirken kullandığımız gibi bir yol (path) kullanırız. Bir fonksiyonu çağırmak için yolunu bilmemiz gerekir.

Bir yol iki biçimde olabilir:

* Mutlak yol (absolute path), crate kökünden başlayan tam yoldur; harici bir crate’ten gelen kod için mutlak yol crate adıyla başlar, mevcut crate’teki kod için ise `crate` anahtar sözcüğü ile başlar.
* Göreli yol (relative path), mevcut modülden başlar ve `self`, `super` veya mevcut modüldeki bir tanımlayıcıyı kullanır.

Hem mutlak hem de göreli yollar, çift iki nokta (`::`) ile ayrılmış bir veya daha fazla tanımlayıcıdan oluşur.

Listing 7-1’e dönersek, diyelim ki `add_to_waitlist` fonksiyonunu çağırmak istiyoruz. Bu, şu soruyu sormakla aynıdır: `add_to_waitlist` fonksiyonunun yolu nedir? Listing 7-3, Listing 7-1’in bazı modüller ve fonksiyonlar kaldırılmış hâlidir.

Yeni tanımladığımız, crate kökünde bulunan `eat_at_restaurant` fonksiyonundan `add_to_waitlist` fonksiyonunu çağırmanın iki yolunu göstereceğiz. Bu yollar doğrudur, fakat bu örneğin derlenmesini engelleyecek başka bir sorun vardır. Bunun nedenini birazdan açıklayacağız.

`eat_at_restaurant` fonksiyonu, kütüphane crate’imizin herkese açık API’sinin bir parçasıdır, bu yüzden onu `pub` anahtar sözcüğüyle işaretleriz. Daha fazla detayı “Yolları pub Anahtar Sözcüğü ile Açığa Çıkarmak” bölümünde inceleyeceğiz.

### 📜 Dosya adı: src/lib.rs

Bu kod derlenmez!

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Listing 7-3: `add_to_waitlist` fonksiyonunu mutlak ve göreli yollarla çağırmak

`eat_at_restaurant` fonksiyonunda `add_to_waitlist` fonksiyonunu ilk kez çağırdığımızda, mutlak bir yol kullanıyoruz. `add_to_waitlist` fonksiyonu, `eat_at_restaurant` ile aynı crate’te tanımlandığından, mutlak yola `crate` anahtar sözcüğü ile başlayabiliriz. Daha sonra sırasıyla her modülü ekleriz ve en sonunda `add_to_waitlist` fonksiyonuna ulaşırız. Aynı yapıya sahip bir dosya sistemi hayal edebilirsiniz: `/front_of_house/hosting/add_to_waitlist` yolunu belirtmek, `add_to_waitlist` programını çalıştırır; crate kökünden `crate` ile başlamak, dosya sisteminde `/` ile kökten başlamaya benzer.

`eat_at_restaurant` fonksiyonunda `add_to_waitlist` fonksiyonunu ikinci kez çağırdığımızda, göreli bir yol kullanıyoruz. Yol, `front_of_house` ile başlar; bu, `eat_at_restaurant` ile aynı seviyede tanımlı bir modüldür. Dosya sistemi karşılığı ise `front_of_house/hosting/add_to_waitlist` yolunu kullanmak olur. Bir modül adıyla başlamak, yolun göreli olduğu anlamına gelir.

Mutlak veya göreli yol kullanma kararı, projenize bağlıdır ve bir öge tanımını kullanan koddaki ögeden bağımsız mı yoksa birlikte mi taşımaya daha yatkın olduğunuza bağlıdır. Örneğin, `front_of_house` modülünü ve `eat_at_restaurant` fonksiyonunu `customer_experience` adlı bir modüle taşırsak, `add_to_waitlist` için mutlak yolu güncellememiz gerekir, ancak göreli yol geçerliliğini korur. Ancak `eat_at_restaurant` fonksiyonunu tek başına `dining` adlı bir modüle taşımış olsaydık, `add_to_waitlist` çağrısının mutlak yolu aynı kalırdı, fakat göreli yolun güncellenmesi gerekirdi. Genel olarak tercihimiz mutlak yolları kullanmaktır, çünkü kod tanımlarını ve öge çağrılarını birbirinden bağımsız taşımak daha olasıdır.

Şimdi Listing 7-3’ü derlemeyi deneyelim ve neden henüz derlenmediğini görelim! Karşılaştığımız hatalar Listing 7-4’te gösterilmiştir.

```bash
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
  |                            |
  |                            private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^
```

Listing 7-4: Listing 7-3’teki kodu derlerken oluşan derleyici hataları

Hata mesajları `hosting` modülünün özel (private) olduğunu söylüyor. Yani `hosting` modülü ve `add_to_waitlist` fonksiyonu için doğru yolları yazmış olsak da, Rust bunları kullanmamıza izin vermiyor çünkü özel alanlara erişimimiz yok. Rust’ta tüm ögeler (fonksiyonlar, metotlar, yapılar (structs), enum’lar, modüller ve sabitler), varsayılan olarak ebeveyn modüllere özel (private) tanımlanır. Eğer bir fonksiyon veya struct gibi bir ögeyi gizlemek istiyorsanız, onu bir modül içine koyarsınız.

Bir ebeveyn modül içindeki ögeler, alt modüllerdeki özel ögeleri kullanamaz, fakat alt modüllerdeki ögeler, ataları olan modüllerdeki ögeleri kullanabilir. Bunun nedeni alt modüllerin, içsel uygulama detaylarını sarmalayıp gizlemesi, ancak tanımlandıkları bağlamı görebilmeleridir. Benzetmeye devam edecek olursak, gizlilik kuralları bir restoranın arka ofisine benzer: orada olup bitenler restoran müşterilerine kapalıdır, ancak ofis yöneticileri restoranın içinde olup biten her şeyi görebilir ve yapabilir.

Rust, modül sisteminin bu şekilde işlemesine karar vermiştir, çünkü içsel uygulama detaylarını gizlemek varsayılan davranıştır. Böylece, hangi iç kod bölümlerini değiştirdiğinizde dış kodu bozmayacağınızı bilirsiniz. Ancak Rust size `pub` anahtar sözcüğünü kullanarak alt modüllerin iç kısımlarını dışarıya açma seçeneği de sunar.

## 🔑 Yolları pub Anahtar Sözcüğü ile Açığa Çıkarmak (exposing paths with the pub keyword)

Listing 7-4’te gördüğümüz, `hosting` modülünün özel (private) olduğunu belirten hataya geri dönelim. `eat_at_restaurant` fonksiyonunun, ebeveyn modülde tanımlı olarak, alt modüldeki `add_to_waitlist` fonksiyonuna erişmesini istiyoruz. Bunu sağlamak için, Listing 7-5’te gösterildiği gibi `hosting` modülünü `pub` anahtar sözcüğü ile işaretleriz.

### 📜 Dosya adı: src/lib.rs

Bu kod derlenmez!

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

// -- snip --
```

Listing 7-5: `hosting` modülünü `pub` olarak tanımlamak, `eat_at_restaurant` fonksiyonundan kullanılabilmesini sağlamak için

Ne yazık ki, Listing 7-5’teki kod hâlâ derleyici hataları üretir. Hatalar Listing 7-6’da gösterilmiştir.

```bash
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:10:37
   |
10 |     crate::front_of_house::hosting::add_to_waitlist();
   |                                     ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:13:30
   |
13 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^
```

Listing 7-6: Listing 7-5’teki kodu derlerken oluşan derleyici hataları

Peki ne oldu? `mod hosting` önüne `pub` eklemek modülü herkese açık hâle getirir. Bu değişiklikle, eğer `front_of_house`’a erişebiliyorsak, `hosting` modülüne de erişebiliriz. Ancak `hosting` içindeki ögeler hâlâ özeldir; bir modülü `pub` yapmak, onun içeriğini otomatik olarak `pub` yapmaz. Bir modülde `pub` kullanmak, sadece üst modüllerdeki kodun ona atıfta bulunabilmesini sağlar, iç koduna erişimini değil. Modüller birer kap (container) olduklarından, yalnızca modülü herkese açık yapmak pek işe yaramaz; modül içindeki bir veya birden fazla ögeyi de ayrıca `pub` ile işaretlememiz gerekir.

Listing 7-6’daki hatalar, `add_to_waitlist` fonksiyonunun özel olduğunu söylüyor. Gizlilik kuralları, yalnızca modüllere değil, aynı zamanda `struct`, `enum`, fonksiyonlar ve metotlara da uygulanır.

Şimdi de `add_to_waitlist` fonksiyonunu `pub` anahtar sözcüğü ile işaretleyelim. Bu, Listing 7-7’de gösterilmiştir.

### 📜 Dosya adı: src/lib.rs

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// -- snip --
```

Listing 7-7: `mod hosting` ve `fn add_to_waitlist`’e `pub` eklemek, fonksiyonun `eat_at_restaurant`’tan çağrılmasına izin verir

Artık kod derlenecektir! `pub` eklemenin, gizlilik kuralları açısından bu yolları `eat_at_restaurant` içinde kullanmamıza nasıl izin verdiğini görmek için mutlak ve göreli yolları inceleyelim.

Mutlak yolda, `crate` ile başlarız; bu, crate’in modül ağacının köküdür. `front_of_house` modülü crate kökünde tanımlıdır. `front_of_house` herkese açık olmasa da, `eat_at_restaurant` fonksiyonu ile aynı modülde tanımlandığından (yani `eat_at_restaurant` ve `front_of_house` kardeştir), `front_of_house`’a `eat_at_restaurant` içinden atıfta bulunabiliriz. Sonraki adımda, `pub` ile işaretlenmiş olan `hosting` modülüne ulaşırız. Ebeveyn modülüne erişebildiğimiz için `hosting`’e de erişebiliriz. Son olarak, `pub` ile işaretlenmiş olan `add_to_waitlist` fonksiyonuna ulaşırız ve ebeveynine erişebildiğimiz için bu fonksiyon çağrısı geçerli olur.

Göreli yolda, mantık mutlak yol ile aynıdır, tek fark ilk adımdır: crate kökünden başlamak yerine, yol `front_of_house` ile başlar. `front_of_house` modülü, `eat_at_restaurant` ile aynı modülde tanımlı olduğundan, `eat_at_restaurant`’ın bulunduğu modülden başlayan göreli yol çalışır. Ardından `hosting` ve `add_to_waitlist` `pub` ile işaretlendiği için yolun geri kalanı geçerli olur ve bu fonksiyon çağrısı da doğru çalışır!

Eğer kütüphane crate’inizi başka projelerle paylaşmayı planlıyorsanız, herkese açık API’niz crate kullanıcılarıyla olan sözleşmenizdir ve onların kodunuzla nasıl etkileşim kurabileceklerini belirler. Herkese açık API’nizde değişiklikleri yönetmenin, başka insanların crate’inize bağımlı olmasını kolaylaştırmaya yönelik birçok yönü vardır. Bu kitabın kapsamı dışında kalsa da, bu konuya ilgi duyuyorsanız Rust API Guidelines’a bakabilirsiniz.
## 📦 İkili (binary) ve Kütüphane (library) İçeren Paketler için En İyi Uygulamalar (best practices for packages with a binary and a library)

Bir paketin hem `src/main.rs` ikili crate kökü (binary crate root) hem de `src/lib.rs` kütüphane crate kökü (library crate root) içerebileceğini belirtmiştik ve varsayılan olarak her iki crate de paketle aynı isme sahip olur. Genellikle hem kütüphane hem de ikili crate içeren paketlerde, ikili crate yalnızca kütüphane crate içinde tanımlanmış kodu çağıran bir yürütülebilir (executable) başlatacak kadar kod içerir. Bu sayede, paketin sunduğu en geniş işlevsellikten diğer projeler de yararlanabilir, çünkü kütüphane crate’teki kod paylaşılabilir.

Modül ağacı `src/lib.rs` içinde tanımlanmalıdır. Daha sonra, herhangi bir herkese açık (`pub`) öge, ikili crate içinde paket adını kullanarak çağrılabilir. İkili crate, kütüphane crate’in bir kullanıcısı hâline gelir; tıpkı tamamen harici bir crate’in kütüphane crate’i kullanacağı gibi: yalnızca herkese açık API’yi kullanabilir. Bu yaklaşım iyi bir API tasarlamanıza yardımcı olur; sadece yazar değil, aynı zamanda bir istemcisiniz de!

Bölüm 12’de bu organizasyon yöntemini, hem ikili crate hem de kütüphane crate içeren bir komut satırı programı ile göstereceğiz.

---

## 🪜 super ile Göreli Yolları Başlatmak (starting relative paths with super)

Göreli yolları, mevcut modülden veya crate kökünden değil, üst modülden başlatabiliriz. Bunun için yolun başına `super` yazarız. Bu, dosya sistemi yolunu `..` ile başlatmaya benzer; yani üst dizine gitmek anlamına gelir. `super` kullanmak, üst modülde olduğunu bildiğimiz bir ögeye atıfta bulunmamıza olanak tanır ve bu, modül üst modülle yakından ilişkiliyse ama ileride üst modül başka bir yere taşınacak olursa modül ağacını yeniden düzenlemeyi kolaylaştırır.

Listing 7-8’deki kodu ele alalım: bu, bir şefin yanlış siparişi düzelttikten sonra bizzat müşteriye götürdüğü durumu modelliyor. `back_of_house` modülünde tanımlı `fix_incorrect_order` fonksiyonu, ebeveyn modülde tanımlı `deliver_order` fonksiyonunu çağırır; bunun için yolun başında `super` kullanır.

### 📜 Dosya adı: src/lib.rs

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

Listing 7-8: `super` ile başlayan göreli bir yol kullanarak fonksiyon çağırmak

`fix_incorrect_order` fonksiyonu `back_of_house` modülünde tanımlıdır, bu yüzden `super` kullanarak `back_of_house` modülünün ebeveynine, yani crate köküne gideriz. Oradan `deliver_order` fonksiyonunu ararız ve buluruz. Başarılı! `back_of_house` modülünün ve `deliver_order` fonksiyonunun birbirine bağlı kalacağına ve eğer modül ağacını yeniden düzenlersek birlikte taşınacağına inanıyoruz. Bu nedenle, gelecekte kod başka bir modüle taşınsa bile güncellenecek daha az yer olması için `super` kullandık.

## 🏗️ Yapıları (structs) ve Enum’ları Herkese Açık Yapmak (making structs and enums public)

`pub` anahtar sözcüğünü, yapıları (structs) ve enum’ları herkese açık (public) olarak işaretlemek için de kullanabiliriz, ancak `pub`’un struct ve enum’larla kullanımında birkaç ek ayrıntı vardır. Bir struct tanımının önünde `pub` kullanırsak, struct herkese açık olur, ancak alanları (fields) varsayılan olarak özel (private) kalır. Her bir alanı tek tek `pub` ile işaretleyebiliriz.

Listing 7-9’da, herkese açık `back_of_house::Breakfast` struct’ı tanımladık; burada `toast` alanı public, ancak `seasonal_fruit` alanı private’tir. Bu, bir restoranda müşterinin yemeğiyle birlikte gelecek ekmek türünü seçebilmesini, ancak mevsim ve stok durumuna göre hangi meyvenin verileceğine şefin karar vermesini modellemektedir. Mevcut meyve hızlı değiştiğinden, müşteriler hangi meyvenin geleceğini seçemez veya göremez.

---

### 📜 Dosya adı: src/lib.rs

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}
```

Listing 7-9: Bazı alanları public, bazı alanları private olan bir struct

---

`back_of_house::Breakfast` struct’ındaki `toast` alanı public olduğundan, `eat_at_restaurant` fonksiyonunda bu alana nokta (`.`) notasyonu ile erişebilir ve değerini değiştirebiliriz. Ancak `seasonal_fruit` alanına erişemeyiz, çünkü o private’tir. `seasonal_fruit` alanını değiştiren satırı yoruma açmayı deneyin ve alacağınız hatayı görün!

Ayrıca dikkat edilmesi gereken bir nokta, `back_of_house::Breakfast` içinde private bir alan olduğundan, `Breakfast` struct’ı bir örneğini oluşturmak için public ilişkili (associated) bir fonksiyon sağlamalıdır (burada `summer` adını verdik). Eğer böyle bir fonksiyon tanımlı olmasaydı, `eat_at_restaurant` içinde `Breakfast` örneği oluşturamazdık; çünkü `seasonal_fruit` alanının değerini dışarıdan belirleyemezdik.

---

Buna karşılık, bir enum’u public yaptığımızda, tüm varyantları (variants) da public olur. Bunun için yalnızca enum tanımının önüne `pub` yazmamız yeterlidir. Bu durum Listing 7-10’da gösterilmiştir.

---

### 📜 Dosya adı: src/lib.rs

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

Listing 7-10: Bir enum’u public yapmak, tüm varyantlarını da public yapar

---

`Appetizer` enum’unu public yaptığımız için, `eat_at_restaurant` fonksiyonunda `Soup` ve `Salad` varyantlarını kullanabiliriz.

Enum’lar, varyantları public olmadıkça pek yararlı değildir; her enum varyantını tek tek `pub` ile işaretlemek zahmetli olacağından, varsayılan olarak enum varyantları public’tir. Struct’lar ise, alanları public olmasa da yararlı olabilir; bu nedenle struct alanları varsayılan olarak private’tir, yalnızca `pub` ile işaretlendiğinde public olurlar.

`pub` ile ilgili bir durumu daha ele almadık; bu da modül sisteminin son özelliği olan `use` anahtar sözcüğüdür. Önce `use`’un kendisini ele alacağız, sonra `pub` ve `use`’u nasıl birleştirebileceğimizi göstereceğiz.
