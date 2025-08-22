## 🧩 Özellikler: Paylaşılan Davranışları Tanımlamak (traits: defining shared behavior)

Bir **özellik (trait)**, belirli bir türün sahip olduğu ve diğer türlerle paylaşabileceği işlevselliği tanımlar. Özellikleri, paylaşılan davranışı soyut (abstract) bir şekilde tanımlamak için kullanabiliriz. Ayrıca, bir **genel türün (generic type)** yalnızca belirli bir davranışa sahip türler olabileceğini belirtmek için **özellik sınırlarını (trait bounds)** kullanabiliriz.

Not: Özellikler, diğer dillerde genellikle **arayüz (interface)** olarak adlandırılan yapıya benzer, ancak bazı farklılıkları vardır.

---

## 🏗️ Bir Özellik Tanımlamak (defining a trait)

Bir türün davranışı, o tür üzerinde çağırabileceğimiz yöntemlerden (methods) oluşur. Farklı türler, aynı yöntemleri çağırabiliyorsak aynı davranışı paylaşır. Özellik tanımları, bir amaca ulaşmak için gerekli olan davranışları belirlemek üzere yöntem imzalarını (method signatures) bir araya getirme yoludur.

Örneğin, farklı türlerde ve miktarlarda metin tutan birkaç `struct`’ımız olduğunu varsayalım:

* Belirli bir yerde yazılmış bir haber hikâyesini tutan bir `NewsArticle` struct’ı
* En fazla 280 karakter uzunluğunda olabilen ve yeni bir gönderi, yeniden gönderi veya başka bir gönderiye yanıt olduğunu belirten meta verileri içeren bir `SocialPost` struct’ı

Bir **medya toplayıcı (media aggregator)** `library crate`’i yapmak istiyoruz; adı `aggregator` olacak ve bir `NewsArticle` ya da `SocialPost` örneğinde depolanabilecek verilerin özetlerini gösterecek. Bunu yapmak için her türden bir özet almamız gerekir ve bu özeti, örnek üzerinde `summarize` adlı bir yöntemi çağırarak isteyeceğiz.

Liste 10-12, bu davranışı ifade eden, `Summary` adlı herkese açık (`pub`) bir özellik tanımını göstermektedir.

**Filename: src/lib.rs**

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Listing 10-12: `summarize` yöntemiyle sağlanan davranıştan oluşan bir `Summary` özelliği

Burada, `trait` anahtar sözcüğünü kullanarak ve ardından özelliğin adını yazarak bir özellik bildiriyoruz; bu durumda ad `Summary`’dir. Ayrıca, bu özelliği `pub` olarak bildiriyoruz, böylece bu `crate`’e bağlı olan `crate`’ler de bu özelliği kullanabilir; bunu birkaç örnekte göreceğiz.

Süslü parantezler içinde, bu özelliği uygulayan türlerin davranışlarını tanımlayan yöntem imzalarını bildiriyoruz; bu örnekte `fn summarize(&self) -> String`.

Yöntem imzasından sonra süslü parantezler içinde bir uygulama vermek yerine noktalı virgül (`;`) kullanıyoruz. Bu özelliği uygulayan her tür, yöntemin gövdesi için kendi özel davranışını sağlamalıdır. Derleyici, `Summary` özelliğine sahip her türün, tam olarak bu imzaya sahip bir `summarize` yöntemine sahip olmasını zorunlu kılacaktır.

Bir özellik, gövdesinde birden fazla yöntem barındırabilir: yöntem imzaları satır satır listelenir ve her satır noktalı virgül ile biter.

## ⚙️ Bir Tür Üzerinde Özellik Uygulamak (implementing a trait on a type)

Artık `Summary` özelliğinin (trait) yöntemleri için gereken imzaları tanımladığımıza göre, bunları medya toplayıcımızdaki (media aggregator) türler üzerinde uygulayabiliriz. Liste 10-13, `Summary` özelliğinin `NewsArticle` `struct`’ı üzerinde nasıl uygulandığını gösteriyor. Burada `summarize`, başlık (`headline`), yazar (`author`) ve konumu (`location`) kullanarak döndürme değerini oluşturur. `SocialPost` `struct`’ı için ise, gönderi içeriğinin zaten 280 karakterle sınırlandırıldığını varsayarak, `summarize`’ı kullanıcı adı (`username`) ve gönderi içeriği (`content`) şeklinde tanımlıyoruz.

