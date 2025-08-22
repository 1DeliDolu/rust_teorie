## 🧵 Tek İş Parçacıklı Sunucuyu Çok İş Parçacıklı Sunucuya Dönüştürmek (turning our single-threaded server into a multithreaded server)

Şu anda sunucu, her isteği sırayla işliyor; yani ilk isteğin işlenmesi bitmeden ikinci bağlantıyı işlemiyor. Sunucu giderek daha fazla istek alırsa, bu **seri yürütme** (serial execution) giderek daha az verimli olur. İşlenmesi uzun süren bir istek gelirse, ardından gelen istekler hızlı işlenebilir olsalar bile bu uzun isteğin bitmesini bekler. Bunu düzeltmemiz gerekecek, ancak önce sorunu uygulamada görelim.

---

## 🐌 Mevcut Sunucu Uygulamasında Yavaş Bir İstek Simülasyonu (simulating a slow request in the current server implementation)

Yavaş işlenen bir isteğin, mevcut sunucu uygulamamızdaki diğer istekleri nasıl etkilediğine bakalım. Liste 21-10, `/sleep` yoluna yapılan isteği beş saniyelik bir gecikmeyle simüle eder: sunucu, yanıt vermeden önce beş saniye uyur.

**Dosya adı:** src/main.rs

```rust
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
// --snip--

fn handle_connection(mut stream: TcpStream) {
    // --snip--

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // --snip--
}
```

Liste 21-10: 5 saniye uyutarak yavaş bir isteği simüle etmek

Üç durumumuz olduğu için `if` yerine `match` kullanıyoruz. `match`, eşitlik yönteminin yaptığı gibi otomatik referans alma/bırakma (automatic referencing/dereferencing) yapmadığından, desen eşleştirmeyi (pattern matching) dize sabitleriyle yapabilmek için `request_line`’ın bir dilimini (slice) açıkça eşleştiriyoruz.

* İlk kol, Liste 21-9’daki `if` bloğuyla aynıdır.
* İkinci kol, `/sleep` isteğini yakalar; bu istek alındığında sunucu, başarılı HTML sayfasını döndürmeden önce beş saniye uyur.
* Üçüncü kol, Liste 21-9’daki `else` bloğuna denktir.

Bu örnek, sunucumuzun ne kadar ilkel olduğunu gösterir: gerçek kütüphaneler, birden fazla isteği tanımayı çok daha az ayrıntıyla halleder!

Sunucuyu `cargo run` ile başlatın. Ardından iki tarayıcı penceresi açın: biri `http://127.0.0.1:7878/`, diğeri `http://127.0.0.1:7878/sleep`. `/` URI’sini birkaç kez girerseniz hızlı yanıt alırsınız. Ancak `/sleep` girdikten sonra `/`’u yüklemeye çalışırsanız, `/` yanıtının, uyku isteği beş saniyesini tamamen doldurana kadar beklediğini görürsünüz.

Ardından gelen isteklerin yavaş bir isteğin arkasında birikmesini önlemenin birden çok tekniği vardır (Bölüm 17’deki gibi **async** kullanmak dâhil); bizim uygulayacağımız yöntem bir **iş parçacığı havuzu** (thread pool) olacaktır.

## 🚀 İş Parçacığı Havuzuyla Aktarımı Artırma (improving throughput with a thread pool)

**İş parçacığı havuzu (thread pool)**, bir görevi ele almak için bekleyen ve hazır durumda tutulan oluşturulmuş iş parçacıkları grubudur. Program yeni bir görev (task) aldığında, havuzdaki iş parçacıklarından biri bu göreve atanır ve görevi işler. Havuzdaki kalan iş parçacıkları, ilk iş parçacığı görevini işlerken gelen diğer görevleri ele almak üzere hazır bekler. İlk iş parçacığı görevini tamamladığında, yeni bir görevi ele almaya hazır hâlde **boşta iş parçacıkları** havuzuna geri döner. Bir iş parçacığı havuzu, bağlantıları eşzamanlı (concurrently) işlemenize imkân tanıyarak sunucunuzun **işleme hacmini (throughput)** artırır.

