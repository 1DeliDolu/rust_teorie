## 📂 Modülleri Farklı Dosyalara Ayırma (separating modules into different files)

Şimdiye kadar bu bölümdeki tüm örnekler, bir dosya içinde birden fazla modül tanımlıyordu. Modüller büyüdüğünde, tanımlarını ayrı dosyalara taşımak kodun okunabilirliğini artırmak için faydalı olabilir.

Örneğin, çoklu `restaurant` modüllerine sahip Listeleme 7-17’deki koddaki noktadan başlayalım. Tüm modülleri `crate root` (kök dosya) içinde tanımlamak yerine bunları dosyalara ayıracağız. Bu durumda `crate root` dosyası `src/lib.rs`’tir, ancak bu prosedür `crate root` dosyası `src/main.rs` olan `binary crate` (ikili crate) için de geçerlidir.

İlk olarak `front_of_house` modülünü kendi dosyasına ayıracağız. `front_of_house` modülünün süslü parantezleri içindeki kodu kaldırıp yalnızca `mod front_of_house;` bildirimini bırakın, böylece `src/lib.rs` dosyası Listeleme 7-21’de gösterilen kodu içerecektir. Dikkat edin, bu kod `src/front_of_house.rs` dosyasını oluşturana kadar derlenmeyecektir (Listeleme 7-22).

### 📄 Dosya Adı: src/lib.rs

Bu kod derlenmez!

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

👉 `front_of_house` modülünün gövdesinin `src/front_of_house.rs` içinde bulunacağını belirten bildirim.

### 📄 Dosya Adı: src/front\_of\_house.rs

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

👉 `front_of_house` modülünün tanımları bu dosya içinde yapılır.

Dikkat edin, `mod` bildirimiyle bir dosyayı yalnızca bir kez modül ağacınıza yüklemeniz gerekir. Derleyici dosyanın projeye dahil olduğunu ve `mod` ifadesini koyduğunuz yere göre modül ağacında nerede yer aldığını öğrendikten sonra, projedeki diğer dosyalar bu dosyadaki koda, bildirildiği yere giden yolu kullanarak başvurmalıdır. Başka bir deyişle, `mod`, başka programlama dillerinde gördüğünüz bir “include” işlemi değildir.

---

Şimdi `hosting` modülünü kendi dosyasına ayıracağız. Bu süreç biraz farklıdır çünkü `hosting`, `root` modülün değil, `front_of_house` modülünün bir alt modülüdür. Bu nedenle, `hosting` için bir dosyayı, modül ağacındaki atasına göre adlandırılmış yeni bir dizine koyacağız. Bu durumda `src/front_of_house` dizinini oluşturacağız.

Öncelikle `src/front_of_house.rs` dosyasını sadece `hosting` modülünün bildirimiyle değiştirelim:

### 📄 Dosya Adı: src/front\_of\_house.rs

```rust
pub mod hosting;
```

👉 Artık yalnızca `hosting` modülünün bildirimi bulunuyor.

Sonra `src/front_of_house` dizinini oluşturup, `hosting` modülündeki tanımları barındıracak `hosting.rs` dosyasını ekleyelim:

### 📄 Dosya Adı: src/front\_of\_house/hosting.rs

```rust
pub fn add_to_waitlist() {}
```

👉 `hosting` modülündeki fonksiyon tanımları bu dosyada yer alır.

Eğer `hosting.rs` dosyasını `src` dizinine koysaydık, derleyici bu dosyanın kodunu `crate root`’ta tanımlanmış bir `hosting` modülünün parçası olarak beklerdi ve `front_of_house` modülünün alt modülü olarak görmezdi. Derleyicinin hangi dosyaların hangi modüllerin kodunu barındıracağını belirleme kuralları sayesinde dizinler ve dosyalar, modül ağacıyla daha yakından eşleşir.

## 📂 Alternatif Dosya Yolları (alternate file paths)

Şimdiye kadar Rust derleyicisinin kullandığı en yaygın (idiomatic) dosya yollarını ele aldık, fakat Rust daha eski bir dosya yolu stilini de destekler. `crate root` içinde tanımlanmış `front_of_house` adlı bir modül için derleyici modülün kodunu şu dosyalarda arar:

* `src/front_of_house.rs` (daha önce işlediğimiz yöntem)
* `src/front_of_house/mod.rs` (eski stil, hâlâ destekleniyor)

`front_of_house` modülünün alt modülü olan `hosting` için derleyici şu dosyalarda arama yapar:

* `src/front_of_house/hosting.rs` (daha önce işlediğimiz yöntem)
* `src/front_of_house/hosting/mod.rs` (eski stil, hâlâ destekleniyor)

Eğer aynı modül için her iki stili de kullanırsanız, derleyici bir hata verecektir. Aynı projede farklı modüller için her iki stili karışık olarak kullanmak mümkündür, ancak projeyi inceleyen kişiler için kafa karıştırıcı olabilir.

`mod.rs` adını kullanan stilin temel dezavantajı, projede çok sayıda `mod.rs` dosyası bulunabilmesidir. Bunları aynı anda editörde açtığınızda karışıklık yaratabilir.

---

Her modülün kodunu ayrı bir dosyaya taşıdık ve modül ağacı aynı şekilde kaldı. `eat_at_restaurant` içindeki fonksiyon çağrıları, tanımlar farklı dosyalarda bulunsa bile, herhangi bir değişiklik yapmadan çalışır. Bu teknik, modüller büyüdükçe onları yeni dosyalara taşımanıza olanak tanır.

Dikkat edin, `src/lib.rs` dosyasındaki `pub use crate::front_of_house::hosting` ifadesi de değişmedi. Ayrıca `use`, `crate`’in parçası olarak hangi dosyaların derleneceği üzerinde bir etkiye sahip değildir. `mod` anahtar kelimesi modülleri bildirir ve Rust, bu modüle ait kodu aynı ada sahip dosyada arar.

---

## 📝 Özet (summary)

Rust, bir paketi (`package`) birden fazla `crate`’e ve bir `crate`’i modüllere ayırmanıza izin verir. Böylece bir modülde tanımlanmış öğelere başka bir modülden erişebilirsiniz. Bunu **mutlak (absolute)** veya **göreli (relative)** yollar belirterek yapabilirsiniz. Bu yolları `use` ifadesiyle kapsamınıza alabilir ve aynı öğeyi birden çok kez kullanırken daha kısa yol kullanabilirsiniz. Modül kodu varsayılan olarak özeldir, ancak `pub` anahtar kelimesini ekleyerek tanımları herkese açık yapabilirsiniz.

Bir sonraki bölümde, standart kütüphanedeki bazı koleksiyon veri yapılarıyla tanışacak ve bunları düzenli yapılandırılmış kodunuzda nasıl kullanabileceğinizi göreceğiz.
