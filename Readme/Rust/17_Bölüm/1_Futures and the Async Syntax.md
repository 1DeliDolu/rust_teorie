## ⏳ Futures ve Async Sözdizimi (Futures and the Async Syntax)

Rust’ta asenkron programlamanın temel öğeleri **futures** ve Rust’ın `async` ile `await` anahtar sözcükleridir.

**Future (gelecek)**, şu anda hazır olmayabilecek fakat ileride bir noktada hazır olacak bir değerdir. (Aynı kavram birçok dilde farklı adlarla görülür; örneğin *task* ya da *promise*.) Rust, farklı asenkron işlemlerin farklı veri yapılarıyla fakat ortak bir arayüz üzerinden uygulanabilmesi için bir yapı taşı olarak `Future` trait’ini sağlar. Rust’ta future, `Future` trait’ini uygulayan (implement eden) türlerdir. Her future, kendi ilerleme bilgisini ve “hazır” olmanın ne anlama geldiğini içerir.

`async` anahtar sözcüğünü bloklara ve fonksiyonlara uygulayarak onların durdurulup tekrar başlatılabileceğini belirtirsiniz. Bir `async` blok veya `async` fonksiyon içerisinde, bir future’ı beklemek için (`hazır` olmasını beklemek) `await` anahtar sözcüğünü kullanabilirsiniz. Bir future üzerinde `await` kullanıldığında, bu `async` blok ya da fonksiyonun duraklatılıp devam ettirilebileceği bir nokta oluşur. Future’ın değerinin hazır olup olmadığını kontrol etme sürecine **polling** denir.

Diğer bazı diller, örneğin **C#** ve **JavaScript**, asenkron programlama için yine `async` ve `await` anahtar sözcüklerini kullanır. Bu dillere aşinaysanız, Rust’ın sözdizimini nasıl ele aldığı da dahil olmak üzere bazı önemli farklılıkları fark edebilirsiniz. Bunun iyi bir nedeni var, ilerleyen kısımlarda bunu göreceğiz!

Asenkron Rust kodu yazarken çoğunlukla `async` ve `await` kullanırız. Rust, bunları `Future` trait’ini kullanarak eşdeğer koda derler; tıpkı `for` döngülerini `Iterator` trait’i ile eşdeğer koda derlemesi gibi. Ancak Rust, `Future` trait’ini sunduğu için, gerektiğinde kendi veri türleriniz için de bu trait’i uygulayabilirsiniz. Bu bölüm boyunca göreceğimiz birçok fonksiyon, kendi `Future` implementasyonlarına sahip türler döndürecektir. Trait’in tanımına bölümün sonunda tekrar döneceğiz ve nasıl çalıştığını daha ayrıntılı inceleyeceğiz, fakat şimdilik ilerlemek için bu kadar bilgi yeterli.

Tüm bunlar biraz soyut gelebilir, bu yüzden ilk asenkron programımızı yazalım: küçük bir web kazıyıcı (web scraper). Komut satırından iki URL alacağız, her ikisini de eşzamanlı (concurrently) çekeceğiz ve hangisi önce biterse onun sonucunu döndüreceğiz. Bu örnekte bir miktar yeni sözdizimi olacak, fakat endişelenmeyin—ilerledikçe bilmeniz gereken her şeyi açıklayacağız.

## 🚀 İlk Async Programımız (Our First Async Program)

Bu bölümün odak noktası ekosistem parçalarıyla uğraşmak değil, `async` öğrenmek olduğu için, **trpl** crate’ini oluşturduk (trpl, *The Rust Programming Language*’in kısaltmasıdır). Bu crate, ihtiyaç duyacağınız tüm türleri, trait’leri ve fonksiyonları yeniden dışa aktarır; çoğunlukla `futures` ve `tokio` crate’lerinden. `futures` crate’i, Rust’ta asenkron kod denemeleri için resmi bir merkezdir ve aslında `Future` trait’i ilk olarak burada tasarlanmıştır. **Tokio**, özellikle web uygulamalarında, günümüzde Rust’ta en çok kullanılan async çalışma zamanıdır (async runtime). Başka iyi çalışma zamanları da vardır ve sizin amaçlarınıza daha uygun olabilir. Biz `trpl` içinde Tokio’yu kullanıyoruz, çünkü çok test edilmiştir ve yaygın olarak kullanılmaktadır.