**Filename: src/lib.rs**

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Listing 10-13: `Summary` özelliğinin `NewsArticle` ve `SocialPost` türlerinde uygulanması

Bir tür üzerinde özellik uygulamak, normal yöntemler uygulamaya benzer. Fark şudur: `impl`’den sonra, uygulamak istediğimiz özelliğin adını yazarız, ardından `for` anahtar sözcüğünü kullanırız ve ardından bu özelliği hangi tür üzerinde uygulayacağımızı belirtiriz. `impl` bloğu içinde, özellik tanımında bildirilmiş yöntem imzalarını yazarız. Ancak, imzadan sonra noktalı virgül koymak yerine süslü parantez açar ve yöntemin gövdesini, o tür için istediğimiz özel davranışla doldururuz.

Artık kütüphane (`library`) `Summary` özelliğini `NewsArticle` ve `SocialPost` üzerinde uyguladığına göre, `crate` kullanıcıları `NewsArticle` ve `SocialPost` örnekleri üzerinde, normal yöntemlerde olduğu gibi bu özelliğin yöntemlerini çağırabilirler. Tek fark, kullanıcıların hem özelliği hem de türleri kapsama alanına (scope) getirmesi gerektiğidir. İşte ikili (`binary crate`) bir `crate`’in `aggregator` kütüphanesini nasıl kullanabileceğine dair bir örnek:

```rust
use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}
```

Bu kod şu çıktıyı verir:
`1 new post: horse_ebooks: of course, as you probably already know, people`

`aggregator` kütüphanesine bağlı olan diğer `crate`’ler de `Summary` özelliğini kapsamlarına alabilir ve kendi türleri üzerinde `Summary` uygulayabilir. Ancak dikkat edilmesi gereken bir kısıtlama vardır: Bir özelliği yalnızca **özellik, tür veya her ikisi de `crate`’imize aitse** bir tür üzerinde uygulayabiliriz.

Örneğin:

* `SocialPost` bizim `crate`’imize ait olduğu için, standart kütüphanedeki `Display` özelliğini `SocialPost` üzerinde uygulayabiliriz.
* Aynı şekilde, `Summary` bizim `crate`’imize ait olduğu için `Vec<T>` üzerinde de uygulayabiliriz.

Ama dış özellikleri (external traits) dış türler (external types) üzerinde uygulayamayız. Örneğin, `Display` özelliğini `Vec<T>` üzerinde `aggregator` kütüphanemizde uygulayamayız çünkü hem `Display` hem de `Vec<T>` standart kütüphanede tanımlanmıştır ve bizim `crate`’imize ait değildir.

Bu kısıtlama, **tutarlılık (coherence)** ilkesinin bir parçasıdır; özellikle de **yetim kuralı (orphan rule)** olarak adlandırılır çünkü “ebeveyn tür” mevcut değildir. Bu kural, başkalarının kodunun sizin kodunuzu, sizin kodunuzun da başkalarının kodunu bozamamasını sağlar. Bu kural olmasaydı, iki `crate` aynı özelliği aynı tür üzerinde uygulayabilir ve Rust hangi uygulamanın kullanılacağını bilemezdi.


## 📝 Varsayılan Uygulamalar (default implementations)

Bazen bir özelliğin (trait) tüm yöntemleri için uygulama yazmak yerine, bazı ya da tüm yöntemler için **varsayılan davranış (default behavior)** sağlamak faydalı olabilir. Böylece, bir tür üzerinde özelliği uygularken, her yöntemin varsayılan davranışını olduğu gibi koruyabilir veya üzerine yazabiliriz (override).

Liste 10-14’te, `Summary` özelliğinin `summarize` yöntemi için yalnızca imza tanımlamak yerine varsayılan bir string belirtiyoruz.

**Filename: src/lib.rs**

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

Listing 10-14: `summarize` yöntemi için varsayılan uygulamaya sahip bir `Summary` özelliği tanımlamak

