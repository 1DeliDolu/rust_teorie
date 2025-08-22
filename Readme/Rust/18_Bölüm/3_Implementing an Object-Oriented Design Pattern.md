## 🏗️ Nesne Yönelimli Bir Tasarım Deseninin Uygulanması (object-oriented design pattern)

**Durum deseni (state pattern)**, nesne yönelimli bir tasarım desenidir. Desenin özü, bir değerin içsel olarak sahip olabileceği bir dizi **durumu (state)** tanımlamaktır. Bu durumlar, bir dizi **durum nesnesi (state object)** ile temsil edilir ve değerin davranışı, durumuna bağlı olarak değişir. Bu bölümde, alanlarından biri durumu tutan bir blog yazısı (`post`) `struct`’ı üzerinden ilerleyeceğiz; bu durum kümesi “**taslak (draft)**”, “**inceleme (review)**” veya “**yayınlanmış (published)**” olacaktır.

Durum nesneleri bazı işlevleri paylaşır: Rust’ta elbette nesneler ve kalıtım yerine `struct` ve `trait`’ler kullanırız. Her durum nesnesi kendi davranışından ve ne zaman başka bir duruma geçmesi gerektiğini belirlemekten sorumludur. Durum nesnesini tutan değer, durumların farklı davranışlarını veya ne zaman geçiş yapılacağını bilmez.

Durum desenini kullanmanın avantajı, programın iş gereksinimleri değiştiğinde, durumu tutan değerin kodunu veya bu değeri kullanan kodu değiştirmek zorunda olmamamızdır. Yalnızca durumlardan birinin içindeki kodu, kuralları değiştirmek veya daha fazla durum nesnesi eklemek için güncellememiz yeterlidir.

Önce durumu daha geleneksel, nesne yönelimli bir şekilde uygulayacağız, ardından Rust’ta daha doğal olan bir yaklaşıma geçeceğiz. Durum desenini kullanarak adım adım bir blog yazısı iş akışını uygulayalım.

Nihai işlevsellik şöyle görünecek:

* Bir blog yazısı, boş bir taslak olarak başlar.
* Taslak hazır olduğunda, yazı için bir inceleme istenir.
* Yazı onaylandığında yayınlanır.
* Yalnızca yayınlanmış blog yazıları yazdırılacak içerik döndürür; böylece onaylanmamış yazılar yanlışlıkla yayınlanamaz.
* Yazı üzerinde yapılan diğer tüm değişiklik girişimleri etkisiz olmalıdır. Örneğin, inceleme talep edilmeden bir taslak yazıyı onaylamaya çalışırsak, yazı yayınlanmamış taslak olarak kalmalıdır.

Listeleme 18-11, bu iş akışını kod biçiminde gösterir: bu, `blog` adlı bir kütüphane crate’inde uygulayacağımız API’nin örnek kullanımıdır. `blog` crate’ini henüz uygulamadığımız için bu kod şu an derlenmez.

**Dosya adı:** src/main.rs
Bu kod derlenmez!

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

Listeleme 18-11: `blog` crate’imizin sahip olmasını istediğimiz hedef davranışı gösteren kod

Kullanıcının `Post::new` ile yeni bir taslak blog yazısı oluşturabilmesini istiyoruz. Kullanıcının blog yazısına metin ekleyebilmesini istiyoruz. Onaydan önce, yazının içeriğini hemen almaya çalışırsak, yazı hâlâ taslak olduğundan hiçbir metin alamamalıyız. Gösterim amacıyla koda `assert_eq!` ekledik. Bunun için mükemmel bir birim testi, taslak bir blog yazısının `content` metodundan boş bir dize döndürdüğünü doğrulamak olurdu, ancak bu örnekte test yazmayacağız.

Sonraki adımda, yazı için bir inceleme talebi yapılabilmesini ve inceleme beklerken `content`’in boş bir dize döndürmeye devam etmesini istiyoruz. Yazı onay aldığında yayınlanmalı, yani `content` çağrıldığında yazının metni döndürülmelidir.