Havuzdaki iş parçacığı sayısını, bizi **hizmet aksatma saldırılarına (DoS attacks)** karşı korumak için küçük bir sayıyla sınırlayacağız; eğer programımız her istek geldiğinde yeni bir iş parçacığı oluştursaydı, sunucumuza 10 milyon istek yapan biri, tüm sunucu kaynaklarımızı tüketip istek işlemesini durma noktasına getirerek karmaşaya yol açabilirdi.

Sınırsız iş parçacığı oluşturmak yerine, havuzda bekleyen sabit sayıda iş parçacığımız olacak. Gelen istekler, işlenmek üzere havuza gönderilecek. Havuz, gelen isteklerden oluşan bir **kuyruk (queue)** tutacak. Havuzdaki her iş parçacığı, bu kuyruktan bir isteği alacak, isteği işleyecek ve ardından kuyruktan bir sonraki isteği talep edecek. Bu tasarımla, **N** iş parçacığı varsa aynı anda en fazla **N** isteği işleyebiliriz. Her iş parçacığı uzun süren bir isteğe yanıt veriyorsa, sonraki istekler kuyrukta birikmeye devam edebilir; ancak o noktaya ulaşmadan önce işleyebildiğimiz uzun süreli istek sayısını artırmış oluruz.

Bu teknik, bir web sunucusunun işleme hacmini artırmanın birçok yolundan sadece biridir. İnceleyebileceğiniz diğer seçenekler arasında **çatalla/birleştir (fork/join) modeli (fork/join model)**, **tek iş parçacıklı eşzamansız G/Ç modeli (single-threaded async I/O model)** ve **çok iş parçacıklı eşzamansız G/Ç modeli (multithreaded async I/O model)** bulunur. Bu konu ilginizi çekiyorsa, diğer çözümler hakkında daha fazla okuyabilir ve bunları uygulamayı deneyebilirsiniz; **Rust** gibi düşük seviyeli (low-level) bir dilde, bu seçeneklerin hepsi mümkündür.

Bir iş parçacığı havuzunu uygulamaya başlamadan önce, havuzu kullanmanın nasıl görüneceğinden bahsedelim. Kod tasarlamaya çalışırken, önce istemci arayüzünü yazmak tasarımınızı yönlendirmeye yardımcı olabilir. Kodu, çağırmak istediğiniz şekilde yapılandırılmış olacak biçimde **API (API)**’sini yazın; sonra işlevselliği, genel **kamuya açık API (public API)**’yi sonradan tasarlamak yerine bu yapı içinde uygulayın.

Bölüm 12’deki projede **test güdümlü geliştirme (test-driven development)** kullandığımız gibi, burada **derleyici güdümlü geliştirme (compiler-driven development)** kullanacağız. İstediğimiz işlevleri çağıran kodu yazacağız ve sonra kodun çalışması için sırada neyi değiştirmemiz gerektiğine karar vermek üzere derleyicinin hatalarına bakacağız. Bunu yapmadan önce, başlangıç noktası olarak kullanmayacağımız tekniği inceleyeceğiz.

---

## 🧵 Her İstek İçin Bir İş Parçacığı Oluşturma (spawning a thread for each request)

Önce, her bağlantı için yeni bir iş parçacığı oluşturduğumuzda kodumuzun nasıl görünebileceğini keşfedelim. Daha önce belirtildiği gibi, potansiyel olarak sınırsız sayıda iş parçacığı oluşturma sorunları nedeniyle bu bizim nihai planımız değildir; ancak önce çalışan çok iş parçacıklı bir sunucu elde etmek için iyi bir başlangıç noktasıdır. Sonrasında, bir iyileştirme olarak iş parçacığı havuzunu ekleyeceğiz ve iki çözümü karşılaştırmak daha kolay olacaktır. Liste 21-11, `for` döngüsü içinde her akışı (stream) ele almak üzere yeni bir iş parçacığı oluşturmak için `main`’de yapılacak değişiklikleri gösterir.

