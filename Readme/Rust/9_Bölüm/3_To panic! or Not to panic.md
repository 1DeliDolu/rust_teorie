## 🤔 `panic!` mı Yoksa `Result` mü?

Peki, ne zaman `panic!` çağırmalı, ne zaman `Result` döndürmelisiniz?

Bir kod paniklediğinde, toparlanma (recovery) imkânı kalmaz. Aslında her hata durumunda `panic!` çağırabilirsiniz, fakat bu durumda çağıran kod adına karar vererek hatanın kurtarılamaz olduğuna hükmetmiş olursunuz. Oysa `Result` döndürmeyi seçtiğinizde, çağıran koda seçenek sunmuş olursunuz. Çağıran kod:

* Duruma uygun şekilde hatadan kurtulmayı deneyebilir,
* Veya `Err`’i kurtarılamaz görüp kendisi `panic!` çağırarak hatayı kurtarılamaz hale dönüştürebilir.

Bu nedenle, başarısız olabilecek bir fonksiyon tanımlarken `Result` döndürmek genellikle **varsayılan en iyi tercih**tir.

---

## 🧪 Örnekler, Prototip Kod ve Testler

* Bir kavramı göstermek için örnek yazarken, sağlam hata işleme kodları örneği karmaşıklaştırabilir. Bu nedenle, örneklerde `unwrap` gibi panik oluşturabilecek metotların kullanılması, gerçek uygulamada hataları nasıl ele almanız gerektiğinin yer tutucusu (placeholder) olarak anlaşılır.

* Prototipleme aşamasında, hataların nasıl işleneceğine karar vermeden önce `unwrap` ve `expect` kullanmak faydalıdır. Bu metotlar, kodunuzu daha sağlam hale getirmeye hazır olduğunuzda kolayca değiştirebileceğiniz net işaretler bırakır.

* Testlerde bir metodun çağrısı başarısız olursa, tüm testin başarısız olması gerekir. Çünkü `panic!`, bir testin başarısız olduğunun göstergesidir. Bu yüzden testlerde `unwrap` veya `expect` kullanmak tamamen doğrudur.

---

## 🧠 Derleyiciden Daha Fazla Bilgiye Sahip Olduğunuz Durumlar

Bazen, `Result` değerinin her zaman `Ok` olacağını bilirsiniz, ama derleyici bunu anlayamaz. Böyle durumlarda `expect` çağırmak uygundur.

Örneğin:

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

Burada `parse` metodu her zaman `Result` döner çünkü genel olarak başarısız olabilir. Ancak biz `"127.0.0.1"`’in geçerli bir IP adresi olduğunu biliyoruz. Bu durumda `expect` kullanmak mantıklıdır.

Tabii ki, eğer IP adresi kullanıcı girdisinden geliyorsa, hata ihtimali vardır ve `Result`’ı daha sağlam şekilde işlemek gerekir. Bu nedenle, mesajda **neden hiçbir zaman `Err` oluşmayacağını** açıklamak (örneğin: “Hardcoded IP address should be valid”) gelecekte kod değişirse hatırlatıcı görevi görür.

---

## 📏 Hata Yönetimi İçin Kılavuzlar

Kodunuzun kötü bir duruma (bad state) düşme ihtimali varsa, `panic!` çağırmak mantıklıdır.
**Kötü durum**, şu şartlardan biri (veya daha fazlası) gerçekleştiğinde ortaya çıkar:

* Beklenmeyen bir durum (örneğin, nadiren gerçekleşen kullanıcı hatası değil, mantıksal bir tutarsızlık).
* Kodunuzun sonraki adımları, bu kötü durumun *olmamasına* dayanıyorsa.
* Kullandığınız tipler bu durumu ifade edemiyorsa.

Örnek: Eğer fonksiyonunuza geçersiz, çelişkili veya eksik değerler verilirse ve bu sizin sözleşmenizi (contract) ihlal ediyorsa, `panic!` çağırmak doğru olabilir. Çünkü bu bir kullanıcı hatası değil, çağıran kodun **bug**’udur.

Buna karşılık:

* Eğer başarısızlık beklenen bir durumsa (ör. hatalı veriyle karşılaşan bir parser, HTTP isteklerinde kota aşıldığını belirten bir yanıt), `panic!` değil, `Result` döndürmek gerekir.

