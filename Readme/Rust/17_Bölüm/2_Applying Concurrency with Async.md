## ⚡ Async ile Eşzamanlılık Uygulama (Applying Concurrency with Async)

Bu bölümde, Bölüm 16’da iş parçacıkları (threads) ile ele aldığımız bazı eşzamanlılık (concurrency) zorluklarına `async` uygulayacağız. Orada birçok temel fikri zaten konuştuğumuz için, burada odak noktamız iş parçacıkları (threads) ile gelecekler (futures) arasındaki farklar olacaktır.

Pek çok durumda, `async` kullanarak eşzamanlılıkla çalışmaya yönelik API’ler, iş parçacıklarını kullanmaya yönelik API’lere oldukça benzer. Bazı durumlarda ise oldukça farklıdırlar. API’ler iş parçacıkları ve `async` arasında benzer görünse bile, çoğu zaman davranışları farklıdır—ve neredeyse her zaman performans özellikleri farklıdır.

## 🛠️ `spawn_task` ile Yeni Bir Görev Oluşturma (Creating a New Task with spawn\_task)

**Yeni Bir İş Parçacığı Başlatma (Creating a New Thread with Spawn)** bölümünde ele aldığımız ilk işlem, iki ayrı iş parçacığında sayım yapmaktı. Aynısını `async` kullanarak yapalım. `trpl` crate’i, `thread::spawn` API’sine çok benzeyen bir `spawn_task` fonksiyonu ve `thread::sleep` API’sinin asenkron sürümü olan bir `sleep` fonksiyonu sağlar. Bunları, Liste 17-6’da gösterildiği gibi sayma örneğini uygulamak için birlikte kullanabiliriz.

**Filename: src/main.rs**

```rust
use std::time::Duration;

fn main() {
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
```

👉 Üst düzey `async` bloğunu `trpl::run` ile çalıştırıyor, bir görevi `spawn_task` ile başlatıyor ve her iki döngüde de yarım saniyelik gecikmeleri `await` ederek bekliyoruz.

Liste 17-6: Ana görev bir şeyler yazdırırken başka bir şeyi yazdırmak için yeni bir görev oluşturma

Başlangıç noktası olarak, üst düzey fonksiyonumuzun `async` olabilmesi için `main` fonksiyonumuzu `trpl::run` ile ayarladık.

Not: Bu noktadan itibaren bölümün geri kalanındaki her örnek, `main` içinde `trpl::run` ile aynı sarmalama kodunu içerecektir; bu yüzden `main`’i çoğunlukla atlayacağız. Kendi kodunuza eklemeyi unutmayın!

Sonra bu blok içinde, her biri yarım saniye (500 milisaniye) bekleyen bir `trpl::sleep` çağrısı içeren iki döngü yazıyoruz. Döngülerden birini `trpl::spawn_task` gövdesine, diğerini de üst düzey `for` döngüsüne koyuyoruz. `sleep` çağrılarından sonra `await` de ekliyoruz.

Bu kod, iş parçacığı tabanlı uygulamayla benzer davranır—kendi terminalinizde çalıştırdığınızda iletilerin farklı sıralarda görünebilmesi dahil:

```
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

Bu sürüm, `spawn_task` ile başlatılan görevin ana fonksiyon bitince kapatılmasından dolayı, üst düzey `async` bloğundaki `for` döngüsü biter bitmez durur. Görevin tamamen tamamlanana kadar çalışmasını istiyorsanız, ilk görevin tamamlanmasını beklemek için bir **join handle (join handle)** kullanmanız gerekir. İş parçacıklarında, iş parçacığı bitene kadar “bloklamak” için `join` metodunu kullanmıştık. Liste 17-7’de, aynı şeyi yapabiliriz; çünkü görev tanıtıcısının (task handle) kendisi bir **future (future)**’dır. Onun `Output` türü bir **Result (Result)** olduğundan, `await` ettikten sonra `unwrap` ederiz.

**Filename: src/main.rs**

```rust
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
```

👉 `spawn_task`’in döndürdüğü görev tanıtıcısını `await` ederek görevin tamamlanmasını bekliyor ve sonucu `unwrap` ediyoruz.

Liste 17-7: Bir görevi tamamına kadar çalıştırmak için join handle ile `await` kullanma

Güncellenmiş bu sürüm, her iki döngü bitene kadar çalışır:

```
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Şu ana kadar, `async` ile iş parçacıklarının (threads) bize farklı sözdizimleriyle aynı temel çıktıları verdiği görülüyor: join handle üzerinde `join` çağırmak yerine `await` kullanmak ve `sleep` çağrılarını `await` etmek gibi.

