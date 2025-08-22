## 🔄 Herhangi Bir Sayıda Gelecek (future) ile Çalışmak

Önceki bölümde iki gelecek (future) yerine üç gelecek kullanmaya geçtiğimizde, `join` yerine `join3` kullanmamız gerekti. Gelecek sayısını değiştirdikçe farklı bir fonksiyon çağırmak zorunda kalmak can sıkıcı olurdu. Neyse ki, istediğimiz kadar argüman alabilen `join!` makrosu mevcut. Bu makro aynı zamanda gelecekleri `await` etmeyi de kendi içinde halleder. Dolayısıyla, Listing 17-13’teki kodu `join3` yerine `join!` ile yeniden yazabiliriz (Listing 17-14).

Filename: src/main.rs

```rust
        trpl::join!(tx1_fut, tx_fut, rx_fut);
```

Listing 17-14: Birden fazla geleceği beklemek için `join!` kullanmak

Bu kesinlikle `join`, `join3`, `join4` vb. arasında gidip gelmekten daha iyidir! Ancak, bu makro formu bile yalnızca gelecek sayısını önceden bildiğimizde çalışır. Gerçek dünya Rust kullanımında ise, gelecekleri (futures) bir koleksiyona koyup bunların bazılarını veya hepsini beklemek oldukça yaygın bir kalıptır.

Bir koleksiyondaki tüm gelecekleri kontrol etmek için, hepsini yineleyip (iterate) üzerinde `join` çalıştırmamız gerekir. `trpl::join_all` fonksiyonu, `Iterator` özelliğini (trait) uygulayan herhangi bir türü kabul eder. (Bu özelliği 13. bölümde, `Iterator` trait ve `next` metodunu işlerken öğrenmiştiniz.) Dolayısıyla bu, tam da işimize yarıyor gibi görünüyor. Hadi, geleceklerimizi bir vektöre koyup `join!` yerine `join_all` kullanalım (Listing 17-15).

Bu kod derlenmiyor!

```rust
        let futures = vec![tx1_fut, rx_fut, tx_fut];

        trpl::join_all(futures).await;
```

Listing 17-15: Anonim gelecekleri bir vektörde saklayıp `join_all` çağırmak

Ne yazık ki bu kod derlenmiyor ve şu hata alınıyor:

```
error[E0308]: mismatched types
   ...
   = note: no two async blocks, even if identical, have the same type
   = help: consider pinning your async block and casting it to a trait object
```

Bu şaşırtıcı olabilir. Sonuçta hiçbir `async` blok bir şey döndürmüyor, yani hepsi `Future<Output = ()>` üretiyor. Ancak unutmayın, `Future` bir trait’tir ve derleyici her `async` blok için benzersiz bir enum oluşturur. Nasıl ki iki farklı elle yazılmış struct’ı `Vec` içine koyamazsınız, aynı kural derleyicinin oluşturduğu farklı enum’lar için de geçerlidir.

Bunu çalıştırmak için, tıpkı 12. bölümde `run` fonksiyonundan hataları döndürürken yaptığımız gibi trait nesneleri (trait objects) kullanmamız gerekir. Trait nesneleri sayesinde, bu anonim geleceklerin hepsini aynı türdeymiş gibi ele alabiliriz; çünkü hepsi `Future` trait’ini uygular.

Not: 8. bölümde, farklı türleri bir `Vec` içinde saklamak için `enum` kullanmayı tartışmıştık. Burada bunu yapamayız, çünkü bu türler anonimdir ve adlandıramayız. Ayrıca, `Vec` ve `join_all` kullanmamızın sebebi zaten dinamik sayıda gelecek ile çalışmak istememizdi; tek önem verdiğimiz şey ise aynı `Output` tipine sahip olmalarıdır.

Önce her future’u `Box::new` ile sarmalayarak başlayalım (Listing 17-16).

Filename: src/main.rs
Bu kod derlenmiyor!