### 🔐 Güvenlik Perspektifi

Kodunuz, geçersiz değerler üzerinde çalıştırıldığında kullanıcıyı riske atabilecekse:

* Önce değerlerin geçerli olup olmadığını kontrol etmelisiniz.
* Geçersizse `panic!` çağırmalısınız.

Bu, güvenlik nedeniyle önemlidir. Örneğin, standart kütüphane dizinin sınırlarının ötesine erişmeye çalıştığınızda `panic!` çağırır, çünkü bu tip bellek taşmaları güvenlik açıklarına yol açabilir.

### 📚 Tür Sistemi ile Güvenlik

Her yerde hata kontrolü yapmak sıkıcı olurdu. Neyse ki Rust’ın tür sistemi bu kontrollerin bir kısmını sizin yerinize yapar.

* Örneğin, fonksiyonunuz `Option<T>` yerine doğrudan `T` alıyorsa, her zaman bir değer olacağını bilirsiniz. `None` durumu derleme zamanında engellenir.
* Bir parametreyi `u32` olarak tanımlarsanız, derleyici bu değerin hiçbir zaman negatif olmayacağını garanti eder.

Böylece kodunuz, daha az kontrol ile daha güvenli hale gelir.


## 🏗️ Doğrulama için Özel Tipler Oluşturma (creating custom types for validation)

Rust’ın tür sistemini kullanarak geçerli bir değere sahip olduğumuzdan emin olma (validation) fikrini bir adım daha ileri götürelim ve doğrulama (validation) için özel bir tip (custom type) oluşturmayı inceleyelim. Bölüm 2’deki tahmin oyunu (guessing game) örneğini hatırlayın; kodumuz kullanıcıdan 1 ile 100 arasında bir sayı tahmin etmesini istiyordu. Gizli sayıyla karşılaştırmadan önce kullanıcının tahmininin bu aralıkta olduğunu asla doğrulamadık; yalnızca tahminin pozitif olduğunu doğruladık. Bu durumda sonuçlar çok ciddi değildi: “Too high” veya “Too low” çıktılarımız yine de doğru olurdu. Ancak, kullanıcıyı geçerli tahminlere yönlendirmek ve kullanıcı aralık dışı bir sayı tahmin ettiğinde, örneğin harfler yazdığında olduğundan farklı bir davranışa sahip olmak faydalı bir iyileştirme olurdu.

Bunu yapmanın bir yolu, potansiyel olarak negatif sayılara izin vermek için tahmini yalnızca `u32` yerine `i32` olarak ayrıştırmak (parse) ve ardından sayının aralıkta olup olmadığını kontrol eklemek olurdu, şöyle:

**Dosya adı:** `src/main.rs`

```rust
    loop {
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
    }
```

`if` ifadesi, değerimizin aralık dışında olup olmadığını kontrol eder, kullanıcıya sorunu bildirir ve döngünün bir sonraki yinelemesine başlamak ve başka bir tahmin istemek için `continue` çağırır. `if` ifadesinden sonra, `guess`’in 1 ile 100 arasında olduğunu bilerek `guess` ile gizli sayı arasındaki karşılaştırmalara devam edebiliriz.

Ancak bu ideal bir çözüm değildir: Programın yalnızca 1 ile 100 arasındaki değerlerle çalışması mutlak derecede kritikse ve bu gereksinime sahip birçok fonksiyon varsa, her fonksiyonda bu tür bir kontrolün bulunması zahmetli olur (ve performansı etkileyebilir).

Bunun yerine, özel bir modülde (module) yeni bir tip (type) oluşturabilir ve doğrulamaları (validations) her yerde tekrar etmek yerine bu tipin bir örneğini (instance) oluşturan bir fonksiyona koyabiliriz. Böylece fonksiyonların imzalarında (signature) yeni tipi kullanması güvenli olur ve aldıkları değerleri güvenle kullanabilirler. Listing 9-13, yalnızca `new` fonksiyonu 1 ile 100 arasında bir değer alırsa `Guess` tipinin (type) bir örneğini oluşturacak şekilde `Guess` tanımlamanın bir yolunu gösterir.