**Dosya adı:** src/main.rs

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
```

Liste 21-11: Her akış için yeni bir iş parçacığı başlatmak

Bölüm 16’da öğrendiğiniz gibi, `thread::spawn` yeni bir iş parçacığı oluşturur ve ardından **kapatma (closure)** içindeki kodu yeni iş parçacığında çalıştırır. Bu kodu çalıştırır ve tarayıcınızda `/sleep` ardından iki farklı sekmede `/` yüklerseniz, `/` isteklerinin `/sleep`’in bitmesini beklemek zorunda olmadığını gerçekten görürsünüz. Ancak belirttiğimiz gibi, bu yaklaşım sonunda sistemi bunaltacaktır çünkü herhangi bir sınır olmaksızın yeni iş parçacıkları oluşturursunuz.

Bölüm 17’den hatırlayabileceğiniz üzere, bu tam da **eşzamansız programlama (async) ve bekleme (await)** tekniklerinin parladığı durumlardandır! İş parçacığı havuzunu inşa ederken bunu aklınızda tutun ve `async` kullanıldığında nelerin farklı veya aynı görüneceğini düşünün.

## ✅ `new` Fonksiyonunda İş Parçacığı Sayısını Doğrulama (validating the number of threads in `new`)

Şu anda `new` ve `execute` fonksiyonlarının parametreleriyle bir şey yapmıyoruz. İstediğimiz davranışla bu fonksiyonların gövdelerini (body) uygulayalım. Öncelikle `new` hakkında düşünelim. Daha önce, havuzdaki iş parçacığı sayısı için işaretsiz (unsigned) bir tür seçmiştik çünkü negatif sayıda iş parçacığı olan bir havuzun bir anlamı yoktur. Ancak sıfır iş parçacığı da bir anlam ifade etmez, oysa sıfır geçerli bir `usize` değeridir. Bu nedenle, bir `ThreadPool` örneğini döndürmeden önce `size > 0` olduğunu doğrulayan ve sıfır verilirse programı durduran (`panic`) bir kod ekleyeceğiz. Bunu `assert!` makrosunu kullanarak yapacağız (Liste 21-13).

**Dosya adı:** src/lib.rs

```rust
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    // --snip--
}
```

Liste 21-13: `ThreadPool::new` içinde sıfır verilirse panic olacak şekilde doğrulama

Burada ayrıca `ThreadPool` için **doküman yorumları (doc comments)** ekledik. İyi dokümantasyon uygulamalarına uyarak, fonksiyonun hangi durumlarda `panic` oluşturabileceğini belirttik (Bölüm 14’te tartışıldığı gibi). `cargo doc --open` komutunu çalıştırıp `ThreadPool` struct’ını tıklayarak `new` fonksiyonu için oluşturulan belgeleri görebilirsiniz.

`assert!` eklemek yerine, `new` fonksiyonunu `build` olarak değiştirebilir ve bir `Result` döndürebilirdik (Liste 12-9’daki `Config::build` örneğinde olduğu gibi). Ancak bu durumda, iş parçacığı olmadan bir havuz yaratmaya çalışmayı **geri döndürülemez bir hata (unrecoverable error)** olarak değerlendiriyoruz. Daha iddialı hissediyorsanız, şu imzaya sahip bir `build` fonksiyonu yazmayı deneyebilirsiniz:

```rust
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

---

## 📦 İş Parçacıklarını Saklamak için Alan Oluşturma (creating space to store the threads)