```rust
        let futures =
            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

        trpl::join_all(futures).await;
```

Listing 17-16: `Vec` içindeki future’ların türünü hizalamak için `Box::new` kullanmak

Yine de bu kod derlenmiyor. Hatta, aynı hatayı ikinci ve üçüncü `Box::new` çağrılarında tekrar alıyoruz. Bu hataları çözmek için türü açıkça belirtmemiz gerekir (Listing 17-17).

Filename: src/main.rs
Bu kod derlenmiyor!

```rust
        let futures: Vec<Box<dyn Future<Output = ()>>> =
            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
```

Listing 17-17: Açık tür bildirimi ile tür uyumsuzluğu hatalarını düzeltmek

Bu tür bildirimi şu şekilde çalışır:

* İçteki tür, future’un kendisidir. Çıkış tipinin `()` olduğunu açıkça `Future<Output = ()>` ile belirtiriz.
* Daha sonra trait’i dinamik (`dyn`) olarak işaretleriz.
* Bu trait nesnesini `Box` içine alırız.
* Son olarak, `futures` değişkeninin bu nesneleri içeren bir `Vec` olduğunu açıkça yazarız.

Bu noktada, artık yalnızca `Unpin` ile ilgili hatalar kalır.

Hata mesajı bize `Box<dyn Future<Output = ()>>` için `Unpin` implementasyonu olmadığını söyler ve `pin!` veya `Box::pin` kullanmamızı önerir. Şimdilik derleyicinin önerisini takip edelim (Listing 17-18).

Filename: src/main.rs

```rust
use std::pin::Pin;

// -- snip --

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
```

Listing 17-18: `Pin` ve `Box::pin` kullanarak `Vec` tipini doğru hale getirmek

Bu haliyle derlenip çalışır ve istediğimiz çıktıyı alırız:

```
received 'hi'
received 'more'
received 'from'
...
```

Ama burada ufak bir ek yük (overhead) vardır: `Pin<Box<T>>` kullanmak, heap üzerinde gereksiz bellek ayırır. Aslında heap kullanmamıza gerek yoktur, çünkü bu future’lar sadece bu fonksiyona özeldir. `Pin` zaten bir sarmalayıcı (wrapper) tiptir, yani heap tahsisi olmadan da aynı etkiyi elde edebiliriz. Bunun için `std::pin::pin` makrosunu kullanırız (Listing 17-19).

Filename: src/main.rs

```rust
use std::pin::{Pin, pin};

// -- snip --

        let tx1_fut = pin!(async move {
            // --snip--
        });

        let rx_fut = pin!(async {
            // --snip--
        });

        let tx_fut = pin!(async move {
            // --snip--
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];
```

Listing 17-19: `pin!` makrosunu doğrudan kullanarak gereksiz heap tahsisinden kaçınmak

Buraya kadar `Output` tiplerinin aynı olduğunu varsaydık. Peki farklı tipler olursa? Örneğin Listing 17-20’de, `a` future’u `Future<Output = u32>`, `b` future’u `Future<Output = &str>`, `c` future’u ise `Future<Output = bool>` üretir.

Filename: src/main.rs

```rust
        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
```

Listing 17-20: Farklı çıkış tiplerine sahip üç future

`trpl::join!` burada çalışır çünkü farklı future tiplerini alıp bunların sonuçlarını bir tuple olarak döndürür. Ancak `trpl::join_all` kullanamayız, çünkü o tüm future’ların aynı tipe sahip olmasını ister. İşte bu hata bizi `Pin!` ile bu yolculuğa çıkartan şeydi.

Bu temel bir denge noktasıdır:

* Eğer dinamik sayıda future ile çalışmak istiyorsak, hepsi aynı tipte olmalı (`join_all`).
* Eğer farklı tiplerle çalışıyorsak, sayıyı sabit tutmalıyız (`join` veya `join!`).