Bazı durumlarda, `trpl` özgün API’leri yeniden adlandırır veya sarmalar, böylece dikkatiniz bu bölüm için gerekli ayrıntılarda kalır. Crate’in ne yaptığını anlamak istiyorsanız, kaynak kodunu incelemenizi öneririz. Her yeniden dışa aktarmanın hangi crate’ten geldiğini görebilir ve crate’in ne yaptığını açıklayan kapsamlı yorumları inceleyebilirsiniz.

Yeni bir **binary project** oluşturalım ve `trpl` crate’ini bağımlılık olarak ekleyelim:

```bash
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

👉 Bu komutlar yeni bir `hello-async` projesi oluşturur ve `trpl` bağımlılığını ekler.

Artık `trpl`’in sağladığı parçaları kullanarak ilk async programımızı yazabiliriz. Küçük bir komut satırı aracı yapacağız: iki web sayfasını alacak, her birinden `<title>` elementini çıkaracak ve süreci önce bitiren sayfanın başlığını ekrana yazdıracak.

---

## 📝 `page_title` Fonksiyonunun Tanımlanması (Defining the page\_title Function)

Bir sayfa URL’si alan, ona istek yapan ve `<title>` elementinin metnini döndüren bir fonksiyon yazarak başlayalım (Bkz. Liste 17-1).

**Filename: src/main.rs**

```rust
use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}
```

👉 Bu kod, verilen URL’ye istek atar, yanıtı alır ve `<title>` etiketinin içeriğini `Option<String>` olarak döndürür.

Önce `page_title` adlı bir fonksiyon tanımlıyoruz ve onu `async` ile işaretliyoruz. Ardından `trpl::get` fonksiyonunu kullanarak verilen URL’yi çekiyoruz ve yanıtı beklemek için (`await`) kullanıyoruz. Yanıtın metnini almak için `text` metodunu çağırıyoruz ve yine `await` ile bekliyoruz. Her iki adım da asenkron (async).

* `get` fonksiyonu için, sunucunun HTTP başlıklarını, çerezleri ve gövdeden ayrı gelebilecek diğer yanıt parçalarını göndermesini beklememiz gerekir. Yanıt gövdesi çok büyükse tümünün gelmesi zaman alabilir. Bu yüzden `text` metodu da async’tir.
* Rust’ta futures **tembeldir (lazy)**: `await` ile açıkça istemedikçe hiçbir şey yapmazlar. (Hatta future’ı kullanmazsanız derleyici uyarı verir.) Bu durum, Bölüm 13’teki yineleyicilere (iterators) benzer: `next` metodu çağrılmadıkça hiçbir şey yapmazlar. Aynı şekilde, future’lar da açıkça çağrılmadıkça çalışmazlar.

Not: Bu durum, önceki bölümde gördüğümüz `thread::spawn` ile farklıdır. Orada, başka bir iş parçacığına verdiğimiz closure hemen çalışmaya başlıyordu. Rust’ın async yaklaşımı, performans garantileri sağlayabilmesi açısından farklıdır.

`response_text` elde ettikten sonra, onu `Html` tipine ayrıştırıyoruz (`Html::parse`). Artık ham bir string yerine, HTML ile daha zengin şekilde çalışabileceğimiz bir veri türümüz var. Burada `select_first` metodunu kullanarak verilen CSS seçicisine uygun ilk öğeyi bulabiliyoruz. `"title"` parametresi ile belgede ilk `<title>` öğesini alırız (varsa). Eğer öğe yoksa `select_first`, `Option<ElementRef>` döner. Son olarak `Option::map` metodunu kullanarak öğe varsa üzerinde işlem yapar, yoksa hiçbir şey yapmayız. Burada `title_element.inner_html()` çağrısı ile başlığın içeriğini `String` olarak alıyoruz. Böylece fonksiyonumuz `Option<String>` döndürür.

Dikkat edin: Rust’ta `await` anahtar sözcüğü **ifadenin sonuna** gelir, önüne değil. Yani bir postfix anahtar sözcüktür. Diğer dillerde async kullanıyorsanız bu farklı gelebilir, fakat Rust’ta bu yöntem method zincirlemeyi (chaining) daha okunabilir hale getirir. Bu nedenle `page_title` gövdesini zincirleme şekilde şu hale getirebiliriz (Bkz. Liste 17-2):

**Filename: src/main.rs**

```rust
let response_text = trpl::get(url).await.text().await;
```

👉 Burada `get` ve `text` çağrıları zincirlenmiş ve her biri `await` ile beklenmiştir.

Böylece ilk async fonksiyonumuzu yazmış olduk! `main` içinde onu çağırmadan önce, yazdığımız kodun nasıl çalıştığını biraz daha inceleyelim.

---

## ⚙️ `async fn` Ne Döner?

Rust bir bloğu `async` ile işaretlediğinde, onu `Future` trait’ini uygulayan özel, anonim bir veri türüne derler. Rust bir fonksiyonu `async` ile işaretlediğinde ise, aslında gövdesi `async` blok olan normal bir fonksiyona derler.

Yani `async fn`, dönüş türünün bir future olduğu bir fonksiyon yazmakla eşdeğerdir. Derleyici için, Liste 17-1’deki `async fn page_title` tanımı aşağıdaki gibi normal bir fonksiyona eşdeğerdir:

```rust
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

