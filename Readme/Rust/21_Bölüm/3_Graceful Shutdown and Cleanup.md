## 🧹 Zarif Kapanış ve Temizlik (graceful shutdown and cleanup)

Liste 21-20’deki kod, amaçladığımız gibi bir iş parçacığı havuzu (thread pool) kullanarak isteklere eşzamanlı yanıt veriyor. Çalışanlar (workers), `id` ve `thread` alanlarıyla ilgili doğrudan kullanım yapmadığımızı belirten bazı uyarılar alıyoruz; bu da bize hiçbir şeyi temizlemediğimizi hatırlatır. Ana iş parçacığını durdurmak için daha az zarif olan `ctrl-c` yöntemini kullandığımızda, diğer tüm iş parçacıkları da hemen durur; hatta bir isteğe hizmet etmenin ortasında olsalar bile.

Sırada, havuzdaki her iş parçacığı üzerinde `join` (join) çağırmak için `Drop` özelliğini (Drop trait) uygulamak var; böylece kapanmadan önce üzerinde çalıştıkları istekleri bitirebilirler. Ardından, iş parçacıklarına yeni istekleri kabul etmeyi bırakmaları ve kapanmaları gerektiğini söylemenin bir yolunu uygulayacağız. Bu kodun çalıştığını görmek için, sunucumuzu yalnızca iki isteği kabul edecek ve ardından iş parçacığı havuzunu zarif biçimde kapatacak şekilde değiştireceğiz.

Şunu not edelim: burada yapılanların hiçbiri closure’ları yürütmeyi (executing the closures) ele alan kod bölümlerini etkilemez; dolayısıyla bir eşzamansız çalışma zamanı (async runtime) için iş parçacığı havuzu kullansaydık da her şey aynı olurdu.

## 🧩 ThreadPool Üzerinde Drop Özelliğinin Uygulanması (implementing the Drop trait on ThreadPool)

`Drop`’u iş parçacığı havuzumuz üzerinde uygulamakla başlayalım. Havuz düşürüldüğünde (dropped), tüm iş parçacıklarımızın işlerini bitirdiklerinden emin olmak için `join` çağrısı yapılmalıdır. Liste 21-22, bir `Drop` uygulamasına ilk denemeyi gösterir; bu kod henüz tam olarak çalışmaz.

**Dosya adı:** src/lib.rs
*Bu kod derlenmez!*

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

Liste 21-22: İş parçacığı havuzu kapsam dışına çıktığında her iş parçacığını join etmek

Önce, iş parçacığı havuzundaki her çalışan (Worker) üzerinde döngü yapıyoruz. Bunu `&mut` ile yapıyoruz çünkü `self` değiştirilebilir bir başvurudur (mutable reference) ve `worker` üzerinde de değişiklik yapabilmemiz gerekir. Her çalışan için, bu belirli `Worker` örneğinin kapandığını bildiren bir mesaj yazdırır ve sonra o `Worker` örneğinin iş parçacığında `join` çağrısı yaparız. `join` çağrısı başarısız olursa, Rust’ın panik (panic) etmesi ve zarif olmayan bir kapanışa geçmesi için `unwrap` kullanırız.

Bu kodu derlediğimizde aldığımız hata:

```
$ cargo check
    Checking hello v0.1.0 (file:///projects/hello)
error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
  --> src/lib.rs:52:13
   |
52 |             worker.thread.join().unwrap();
   |             ^^^^^^^^^^^^^ ------ `worker.thread` moved due to this method call
   |             |
   |             move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
   |
note: `JoinHandle::<T>::join` takes ownership of the receiver `self`, which moves `worker.thread`
  --> /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library/std/src/thread/mod.rs:1876:17

For more information about this error, try `rustc --explain E0507`.
error: could not compile `hello` (lib) due to 1 previous error
```

Hata, yalnızca her çalışan üzerinde değiştirilebilir ödünç alma (mutable borrow) yaptığımız için `join` çağıramayacağımızı ve `join`’in bağımsız değişkeninin sahipliğini (ownership) alacağını söylüyor. Bu sorunu çözmek için, `join`’in iş parçacığını tüketebilmesi (consume) adına, `thread`’i ona sahip olan `Worker` örneğinin içinden **taşımamız** (move) gerekir. Bunu yapmanın bir yolu, Liste 18-15’teki yaklaşımla aynıdır. `Worker`, `Option<thread::JoinHandle<()>>` tutsaydı, `Option` üzerindeki `take` metodunu çağırarak değeri `Some` varyantının içinden hareket ettirebilir (move) ve yerine `None` bırakabilirdik. Başka bir deyişle, çalışan bir `Worker`’ın `thread` alanında `Some` olurdu; bir `Worker`’ı temizlemek istediğimizde `Some`’u `None` ile değiştiririz, böylece `Worker`’ın çalıştırılacak bir iş parçacığı olmaz.