Bu Rust’ta diğer türlerle çalışırken karşılaştığımız durumun aynısıdır. Future’lar özel değildir, sadece onlarla çalışmayı kolaylaştıran güzel bir sözdizimi (syntax) vardır — bu da gayet iyi bir şeydir.


## 🏁 Gelecekleri (futures) Yarıştırmak

`join` ailesindeki fonksiyon ve makrolarla gelecekleri (futures) birleştirdiğimizde, devam etmeden önce hepsinin tamamlanmasını bekleriz. Ancak bazen yalnızca bir geleceğin tamamlanması yeterlidir — bu, bir geleceği diğerine karşı yarıştırmaya benzer.

Listing 17-21’de, yine `trpl::race` kullanarak iki future’u, `slow` ve `fast`, birbirine karşı çalıştırıyoruz.

Filename: src/main.rs

```rust
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
```

Listing 17-21: Hangi future önce bitmişse onun sonucunu almak için `race` kullanmak

Her future çalışmaya başladığında bir mesaj yazdırır, ardından `sleep` ile bir süre bekleyip tekrar mesaj yazdırır. Sonrasında her ikisini `trpl::race` fonksiyonuna geçirip biri tamamlanana kadar bekleriz. (Sonuç sürpriz değil: `fast` kazanır.) Burada `race`’in döndürdüğü `Either` örneğini görmezden geliyoruz, çünkü ilginç olan şeyler zaten `async` blokların gövdesinde oluyor.

Eğer `race` fonksiyonundaki argümanların sırasını değiştirirseniz, “started” mesajlarının sırası da değişir, ama `fast` her zaman önce tamamlanır. Bunun nedeni bu `race` fonksiyonunun adil (fair) olmamasıdır. Argümanları hangi sırayla verdiyseniz, o sırayla çalıştırır. Bazı başka implementasyonlar adildir ve ilk hangi future’u yoklayacaklarını rastgele seçerler. Ancak hangi implementasyonu kullanırsanız kullanın, bir future gövdesindeki ilk `await` noktasına kadar çalıştırılır.

İlk `async` programımızdan hatırlayın: her `await` noktasında, eğer beklenen future hazır değilse, Rust çalışma zamanına (runtime) görevi duraklatıp başka birine geçme şansı verir. Tersi de doğrudur: Rust yalnızca `await` noktalarında kontrolü geri verir. Aradaki her şey senkron çalışır.

Bu da şu anlama gelir: Bir `async` blok içinde `await` olmadan çok iş yaparsanız, o future diğer future’ların ilerlemesini engeller. Buna bazen bir future’un diğerlerini aç bırakması denir. Küçük işler için sorun olmayabilir, ama eğer pahalı kurulumlar, uzun süren işler veya sonsuza dek sürecek görevler varsa, kontrolü ne zaman ve nerede çalışma zamanına geri vereceğinizi düşünmeniz gerekir.

Aynı şekilde, uzun süre bloklayan işlemler varsa, `async` programlama farklı kısımların birbirleriyle etkileşime girmesi için faydalı bir araç olabilir.

Ama böyle durumlarda kontrolü çalışma zamanına nasıl geri verirsiniz?

---

## ⏸️ Çalışma Zamanına Kontrol Vermek

Uzun süren bir işlemi simüle edelim. Listing 17-22, yavaş bir fonksiyon tanıtıyor.

Filename: src/main.rs

```rust
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
```

Listing 17-22: Yavaş işlemleri simüle etmek için `thread::sleep` kullanmak

Bu kod `trpl::sleep` yerine `std::thread::sleep` kullanır, yani `slow` çağrıldığında mevcut iş parçacığı belirli bir süre tamamen bloklanır. Böylece hem uzun süren hem de bloklayan gerçek dünya işlemlerini taklit edebiliriz.

Listing 17-23’te, `slow` fonksiyonunu iki future içinde CPU’ya yük bindiren işler gibi kullandık.

Filename: src/main.rs

```rust
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
```