Asıl büyük fark, bunu yapmak için başka bir işletim sistemi iş parçacığı başlatmamıza gerek olmamasıdır. Hatta burada bir görev (task) başlatmamıza bile gerek yok. `async` bloklar anonim **future (future)**’lara derlendiğinden, her döngüyü bir `async` bloğa koyabilir ve çalışma zamanının (runtime) her ikisini de `trpl::join` fonksiyonunu kullanarak tamamına kadar çalıştırmasını sağlayabiliriz.

**Tüm İş Parçacıklarının Bitmesini join Handles Kullanarak Bekleme (Waiting for All Threads to Finishing Using join Handles)** bölümünde, `std::thread::spawn` çağrıldığında dönen `JoinHandle` türü üzerindeki `join` metodunu nasıl kullanacağımızı göstermiştik. `trpl::join` fonksiyonu benzer şekilde çalışır, ancak **future (future)**’lar için. İki future verdiğinizde, her ikisi tamamlandığında çıktıları bir demet (tuple) olarak içeren tek bir yeni future üretir. Bu nedenle, Liste 17-8’de `trpl::join`’u kullanarak `fut1` ve `fut2`’nin bitmesini bekliyoruz. `fut1` ve `fut2`’yi ayrı ayrı `await` etmiyoruz; bunun yerine `trpl::join`’un ürettiği yeni future’ı `await` ediyoruz. Çıktıyı yok sayıyoruz, çünkü yalnızca iki birim değer (unit values) içeren bir tuple.

**Filename: src/main.rs**

```rust
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
```

👉 İki anonim `async` bloğunu `trpl::join` ile birlikte bekliyoruz; her ikisi de bittiğinde `await` tamamlanır.

Liste 17-8: İki anonim future’ı beklemek için `trpl::join` kullanma

Bunu çalıştırdığımızda, her iki future’ın da tamamlandığını görürüz:

```
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Şimdi, her seferinde tamamen aynı sırayı göreceksiniz ki bu, iş parçacıklarıyla gördüğümüzden çok farklıdır. Bunun sebebi, `trpl::join` fonksiyonunun adil (fair) olmasıdır; yani her future’ı eşit sıklıkta kontrol eder, aralarında dönüşümlü olarak ilerler ve diğeri hazırsa birinin öne geçmesine izin vermez. İş parçacıklarında hangi iş parçacığının kontrol edileceğine ve ne kadar süre çalışacağına işletim sistemi karar verir. `async` Rust’ta ise hangi görevin kontrol edileceğine çalışma zamanı (runtime) karar verir. (Uygulamada ayrıntılar karmaşıklaşır; çünkü bir async çalışma zamanı, eşzamanlılığı yönetmek için kaputun altında işletim sistemi iş parçacıklarını kullanabilir; adilliği garanti etmek runtime için daha fazla iş olabilir—ama yine de mümkündür!) Çalışma zamanlarının belirli bir işlem için adilliği garanti etmesi gerekmez ve genellikle adillik isteyip istemediğinizi seçebilmeniz için farklı API’ler sunarlar.

Aşağıdaki future’ları bekleme (awaiting) varyasyonlarından bazılarını deneyin ve ne yaptıklarını görün:

* Döngülerin birinin veya her ikisinin etrafındaki `async` bloğunu kaldırın.
* Her `async` bloğunu tanımladıktan hemen sonra `await` edin.
* Yalnızca ilk döngüyü bir `async` bloğa alın ve oluşan future’ı ikinci döngünün gövdesinden sonra `await` edin.

Ek bir meydan okuma olarak, kodu çalıştırmadan önce her durumda çıktının ne olacağını tahmin etmeye çalışın!

## 📨 Mesaj Geçişi Kullanarak İki Görevde Sayım Yapma (Counting Up on Two Tasks Using Message Passing)

Future’lar (futures) arasında veri paylaşımı da tanıdık gelecektir: yine **mesaj geçişi (message passing)** kullanacağız, ancak bu kez türlerin ve işlevlerin asenkron sürümleriyle. Bu bölümde, **İş Parçacıkları Arasında Veriyi Mesaj Geçişiyle Aktarma (Using Message Passing to Transfer Data Between Threads)** bölümünde izlediğimiz yoldan biraz farklı ilerleyerek, iş parçacığı temelli ve future temelli eşzamanlılık arasındaki bazı temel farkları göstereceğiz. Liste 17-9’da, ayrı bir iş parçacığı (thread) başlatmak bir yana, ayrı bir görev (task) bile başlatmadan, tek bir **asenkron blok (async block)** ile başlıyoruz.

**Filename: src/main.rs**

```rust
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
```

Liste 17-9: Asenkron bir kanal oluşturup iki yarıyı `tx` ve `rx` değişkenlerine atama

Burada, Bölüm 16’da iş parçacıklarıyla kullandığımız **çoklu-üretici, tek-tüketici kanal (multiple-producer, single-consumer channel)** API’sinin asenkron sürümü olan `trpl::channel`’ı kullanıyoruz. API’nin asenkron sürümü, iş parçacığı temelli sürümden yalnızca biraz farklıdır: alıcı `rx` değişkeni değiştirilebilir (mutable) olur ve `recv` metodu değeri doğrudan üretmek yerine beklememiz (await) gereken bir **future (future)** döndürür. Artık göndericiden alıcıya mesajlar gönderebiliriz. Dikkat edin: Ayrı bir iş parçacığı ya da görev başlatmamıza gerek yok; yalnızca `rx.recv` çağrısında **beklememiz (await)** yeterli.

Senkron `std::mpsc::channel` içindeki `Receiver::recv` metodu, bir mesaj alana kadar **engelleyici (blocking)** çalışır. `trpl::Receiver::recv` metodu ise **asenkron (async)** olduğundan engellemez. Engellemek yerine, bir mesaj gelene veya kanalın gönderim (send) tarafı kapanana kadar kontrolü çalışma zamanına (**runtime (runtime)**) geri verir. Buna karşılık, gönderim (`send`) çağrısında beklemeyiz; çünkü engellemez. Engellemesine gerek yoktur; zira gönderdiğimiz kanal **sınırsız (unbounded)** bir kanaldır.

Not: Tüm bu asenkron kod bir `trpl::run` çağrısındaki **asenkron blok (async block)** içinde çalıştığı için, blok içindeki her şey engellemeden çalışabilir. Ancak dışarıdaki kod, `run` fonksiyonunun dönmesini bekler ve **engellenir (block)**. `trpl::run` fonksiyonunun amacı budur: bir grup asenkron kod üzerinde **nerede engelleneceğinizi** seçmenize ve böylece senkron-asenkron geçişini nerede yapacağınıza karar vermenize olanak tanır. Çoğu asenkron çalışma zamanında, bu nedenle `run` yerine fonksiyonun adı **`block_on` (block\_on)**’dur.

Bu örnekte iki şeye dikkat edin. Birincisi, mesaj hemen ulaşacaktır. İkincisi, burada future kullansak da henüz **eşzamanlılık (concurrency)** yoktur. Listede olan her şey, future’lar yokmuş gibi **ardışık (sequential)** şekilde gerçekleşir.

İlk kısmı ele almak için, mesajları bir dizi halinde gönderip aralarına **uyku (sleep)** koyacağız; Liste 17-10’a bakın.

**Filename: src/main.rs**

```rust
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
```

Liste 17-10: Asenkron kanaldan birden çok mesaj gönderip almak ve her mesaj arasında `await` ile uyumak

Mesajları göndermenin yanı sıra almamız da gerekir. Bu durumda kaç mesaj geleceğini bildiğimiz için, teoride `rx.recv().await` çağrısını dört kez tekrarlayarak bunu elle yapabiliriz. Ancak gerçek dünyada genellikle **bilinmeyen sayıda mesaj** bekliyor olacağız; bu yüzden artık mesaj kalmadığını anlayana kadar beklemeyi sürdürmemiz gerekir.

Bölüm 16’daki Liste 16-10’da, senkron bir kanaldan alınan tüm öğeleri işlemek için `for` döngüsü kullanmıştık. Rust’ta, henüz **asenkron öğe dizisi (asynchronous series of items)** üzerinde `for` yazmanın bir yolu yoktur; bu nedenle daha önce görmediğimiz bir döngüye ihtiyacımız var: **`while let` koşullu döngüsü (while let conditional loop)**. Bu, Bölüm 6’daki **`if let` (if let)** yapısının döngü sürümüdür. Belirlediği desen (pattern) değeri eşleştirdiği sürece döngü çalışmayı sürdürür.

`rx.recv` çağrısı bir **future (future)** üretir; biz de onu bekleriz (`await`). Çalışma zamanı, future hazır olana kadar onu duraklatır. Mesaj geldiğinde, future her mesaj gelişinde `Some(message)` olarak çözümlenir. Kanal kapandığında ise (mesaj gelmiş olsa da olmasa da), future `None` döndürerek artık başka değer olmadığını ve **anketlemeyi (polling)**—yani beklemeyi—bırakmamız gerektiğini belirtir.

`while let` döngüsü tüm bunları bir araya getirir. `rx.recv().await` çağrısının sonucu `Some(message)` ise, mesaja erişir ve tıpkı `if let`’te olduğu gibi gövde içinde kullanırız. Sonuç `None` olduğunda döngü biter. Her tur tamamlandığında yeniden bir `await` noktasına gelinir; çalışma zamanı, başka bir mesaj gelene kadar döngüyü yine duraklatır.

Kod artık mesajların tamamını başarıyla gönderip alıyor. Ne yazık ki hâlâ birkaç sorun var. Birincisi, mesajlar **yarım saniye aralıklarla** gelmiyor; programı başlattıktan **2 saniye (2000 milisaniye)** sonra hepsi birden geliyor. İkincisi, program hiç bitmiyor! Bunun yerine sonsuza kadar yeni mesajları bekliyor. Kapatmak için `ctrl-c` kullanmanız gerekecek.

Mesajların neden her birinin arasında gecikme olacak yerde, tam süre dolduktan sonra birden geldiğini inceleyerek başlayalım. Belirli bir **asenkron blok (async block)** içinde, `await` anahtar sözcüklerinin kodda göründüğü sıra, program çalıştığında yürütüldükleri sıra ile aynıdır.

Liste 17-10’da yalnızca **bir** asenkron blok vardır; bu yüzden içindekilerin tamamı **doğrusal (lineer)** çalışır. Hâlâ **eşzamanlılık (concurrency)** yoktur. Tüm `tx.send` çağrıları, aralarına serpiştirilmiş `trpl::sleep` çağrıları ve onların `await` noktaları ile gerçekleşir. Ancak **ondan sonra** `while let` döngüsü, `recv` çağrılarındaki `await` noktalarından geçme fırsatı bulur.

İstediğimiz davranışa—her mesaj arasında uyku gecikmesi olmasına—ulaşmak için `tx` ve `rx` işlemlerini kendi **asenkron blokları (async blocks)** içine koymamız gerekir; Liste 17-11’e bakın. Sonrasında çalışma zamanı, sayma örneğinde olduğu gibi, her birini ayrı ayrı `trpl::join` kullanarak çalıştırabilir. Yine, **tek tek future’ları değil**, `trpl::join` çağrısının döndürdüğü **yeni future’ı** `await` ederiz. Future’ları sırayla `await` etseydik, yeniden **ardışık (sequential)** akışa dönmüş olurduk; tam da kaçınmak istediğimiz şey.

**Filename: src/main.rs**

```rust
        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