`NewsArticle` örneklerini özetlemek için varsayılan uygulamayı kullanmak istersek, boş bir `impl` bloğu belirtiriz:

```rust
impl Summary for NewsArticle {}
```

Artık `NewsArticle` üzerinde doğrudan `summarize` tanımlamamış olsak da, varsayılan uygulama sağladığımız ve `NewsArticle`’ın `Summary` özelliğini uyguladığını belirttiğimiz için `NewsArticle` örneği üzerinde `summarize` çağırabiliriz:

```rust
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
    ),
};

println!("New article available! {}", article.summarize());
```

Bu kod şu çıktıyı verir:
`New article available! (Read more...)`

Varsayılan bir uygulama oluşturmak, Liste 10-13’teki `SocialPost` üzerinde `Summary` uygulamasını değiştirmemizi gerektirmez. Bunun nedeni, varsayılan bir uygulamanın üzerine yazmak için kullanılan sözdiziminin, varsayılanı olmayan bir özellik yöntemini uygulamak için kullanılan sözdizimiyle aynı olmasıdır.

---

Varsayılan uygulamalar, aynı özellikteki diğer yöntemleri çağırabilir, hatta bu diğer yöntemlerin varsayılan bir uygulaması olmasa bile. Böylece bir özellik, pek çok faydalı işlevsellik sağlayabilir ve uygulayıcılardan yalnızca küçük bir kısmını belirtmelerini isteyebilir.

Örneğin, `Summary` özelliğini şu şekilde tanımlayabiliriz:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

Bu sürümü kullanmak için, bir tür üzerinde özelliği uygularken yalnızca `summarize_author` tanımlamamız gerekir:

```rust
impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

Artık `summarize_author` tanımladığımıza göre, `SocialPost` örnekleri üzerinde `summarize` çağırabiliriz; `summarize`’ın varsayılan uygulaması, bizim yazdığımız `summarize_author` tanımını çağıracaktır. Böylece yalnızca `summarize_author` yazarak, ek kod yazmadan `summarize` davranışını kazanmış oluruz:

```rust
let post = SocialPost {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    repost: false,
};