Listing 17-23: Yavaş işlemleri simüle etmek için `thread::sleep` kullanmak

Başta her future kontrolü runtime’a ancak bir dizi yavaş işlemi tamamladıktan sonra geri verir. Çalıştırdığınızda şöyle bir çıktı görürsünüz:

```
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

Önce `a` çalışır, sonra `b`, sonra tekrar `a`. İşler arasında hiçbir geçiş yoktur çünkü `await` noktası yoktur. Bu işleri birbirine karışacak şekilde ilerletebilmek için `await` noktaları gerekir. Yani bekleyebileceğimiz bir şeye ihtiyacımız var.

Listing 17-24’te her `slow` çağrısından sonra kısa bir `trpl::sleep` ekliyoruz.

Filename: src/main.rs

```rust
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };
```

Listing 17-24: İşlerin ilerlemesini sırayla paylaşmak için `sleep` kullanmak

Bu sefer çıktı şu şekilde karışır:

```
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

Artık future’lar sırayla ilerliyor. Fakat aslında uyumak istemiyoruz, sadece kontrolü runtime’a vermek istiyoruz. Bunu doğrudan `yield_now` fonksiyonu ile yapabiliriz.

Listing 17-25’te tüm `sleep` çağrılarını `yield_now` ile değiştirdik.

Filename: src/main.rs

```rust
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };
```

Listing 17-25: İşlerin ilerlemesini sırayla paylaşmak için `yield_now` kullanmak

Bu kod hem daha niyetimizi açıkça ifade eder, hem de `sleep`’ten çok daha hızlıdır. Çünkü `sleep` her zaman en az 1 ms bekler, ama bilgisayarlar 1 ms içinde çok iş yapabilir.

Bunu göstermek için küçük bir kıyas testi yapalım (Listing 17-26).

Filename: src/main.rs

```rust
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );
```

Listing 17-26: `sleep` ve `yield_now` performansını karşılaştırmak

Burada her biri tek başına 1000 kez çalıştırılır ve süre ölçülür. Sonuç: `yield_now` versiyonu çok daha hızlıdır!

---

## ⚙️ Sonuç

Bu, `async`’in yalnızca IO değil, CPU-yoğun işler için de faydalı olabileceğini gösterir. Çünkü farklı kısımlar arasındaki ilişkiyi yapılandırmak için bize araç sağlar. Bu, işbirlikçi çoklu görev (cooperative multitasking) biçimidir: her future `await` noktasıyla kontrolü ne zaman bırakacağına kendisi karar verir. Ama aynı zamanda çok uzun süre bloklamama sorumluluğu da vardır. Bazı Rust tabanlı gömülü işletim sistemlerinde bu tek çoklu görev biçimidir!

Elbette gerçek kodlarda her satıra `await` koymazsınız. Kontrolü bırakmak ucuzdur ama bedava değildir. Bazen işleri bölmek performansı kötüleştirebilir, bu yüzden bazen kısa bir bloklama daha verimlidir. Her zaman ölçüm yapmak gerekir. Ancak akılda tutulması gereken şey şudur: eğer işler beklediğinizden seri ilerliyorsa, belki de yeterince `await` noktası eklememişsinizdir.


## 🛠️ Kendi Asenkron Soyutlamalarımızı (async abstractions) İnşa Etmek

Gelecekleri (futures) birleştirerek yeni kalıplar oluşturabiliriz. Örneğin, elimizdeki asenkron yapı taşlarını kullanarak bir zaman aşımı (timeout) fonksiyonu yazabiliriz. Sonuçta elde ettiğimiz şey, başka asenkron soyutlamalar üretmek için de kullanabileceğimiz yeni bir yapı taşı olacaktır.

Listing 17-27’de bu zaman aşımı fonksiyonunun yavaş bir future ile nasıl çalışmasını beklediğimizi gösteriyoruz.

Filename: src/main.rs

```rust
        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
```

Listing 17-27: Zaman sınırıyla yavaş bir işlemi çalıştırmak için hayali `timeout` fonksiyonunu kullanmak