Ancak bu durum yalnızca `Worker` düşürülürken (drop) ortaya çıkar. Buna karşılık, `worker.thread`’e eriştiğimiz her yerde `Option<thread::JoinHandle<()>>` ile uğraşmamız gerekir. İdiomatik Rust, `Option`’ı sık kullanır; fakat **her zaman mevcut olduğunu bildiğiniz** bir şeyi böyle bir geçici çözüm olarak `Option` ile sarmaladığınızda, alternatif yaklaşımlar aramak iyi bir fikirdir. Bu, kodunuzu daha temiz ve hataya daha az açık hâle getirebilir.

Bu durumda daha iyi bir alternatif vardır: `Vec::drain` yöntemi (Vec::drain method). Bu yöntem, `Vec`’ten hangi öğelerin kaldırılacağını belirtmek için bir aralık parametresi alır ve bu öğelerin bir yineleyicisini (iterator) döndürür. `..` aralık söz dizimini (range syntax) geçirmek, `Vec`’ten tüm değerleri kaldırır.

Bu nedenle, `ThreadPool` için `drop` uygulamasını şu şekilde güncellememiz gerekir:

**Dosya adı:** src/lib.rs

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

Bu, derleyici hatasını çözer ve kodumuzda başka değişiklik gerektirmez.

## 🛑 İş Parçacıklarına İş Dinlemeyi Durdurmalarını Sinyallemek (signaling to the threads to stop listening for jobs)

Yaptığımız tüm değişikliklerle birlikte, kodumuz artık uyarı olmadan derleniyor. Ancak kötü haber şu ki, bu kod hâlâ istediğimiz şekilde çalışmıyor. Anahtar nokta, **Worker** örneklerinin iş parçacıklarında (threads) çalıştırılan kapatmaların (closures) içindeki mantıkta yatıyor: şu anda `join` çağırıyoruz, fakat iş parçacıkları işleri aramak için sonsuza dek döngüde kaldıklarından bu, iş parçacıklarını kapatmayacak. Mevcut `drop` uygulamamızla `ThreadPool`’u düşürmeye (drop) çalışırsak, ana iş parçacığı ilk iş parçacığının bitmesini beklerken sonsuza kadar bloklanacaktır.

Bu sorunu düzeltmek için, önce `ThreadPool`’un `drop` uygulamasında, ardından **Worker** döngüsünde değişiklik yapmamız gerekecek.

Önce `ThreadPool`’un `drop` uygulamasını, iş parçacıklarının bitmesini beklemeden önce **göndericiyi (sender)** açıkça düşürecek (drop) şekilde değiştireceğiz. Liste 21-23, `sender`’ı açıkça düşürecek şekilde `ThreadPool`’da yapılan değişiklikleri gösterir. İş parçacığında (thread) olduğu gibi değil, burada `ThreadPool`’dan `sender`’ı `Option::take` ile çıkartabilmek için `Option` kullanmamız gerekir.

**Dosya adı:** src/lib.rs
*Bu kod istenen davranışı üretmez.*

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
// --snip--
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // --snip--

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

Liste 21-23: Worker iş parçacıklarını birleştirmeden (join) önce `sender`’ı açıkça düşürmek (drop)

`sender`’ı düşürmek kanalı kapatır; bu da daha fazla mesaj gönderilmeyeceğini belirtir. Bu gerçekleştiğinde, sonsuz döngüdeki Worker örneklerinin yaptığı tüm `recv` çağrıları bir hata döndürür. Liste 21-24’te, bu durumda döngüden zarifçe çıkmak için Worker döngüsünü değiştiriyoruz; bu da `ThreadPool`’un `drop` uygulaması iş parçacıkları üzerinde `join` çağırdığında iş parçacıklarının tamamlanacağı anlamına gelir.

**Dosya adı:** src/lib.rs

```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
```