Artık havuza saklanacak geçerli sayıda iş parçacığı olduğundan emin olduğumuza göre, bu iş parçacıklarını oluşturup `ThreadPool` struct’ında saklayabiliriz. Ama bir iş parçacığını nasıl “saklarız”? Bunun için `thread::spawn` fonksiyonunun imzasına tekrar bakalım:

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
```

`spawn` fonksiyonu bir `JoinHandle<T>` döndürür; burada `T`, closure’ın döndürdüğü türdür. Bizim durumumuzda iş parçacığı havuzuna ilettiğimiz closure’lar bağlantıyı işleyecek ve bir şey döndürmeyecek, yani `T` birim tür (`()`) olacaktır.

Liste 21-14’teki kod derlenecektir ama henüz iş parçacığı oluşturmaz. `ThreadPool`’un tanımını, içinde `thread::JoinHandle<()>` örneklerinden oluşan bir vektör (`Vec`) tutacak şekilde değiştirdik, bu vektörü `size` kapasitesiyle başlattık, bir `for` döngüsü hazırladık ve bu vektöre iş parçacıkları ekleyeceğimiz yeri bıraktık. Son olarak, `ThreadPool` örneğini bu vektörle birlikte döndürdük.

**Dosya adı:** src/lib.rs

```rust
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
    }
    // --snip--
}
```

Liste 21-14: `ThreadPool` içinde iş parçacıklarını saklamak için vektör oluşturma

Burada, `ThreadPool` içindeki öğelerin türü `thread::JoinHandle<()>` olduğu için, `std::thread`’i kapsama (scope) aldık.

Geçerli bir boyut alındığında, `ThreadPool` bu boyutta öğe tutabilecek bir vektör oluşturur. `with_capacity` fonksiyonu, `Vec::new` ile aynı işi yapar ama önemli bir farkla: belleği önceden ayırır. Kaç öğe saklayacağımızı bildiğimiz için, bu yaklaşım `Vec::new` kullanmaktan biraz daha verimlidir (çünkü `Vec::new` öğeler eklenirken kendini yeniden boyutlandırır).

`cargo check` komutunu çalıştırırsanız, kodun başarıyla derlendiğini görmelisiniz.

---

## 👷 Worker Struct: ThreadPool’dan İş Parçacığına Kod Göndermek (a worker struct responsible for sending code from the ThreadPool to a thread)

Liste 21-14’te iş parçacıklarını oluşturduğumuz yere bir yorum bırakmıştık. Şimdi gerçekten iş parçacıkları oluşturmayı nasıl yapacağımıza bakalım. Standart kütüphane `thread::spawn` ile iş parçacıkları oluşturmayı sağlar ve bu fonksiyon, iş parçacığı oluşturulduğu anda çalıştırılacak bir kod alır. Ancak bizim durumumuzda, iş parçacıklarını oluşturmak ve onların, sonradan göndereceğimiz kodu beklemesini istiyoruz. Standart kütüphanedeki iş parçacığı uygulaması bunu yapmanın bir yolunu içermez; bunu kendimiz uygulamalıyız.

Bu davranışı, `ThreadPool` ile iş parçacıkları arasında yeni bir veri yapısı tanıtarak uygulayacağız. Bu yapıya **Worker** adını vereceğiz; bu, havuzlama (pooling) uygulamalarında yaygın bir terimdir. Worker, çalıştırılması gereken kodu alır ve kendi iş parçacığında çalıştırır.

Bir restoranın mutfağındaki çalışanları düşünün: işçiler müşteri siparişleri gelene kadar beklerler, ardından siparişleri alıp yerine getirirler.

Artık `ThreadPool` içinde `JoinHandle<()>`’lardan oluşan bir vektör tutmak yerine, **Worker struct**’ları saklayacağız. Her Worker, tek bir `JoinHandle<()>` saklayacak. Daha sonra Worker’a, çalıştırılacak kodu alan ve bunu halihazırda çalışan iş parçacığına gönderen bir yöntem (method) uygulayacağız. Ayrıca, havuzdaki Worker örneklerini ayırt edebilmek için her birine bir `id` vereceğiz.

Yeni `ThreadPool` oluşturulduğunda süreç şöyle olacak:

1. `id` ve `JoinHandle<()>` tutan bir `Worker` struct tanımlayın.
2. `ThreadPool`’u, Worker örneklerinden oluşan bir vektör tutacak şekilde değiştirin.
3. `Worker::new` fonksiyonunu tanımlayın: bir `id` alsın ve bu `id` ile birlikte boş bir closure çalıştıran bir iş parçacığı oluştursun.
4. `ThreadPool::new` içinde `for` döngüsünün sayacını `id` olarak kullanın, yeni bir Worker oluşturun ve vektöre ekleyin.

Hazır mısınız? İşte bu değişiklikleri uygulayan Liste 21-15:

**Dosya adı:** src/lib.rs

```rust
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }
    // --snip--
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
```

Liste 21-15: `ThreadPool` içinde iş parçacıklarını doğrudan tutmak yerine Worker örnekleri saklamak

Burada, `ThreadPool` içindeki alanın adını `threads`’ten `workers`’a değiştirdik çünkü artık `JoinHandle<()>` değil, Worker örnekleri tutuyoruz. `for` döngüsündeki sayacı `Worker::new`’a argüman olarak veriyoruz ve her yeni Worker’ı `workers` adlı vektörde saklıyoruz.

Dışarıdaki kod (örneğin `src/main.rs` içindeki sunucumuz) `ThreadPool` içinde Worker struct kullanıldığını bilmek zorunda değildir; bu nedenle Worker struct ve onun `new` fonksiyonunu **özel (private)** yapıyoruz. `Worker::new`, kendisine verdiğimiz `id`’yi saklar ve boş bir closure çalıştırarak oluşturulan bir `JoinHandle<()>` örneğini barındırır.

Not: Eğer işletim sistemi yeterli kaynak olmadığı için iş parçacığı oluşturamazsa, `thread::spawn` **panic** oluşturur. Bu durumda, bazı iş parçacıkları başarıyla oluşturulsa bile tüm sunucumuz panic ile çöker. Basitlik açısından bu davranış kabul edilebilir, ancak üretim (production) düzeyinde bir iş parçacığı havuzu uygulamasında `std::thread::Builder` ve onun `Result` döndüren `spawn` metodunu kullanmak daha doğru olur.

Bu kod derlenecek ve `ThreadPool::new` fonksiyonuna verdiğimiz sayı kadar Worker örneğini saklayacaktır. Ancak hâlâ `execute` fonksiyonuna verilen closure’ı işlemiyoruz. Bir sonraki adımda bunun nasıl yapılacağını inceleyeceğiz.

## ⚙️ `execute` Metodunun Uygulanması (implementing the execute method)

Şimdi nihayet `ThreadPool` üzerinde `execute` metodunu uygulayalım. Ayrıca `Job` tipini bir `struct` olmaktan çıkarıp, `execute`’ın aldığı closure türünü tutan bir **trait nesnesi (trait object)** için **tip takma adı (type alias)** hâline getireceğiz. Bölüm 20’de “Type Aliases ile Tür Eşanlamlıları Oluşturmak” kısmında tartışıldığı gibi, tip takma adları uzun türleri kısaltarak kullanımı kolaylaştırır. Liste 21-19’a bakın.

**Dosya adı:** src/lib.rs

```rust
// --snip--

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// --snip--
```

Liste 21-19: Her closure’ı tutan bir Box için `Job` tip takma adı oluşturmak ve işi kanaldan (channel) göndermek

Burada, `execute` ile aldığımız closure’dan yeni bir `Job` örneği oluşturuyoruz ve bu işi kanalın gönderici (sender) ucundan gönderiyoruz. `send` çağrısına `unwrap` ekliyoruz; bu, gönderme başarısız olursa panic olacaktır. Bu başarısızlık örneğin tüm iş parçacıklarını durdurduğumuzda (alıcı taraf yeni mesaj almıyorsa) olabilir. Şu anda iş parçacıklarını durduramıyoruz; havuz var oldukça çalışmaya devam ediyorlar. `unwrap` kullanmamızın nedeni, başarısızlık durumunun gerçekleşmeyeceğini biliyor olmamız ama derleyicinin bunu bilmemesidir.

---

## 👷 Worker İçinde İşleri Alıp Çalıştırma (receiving and executing jobs in the worker)

Ama işimiz tam bitmedi! Worker içinde `thread::spawn`’a verdiğimiz closure hâlâ yalnızca kanalın alıcı (receiver) ucuna referans veriyor. Bunun yerine closure, sonsuza dek döngüye girip kanaldan iş istemeli ve iş geldiğinde onu çalıştırmalı. `Worker::new` içinde bu değişikliği yapalım (Liste 21-20).

**Dosya adı:** src/lib.rs

```rust
// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
    }
}
```

Liste 21-20: Worker örneğinin iş parçacığında işleri almak ve yürütmek

Burada önce `receiver.lock()` çağırarak mutex kilidini (lock) alıyoruz, sonra `unwrap` ile olası hatalarda panic oluşturuyoruz. Kilit almak başarısız olabilir; bu, başka bir iş parçacığı kilidi bırakmadan panic ettiğinde (mutex’in zehirli olduğu durumda, poisoned) gerçekleşebilir. Bu durumda panic etmek doğru davranıştır. Dilerseniz `unwrap` yerine `expect` kullanıp anlamlı bir hata mesajı yazabilirsiniz.

Kilit alındığında `recv` çağırarak kanaldan bir `Job` alıyoruz. Son bir `unwrap`, burada da olası hataları geçer (örneğin gönderici taraf kapatıldığında).

`recv` çağrısı bloklanır; yani henüz iş yoksa, mevcut iş parçacığı yeni bir iş gelene kadar bekler. `Mutex<T>`, aynı anda yalnızca bir Worker iş parçacığının iş talep etmesini sağlar.

Artık iş parçacığı havuzumuz çalışır durumda! `cargo run` ile çalıştırın ve bazı istekler yapın:

```
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field `workers` is never read
 --> src/lib.rs:7:5
  |