Bunu implement edelim! Öncelikle `timeout` fonksiyonunun API’si hakkında düşünelim:

* Kendisi `async` bir fonksiyon olmalı, böylece onu `await` edebiliriz.
* İlk parametresi çalıştırılacak future olmalı. Bunu generik yaparak herhangi bir future ile çalışmasını sağlayabiliriz.
* İkinci parametresi maksimum bekleme süresi olmalı. `Duration` kullanırsak, bunu kolayca `trpl::sleep`’e verebiliriz.
* Bir `Result` döndürmeli: future başarıyla tamamlarsa `Ok` içinde sonucu döndürür; eğer zaman aşımı olursa `Err` içinde beklenen süreyi döndürür.

Listing 17-28 bu bildirimi gösteriyor.

Filename: src/main.rs

```rust
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    // Implementasyon buraya gelecek!
}
```

Listing 17-28: `timeout` fonksiyonunun imzasını tanımlamak

Türler açısından hedefimize ulaştık. Şimdi davranışı düşünelim: geçilen future’u süreye karşı yarıştırmak istiyoruz. Bunun için `trpl::sleep` ile bir sayaç future’u oluşturup, bunu `future_to_try` ile birlikte `trpl::race` fonksiyonuna verebiliriz.

`race` fonksiyonunun adil (fair) olmadığını biliyoruz: argümanları verilen sırayla yoklar. Bu nedenle `future_to_try`’ı ilk sıraya koyarız, böylece çok kısa sürelerde bile şansı olur. Eğer `future_to_try` önce tamamlarsa `race` bize `Left` döndürür. Eğer sayaç önce biterse, `race` bize `Right(())` döndürür.

Listing 17-29’da `trpl::race` sonucunu `match` ile ele alıyoruz.

Filename: src/main.rs

```rust
use trpl::Either;

// --snip--

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
```

Listing 17-29: `race` ve `sleep` kullanarak `timeout` tanımlamak

Eğer `future_to_try` başarılı olursa ve `Left(output)` dönerse, `Ok(output)` döndürürüz. Eğer sayaç önce dolarsa ve `Right(())` dönerse, `()` değerini `_` ile yok sayar, bunun yerine `Err(max_time)` döndürürüz.

Böylece, iki yardımcı asenkron yapıdan oluşan çalışan bir `timeout` fonksiyonumuz oldu. Çalıştırdığımızda zaman aşımı gerçekleşir:

```
Failed after 2 seconds
```

Gelecekler (futures) birbirleriyle birleşebildiği için, küçük asenkron yapı taşlarından güçlü araçlar üretebilirsiniz. Örneğin, bu yaklaşımı yeniden denemeler (retry) ile birleştirebilir, sonra bunları ağ çağrıları gibi işlemlerle birlikte kullanabilirsiniz.

Pratikte, genellikle doğrudan `async` ve `await` ile çalışırsınız; ikinci olarak da `join`, `join_all`, `race` gibi fonksiyon ve makrolarla. Yalnızca bu API’lerle çalışmak için ara sıra `pin` kullanmanız gerekir.

Artık aynı anda birden fazla future ile çalışmanın çeşitli yollarını gördük. Sırada, birden fazla future ile **zamana yayılmış sırayla** çalışmanın yolları var: akışlar (streams). Ancak önce şunları da düşünebilirsiniz:

* `join_all` ile bir `Vec` içindeki tüm future’ları bekledik. Peki bir `Vec` içindeki future’ları sırayla işlemek için nasıl kullanabilirsiniz? Bunun artıları ve eksileri nelerdir?
* `futures` kütüphanesindeki `futures::stream::FuturesUnordered` tipine bakın. Bunu kullanmak `Vec` kullanmaktan nasıl farklı olurdu? (Kütüphanenin stream kısmından geliyor olsa da, herhangi bir future koleksiyonu ile çalışır.)