Liste 21-24: `recv` hata döndürdüğünde döngüden açıkça çıkmak

Bu kodu çalışırken görmek için, Liste 21-25’te gösterildiği gibi, sunucunun zarif biçimde kapanmadan önce yalnızca iki isteği kabul etmesini sağlayacak şekilde `main`’i değiştirelim.

**Dosya adı:** src/main.rs

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
```

Liste 21-25: Döngüden çıkarak iki isteğe hizmet verdikten sonra sunucuyu kapatmak

Gerçek dünyadaki bir web sunucusunun yalnızca iki isteğe hizmet verdikten sonra kapanmasını istemezsiniz. Bu kod, yalnızca zarif kapatma (graceful shutdown) ve temizlemenin (cleanup) çalıştığını göstermektedir.

`take` yöntemi, **Iterator (iterator)** trait’i içinde tanımlanır ve yinelemeyi en fazla ilk iki öğeyle sınırlar. `main` sonunda `ThreadPool` kapsam dışına çıkacak (out of scope) ve `drop` uygulaması çalışacaktır.

Sunucuyu `cargo run` ile başlatın ve üç istek yapın. Üçüncü istekte hata oluşmalı ve terminalinizde buna benzer bir çıktı görmelisiniz:

```
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

Yazdırılan Worker kimliklerinin (ID) ve mesajların sırası farklı olabilir. Mesajlardan kodun nasıl çalıştığını görebiliriz: Worker 0 ve 3 ilk iki isteği aldı. Sunucu ikinci bağlantıdan sonra bağlantı almayı durdurdu ve `ThreadPool` üzerindeki `Drop` uygulaması, Worker 3 işine başlamadan önce çalışmaya başladı. `sender`’ı düşürmek, tüm Worker örneklerinin bağlantısını keser ve onlara kapanmalarını söyler. Worker örneklerinin her biri bağlantıları kesildiğinde bir mesaj yazdırır ve ardından iş parçacığı havuzu, her Worker iş parçacığının bitmesini beklemek için `join` çağırır.

Bu belirli çalıştırmanın ilginç bir yönüne dikkat edin: `ThreadPool`, `sender`’ı düşürdü ve herhangi bir Worker hata almadan önce Worker 0’ı birleştirmeyi (`join`) denedik. Worker 0 henüz `recv`’den bir hata almamıştı, bu nedenle ana iş parçacığı Worker 0’ın bitmesini bekleyerek bloklandı. Bu sırada Worker 3 bir iş aldı ve ardından tüm iş parçacıkları hata aldı. Worker 0 bittiğinde, ana iş parçacığı kalan Worker örneklerinin bitmesini bekledi. O noktada hepsi döngülerinden çıkmış ve durdurulmuştu.

Tebrikler! Artık iş parçacığı havuzu kullanan ve eşzamanlı olarak yanıt veren temel bir web sunucusunu tamamladık. Sunucuyu zarif biçimde kapatabiliyor ve havuzdaki tüm iş parçacıklarını temizleyebiliyoruz.

---

## 📎 Başvuru için Tam Kod (full code for reference)

**Dosya adı:** src/main.rs

```rust
use hello::ThreadPool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
```

**Dosya adı:** src/lib.rs

```rust
use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

---

## 🧭 Devam Etmek İçin Fikirler (ideas to continue)

* `ThreadPool` ve genel yöntemlerine daha fazla dokümantasyon ekleyin.
* Kütüphane işlevselliği için testler ekleyin.
* `unwrap` çağrılarını daha sağlam hata yönetimiyle değiştirin.
* `ThreadPool`’u web istekleri sunmak dışında başka görevleri yürütmek için kullanın.
* crates.io’da bir iş parçacığı havuzu (thread pool) paketi bulun ve benzer bir web sunucusunu bu paketle uygulayın. Ardından API’sini ve sağlamlığını bizim uygulamamızla karşılaştırın.

---

## 🧾 Özet (summary)

Aferin! Kitabın sonuna ulaştınız! Rust turumuzda bize eşlik ettiğiniz için teşekkür ederiz. Artık kendi Rust projelerinizi uygulamaya ve başkalarının projelerine yardım etmeye hazırsınız. Rust yolculuğunuzda karşılaşabileceğiniz zorluklarda yardımcı olmaktan mutluluk duyacak, sıcak bir **Rustacean** topluluğu olduğunu unutmayın.