println!("1 new post: {}", post.summarize());
```

Bu kod şu çıktıyı verir:
`1 new post: (Read more from @horse_ebooks...)`

---

Not: Bir yöntemin varsayılan uygulaması, o yöntemin üzerine yazılan uygulama içinden çağrılamaz.

## 🧾 Özellikleri Parametre Olarak Kullanmak (traits as parameters)

Artık özellikleri (trait) nasıl tanımlayıp uygulayacağınızı bildiğinize göre, birçok farklı türü kabul eden fonksiyonları tanımlamak için özellikleri nasıl kullanabileceğimizi inceleyebiliriz. Liste 10-13’te `NewsArticle` ve `SocialPost` türleri üzerinde uyguladığımız `Summary` özelliğini kullanarak, parametresindeki öğe (`item`) üzerinde `summarize` yöntemini çağıran bir `notify` fonksiyonu tanımlayacağız. Bunu yapmak için `impl Trait` (impl Trait syntax) sözdizimini şöyle kullanırız:

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

`item` parametresi için somut bir tür yerine, `impl` anahtar sözcüğünü ve özellik adını belirtiriz. Bu parametre, belirtilen özelliği (trait) uygulayan herhangi bir türü kabul eder. `notify` gövdesinde, `item` üzerinde `Summary` özelliğinden gelen `summarize` gibi herhangi bir yöntemi çağırabiliriz. `notify`’i çağırıp `NewsArticle` veya `SocialPost` örnekleri geçebiliriz. `String` ya da `i32` gibi başka türlerle fonksiyonu çağıran kod ise derlenmez; çünkü bu türler `Summary`’yi uygulamaz.

## 📐 Özellik Sınırı Sözdizimi (trait bound syntax)

`impl Trait` sözdizimi, basit durumlar için çalışır; ancak aslında daha uzun bir biçim olan **özellik sınırı (trait bound)** için bir sözdizimi şekeridir (syntax sugar). Şuna benzer:

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

Bu daha uzun biçim, önceki bölümdeki örnekle eşdeğerdir; yalnızca daha ayrıntılıdır. Genel tür parametresi (generic type parameter) bildirimi içinde, iki nokta üst üste (`:`) sonrasında özellik sınırını yazarız.

`impl Trait` sözdizimi kullanışlıdır ve basit durumlarda daha özlü kod üretir; buna karşın daha kapsamlı **özellik sınırı (trait bound)** sözdizimi, başka durumlarda daha karmaşık ifadeleri anlatabilir. Örneğin, `Summary`’yi uygulayan iki parametre alabiliriz. Bunu `impl Trait` ile şöyle yaparız:

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

`impl Trait` kullanmak, bu fonksiyonun `item1` ve `item2` için farklı türlere (her ikisi de `Summary`’yi uyguladığı sürece) izin vermesini istiyorsak uygundur. Ancak her iki parametrenin de aynı türde olmasını zorlamak istiyorsak, bir **özellik sınırı (trait bound)** kullanmalıyız:

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

Burada genel tür `T`, `item1` ve `item2` parametrelerinin türü olarak belirtilmiştir ve bu, `item1` ile `item2` için geçirilen somut türlerin aynı olmasını zorunlu kılar.

## ➕ Birden Fazla Özellik Sınırı Belirtmek (+ syntax)

Birden fazla özellik sınırı (trait bound) da belirtebiliriz. Diyelim ki `notify`, `item` üzerinde hem **görüntüleme biçimlendirmesi (display formatting)** hem de `summarize` kullanacak; bu durumda `notify` tanımında `item`’ın hem `Display` (Display) hem de `Summary` (Summary) uyguladığını belirtiriz. Bunu `+` sözdizimiyle yapabiliriz:

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

`+` sözdizimi, genel türler üzerindeki özellik sınırlarıyla da geçerlidir:

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

İki özellik sınırı belirtildiğinde, `notify` gövdesi `summarize` çağırabilir ve `{}` kullanarak `item`’ı biçimlendirebilir.

## 🧹 `where` Koşullarıyla Daha Açık Özellik Sınırları (clearer trait bounds with where clauses)

Çok fazla **özellik sınırı (trait bound)** kullanmanın bazı olumsuz yönleri vardır. Her genel türün (generic) kendi özellik sınırları olduğundan, birden çok genel tür parametresi olan fonksiyonlar, fonksiyon adından parametre listesine kadar pek çok özellik sınırı bilgisi içerebilir ve bu da fonksiyon imzasını (function signature) okunması zor hâle getirebilir. Bu nedenle, Rust, özellik sınırlarını fonksiyon imzasından sonra bir **`where` koşulu (where clause)** içinde belirtmek için alternatif bir sözdizimi sunar. Şu şekilde yazmak yerine:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

`where` koşulunu şöyle kullanabiliriz:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

Bu fonksiyonun imzası daha az kalabalıktır: fonksiyon adı, parametre listesi ve dönüş türü, çok sayıda özellik sınırı olmayan bir fonksiyona benzer biçimde birbirine daha yakındır.

## 🔙 Özellikleri Uygulayan Türleri Döndürmek (returning types that implement traits)

Dönüş konumunda **`impl Trait` (impl Trait syntax)** sözdizimini kullanarak, bir özelliği (trait) uygulayan bir türün değerini döndürebiliriz; aşağıda gösterildiği gibi:

```rust
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}
```

Dönüş türü için `impl Summary` kullanarak, `returns_summarizable` fonksiyonunun, somut türü (concrete type) adlandırmadan, `Summary` özelliğini uygulayan bir tür döndürdüğünü belirtiriz. Bu durumda `returns_summarizable`, bir `SocialPost` döndürür; ancak bu fonksiyonu çağıran kodun bunu bilmesine gerek yoktur.

Yalnızca uyguladığı özellikle bir dönüş türü belirtebilme yeteneği, özellikle **kapatmalar (closures)** ve **yineleyiciler (iterators)** bağlamında kullanışlıdır (Bölüm 13’te ele alınacaktır). Kapatmalar ve yineleyiciler, yalnızca derleyicinin bildiği türler veya belirtilmesi çok uzun olan türler oluşturur. `impl Trait` sözdizimi, bir fonksiyonun `Iterator` özelliğini uygulayan bir tür döndürdüğünü, çok uzun bir tür yazmadan özlü biçimde belirtmenizi sağlar.

Ancak, yalnızca tek bir tür döndürüyorsanız `impl Trait` kullanabilirsiniz. Örneğin, dönüş türü `impl Summary` olarak belirtilmişken, bazen bir `NewsArticle` bazen bir `SocialPost` döndüren aşağıdaki kod çalışmaz:

This code does not compile!

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        }
    }
}
```

