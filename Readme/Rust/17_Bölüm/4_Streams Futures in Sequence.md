## 🌊 Akışlar (Streams): Sıralı Gelecekler (Futures in Sequence)

Bu bölüme kadar çoğunlukla tekil future’lar ile çalıştık. Bunun tek büyük istisnası, kullandığımız asenkron kanal (async channel) idi. Hatırlayın, bu kanaldaki alıcıyı (“Message Passing” kısmında) nasıl kullandığımızı. Asenkron `recv` metodu, zaman içinde öğelerin sıralı bir şekilde gelmesini sağlıyordu. Bu, daha genel bir desenin örneğidir: **stream (akış)**.

Bölüm 13’te, `Iterator` trait’ini ve `next` metodunu incelerken de sıralı öğeler görmüştük, ancak iki fark vardır:

1. **Zaman**: İteratörler eşzamanlıdır (synchronous), kanal alıcısı ise asenkron çalışır.
2. **API**: `Iterator` ile doğrudan eşzamanlı `next` metodunu çağırırız. `trpl::Receiver` stream’i ile ise eşzamansız `recv` metodunu çağırdık.

Bunun dışında API’ler oldukça benzerdir — bu da tesadüf değildir. Bir stream, **asenkron bir yineleme (iteration)** gibidir. `trpl::Receiver` özelinde mesaj almayı bekler, ancak genel amaçlı stream API’si daha geniştir: İteratörün `next` yaptığı şeyi asenkron olarak yapar.

Rust’ta iteratörler ile stream’ler arasındaki bu benzerlik sayesinde, herhangi bir iteratörden stream oluşturabiliriz. Tıpkı iteratörlerde olduğu gibi stream ile de `next` metodunu çağırabilir ve sonucunu `await` edebiliriz (Listing 17-30).

Filename: src/main.rs
Bu kod derlenmiyor!

```rust
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
```

Listing 17-30: Bir iteratörden stream oluşturmak ve değerlerini yazdırmak

Başlangıçta bir sayı dizisi alıp iteratöre dönüştürüyoruz, ardından `map` ile her değeri ikiyle çarpıyoruz. Sonra `trpl::stream_from_iter` ile bu iteratörü stream’e çeviriyoruz. Ardından `while let` döngüsüyle stream’den gelen öğeleri yazdırıyoruz.

Ama bu kod derlenmiyor ve şu hata çıkıyor:

```
error[E0599]: no method named `next` found for struct `Iter` in the current scope
   ...
help: the following traits which provide `next` are implemented but not in scope
  + use crate::trpl::StreamExt;
```

Buradaki sorun, `next` metodunu kullanabilmek için doğru trait’in scope’a alınmamış olmasıdır. Mantıken bunun `Stream` trait’i olduğunu düşünebilirsiniz, ama aslında `StreamExt` trait’idir. Rust topluluğunda `Ext`, bir trait’i başka bir trait ile genişletmek için yaygın kullanılan bir isimlendirme kalıbıdır.

`Stream` trait’i düşük seviyeli arayüzü tanımlar (temelde `Iterator` ve `Future` karışımı gibi). `StreamExt` ise bunun üstüne `next` ve benzeri yardımcı metodları ekler. Henüz Rust standart kütüphanesinin parçası olmasalar da, ekosistemde çoğu crate aynı tanımı kullanır.

Derleme hatasını düzeltmek için `use trpl::StreamExt;` eklememiz yeterlidir (Listing 17-31).

Filename: src/main.rs

```rust
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
```

Listing 17-31: Bir iteratörü stream’e dönüştürüp `StreamExt` ile başarıyla kullanmak

Artık kod çalışıyor! Üstelik `StreamExt` scope’ta olduğundan, tüm yardımcı metodlarını da kullanabiliriz. Örneğin, Listing 17-32’de `filter` metodunu kullanarak sadece üç ve beşin katlarını geçiriyoruz.

Filename: src/main.rs

```rust
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered =
            stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}
```

Listing 17-32: `StreamExt::filter` metodu ile stream’i filtrelemek

Tabii ki bu örnek çok ilginç değil, çünkü aynı şeyi normal iteratörlerle ve hiç `async` kullanmadan da yapabilirdik. Şimdi stream’lere özgü, gerçekten farklı neler yapabileceğimize bakalım.

## 🔗 Akışları Birleştirme (composing streams)