👉 Bu dönüşüm, `async fn`’in aslında `Future` döndüren bir fonksiyon olduğunu gösterir.

Her kısmı inceleyelim:

* `impl Trait` sözdizimi (Bölüm 10’da tartışılmıştı) kullanılıyor.
* Döndürülen tür `Future` olup `Output` ilişkili türü vardır. Buradaki `Output`, `Option<String>`’dir.
* Orijinal fonksiyonun gövdesi `async move` bloğu içine alınmıştır. Bu blok, fonksiyonun dönüş ifadesidir.
* Bu `async` blok, `Option<String>` değer üretir, bu da `Output` türüyle eşleşir.
* Fonksiyon gövdesi `async move`’dur, çünkü `url` parametresini nasıl kullandığımıza bağlıdır. (`async` ile `async move` farkına ileride ayrıntılı değineceğiz.)

Artık `main` fonksiyonunda `page_title` fonksiyonunu çağırabiliriz.

## 📄 Tek Bir Sayfanın Başlığını Belirleme (Determining a Single Page’s Title)

Başlangıç olarak yalnızca tek bir sayfanın başlığını alacağız. Liste 17-3’te, Bölüm 12’deki **Komut Satırı Argümanlarını Alma (Accepting Command Line Arguments)** bölümünde kullandığımız aynı deseni takip ediyoruz. İlk URL’yi `page_title` fonksiyonuna veriyoruz ve sonucu `await` ile bekliyoruz. Future’ın ürettiği değer `Option<String>` olduğundan, sayfada `<title>` olup olmamasına göre farklı mesajlar yazdırmak için `match` ifadesi kullanıyoruz.

**Filename: src/main.rs**

```rust
This code does not compile!
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    match page_title(url).await {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
}
```

👉 Bu kod derlenmez çünkü `main` fonksiyonunu doğrudan `async` olarak işaretleyemeyiz.

**Hata mesajı:**

