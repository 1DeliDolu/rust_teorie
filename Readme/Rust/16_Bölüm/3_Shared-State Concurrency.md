## 🤝 Paylaşılan-Durumlu Eşzamanlılık (shared-state concurrency)

Mesaj geçişi (message passing), eşzamanlılığı ele almanın iyi bir yoludur; ancak tek yol değildir. Başka bir yöntem de birden fazla iş parçacığının aynı paylaşılan veriye erişmesidir. Go dili dokümantasyonundaki slogana tekrar bakalım:
“Belleği paylaşarak iletişim kurmayın.”

Peki, belleği paylaşarak iletişim kurmak nasıl görünürdü? Ayrıca neden mesaj geçişi taraftarları bellek paylaşımını kullanmamanız konusunda uyarıyor?

Bir bakıma, herhangi bir programlama dilindeki kanallar tek sahiplik (single ownership) gibidir. Çünkü bir değeri kanaldan aktardıktan sonra o değeri artık kullanmamanız gerekir. Paylaşılan-bellekli eşzamanlılık (shared-memory concurrency) ise çoklu sahiplik (multiple ownership) gibidir: birden fazla iş parçacığı aynı bellek konumuna aynı anda erişebilir. Bölüm 15’te gördüğünüz gibi, akıllı işaretçiler (smart pointers) çoklu sahipliği mümkün kılar, ama bu da karmaşıklık getirir çünkü farklı sahiplerin yönetilmesi gerekir. Rust’ın tür sistemi ve sahiplik kuralları bu yönetimin doğru yapılmasına büyük ölçüde yardımcı olur. Örnek olarak, paylaşılan bellekte en yaygın eşzamanlılık yapıtaşlarından biri olan **kilitler (mutexes)** üzerinde duralım.

---

## 🔒 Bir Seferde Yalnızca Bir İş Parçacığının Veriye Erişmesini Sağlamak için Mutex Kullanımı

**Mutex**, *mutual exclusion* (karşılıklı dışlama) ifadesinin kısaltmasıdır. Yani bir mutex, aynı anda yalnızca bir iş parçacığının belirli verilere erişmesine izin verir. Bir mutex içindeki verilere erişmek için, iş parçacığı önce mutex’in kilidini (lock) alarak erişim istediğini belirtmelidir. Kilit, mutex’in bir parçası olan ve şu anda veriye kimin münhasır erişimi olduğunu takip eden bir veri yapısıdır. Bu yüzden mutex, tuttuğu veriyi kilitleme sistemiyle koruyan (guarding) bir yapı olarak tanımlanır.

Mutex kullanırken iki kuralı hatırlamanız gerekir:

1. Veriyi kullanmadan önce kilidi edinmelisiniz (lock almak).
2. Veriyi kullanmayı bitirdiğinizde kilidi serbest bırakmalısınız (unlock), böylece diğer iş parçacıkları kilidi alabilir.

Gerçek hayatta bir mutex benzetmesi yapmak gerekirse: bir konferansta panel tartışması sırasında yalnızca bir mikrofon olduğunu düşünün. Bir panelistin konuşabilmesi için önce mikrofonu istemesi gerekir. Mikrofonu aldığında istediği kadar konuşabilir, sonra konuşmak isteyen bir sonraki kişiye mikrofonu vermelidir. Eğer panelist mikrofonu geri vermezse, başka hiç kimse konuşamaz. Ortak mikrofonun yönetimi doğru yapılmazsa, panel çalışmaz!

Mutex yönetimini doğru yapmak oldukça zor olabilir, bu yüzden birçok insan kanalları daha kolay bulur. Ancak Rust’ın tür sistemi ve sahiplik kuralları sayesinde kilitleme ve kilidi serbest bırakma hatalarını yapamazsınız.

---

## 📦 `Mutex<T>` API’si

Bir mutex kullanımını göstermek için önce tek iş parçacıklı bir bağlamda (basitlik adına) kullanalım. Örnek 16-12’ye bakalım:

Filename: `src/main.rs`

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}
```

Listing 16-12: Basitlik için tek iş parçacıklı bağlamda `Mutex<T>` API’sini keşfetmek

Birçok türde olduğu gibi, `Mutex<T>`’yi ilişkili fonksiyon `new` ile oluştururuz. İçindeki verilere erişmek için `lock` metodunu kullanarak kilidi ediniriz. Bu çağrı, kilidi alma sırası bize gelene kadar mevcut iş parçacığını bloke eder.

Eğer kilidi elinde tutan başka bir iş parçacığı panik yaparsa, `lock` çağrısı başarısız olur. Bu durumda kimse kilidi alamazdı, bu yüzden biz burada `unwrap` kullanarak böyle bir durumda panik etmeyi seçtik.

Kilit alındıktan sonra, dönen değeri (burada `num` olarak adlandırdık) içteki verilere yönelik değiştirilebilir bir referans (mutable reference) gibi kullanabiliriz. Tür sistemi, değeri kullanmadan önce kilidi edinmemizi garanti eder. `m`’nin türü `Mutex<i32>`’dir, `i32` değildir. Bu nedenle içteki `i32` değerini kullanabilmek için `lock` çağrısı yapmak zorundayız. Aksi halde tür sistemi `i32`’ye doğrudan erişmemize izin vermez.

Tahmin edeceğiniz gibi, `Mutex<T>` bir akıllı işaretçidir (smart pointer). Daha doğru söylemek gerekirse, `lock` çağrısı bir **`MutexGuard`** döndürür (bunu `LockResult` içine sarılı olarak alıyoruz ve `unwrap` ile açıyoruz). `MutexGuard` akıllı işaretçisi, `Deref` implementasyonu sayesinde iç veriye işaret eder; ayrıca `Drop` implementasyonu sayesinde kapsam (scope) sona erdiğinde kilidi otomatik olarak serbest bırakır. Bu örnekte iç bloğun sonunda olur.

Böylece kilidi serbest bırakmayı unutup diğer iş parçacıklarının mutex’i kullanmasını engelleme riskimiz olmaz, çünkü kilidin bırakılması otomatik gerçekleşir.

Kilit bırakıldıktan sonra mutex değerini yazdırabiliriz ve içteki `i32` değerini 6’ya değiştirdiğimizi görebiliriz.

## 🤝 Birden Fazla İş Parçacığı Arasında Bir `Mutex<T>` Paylaşmak (sharing a `Mutex<T>` between multiple threads)

Şimdi bir değeri birden fazla iş parçacığı (threads) arasında **`Mutex<T>`** kullanarak paylaşmayı deneyelim. 10 iş parçacığı başlatıp her birinin sayaç değerini 1 artırmasını sağlayacağız; böylece sayaç 0’dan 10’a yükselecek. Örnek 16-13 derleyici hatası verecek ve bu hatayı kullanarak **`Mutex<T>`** kullanımını ve Rust’ın onu doğru kullanmamıza nasıl yardım ettiğini daha iyi anlayacağız.

Filename: `src/main.rs`
Bu kod derlenmez!

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Listing 16-13: Her biri `Mutex<T>` ile korunan sayacı artıran on iş parçacığı

`i32` tutan bir `Mutex<T>` içinde bir sayaç değişkeni oluşturuyoruz (Listing 16-12’de yaptığımız gibi). Sonra bir sayı aralığı üzerinde yineleyerek 10 iş parçacığı oluşturuyoruz. `thread::spawn` kullanıyoruz ve tüm iş parçacıklarına aynı kapanışı (closure) veriyoruz: bu kapanış, sayacı iş parçacığına **taşıyor** (move), `lock` metodunu çağırarak **kilidi** (lock) alıyor ve ardından mutex içindeki değere 1 ekliyor. İş parçacığı kapanışı çalıştırmayı bitirdiğinde, `num` kapsamdan (scope) çıkar ve kilidi serbest bırakır; böylece başka bir iş parçacığı kilidi edinebilir.

Ana iş parçacığında tüm **join handle**’ları (join handles) topluyoruz. Ardından, Listing 16-2’de yaptığımız gibi, her bir handle üzerinde `join` çağırarak tüm iş parçacıklarının bitmesini sağlıyoruz. Bu noktada ana iş parçacığı kilidi edinir ve programın sonucunu yazdırır.

Bu örneğin derlenmeyeceğini ima etmiştik. Nedenini görelim!

```
$ cargo run
   Compiling shared-state v0.1.0 (file:///projects/shared-state)
error[E0382]: borrow of moved value: `counter`
  --> src/main.rs:21:29
   |
5  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
8  |     for _ in 0..10 {
   |     -------------- inside of this loop
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved into closure here, in previous iteration of loop
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value borrowed here after move
   |
help: consider moving the expression out of the loop so it is only moved once
   |
8  ~     let mut value = counter.lock();
9  ~     for _ in 0..10 {
10 |         let handle = thread::spawn(move || {
11 ~             let mut num = value.unwrap();
   |
```

Hata iletisi, `counter` değerinin döngünün önceki yinelemesinde **taşındığını** (move) söylüyor. Rust, kilitli sayacın sahipliğini birden fazla iş parçacığına **taşıyamayacağımızı** bildiriyor. Bu derleyici hatasını, Bölüm 15’te tartıştığımız **çoklu sahiplik** (multiple ownership) yöntemini kullanarak düzeltelim.

---

## 👥 Çoklu İş Parçacığı ile Çoklu Sahiplik (multiple ownership with multiple threads)

Bölüm 15’te, bir değeri birden fazla sahibe vermek için **`Rc<T>`** akıllı işaretçisini (smart pointer) kullanarak **referans sayımlı** (reference counted) bir değer oluşturmuştuk. Burada da aynısını deneyelim ve ne olduğunu görelim. Listing 16-14’te `Mutex<T>`’yi `Rc<T>` ile sarmalayıp, iş parçacığına sahipliği taşımadan önce `Rc<T>`’yi **klonlayacağız** (`clone`).

Filename: `src/main.rs`
Bu kod derlenmez!

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Listing 16-14: `Mutex<T>`’ye çoklu sahiplik vermek için `Rc<T>` kullanmayı denemek

Tekrar derliyoruz ve… farklı hatalar alıyoruz! Derleyici bize çok şey öğretiyor.

```
$ cargo run
   Compiling shared-state v0.1.0 (file:///projects/shared-state)
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
  --> src/main.rs:11:36
   |
11 |           let handle = thread::spawn(move || {
   |                        ------------- ^------
   |                        |             |
   |  ______________________|_____________within this `{closure@src/main.rs:11:36: 11:43}`
   | |                      |
   | |                      required by a bound introduced by this call
12 | |             let mut num = counter.lock().unwrap();
13 | |
14 | |             *num += 1;
15 | |         });
   | |_________^ `Rc<Mutex<i32>>` cannot be sent between threads safely
   |
   = help: within `{closure@src/main.rs:11:36: 11:43}`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
note: required because it's used within this closure
  --> src/main.rs:11:36
   |
11 |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^
note: required by a bound in `spawn`
  --> /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library/std/src/thread/mod.rs:728:1
```

Oldukça uzun bir hata iletisi! Odaklanmamız gereken kısım şu: **`Rc<Mutex<i32>>` iş parçacıkları arasında güvenli bir şekilde gönderilemez**. Derleyici nedeni de söylüyor: **`Rc<Mutex<i32>>` için `Send` özniteliği (trait) uygulanmamıştır (not implemented)**. `Send` konusunu bir sonraki bölümde konuşacağız; bu öznitelik, iş parçacıklarıyla kullandığımız türlerin eşzamanlı durumlar için uygun olmasını güvence altına alan özniteliklerden biridir.

Ne yazık ki **`Rc<T>`**, iş parçacıkları arasında paylaşım için güvenli değildir. `Rc<T>` referans sayımını yönetirken her `clone` çağrısında sayacı artırır ve her klon düşürüldüğünde (drop) sayacı azaltır; ancak bu sayaç değişimlerinin başka bir iş parçacığı tarafından kesintiye uğratılamamasını sağlamak için herhangi bir eşzamanlılık ilkelini (concurrency primitive) kullanmaz. Bu da yanlış sayımlara yol açabilir—bu tarz ince hatalar, bellek sızıntılarına ya da bir değerin işimiz bitmeden düşürülmesine neden olabilir. İhtiyacımız olan şey, `Rc<T>`’ye çok benzeyen ancak referans sayacındaki değişiklikleri **iş parçacığı güvenli** (thread-safe) şekilde yapan bir türdür.