Pek çok kavram doğal olarak akışlar (streams) ile temsil edilir: bir kuyruğa öğelerin gelmesi, tüm veri kümesi bilgisayarın belleği için fazla büyük olduğunda dosya sisteminden verinin artımlı olarak çekilen parçaları, ya da verinin zaman içinde ağ üzerinden gelmesi. Akışlar gelecekler (futures) olduğundan, onları diğer her tür gelecekle birlikte kullanabilir ve ilginç şekillerde birleştirebiliriz. Örneğin, çok fazla ağ çağrısını tetiklememek için olayları toplu işleyebilir (batch), uzun süren işlemler dizilerine zaman aşımı (timeout) koyabilir veya gereksiz işi önlemek için kullanıcı arayüzü olaylarını kısıtlayabilir (throttle)iz.

Başlangıç olarak, bir WebSocket ya da başka bir gerçek zamanlı iletişim protokolünden görebileceğimiz veri akışının yerine geçecek küçük bir mesaj akışı (stream) oluşturalım (Listing 17-33).

Filename: src/main.rs

```rust
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
```

Listing 17-33: `rx` alıcısını (receiver) bir `ReceiverStream` olarak kullanmak

Önce `impl Stream<Item = String>` döndüren `get_messages` adlı bir fonksiyon oluşturuyoruz. Uygulamasında bir asenkron kanal (async channel) oluşturuyor, İngiliz alfabesinin ilk 10 harfi üzerinde döngü yapıyor ve bunları kanal üzerinden gönderiyoruz.

Ayrıca yeni bir tür kullanıyoruz: `ReceiverStream`. Bu tür, `trpl::channel` içindeki `rx` alıcısını bir `next` metoduna sahip bir `Stream`’e dönüştürür. `main` içinde ise bir `while let` döngüsüyle akıştan gelen tüm mesajları yazdırıyoruz.

Kodu çalıştırdığımızda tam da beklediğimiz sonuçları alırız:

```
Message: 'a'
Message: 'b'
Message: 'c'
Message: 'd'
Message: 'e'
Message: 'f'
Message: 'g'
Message: 'h'
Message: 'i'
Message: 'j'
```

Bunu normal `Receiver` API’siyle ya da hatta normal yineleyici (iterator) API’siyle de yapabilirdik; bu yüzden akışların gerekli olduğu bir özellik ekleyelim: akıştaki her öğeye uygulanan bir zaman aşımı (timeout) ve yayımladığımız öğelere bir gecikme (delay) ekleyelim (Listing 17-34).

Filename: src/main.rs

```rust
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages =
            pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}
```

Listing 17-34: Bir akıştaki öğelere zaman sınırı koymak için `StreamExt::timeout` metodunu kullanmak

Önce `StreamExt` trait’inden gelen `timeout` yöntemiyle akışa bir zaman aşımı ekliyoruz. Sonra `while let` gövdesini güncelliyoruz, çünkü akış artık bir `Result` döndürüyor. `Ok` varyantı mesajın zamanında geldiğini, `Err` varyantı ise zaman aşımının, herhangi bir mesaj gelmeden önce dolduğunu gösterir. Sonucu eşleştirip, başarıyla aldığımızda mesajı, aksi halde zaman aşımı bildirimini yazdırıyoruz. Son olarak, `timeout` yardımcı işlevi yoklanabilmesi için pin’lenmesi gereken bir akış ürettiğinden, `timeout` uygulandıktan sonra `messages`’ı pin’lediğimize dikkat edin.

Ancak mesajlar arasında gecikme olmadığı için, bu zaman aşımı programın davranışını değiştirmez. O halde gönderdiğimiz mesajlara değişken bir gecikme ekleyelim (Listing 17-35).

Filename: src/main.rs

```rust
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
```

Listing 17-35: `get_messages` bir async fonksiyon yapılmadan, `tx` üzerinden mesajları asenkron gecikmeyle göndermek

`get_messages` içinde, `messages` dizisiyle `enumerate` yineleyici metodunu kullanarak her öğeyle birlikte öğenin indeksini de alıyoruz. Ardından, gerçek dünyadaki bir mesaj akışında görebileceğimiz farklı gecikmeleri simüle etmek için çift indeksli öğelere 100 ms, tek indeksli öğelere 300 ms gecikme uyguluyoruz. Zaman aşımı 200 ms olduğundan, bu durum mesajların yarısını etkilemelidir.