Dikkat edilirse, crate’ten etkileşim kurduğumuz tek tür `Post` türüdür. Bu tür, durum desenini kullanacak ve bir yazının içinde bulunabileceği çeşitli durumları temsil eden üç durum nesnesinden biri olan bir değeri tutacaktır: **taslak (draft)**, **inceleme (review)** veya **yayınlanmış (published)**. Bir durumdan diğerine geçişler `Post` türünün içinde, dahili olarak yönetilecektir. Geçişler, kütüphanemizin kullanıcılarının `Post` örneği üzerinde çağırdığı metotlara tepki olarak gerçekleşecek, ancak kullanıcılar durum değişimlerini doğrudan yönetmek zorunda kalmayacaktır. Ayrıca kullanıcılar, örneğin bir yazıyı incelemeden önce yayınlamak gibi durumsal hatalar yapamayacaktır.

## 🧩 Post Tanımlamak ve Taslak Durumunda Yeni Bir Örnek Oluşturmak (Defining Post and Creating a New Instance in the Draft State)

Haydi kütüphanenin uygulanmasına başlayalım! Public bir `Post` `struct`’ına (struct) ve içeriği tutacak bir alana ihtiyacımız var; bu nedenle Listeleme 18-12’de gösterildiği gibi `struct` tanımıyla ve `Post` örneği oluşturmak için ilişkili public `new` fonksiyonuyla başlayacağız. Ayrıca, bir `Post` için tüm durum nesnelerinin sahip olması gereken davranışı tanımlayan private bir `State` `trait`’i (trait) oluşturacağız.

Ardından `Post`, durum nesnesini tutmak için `state` adlı private bir alanda `Option<Box<dyn State>>` (trait object) barındıracak. `Option<T>`’ye (Option<T>) neden ihtiyaç duyduğumuzu az sonra göreceksiniz.

**Dosya adı:** src/lib.rs