**Dosya adı:** `src/guessing_game.rs`

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

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

Listing 9-13: Yalnızca 1 ile 100 arasındaki değerlerle devam edecek bir `Guess` tipi

`src/guessing_game.rs` içindeki bu kodun, burada göstermediğimiz `src/lib.rs` içinde `mod guessing_game;` modül bildirimini (module declaration) eklemeye bağlı olduğunu unutmayın. Bu yeni modülün dosyasında, `Guess` adlı bir `struct` tanımlarız ve `value` adlı bir alana (field) sahip olur; bu alan bir `i32` tutar. Sayı burada saklanacaktır.

Ardından `Guess` üzerinde `new` adlı bir ilişkili fonksiyon (associated function) uygularız; bu fonksiyon `Guess` değerlerinin örneklerini (instances) oluşturur. `new` fonksiyonu, `i32` tipinde `value` adlı tek bir parametreye sahiptir ve bir `Guess` döndürür. `new` fonksiyonunun gövdesindeki kod, `value`’nun 1 ile 100 arasında olduğundan emin olmak için testi yapar. `value` bu testi geçemezse, `panic!` çağrısı yaparız; bu, çağıran kodu yazan programcıya düzeltmesi gereken bir hata (bug) olduğunu bildirir, çünkü bu aralık dışındaki bir değerle `Guess` oluşturmak, `Guess::new`’in dayandığı sözleşmeyi (contract) ihlal eder. `Guess::new`’in panik oluşturabileceği koşullar, herkese açık API dokümantasyonunda (API documentation) tartışılmalıdır; Bölüm 14’te oluşturduğunuz API dokümantasyonunda panik olasılığını belirtme konusundaki yazım kurallarını ele alacağız. `value` testi geçerse, `value` alanı `value` parametresine ayarlanmış yeni bir `Guess` oluşturur ve `Guess`’i döndürürüz.

Sonraki adımda, `self`’i ödünç alan (borrows) ve başka parametre almayan, bir `i32` döndüren `value` adlı bir metot uygularız. Bu tür bir metoda bazen “getter” denir, çünkü amacı alanlarından bazı verileri alıp döndürmektir. Bu genel (public) metot gereklidir çünkü `Guess` yapısının (struct) `value` alanı özeldir (private). `value` alanının özel olması önemlidir; böylece `Guess` yapısını kullanan kodun `value`’yu doğrudan ayarlamasına izin verilmez: `guessing_game` modülü dışındaki kodlar bir `Guess` örneği oluşturmak için `Guess::new` fonksiyonunu kullanmak zorundadır ve böylece bir `Guess`’in `Guess::new` içindeki koşullarca kontrol edilmemiş bir değere sahip olmasının yolu kalmaz.

Sonuç olarak, parametre olarak yalnızca 1 ile 100 arasındaki sayıları alan veya bunları döndüren bir fonksiyon, imzasında `i32` yerine `Guess` aldığını veya `Guess` döndürdüğünü belirtebilir ve gövdesinde herhangi bir ek kontrol yapmak zorunda kalmaz.

## 📚 Özet (summary)

Rust’ın hata yönetimi (error handling) özellikleri, daha sağlam kod yazmanıza yardımcı olmak için tasarlanmıştır. `panic!` makrosu, programınızın ele alamayacağı bir durumda olduğunu işaret eder ve geçersiz veya hatalı değerlerle devam etmeye çalışmak yerine süreci durdurmanızı sağlar. `Result` enum’u ise, işlemlerin kurtarılabilir şekilde başarısız olabileceğini tür sistemiyle (type system) belirtir. `Result` kullanarak, kodunuzu çağıran koda olası başarı veya başarısızlığı ele alması gerektiğini bildirebilirsiniz. Uygun durumlarda `panic!` ve `Result` kullanmak, kaçınılmaz sorunlar karşısında kodunuzu daha güvenilir hale getirir.

Artık standart kütüphanenin `Option` ve `Result` enum’larıyla birlikte generics (generics) kullanımına dair faydalı yolları gördüğünüze göre, sırada generics’in nasıl çalıştığını ve kendi kodunuzda onları nasıl kullanabileceğinizi ele almak var.