`get_messages` fonksiyonunda mesajlar arasında bekleme yapmak için engellemeden (blocking) uyumamız gerekir; bunun için de asenkronluk gerekir. Ancak `get_messages`’ı bir async fonksiyon yapamayız; çünkü o zaman `Stream<Item = String>` yerine `Future<Output = Stream<Item = String>>` döndürürüz. Çağıran, akışa erişmek için `get_messages`’ı beklemek (await) zorunda kalır. Fakat unutmayın: belirli bir geleceğin (future) içindeki her şey doğrusal olarak gerçekleşir; eşzamanlılık (concurrency) gelecekler arasında ortaya çıkar. `get_messages`’ı beklemek, alıcı akışını döndürmeden önce tüm mesajları — her bir mesaj arasındaki `sleep` gecikmeleri dâhil — göndermesini gerektirir. Sonuç olarak zaman aşımı işe yaramaz hale gelir; gecikmeler akışta değil, akış hazır olmadan önce gerçekleşirdi.

Bunun yerine, `get_messages`’ı bir akış döndüren normal bir fonksiyon olarak bırakıyoruz ve asenkron `sleep` çağrılarını işlemek için bir görev (task) başlatıyoruz (spawn).

Not: Bu şekilde `spawn_task` çağırmak, çalışma zamanını (runtime) zaten kurmuş olduğumuz için çalışır; aksi halde panic olurdu. Diğer implementasyonlar farklı ödünler (tradeoff) seçebilir: yeni bir runtime başlatıp panikten kaçınabilir ama biraz ek yük getirir; ya da bir runtime’a referans olmadan bağımsız görev oluşturma yolu hiç sunmayabilir. Kullandığınız runtime’ın hangi ödünü seçtiğini bildiğinizden emin olun ve kodunuzu buna göre yazın!

Artık kodumuz çok daha ilginç bir sonuç üretir. Her iki mesajdan biri arasında `Problem: Elapsed(())` hatası görünür.

```
Message: 'a'
Problem: Elapsed(())
Message: 'b'
Message: 'c'
Problem: Elapsed(())
Message: 'd'
Message: 'e'
Problem: Elapsed(())
Message: 'f'
Message: 'g'
Problem: Elapsed(())
Message: 'h'
Message: 'i'
Problem: Elapsed(())
Message: 'j'
```

Zaman aşımı, mesajların eninde sonunda gelmesini engellemez. Kanalımız sınırsız (unbounded) olduğundan, belleğe sığdığı sürece istediğimiz kadar mesaj tutabiliriz; bu nedenle tüm özgün mesajları yine alırız. Mesaj zaman aşımı süresi içinde gelmezse akış işleyicimiz bunu not eder; ancak akışı tekrar yokladığında (poll), mesaj gelmiş olabilir.

Gerekirse farklı davranışlar elde etmek için başka kanal türleri ya da daha genel olarak başka akış türleri kullanabilirsiniz. Bunu uygulamada görmek için, zaman aralıklarından oluşan bir akış ile bu mesaj akışını birleştirelim.

## 🔀 Akışları Birleştirme (merging streams)

Öncelikle, her milisaniyede bir öğe üreten başka bir akış oluşturalım. Basitlik için, `sleep` fonksiyonunu kullanarak gecikmeli bir mesaj gönderebilir ve `get_messages`’ta yaptığımız gibi bir kanaldan akış üretebiliriz. Fark şu ki, bu kez geriye geçen aralıkların sayacını döndüreceğiz. Dolayısıyla dönüş tipi `impl Stream<Item = u32>` olacak ve fonksiyonumuzun adı `get_intervals` olacak (Listing 17-36).

Filename: src/main.rs

```rust
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
```

Listing 17-36: Her milisaniyede bir sayaç değeri gönderen bir akış oluşturmak

Görevde (task) bir sayaç tanımlıyoruz. Ardından sonsuz bir döngü oluşturuyoruz. Her iterasyonda bir milisaniye uyuyor, sayacı artırıyor ve kanaldan gönderiyoruz. Tüm bunlar `spawn_task` tarafından başlatılan görev içinde olduğundan, runtime kapatıldığında bu döngü de temizlenir.

Bu türden sonsuz döngüler, yalnızca runtime sonlandığında biten yapılar, async Rust’ta yaygındır: birçok program süresiz çalışmalıdır. Async sayesinde, her iterasyonda en az bir `await` noktası olduğu sürece bu başka işleri engellemez.