```
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

Bunun nedeni, `main` fonksiyonunun async işaretlenememesidir. Asenkron kodun bir çalışma zamanına (runtime) ihtiyacı vardır: asenkron kodun nasıl yürütüleceğini yöneten bir Rust crate’i. Bir programın `main` fonksiyonu bir runtime başlatabilir, ancak kendi başına bir runtime değildir. Her Rust programı, async kod çalıştırıyorsa, en az bir yerde runtime başlatıp futures yürütmek zorundadır.

Çoğu dil async desteğini gömülü runtime ile birlikte sunar, fakat Rust sunmaz. Bunun yerine farklı senaryolar için farklı tradeoff’lar yapan birçok async runtime vardır. Örneğin:

* Birden çok CPU çekirdeğine ve bol RAM’e sahip, yüksek throughput’lu bir web sunucusu
* Tek çekirdekli, az RAM’li ve heap tahsisi olmayan bir mikrodenetleyici

Bu kullanım senaryoları çok farklı ihtiyaçlara sahiptir. Runtimeları sağlayan crate’ler genellikle dosya veya ağ I/O gibi yaygın işlevlerin async sürümlerini de sağlar.

Bu bölümde ve kitabın geri kalanında `trpl` crate’indeki `run` fonksiyonunu kullanacağız. `run`, bir future alır ve onu tamamlanana kadar çalıştırır. Arka planda, bir runtime kurar ve verilen future’ı çalıştırır. Future tamamlandığında `run`, ürettiği değeri döndürür.

Liste 17-3’te yaptığımız gibi `page_title`’dan dönen future’ı doğrudan `run`’a verebilirdik. Ancak çoğu örnekte (ve gerçek dünyadaki async kodda) yalnızca tek bir async fonksiyon çağrısı yapmayacağız. Bunun yerine bir `async` blok verip `page_title` sonucunu açıkça `await` edeceğiz (Bkz. Liste 17-4).

**Filename: src/main.rs**

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
```

👉 Bu örnekte `main` normal bir fonksiyon, ancak içinde `trpl::run` çağrısıyla bir async blok çalıştırıyoruz.

**Çalıştırma sonucu:**

```bash
$ cargo run -- https://www.rust-lang.org
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/async_await 'https://www.rust-lang.org'`
The title for https://www.rust-lang.org was
            Rust Programming Language