```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

Listeleme 18-12: Bir `Post` `struct`’ı ve yeni bir `Post` örneği oluşturan `new` fonksiyonu, bir `State` `trait`’i ve bir `Draft` `struct`’ının tanımı

`State` `trait`’i (State trait), farklı yazı durumları arasında paylaşılan davranışı tanımlar. Durum nesneleri `Draft`, `PendingReview` ve `Published` olacak ve hepsi `State` `trait`’ini uygulayacak. Şimdilik `trait`’in metodu yok; yazının başlamasını istediğimiz durum olan `Draft`’ı tanımlayarak başlıyoruz.

Yeni bir `Post` oluşturduğumuzda, `state` alanını `Box` (Box) içinde tutulmuş yeni bir `Draft` örneğine işaret eden `Some` ile ayarlarız. Böylece her yeni `Post` örneği taslak (draft) olarak başlar. `Post`’un `state` alanı private olduğundan, başka bir durumda `Post` oluşturmanın yolu yoktur! `Post::new` içinde `content` alanını yeni, boş bir `String` ile ayarlarız.

---

## 📝 Yazı İçeriğinin Metnini Saklamak (Storing the Text of the Post Content)

Listeleme 18-11’de, `add_text` adlı bir metodu çağırıp ona bir `&str` geçirerek bunun blog yazısının metin içeriğine eklenmesini istediğimizi gördük. Bunu `content` alanını `pub` yapıp doğrudan ortaya çıkarmak yerine bir metod olarak uygularız; böylece daha sonra `content` alanındaki verinin nasıl okunacağını kontrol eden bir metot ekleyebiliriz. `add_text` metodu oldukça basittir; bu nedenle Listeleme 18-13’teki uygulamayı `impl Post` bloğuna ekleyelim.

**Dosya adı:** src/lib.rs

```rust
impl Post {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

Listeleme 18-13: Bir yazının içeriğine metin eklemek için `add_text` metodunun uygulanması

`add_text` metodu, üzerinde çağrıldığı `Post` örneğini değiştirdiğimiz için `self`’in mutable referansını alır. Ardından `content` içindeki `String` üzerinde `push_str` çağırır ve kaydedilen içeriğe eklenecek `text` argümanını geçiririz. Bu davranış yazının bulunduğu duruma bağlı değildir; bu yüzden durum deseninin bir parçası değildir. `add_text` metodu `state` alanıyla etkileşime girmez, ancak desteklemek istediğimiz davranışın bir parçasıdır.

---

## 🧼 Taslak Bir Yazının İçeriğinin Boş Olduğundan Emin Olmak (Ensuring the Content of a Draft Post Is Empty)

`add_text` çağrılıp yazımıza içerik ekledikten sonra bile, yazı hâlâ taslak (draft) durumunda olduğu için `content` metodunun boş bir dize dilimi döndürmesini istiyoruz; bu, Listeleme 18-11’in 7. satırında gösterildiği gibidir. Şimdilik, bu gereksinimi karşılayacak en basit şeyi yapalım: her zaman boş bir dize dilimi döndürmek. Yayınlanabilir duruma geçişi uyguladığımızda bunu değiştireceğiz. Şu ana kadar yazılar yalnızca taslak durumunda olabilir, bu nedenle yazı içeriği her zaman boş olmalıdır. Listeleme 18-14 bu yer tutucu (placeholder) uygulamayı gösterir.

**Dosya adı:** src/lib.rs

```rust
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}
```

Listeleme 18-14: `Post` üzerindeki `content` metoduna her zaman boş bir dize dilimi döndüren yer tutucu bir uygulama eklemek

Bu `content` metodunu eklediğimizde, Listeleme 18-11’de 7. satıra kadar her şey beklendiği gibi çalışır.

---

## 🔄 İnceleme İstemek Yazının Durumunu Değiştirir (Requesting a Review Changes the Post’s State)

Sırada, bir yazı için inceleme isteme işlevini eklememiz gerekiyor; bu, durumun `Draft`’tan `PendingReview`’e değişmesini sağlamalıdır. Listeleme 18-15 bu kodu gösterir.

**Dosya adı:** src/lib.rs

```rust
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

Listeleme 18-15: `Post` ve `State` `trait`’i üzerinde `request_review` metotlarının uygulanması

`Post`’a, `self`’in mutable referansını alan public bir `request_review` metodu veriyoruz. Ardından `Post`’un mevcut durumu üzerinde dahili bir `request_review` metodu çağırıyoruz; bu ikinci `request_review`, mevcut durumu tüketir ve yeni bir durum döndürür.

`request_review` metodunu `State` `trait`’ine ekliyoruz; artık `trait`’i uygulayan tüm türlerin `request_review` metodunu da uygulaması gerekir. Dikkat edin: metodun ilk parametresi olarak `self`, `&self` veya `&mut self` yerine `self: Box<Self>` kullanıyoruz. Bu sözdizimi, metodun yalnızca türü tutan bir `Box` üzerinde çağrıldığında geçerli olduğu anlamına gelir. Bu, `Box<Self>`’in sahipliğini alır; eski durumu geçersiz kılar; böylece `Post`’un `state` değeri yeni bir duruma dönüşebilir.

Eski durumu tüketmek için `request_review` metodunun durum değerinin sahipliğini alması gerekir. İşte `Post`’taki `state` alanının `Option` (Option) olmasının nedeni: `take` metodunu çağırarak `state` alanındaki `Some` değeri alır ve yerine `None` bırakırız; çünkü Rust, `struct` alanlarının boş bırakılmasına izin vermez. Bu sayede, durumu ödünç almak yerine `Post`’tan taşıyabiliriz. Sonrasında yazının `state` değerini bu işlemin sonucuna ayarlarız.

Sahipliği elde etmek için `state`’i geçici olarak `None`’a ayarlamamız gerekir; bu yüzden `self.state = self.state.request_review();` gibi doğrudan atama yapamayız. Bu, `Post`’un yeni duruma dönüştürdükten sonra eski durumu kullanamamasını sağlar.

`Draft` üzerindeki `request_review` metodu, bir inceleme bekleme durumunu temsil eden yeni bir `PendingReview` `struct`’ının boxed (boxed) bir örneğini döndürür. `PendingReview` `struct`’ı da `request_review` metodunu uygular ancak dönüşüm yapmaz; kendisini döndürür. Çünkü zaten `PendingReview` durumundaki bir yazı için yeniden inceleme istendiğinde, durumda bir değişiklik olmamalıdır.

Böylece durum deseninin avantajlarını görmeye başlıyoruz: `Post` üzerindeki `request_review` metodu, `state` değeri ne olursa olsun aynıdır. Her durum kendi kurallarından sorumludur.

`Post` üzerindeki `content` metodunu olduğu gibi bırakıyoruz; boş bir dize dilimi döndürmeye devam edecek. Artık bir `Post`, `Draft`’ın yanı sıra `PendingReview` durumunda da olabilir; ancak `PendingReview` durumunda da aynı davranışı istiyoruz. Listeleme 18-11 artık 10. satıra kadar çalışıyor!

## ✅ `approve` Ekleyerek `content` Davranışını Değiştirmek (Adding approve to Change the Behavior of content)

`approve` metodu, `request_review` metoduna benzer şekilde çalışır: yazının mevcut durumuna göre onaylandığında hangi duruma geçmesi gerekiyorsa `state` alanını ona ayarlayacaktır (Listeleme 18-16’da gösterildiği gibi).

**Dosya adı:** src/lib.rs

```rust
impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

Listeleme 18-16: `Post` ve `State` `trait`’i üzerinde `approve` metodunun uygulanması

Burada `State` `trait`’ine `approve` metodunu ekliyoruz ve `State`’i uygulayan yeni bir `Published` `struct` tanımlıyoruz.

* `Draft` üzerindeki `approve`, `self` döndürür → yani bir taslak yazıyı onaylamaya çalışmak etkisizdir.
* `PendingReview` üzerindeki `approve`, yeni bir `Published` nesnesi döndürür.
* `Published` yapısı hem `request_review` hem de `approve` için kendisini döndürür → yani zaten yayınlanmış bir yazı bu durumda kalmalıdır.

---

## 📖 `content` Metodunu Güncellemek (Updating the content Method)

Artık `content` metodunun döndürdüğü değerin yazının mevcut durumuna bağlı olmasını istiyoruz. Bunun için, `Post` `struct`’ındaki `content` metodunu `state` üzerinden çağrılacak bir `content` metoduna yönlendiriyoruz (Listeleme 18-17).

**Dosya adı:** src/lib.rs
Bu kod derlenmez!

```rust
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // --snip--
}
```

Listeleme 18-17: `Post` üzerindeki `content` metodunu `State` üzerindeki `content` metoduna yönlendirmek

Burada:

* `as_ref` çağrısı ile `Option<Box<dyn State>>` → `Option<&Box<dyn State>>` dönüşür. Sahiplik (ownership) değil yalnızca referans alıyoruz.
* `unwrap` çağrısı güvenlidir, çünkü `Post` metotları `state`’in her zaman `Some` olmasını garanti eder.

Bu noktada, `&Box<dyn State>` üzerindeki `content` çağrısı, Rust’ın deref coercion kuralıyla `State`’i uygulayan tipe yönlendirilir. Yani `State` `trait`’inde bir `content` metodu eklememiz gerekir.

---

## 📝 `State` Üzerine `content` Eklemek (Adding the content Method to the State Trait)

Şimdi `State` `trait`’ine `content` metodunu ekliyoruz. Varsayılan uygulama boş bir dize dilimi döndürür. Bu sayede `Draft` ve `PendingReview` için ayrı `content` yazmaya gerek kalmaz. `Published` ise `content` metodunu geçersiz kılar (override) ve `post.content` değerini döndürür (Listeleme 18-18).

**Dosya adı:** src/lib.rs

```rust
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

Listeleme 18-18: `State` `trait`’ine `content` metodunun eklenmesi

Burada yaşam süreleri (lifetimes) önemlidir: `post`’a referans alıyoruz ve onun bir parçasına referans döndürüyoruz, bu yüzden dönen referansın yaşam süresi `post` ile ilişkilidir.

---

## 🎉 Sonuç

Artık Listeleme 18-11’deki tüm kod çalışır! Durum desenini (state pattern) blog yazısı iş akışı kurallarıyla birlikte uygulamış olduk. Kurallara ilişkin mantık `Post`’a dağılmak yerine durum nesnelerinin içinde yaşıyor.

---

## ❓ Neden Bir `enum` Değil? (Why Not An Enum?)

Merak ediyor olabilirsiniz: neden farklı yazı durumlarını varyantlar olarak içeren bir `enum` kullanmadık? Bu kesinlikle olası bir çözümdür; deneyip sonucu karşılaştırabilirsiniz.

Ancak `enum` kullanmanın dezavantajı, her yerde `match` veya benzeri kontrollerle tüm varyantları ele almak zorunda kalmanızdır. Bu, trait nesnesi (trait object) çözümüne kıyasla daha tekrarlı hale gelebilir.

## ⚖️ Durum Deseninin Artıları ve Eksileri (Trade-offs of the State Pattern)

Rust’ın, bir yazının her durumunda sahip olması gereken farklı davranışları kapsüllemek için nesne yönelimli **durum desenini (state pattern)** uygulayabildiğini gördük. `Post` üzerindeki metotlar bu davranışların hiçbirini bilmez. Kodumuzu organize ediş biçimimiz sayesinde, yayınlanmış bir yazının tüm olası davranışlarını tek bir yerde görebiliriz: `Published` struct’ı üzerindeki `State` `trait` implementasyonu.

Durum desenini kullanmasaydık, bunun yerine `Post` metotlarında veya doğrudan ana kodda `match` ifadeleri kullanarak yazının durumunu kontrol edip davranışı orada değiştirebilirdik. Bu durumda, yayınlanmış bir yazının etkilerini anlamak için birçok farklı yere bakmamız gerekirdi. Daha fazla durum ekledikçe bu `match` ifadeleri de daha karmaşık hale gelir, çünkü her seferinde yeni bir `arm` eklenmesi gerekir.

Durum deseniyle birlikte:

* `Post` metotlarında ve `Post`’u kullandığımız yerlerde `match` ifadelerine gerek yoktur.
* Yeni bir durum eklemek için yalnızca yeni bir `struct` ekleyip `trait` metotlarını o `struct` üzerinde uygularız.

Bu yaklaşımın genişletilmesi kolaydır. Kodu bakımının basitliğini görmek için şu önerileri deneyebilirsiniz:

* `reject` metodu ekleyip yazının durumunu `PendingReview`’dan `Draft`’a geri döndürmek.
* Yayınlanmış duruma geçebilmek için iki kez `approve` çağrısı gerektirmek.
* Kullanıcılara yalnızca yazı `Draft` durumundayken içerik eklemeye izin vermek. İpucu: içerikte nelerin değişebileceğinden **durum nesnesi** sorumlu olabilir, ama `Post`’un kendisini değiştirmekten sorumlu olmamalıdır.

---

### 🚧 Eksiler (Downsides)

* **Durumlar birbirine bağlıdır (coupling):** Durum geçişlerini durumlar yönettiği için bazı durumlar birbirine bağımlıdır. Örneğin, `PendingReview` ile `Published` arasına `Scheduled` adında yeni bir durum eklemek istersek, `PendingReview` içindeki kodu değiştirip `Published` yerine `Scheduled`’a geçiş yapacak hale getirmeliyiz. Daha iyi olurdu ki `PendingReview`’i değiştirmemiz gerekmesin, ancak bu durumda başka bir tasarım deseni kullanmamız gerekir.

* **Tekrar eden (duplicated) mantık:**

  * `State` `trait`’indeki `request_review` ve `approve` metotları birbirine benzer. Bunlar için varsayılan uygulamalar (`default implementations`) yazmak isteyebiliriz, ancak bu çalışmaz: çünkü `State` bir trait nesnesi olarak kullanıldığında somut `self` tipi derleme zamanında bilinmez. (Bu, daha önce bahsettiğimiz `dyn` uyumluluk kurallarından biridir.)
  * `Post` üzerindeki `request_review` ve `approve` metotları da birbirine çok benzer: `Option::take` ile `state` değerini alıyor, `Some` ise sarmalanmış değerin ilgili metoduna yönlendiriyor ve sonucu `state` alanına koyuyor. Eğer bu deseni takip eden çok fazla metot olsaydı, bu tekrarı ortadan kaldırmak için bir **makro (macro)** tanımlamayı düşünebilirdik (Bkz. Bölüm 20, “Macros”).

Sonuç olarak, durum desenini nesne yönelimli dillerde tanımlandığı şekliyle uygularsak, Rust’ın güçlü yanlarından tam olarak yararlanmamış oluruz. Şimdi `blog` crate’inde bazı değişiklikler yaparak geçersiz durumların ve geçişlerin **derleme zamanı hataları**na dönüşmesini sağlayabileceğimiz bir yaklaşım göreceğiz.

---

## 🔡 Durumları ve Davranışı Tür Olarak Kodlamak (Encoding States and Behavior as Types)

Durum desenini yeniden düşünerek farklı bir denge elde edebiliriz. Durumları tamamen kapsüllemek yerine, onları **farklı türlere (types)** kodlayabiliriz. Böylece Rust’ın **tip denetim sistemi (type checking system)**, yalnızca yayınlanmış yazıların kabul edildiği yerlerde taslak yazıları kullanmaya çalıştığımızda derleyici hatası verecektir.

Örneğin, Listeleme 18-11’in başındaki `main` fonksiyonunu ele alalım:

**Dosya adı:** src/main.rs

```rust
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

Yeni yazıları `Post::new` ile taslak (draft) olarak oluşturmayı ve içeriğe metin eklemeyi hâlâ mümkün kılacağız. Ancak, taslak bir yazı için `content` metodunun boş dize döndürmesi yerine, taslak yazıların `content` metodunun hiç olmamasını sağlayacağız. Böylece bir taslak yazının içeriğini almaya çalışırsak, **derleyici hatası** alacağız çünkü bu metot yok. Bunun sonucunda, taslak içerikleri yanlışlıkla üretim ortamında göstermek imkânsız olacak çünkü bu kod derlenmeyecek.

Listeleme 18-19’da, bir `Post` `struct`’ı ve bir `DraftPost` `struct`’ı ile her biri üzerindeki metotlar gösterilmektedir:

**Dosya adı:** src/lib.rs

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

Listeleme 18-19: `content` metoduna sahip `Post` ve `content` metoduna sahip olmayan `DraftPost`

Burada:

* Hem `Post` hem de `DraftPost` blog yazısı metnini saklayan private bir `content` alanına sahiptir.
* Artık `state` alanı yoktur, çünkü durumların kodlanması struct türlerine taşınmıştır.
* `Post`, yayınlanmış (published) bir yazıyı temsil eder ve `content` metoduna sahiptir.
* `Post::new`, bir `Post` yerine bir `DraftPost` döndürür.
* `DraftPost` üzerinde `add_text` metodu vardır, ama `content` metodu **yoktur**.

Böylece:

* Tüm yazılar taslak olarak başlar.
* Taslak yazıların içeriği erişime açık değildir.
* Bu kısıtları aşmaya yönelik her girişim derleyici hatası üretir.

## 🔄 Geçişleri Farklı Türlere Dönüşüm Olarak Uygulamak (Implementing Transitions as Transformations into Different Types)

Peki, yayınlanmış (`published`) bir yazıyı nasıl elde ederiz? Kuralı zorlamak istiyoruz: bir taslak (`draft`) yazı, yayınlanmadan önce incelenmeli (`review`) ve onaylanmalıdır (`approve`). İnceleme bekleyen (`pending review`) bir yazı da hâlâ içeriğini göstermemelidir. Bu kısıtları uygulamak için `PendingReviewPost` adında başka bir `struct` ekleyelim; `DraftPost` üzerinde `request_review` metodunu tanımlayıp bunun bir `PendingReviewPost` döndürmesini sağlayalım; ardından `PendingReviewPost` üzerinde `approve` metodunu tanımlayıp bunun bir `Post` döndürmesini sağlayalım (Listeleme 18-20).

**Dosya adı:** src/lib.rs

```rust
impl DraftPost {
    // --snip--
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

Listeleme 18-20: `DraftPost` üzerindeki `request_review` çağrısıyla oluşturulan `PendingReviewPost` ve `approve` çağrısıyla `PendingReviewPost`’u yayınlanmış bir `Post`’a dönüştüren metot

`request_review` ve `approve` metotları `self`’in sahipliğini alır, yani `DraftPost` ve `PendingReviewPost` örneklerini tüketerek sırasıyla `PendingReviewPost` ve yayınlanmış bir `Post`’a dönüştürür. Bu şekilde, `request_review` çağrısından sonra elimizde kalan eski bir `DraftPost` olmaz. Aynı şekilde, `PendingReviewPost` üzerinde `content` metodu tanımlı değildir, yani içeriğini okumaya çalışmak derleyici hatasına yol açar.

Çünkü:

* Yayınlanmış (`Post`) bir örneği elde etmenin tek yolu `PendingReviewPost` üzerinde `approve` çağırmaktır.
* `PendingReviewPost` elde etmenin tek yolu ise `DraftPost` üzerinde `request_review` çağırmaktır.

Böylece blog yazısı iş akışını doğrudan **tip sistemine (type system)** kodlamış olduk.

---

## 📝 `main` Fonksiyonunu Güncellemek (Updating main)

Ancak, `main` üzerinde küçük değişiklikler yapmamız gerekiyor. `request_review` ve `approve` metotları yeni örnekler döndürdüğü için, çağrıldıkları `struct`’ı değiştirmezler. Bu yüzden dönen örnekleri kaydetmek için `let post =` şeklinde gölgeleme (shadowing) atamaları eklememiz gerekir. Ayrıca, artık taslak ve incelemede olan yazıların içeriklerinin boş dizge olup olmadığını test eden `assert`’lara ihtiyacımız yoktur; çünkü bu içeriklere erişmeye çalışmak derleme hatası verir.

Güncellenmiş `main`, Listeleme 18-21’de gösterilmiştir:

**Dosya adı:** src/main.rs

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

Listeleme 18-21: Blog yazısı iş akışının yeni uygulamasını kullanan `main`

---

## ✅ Sonuç ve Değerlendirme

`main`’de `post`’u yeniden atamak zorunda kalmamız, bu uygulamanın artık tam olarak nesne yönelimli durum desenine uymadığı anlamına geliyor: durumlar arası dönüşümler artık tamamen `Post` implementasyonu içinde kapsüllenmemiştir. Ancak kazandığımız şey şudur: **geçersiz durumlar tip sistemi sayesinde imkânsız hale gelmiştir**. Böylece, örneğin yayınlanmamış bir yazının içeriğini göstermeye çalışan hatalar, üretime ulaşmadan önce derleyici tarafından engellenir.

Bu bölümün başındaki önerilen görevleri, Listeleme 18-21’den sonraki `blog` crate’i üzerinde deneyin ve bu sürümün tasarımı hakkında ne düşündüğünüzü görün. Bazılarının bu tasarımda zaten çözülmüş olduğunu fark edebilirsiniz.

---

## 📌 Özet (Summary)

* Rust, nesne yönelimli tasarım desenlerini uygulayabilecek kapasitededir.
* **Trait nesneleri (trait objects)** kullanarak nesne yönelimli özelliklerden yararlanabiliriz. Bu, dinamik yönlendirme (dynamic dispatch) getirir: çalışma zamanı esnekliği karşılığında biraz performans maliyeti.
* Bu esneklik, kodun bakımını kolaylaştırabilecek nesne yönelimli desenleri uygulamaya izin verir.
* Rust ayrıca sahiplik (ownership) gibi nesne yönelimli dillerde bulunmayan özelliklere sahiptir. Bu yüzden nesne yönelimli desenler her zaman Rust’ın güçlü yönlerinden yararlanmanın en iyi yolu olmayabilir, ama mevcut seçeneklerden biridir.

---

## 🔜 Sonraki Konu

Şimdi sırada **desenler (patterns)** var. Desenler, Rust’ın çok esneklik sağlayan başka bir özelliğidir. Kitap boyunca onlara kısaca değindik, ancak tam yeteneklerini henüz görmedik. Haydi başlayalım!
ö