Şimdi, `main` fonksiyonundaki async blokta `messages` ve `intervals` akışlarını birleştirmeyi deneyelim (Listing 17-37).

Filename: src/main.rs
Bu kod derlenmiyor!

```rust
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals();
        let merged = messages.merge(intervals);
```

Listing 17-37: `messages` ve `intervals` akışlarını birleştirmeyi denemek

`get_intervals`’i çağırıyoruz. Sonra `merge` metodu ile `messages` ve `intervals` akışlarını birleştiriyoruz. `merge`, öğeleri kaynak akışlardan herhangi birinde hazır olur olmaz üreten tek bir akış döndürür, özel bir sıralama dayatmaz. Son olarak, `messages` yerine bu birleşik akış üzerinde döngü kurmayı planlıyoruz.

Bu noktada, `messages` ya da `intervals`’ın pin’lenmesine veya mutable olmasına gerek yoktur, çünkü her ikisi birleşik akışa katılacaktır. Ancak `merge` çağrısı derlenmez! Bunun sebebi akışların farklı türlere sahip olmasıdır: `messages` akışı `Timeout<impl Stream<Item = String>>` tipindedir, `intervals` akışı ise `impl Stream<Item = u32>` tipindedir. Birleştirmek için tiplerini hizalamamız gerekir. `messages` zaten istediğimiz formatta olduğundan, `intervals` akışını dönüştürürüz (Listing 17-38).

Filename: src/main.rs

```rust
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals);
        let mut stream = pin!(merged);
```

Listing 17-38: `intervals` akışının türünü `messages` akışına hizalamak

Önce, `map` metoduyla `intervals`’ı string’e dönüştürüyoruz. Sonra `messages`’taki `Timeout`’a uyması için `intervals`’a da bir `timeout` ekliyoruz. Burada aslında `intervals` için zaman aşımı istemiyoruz, ama 10 saniyelik uzun bir `timeout` vererek hizalamayı sağlıyoruz. Son olarak, birleşik akışı `mut` yapıyoruz ve `pin!` ile sabitliyoruz. Artık tipler uyumlu hale gelir. Ancak iki sorun kalır: program asla durmaz (ctrl-c ile durdurmak gerekir) ve alfabe mesajları interval mesajları arasında kaybolur.

Listing 17-39 bu iki soruna çözüm gösteriyor.

Filename: src/main.rs

```rust
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);
```

Listing 17-39: Birleştirilmiş akışları yönetmek için `throttle` ve `take` kullanmak

Önce `intervals` akışına `throttle` ekliyoruz; böylece `messages`’ı boğmaz. `Throttle`, bir fonksiyonun ne sıklıkla çağrılacağını veya bu durumda bir akışın ne sıklıkla yoklanacağını sınırlar. 100 milisaniyede bir yeterlidir, çünkü mesajlarımız yaklaşık bu sıklıkta gelir.

Sonra `take` metodunu birleşik akışa uyguluyoruz. Böylece toplamda yalnızca 20 öğe alırız. Program bu noktadan sonra durur.

Artık çıktımız şöyle olur:

```
Interval: 1
Message: 'a'
Interval: 2
Interval: 3
Problem: Elapsed(())
Interval: 4
Message: 'b'
Interval: 5
Message: 'c'
...
```

Burada dikkat edilmesi gereken: `throttle` yeni bir akış döndürür, bu akış orijinal akışı yalnızca throttle hızında yoklar. Yani atlanan interval mesajları aslında hiç üretilmez. Bu Rust future’larının doğasındaki “tembellik”tir (laziness).

Son olarak, **hatalar**ı ele almalıyız. Kanal tabanlı akışlarda, alıcı kapandığında gönderimler başarısız olabilir. Şimdiye kadar `unwrap` ile bunu yok saydık, ama gerçek bir uygulamada döngüyü sonlandırarak bu hatayı işlemeliyiz. Listing 17-40 basit bir hata stratejisi gösteriyor.

Filename: src/main.rs

```rust
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
```

Listing 17-40: Hataları işlemek ve döngüleri kapatmak

Her zamanki gibi, mesaj gönderme hatasını ele almanın doğru yolu duruma göre değişir; ancak en azından bir stratejiye sahip olmak gerekir.

Artık pek çok async örneği gördük. Şimdi biraz geri çekilip, Rust’ın `Future`, `Stream` ve asenkron çalışmayı mümkün kılan diğer önemli trait’lerini daha ayrıntılı inceleyelim.