6 | pub struct ThreadPool {
  |            ---------- field in this struct
7 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
  --> src/lib.rs:48:5
   |
47 | struct Worker {
   |        ------ fields in this struct
48 |     id: usize,
   |     ^^
49 |     thread: thread::JoinHandle<()> ,
   |     ^^^^^^
...
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
...
```

Başarı! Artık bağlantıları eşzamanlı olarak çalıştıran bir iş parçacığı havuzumuz var. Dört iş parçacığından fazlası hiçbir zaman oluşturulmaz, bu yüzden sunucu çok sayıda istek alsa bile sistemimiz aşırı yüklenmez. `/sleep` isteği yaptığınızda, diğer istekler başka bir iş parçacığı tarafından yürütülerek sunucu hizmet vermeye devam eder.

Not: `/sleep` adresini aynı anda birden fazla tarayıcı penceresinde açarsanız, sayfalar beşer saniye aralıklarla yüklenebilir. Bazı tarayıcılar, önbellekleme nedeniyle aynı isteğin birden fazla örneğini sırasıyla çalıştırır. Bu sınırlama bizim sunucumuzdan değil, tarayıcıdan kaynaklanır.

---

## 🔄 `while let` ile Alternatif Worker Uygulaması (alternative implementation with `while let`)

Bölüm 17 ve 18’deki `while let` döngüsünü öğrendikten sonra, neden Worker kodunu Liste 21-21’deki gibi yazmadığımızı merak edebilirsiniz.

**Dosya adı:** src/lib.rs
*Bu kod istenen davranışı sağlamaz.*

```rust
// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker { id, thread }
    }
}
```

Liste 21-21: Worker::new için `while let` kullanan alternatif uygulama

Bu kod derlenir ve çalışır, ancak istenen iş parçacığı davranışını vermez: yavaş bir istek, diğer isteklerin de beklemesine neden olur. Sebep biraz incedir: `Mutex` struct’ının herkese açık bir `unlock` metodu yoktur çünkü kilidin sahipliği, `lock` metodunun döndürdüğü `LockResult<MutexGuard<T>>` içindeki `MutexGuard<T>`’nin ömrüne bağlıdır. Böylece derleme zamanı (compile time) denetleyicisi (borrow checker), bir kaynağa yalnızca kilidi tuttuğumuz sürece erişilebileceğini zorunlu kılar.

Ancak bu uygulama, `MutexGuard<T>`’nin ömrüne dikkat etmezsek kilidin istenenden daha uzun süre tutulmasına yol açabilir.

Liste 21-20’de kullandığımız `let job = receiver.lock().unwrap().recv().unwrap();` ifadesi işe yarar çünkü `let` içinde sağ taraftaki ifadedeki geçici değerler, `let` ifadesi bittiğinde hemen düşürülür (drop edilir). Oysa `while let` (ve `if let`, `match`) geçici değerleri ilgili blok bitene kadar düşürmez. Liste 21-21’de, kilit `job()` çağrısı boyunca tutulur, bu da diğer Worker örneklerinin iş almasını engeller.