`impl Trait` sözdiziminin derleyicide nasıl uygulandığına ilişkin kısıtlamalar nedeniyle, ya `NewsArticle` ya da `SocialPost` döndürmek izinli değildir. Bu davranışa sahip bir fonksiyonun nasıl yazılacağını, Bölüm 18’deki “Farklı Türlerden Değerleri Kabul Eden Özellik Nesnelerini (trait objects) Kullanmak” bölümünde ele alacağız.


## 🔒 Özellik Sınırlarını Kullanarak Koşullu Yöntem Uygulamak (using trait bounds to conditionally implement methods)

Genel tür parametreleri (generic type parameters) kullanan bir `impl` bloğunda **özellik sınırları (trait bounds)** belirterek, yalnızca belirtilen özellikleri uygulayan türler için yöntemler tanımlayabiliriz.

Örneğin, Liste 10-15’teki `Pair<T>` türü, her zaman yeni bir `Pair<T>` örneği döndüren `new` fonksiyonunu uygular (Bölüm 5’teki “Yöntemleri Tanımlamak” kısmından hatırlayın: `Self`, o `impl` bloğunun türü için bir takma addır; bu durumda `Pair<T>`). Ancak sonraki `impl` bloğunda, `Pair<T>`, yalnızca iç tür `T` karşılaştırmayı (comparison) sağlayan `PartialOrd` ve yazdırmayı (printing) sağlayan `Display` özelliklerini uygularsa `cmp_display` yöntemini uygular.

**Filename: src/lib.rs**

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

Listing 10-15: Özellik sınırlarına bağlı olarak genel bir tür üzerinde koşullu yöntem uygulamak

---

Ayrıca, başka bir özelliği uygulayan herhangi bir tür için koşullu olarak bir özellik de uygulayabiliriz. Özellik sınırlarını karşılayan herhangi bir tür üzerindeki bu uygulamalara **genel (blanket) uygulamalar** denir ve Rust standart kütüphanesinde yaygın biçimde kullanılır.

Örneğin, standart kütüphane, `Display` özelliğini uygulayan her tür üzerinde `ToString` özelliğini uygular. Standart kütüphanedeki `impl` bloğu şu koda benzer:

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

Standart kütüphane bu genel uygulamaya sahip olduğundan, `Display` özelliğini uygulayan herhangi bir tür üzerinde `ToString` özelliği tarafından tanımlanan `to_string` yöntemini çağırabiliriz. Örneğin, tamsayılar `Display` özelliğini uyguladığı için, tamsayıları karşılık gelen `String` değerlerine dönüştürebiliriz:

```rust
let s = 3.to_string();
```

Genel uygulamalar, ilgili özelliğin belgelerindeki **“Implementors”** bölümünde görünür.

---

Özellikler (traits) ve özellik sınırları (trait bounds), genel tür parametreleriyle yinelenmeyi azaltan kod yazmamıza olanak tanır, ancak aynı zamanda derleyiciye bu genel türün belirli davranışlara sahip olmasını istediğimizi belirtir.

Derleyici, ardından bu özellik sınırı bilgisini kullanarak, kodumuzla kullanılan tüm somut türlerin doğru davranışı sağladığını kontrol edebilir. Dinamik olarak türlenen dillerde, bir türde tanımlanmamış bir yöntemi çağırırsak çalışma zamanında (runtime) hata alırız. Ancak Rust, bu hataları **derleme zamanına (compile time)** taşır; böylece kodumuz çalışmadan önce sorunları düzeltmek zorunda kalırız. Ayrıca, davranış kontrolünü çalışma zamanında yapan kod yazmamıza gerek yoktur çünkü zaten derleme zamanında kontrol yapılmıştır.

Bunu yapmak, esnekliği kaybetmeden genel türlerin (generics) performansını artırır.