```

Liste 17-11: `send` ve `recv` işlemlerini kendi asenkron bloklarına ayırıp bu blokların future’larını beklemek

Liste 17-11’deki güncel kodla, mesajlar **2 saniye dolduktan sonra bir anda** değil, **500 milisaniyelik aralıklarla** yazdırılır.

Yine de program bitmiyor; bunun nedeni `while let` döngüsünün `trpl::join` ile etkileşim şeklidir:

* `trpl::join` tarafından döndürülen future, yalnızca kendisine verilen iki future’ın **ikisi de** tamamlandığında tamamlanır.
* `tx` future’ı, `vals` içindeki son mesajı gönderdikten sonra uyumayı bitirdiğinde tamamlanır.
* `rx` future’ı, `while let` döngüsü bitene kadar tamamlanmaz.
* `while let` döngüsü, `rx.recv` beklemesinin `None` döndürmesiyle sona erer.
* `rx.recv` beklemesi, ancak kanalın diğer ucu kapandığında `None` döndürür.
* Kanal, `rx.close` çağrılırsa ya da gönderici taraf (**tx**) düşürülür (**drop**) ise kapanır.
* Hiçbir yerde `rx.close` çağırmıyoruz ve `tx`, `trpl::run`’a verilen en dıştaki asenkron blok bitene kadar düşmeyecek.
* Bu blok bitemez; çünkü `trpl::join` tamamlanana kadar beklemektedir. Bu da bizi listenin başına geri götürür.

`rx`’i bir yerde `rx.close` çağırarak elle kapatabiliriz; ancak bu çok mantıklı değildir. Rastgele sayıda mesaj işledikten sonra durmak programı kapatır; fakat mesajları kaçırabiliriz. Bunun yerine, fonksiyonun bitiminden önce `tx`’in düşürüldüğünden (drop) emin olmanın başka bir yoluna ihtiyacımız var.

Şu anda, mesajları gönderdiğimiz asenkron blok, `tx`’i yalnızca **ödünç almaktadır (borrow)**; çünkü mesaj göndermek sahiplik (**ownership**) gerektirmez. Ancak `tx`’i bu asenkron bloğun \*\*içine taşıyabilir (move)\*\*sek, blok bittiğinde düşer (drop edilir). Bölüm 13’teki **Başvuruları Yakalamak veya Sahipliği Taşımak (Capturing References or Moving Ownership)** kısmında `move` anahtar sözcüğünü **kapatmalar (closures)** ile nasıl kullanacağınızı öğrenmiştiniz ve Bölüm 16’daki **İş Parçacıklarıyla `move` Kapatmaları Kullanma (Using move Closures with Threads)** kısmında iş parçacıklarıyla çalışırken veriyi kapatmalara taşımamız gerektiğini görmüştük. Aynı temel dinamikler **asenkron bloklar (async blocks)** için de geçerlidir; bu yüzden `move` anahtar sözcüğü asenkron bloklarda da kapatmalarda olduğu gibi çalışır.

Liste 17-12’de, mesajları gönderen bloğu `async`’ten `async move`’a çeviriyoruz. Bu sürümü çalıştırdığımızda, son mesaj gönderilip alındıktan sonra program düzgün şekilde kapanır.

**Filename: src/main.rs**

```rust
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
```

Liste 17-12: Liste 17-11’deki kodun, tamamlandığında düzgün şekilde kapanan düzeltilmiş sürümü

Bu asenkron kanal aynı zamanda **çoklu üreticili (multiple-producer)** bir kanaldır; bu nedenle birden fazla future’dan mesaj göndermek istiyorsak, `tx` üzerinde `clone` çağırabiliriz; Liste 17-13’te gösterildiği gibi.

**Filename: src/main.rs**

```rust
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
```

Liste 17-13: Asenkron bloklarla birden çok üretici kullanma

Önce `tx`’i kopyalayıp (`clone`) ilk asenkron bloğun dışında `tx1` oluşturuyoruz. `tx1`’i, tıpkı az önce `tx` ile yaptığımız gibi, o bloğun içine **taşıyoruz (move)**. Daha sonra, orijinal `tx`’i yeni bir asenkron bloğa taşıyıp, biraz daha yavaş bir gecikmeyle daha fazla mesaj gönderiyoruz. Bu yeni asenkron bloğu, alım yapan asenkron bloğun **sonrasına** koyduk; fakat **öncesine** de koyabilirdik. Önemli olan, future’ların **hangi sırayla bekletildiği (await)**; **hangi sırayla oluşturuldukları** değil.

Mesaj gönderen her iki asenkron bloğun da `async move` olması gerekir; böylece `tx` ve `tx1`, bu bloklar bittiğinde **düşer (drop edilir)**. Aksi takdirde, başladığımız aynı sonsuz döngüye geri döneriz. Son olarak, ek future’ı da beklemek için `trpl::join` yerine `trpl::join3`’e geçiyoruz.

Artık her iki gönderici future’dan gelen tüm mesajları görüyoruz ve gönderici future’lar gönderimden sonra biraz farklı gecikmeler kullandığı için, mesajlar da bu farklı aralıklarla alınıyor:

```
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

Bu iyi bir başlangıç, ancak bizi yalnızca sınırlı sayıda future ile sınırlar: `join` ile iki, `join3` ile üç. Şimdi daha fazla future ile nasıl çalışabileceğimize bakalım.