```

🎉 Artık çalışan bir async kodumuz var!

---

## ⚙️ Futures’ın İşleyişi

Her `await` noktası, yani `await` kullanılan her yer, kontrolün runtime’a devredildiği yerdir. Bunun çalışabilmesi için Rust, async blok içindeki durumu takip etmelidir; böylece runtime başka işleri başlatabilir ve sonra ilk işe geri dönüp onu ilerletmeyi deneyebilir.

Bu, görünmez bir **durum makinesi (state machine)** gibidir. Sanki her `await` noktasındaki durumu kaydetmek için şöyle bir `enum` yazmışsınız gibi:

```rust
enum PageTitleFuture<'a> {
    Initial { url: &'a str },
    GetAwaitPoint { url: &'a str },
    TextAwaitPoint { response: trpl::Response },
}
```

👉 Bu, derleyicinin perde arkasında yaptığı işi temsil eder: `await` noktaları için durum saklamak.

El ile bu geçişleri yazmak yorucu ve hata yapmaya açık olurdu, özellikle kod büyüdükçe ve yeni durumlar eklendikçe. Neyse ki Rust derleyicisi, async kod için bu durum makinesi veri yapılarını otomatik olarak oluşturur ve yönetir. Ayrıca sahiplik (ownership) ve ödünç alma (borrowing) kuralları da geçerliliğini korur; derleyici bunları da kontrol eder ve anlamlı hata mesajları verir.

Sonuçta, bu durum makinesini çalıştıracak bir şeye ihtiyaç vardır; işte bu da runtime’dır. (Runtimelarla ilgili araştırma yaparken “executor” terimini görebilirsiniz. Executor, async kodu çalıştırmaktan sorumlu runtime bileşenidir.)

Şimdi derleyicinin neden Liste 17-3’te `main` fonksiyonunu `async` yapmamıza izin vermediğini görebilirsiniz. Eğer `main` async olsaydı, `main`’in döndürdüğü future’ı çalıştırmak için bir şeyin state machine’i yönetmesi gerekirdi. Ama `main`, programın başlangıç noktasıdır! Bunun yerine `main` fonksiyonunda `trpl::run` çağrısı yaptık ve async bloktan dönen future’ı tamamlanana kadar çalıştırdık.

Not: Bazı runtimelar, `async main` fonksiyonu yazabilmeniz için makrolar sağlar. Bu makrolar `async fn main() { ... }`’i normal bir `fn main` fonksiyonuna çevirir. Bu fonksiyon da bizim Liste 17-4’te yaptığımızı yapar: bir future’ı `run` fonksiyonu ile çalıştırır.

---

Şimdi bu parçaları birleştirelim ve eşzamanlı (concurrent) kodu nasıl yazabileceğimizi görelim.

## 🏁 İki URL’yi Birbirine Karşı Yarıştırma (Racing Our Two URLs Against Each Other)

Liste 17-5’te, komut satırından verilen iki farklı URL için `page_title` fonksiyonunu çağırıyor ve onları yarıştırıyoruz.

**Filename: src/main.rs**

```rust
use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
```

👉 Bu kod, iki URL’yi aynı anda işlemeye başlar ve hangisi önce bitirse onun sonucunu ekrana yazdırır.

---

Önce, kullanıcıdan alınan iki URL için `page_title` fonksiyonunu çağırıyoruz. Dönen future’ları `title_fut_1` ve `title_fut_2` olarak kaydediyoruz. Bunlar henüz hiçbir şey yapmaz, çünkü futures **tembeldir (lazy)**; onları çalıştırmak için `await` gerekir. Daha sonra bu future’ları `trpl::race` fonksiyonuna veriyoruz. Bu fonksiyon, kendisine verilen futures’tan hangisi önce tamamlarsa onun sonucunu döndürür.

**Not:** `race`, aslında daha genel bir fonksiyon olan `select` üzerine inşa edilmiştir. Gerçek dünyadaki Rust kodlarında `select` ile daha sık karşılaşırsınız. `select`, `race`’in yapamadığı birçok şeyi yapabilir, ancak fazladan karmaşıklık getirir. Bu bölümde basitlik için `race` kullanıyoruz.

---

Her iki future da geçerli şekilde “kazanabilir”, bu yüzden bir `Result` döndürmek mantıklı değildir. Bunun yerine `race`, daha önce görmediğimiz bir tür döndürür: `trpl::Either`.

`Either` tipi, `Result`’a biraz benzer; iki varyantı vardır. Ancak `Result`’tan farklı olarak, `Either`’da başarı ya da hata kavramı yoktur. Sadece “ya bu ya da şu”yu ifade eder:

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

* Eğer ilk future önce tamamlarsa, `race` `Left` döndürür.
* Eğer ikinci future önce tamamlarsa, `race` `Right` döndürür.

Bu, fonksiyon çağrısındaki argümanların sırasıyla uyumludur: ilk argüman soldadır (`Left`), ikinci argüman sağdadır (`Right`).

---

Ayrıca `page_title` fonksiyonunu, kendisine verilen URL’yi de döndürecek şekilde güncelledik. Böylece eğer dönen sayfada `<title>` bulunmazsa bile hangi URL’nin önce tamamlandığını ekrana yazdırabiliriz.

Son olarak, `println!` çıktısını güncelleyerek hem hangi URL’nin önce bittiğini hem de varsa sayfanın `<title>` bilgisini ekrana yazdırıyoruz.

---

🎉 Artık küçük, çalışan bir **web scraper** yazdınız!

Komut satırından birkaç URL seçip çalıştırın. Bazı sitelerin sürekli daha hızlı döndüğünü görebilirsiniz; bazı durumlarda ise hangi sitenin daha hızlı olduğu çalıştırmadan çalıştırmaya değişebilir. Daha da önemlisi, artık **futures ile çalışma** temellerini öğrendiniz. Bundan sonra async ile neler yapabileceğimize daha derinlemesine bakacağız.
