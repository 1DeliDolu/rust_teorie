## 📡 Kanallar (Channels)

Rust, iş parçacıkları (threads) arasında iletişim için eşzamansız (asynchronous) kanallar sağlar. Kanallar, iki uç nokta arasında tek yönlü bilgi akışına izin verir: `Sender` (Gönderici) ve `Receiver` (Alıcı).

```rust
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn main() {
    // Kanalların iki ucu vardır: `Sender<T>` ve `Receiver<T>`,
    // burada `T`, aktarılacak mesajın türüdür
    // (tür belirtimi gereksizdir)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // Gönderici ucu kopyalanabilir
        let thread_tx = tx.clone();

        // Her iş parçacığı kimliğini kanal üzerinden gönderecek
        let child = thread::spawn(move || {
            // İş parçacığı `thread_tx` üzerinde sahiplik alır
            // Her iş parçacığı kanala bir mesaj kuyruğa ekler
            thread_tx.send(id).unwrap();

            // Gönderme işlemi engelleyici değildir, iş parçacığı
            // mesajını gönderdikten hemen sonra çalışmaya devam eder
            println!("iş parçacığı {} bitti", id);
        });

        children.push(child);
    }

    // Burada tüm mesajlar toplanır
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // `recv` metodu kanaldan bir mesaj alır
        // Eğer mevcut mesaj yoksa `recv`, geçerli iş parçacığını engeller
        ids.push(rx.recv());
    }
    
    // İş parçacıklarının kalan işleri tamamlamasını bekle
    for child in children {
        child.join().expect("eyvah! alt iş parçacığı panikledi");
    }

    // Mesajların hangi sırayla gönderildiğini göster
    println!("{:?}", ids);
}
```
