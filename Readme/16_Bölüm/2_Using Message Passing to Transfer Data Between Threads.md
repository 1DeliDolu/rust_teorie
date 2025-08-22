## ✉️ İş Parçacıkları Arasında Veri Aktarmak için Mesaj Geçişi (message passing) Kullanma

Güvenli eşzamanlılığı sağlamak için giderek daha popüler hale gelen yaklaşımlardan biri mesaj geçişidir (message passing). Bu modelde iş parçacıkları (threads) veya aktörler (actors), veri içeren mesajlar göndererek birbirleriyle iletişim kurar. Go dilinin dokümantasyonundan bir slogan bu fikri şöyle özetler:
“Belleği paylaşarak iletişim kurmayın; iletişim kurarak belleği paylaşın.”

Mesaj tabanlı eşzamanlılığı gerçekleştirmek için Rust standart kütüphanesi `channel` (kanal) implementasyonu sağlar. Kanal, bir iş parçacığından diğerine veri göndermeyi sağlayan genel bir programlama kavramıdır.

Kanalı bir akarsuya benzetebilirsiniz. Örneğin, bir lastik ördeği nehre bırakırsanız, suyun akışı onu aşağıya doğru taşır ve sonunda nehrin ucuna ulaştırır.

Bir kanal iki parçadan oluşur: **gönderici (transmitter)** ve **alıcı (receiver)**. Gönderici kısmı, ördeği suya bıraktığınız yukarı akımdır; alıcı kısmı ise ördeğin ulaştığı aşağı akımdır. Kodunuzun bir bölümü, göndermek istediğiniz verilerle gönderici üzerinde metotlar çağırır; başka bir bölüm ise alıcı tarafı kontrol ederek mesajları alır. Eğer gönderici veya alıcıdan biri bırakılırsa (drop), kanal kapatılmış (closed) olur.

Burada, bir iş parçacığının değerler üretip kanala göndereceği ve başka bir iş parçacığının bu değerleri alıp yazdıracağı bir program yazacağız. Başlangıçta, özellikleri göstermek için basit değerleri iş parçacıkları arasında aktaracağız. Bu tekniği kavradığınızda, örneğin bir sohbet sistemi ya da hesaplamanın parçalarını yapan birçok iş parçacığının sonuçlarını birleştiren merkezi bir iş parçacığı gibi daha karmaşık senaryolarda da kanalları kullanabilirsiniz.

---

## 🆕 İlk Kanalı Oluşturma

İlk olarak Örnek 16-6’da bir kanal oluşturacağız, ancak henüz onunla hiçbir şey yapmayacağız. Bu kod şu anda derlenmeyecek çünkü Rust, kanal üzerinden hangi türde veriler göndermek istediğimizi anlayamıyor.

Filename: `src/main.rs`
Bu kod derlenmez!

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

Listing 16-6: Bir kanal oluşturma ve iki ucunu `tx` ile `rx` değişkenlerine atama

Yeni bir kanal `mpsc::channel` fonksiyonu ile oluşturulur. `mpsc`, **multiple producer, single consumer** (çoklu üretici, tek tüketici) ifadesinin kısaltmasıdır. Yani Rust’ın standart kütüphanedeki kanal implementasyonu, bir kanalın birçok gönderici ucu olabileceğini (değer üreten), ama yalnızca tek bir alıcı ucu olabileceğini (değer tüketen) belirtir. Bunu, birçok küçük akarsuyun birleşerek tek bir büyük nehre akması gibi düşünebilirsiniz.

`mpsc::channel` fonksiyonu bir ikili (tuple) döndürür: ilk eleman gönderici uç (transmitter), ikinci eleman alıcı uç (receiver)’dur. Gönderici için `tx`, alıcı için `rx` adlarının kullanımı pek çok alanda geleneksel olduğundan, değişkenlerimizi bu şekilde adlandırırız. Burada `let` ifadesiyle tuple parçalarını desen eşleme (destructuring) kullanarak ayırıyoruz; `let` ifadelerinde desenlerin kullanımını Bölüm 19’da tartışacağız. Şimdilik bilin ki bu yöntem, `mpsc::channel`’ın döndürdüğü tuple’ın parçalarını çıkarmanın uygun bir yoludur.

---

## 🚀 Gönderici Ucu İş Parçacığına Taşımak

Şimdi gönderici ucunu (`tx`) oluşturulan bir iş parçacığına taşıyıp bir string göndereceğiz. Bu durumda oluşturulan iş parçacığı ana iş parçacığıyla iletişim kurmuş olacak. Bu, yukarı akıma lastik ördek bırakmaya ya da bir iş parçacığından diğerine sohbet mesajı göndermeye benzer.

Filename: `src/main.rs`

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
```

Listing 16-7: `tx`’yi oluşturulan iş parçacığına taşıyıp `"hi"` mesajı göndermek

Burada yine `thread::spawn` kullanarak yeni bir iş parçacığı oluşturuyoruz ve `move` ile `tx`’yi kapanışa taşıyarak oluşturulan iş parçacığının `tx`’nin sahibi olmasını sağlıyoruz. Mesaj gönderebilmek için iş parçacığının göndericinin sahibi olması gerekir.

Göndericinin `send` metodu vardır. Bu metod, göndermek istediğimiz değeri alır. `send` metodu `Result<T, E>` döndürür; yani alıcı taraf daha önce bırakılmışsa ve gönderilecek bir hedef kalmamışsa, `send` bir hata döndürür. Burada örnek basit olduğundan `unwrap` kullanıyoruz, hata durumunda panik olacak. Gerçek uygulamalarda ise Bölüm 9’da incelediğimiz hata yönetimi stratejileri kullanılmalıdır.

---

## 📥 Ana İş Parçacığında Mesajı Alma

Örnek 16-8’de, alıcı ucundan (receiver) değeri ana iş parçacığında alacağız. Bu, nehrin sonunda ördeği yakalamaya ya da sohbet mesajı almaya benzer.

Filename: `src/main.rs`

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```

Listing 16-8: `"hi"` değerini ana iş parçacığında alıp yazdırmak

Alıcı ucunun iki kullanışlı metodu vardır: `recv` ve `try_recv`.

* `recv` (receive’in kısaltması), ana iş parçacığını bloke eder ve kanaldan bir değer gönderilene kadar bekler. Değer gönderildiğinde, `recv` bunu `Result<T, E>` içinde döndürür. Gönderici kapandığında ise `recv` artık değer gelmeyeceğini bildiren bir hata döndürür.
* `try_recv` ise bloke etmez, hemen `Result<T, E>` döndürür: Eğer mesaj mevcutsa `Ok` ile, yoksa `Err` ile. Bu, iş parçacığının mesaj beklerken başka işler yapması gerektiğinde faydalıdır. Örneğin periyodik olarak `try_recv` çağırabilir, mesaj varsa işleyebilir, yoksa başka işler yapmaya devam edip sonra tekrar kontrol edebilirsiniz.

Bu örnekte sadelik adına `recv` kullandık; çünkü ana iş parçacığının yapacak başka işi yok, bu yüzden onu bloke etmek uygundur.

Bu kodu çalıştırdığımızda şu çıktıyı görürüz:

```
Got: hi
```

Mükemmel! ✅

## 🔑 Kanallar ve Sahiplik Aktarımı (channels and ownership transference)

Sahiplik (ownership) kuralları, mesaj gönderiminde çok önemli bir rol oynar çünkü güvenli, eşzamanlı kod yazmanıza yardımcı olur. Eşzamanlı programlamada hataları önlemek, Rust programlarınızda sahiplik kavramını her yerde düşünmenin avantajıdır. Kanallar ve sahipliğin nasıl birlikte çalışarak sorunları önlediğini göstermek için küçük bir deney yapalım: Bir değeri (`val`) kanaldan gönderdikten sonra oluşturulan iş parçacığında tekrar kullanmaya çalışalım. Örnek 16-9’daki kodu derlemeyi deneyin, neden bu koda izin verilmediğini göreceksiniz.

Filename: `src/main.rs`
Bu kod derlenmez!

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```

Listing 16-9: Bir değeri (`val`) kanaldan gönderdikten sonra kullanmayı denemek

Burada `val` değerini `tx.send` ile kanaldan gönderdikten sonra yazdırmaya çalışıyoruz. Buna izin verilseydi büyük bir sorun olurdu: Değer başka bir iş parçacığına geçtiğinde, o iş parçacığı bu değeri değiştirebilir veya düşürebilir (drop). Daha sonra tekrar kullanmaya çalıştığımızda tutarsız ya da geçersiz veriler yüzünden hatalar veya beklenmedik sonuçlar ortaya çıkabilirdi. Ancak Rust bu kodu derlemeye çalıştığınızda bir hata verir:

```
error[E0382]: borrow of moved value: `val`
...
   |         tx.send(val).unwrap();
   |                 --- value moved here
   |         println!("val is {val}");
   |                          ^^^^^ value borrowed here after move
```

Bu eşzamanlılık hatamız, derleme zamanı (compile time) hatasına dönüştü. Çünkü `send` fonksiyonu parametresinin sahipliğini alır ve değer gönderildiğinde alıcı iş parçacığı sahip olur. Böylece gönderdikten sonra değeri yanlışlıkla tekrar kullanmamızı engeller. Sahiplik sistemi, her şeyin yolunda olup olmadığını kontrol eder.

---

## 🔄 Birden Fazla Değer Gönderme ve Alıcının Beklemesini Görmek

Örnek 16-8’deki kod derlenip çalıştı ama iki ayrı iş parçacığının kanal üzerinden iletişim kurduğunu net olarak göstermedi. Örnek 16-10’da bazı değişiklikler yaptık: Artık oluşturulan iş parçacığı birden fazla mesaj gönderecek ve her bir mesaj arasında bir saniye bekleyecek.

Filename: `src/main.rs`

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
```

Listing 16-10: Birden fazla mesaj göndermek ve her birinin arasında beklemek

Bu sefer, oluşturulan iş parçacığında ana iş parçacığına göndermek istediğimiz stringlerden oluşan bir vektör var. Bunları tek tek gönderiyoruz ve her gönderim arasında `thread::sleep` ile bir saniye bekliyoruz.

Ana iş parçacığında ise artık `recv` fonksiyonunu doğrudan çağırmıyoruz. Bunun yerine `rx`’i bir yineleyici (iterator) gibi kullanıyoruz. Her yeni değer geldiğinde yazdırıyoruz. Kanal kapandığında yineleme (iteration) sona eriyor.

Çalıştırıldığında şu çıktıyı göreceksiniz (satırlar arasında birer saniye bekleme ile):

```
Got: hi
Got: from
Got: the
Got: thread
```

Ana iş parçacığında döngüde herhangi bir gecikme olmadığı için, ana iş parçacığının gerçekten oluşturulan iş parçacığından gelen mesajları beklediğini anlayabiliyoruz.

---

## 👥 Göndericiyi Klonlayarak Çoklu Üreticiler (multiple producers) Oluşturmak

Başta `mpsc`’nin "multiple producer, single consumer" kısaltması olduğunu söylemiştik. Şimdi bunu uygulayalım ve Örnek 16-10’daki kodu genişleterek aynı alıcıya (receiver) mesaj gönderen birden fazla iş parçacığı oluşturalım. Bunu yapmak için göndericiyi (transmitter) `clone` edebiliriz. Örnek 16-11’de gösterildiği gibi:

Filename: `src/main.rs`

```rust
// --snip--

let (tx, rx) = mpsc::channel();

let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {received}");
}

// --snip--
```

Listing 16-11: Birden fazla üreticiden (producer) bir alıcıya (receiver) mesaj göndermek

Bu defa, ilk iş parçacığını oluşturmadan önce göndericiyi `clone` ediyoruz. Bu, bize ilk iş parçacığına verebileceğimiz yeni bir gönderici sağlar. Orijinal göndericiyi ise ikinci iş parçacığına veriyoruz. Böylece her biri farklı mesajlar gönderen iki iş parçacığına sahip oluyoruz.

Çalıştırıldığında çıktı aşağıdaki gibi görünebilir:

```
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

Sisteminizin davranışına göre değerler farklı bir sırada görünebilir. İşte bu, eşzamanlılığı hem ilginç hem de zor yapan şeydir. Eğer `thread::sleep` sürelerini farklı iş parçacıklarında değiştirirseniz, her çalıştırmada daha fazla belirsizlik (nondeterminism) ortaya çıkar ve her seferinde farklı çıktılar üretir.

---

Artık kanalların nasıl çalıştığını gördük. Şimdi eşzamanlılığa farklı bir yöntemle bakalım.

