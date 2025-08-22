## 🧵 Kodun Eşzamanlı Çalıştırılması için İş Parçacıkları (threads) Kullanma

Çoğu modern işletim sisteminde çalıştırılan bir programın kodu bir işlem (process) içinde çalıştırılır ve işletim sistemi aynı anda birden fazla işlemi yönetir. Bir programın içinde de aynı anda çalışabilen bağımsız parçalar olabilir. Bu bağımsız parçaları çalıştıran özelliklere iş parçacıkları (threads) denir. Örneğin, bir web sunucusu aynı anda birden fazla isteğe yanıt verebilmek için birden çok iş parçacığına sahip olabilir.

Programınızdaki hesaplamayı birden fazla iş parçacığına bölmek, aynı anda birden çok görevi çalıştırmanızı sağlayarak performansı artırabilir, ancak aynı zamanda karmaşıklığı da artırır. İş parçacıkları aynı anda çalışabildiği için, farklı iş parçacıklarındaki kod parçalarının hangi sırayla çalışacağına dair bir garanti yoktur. Bu durum şu tür sorunlara yol açabilir:

* Yarış durumları (race conditions): İş parçacıkları verilere veya kaynaklara tutarsız bir sırayla eriştiğinde oluşur.
* Kilitlenmeler (deadlocks): İki iş parçacığının birbirini beklediği, böylece ikisinin de devam edemediği durumlar.
* Yalnızca belirli durumlarda meydana gelen, yeniden üretmesi ve güvenilir bir şekilde düzeltmesi zor hatalar.

Rust, iş parçacıklarının kullanılmasının olumsuz etkilerini azaltmaya çalışır, ancak çok iş parçacıklı (multithreaded) bir bağlamda programlama hâlâ dikkatli düşünmeyi ve tek iş parçacıklı programlardan farklı bir kod yapısı gerektirir.

Programlama dilleri iş parçacıklarını birkaç farklı şekilde uygular ve birçok işletim sistemi yeni iş parçacıkları oluşturmak için dilin çağırabileceği bir API sağlar. Rust standart kütüphanesi, her bir dil iş parçacığı için bir işletim sistemi iş parçacığı kullanan 1:1 iş parçacığı modelini uygular. 1:1 modeline farklı alternatifler sunan başka iş parçacığı modellerini uygulayan `crate`ler de vardır. (Rust’ın bir sonraki bölümde göreceğimiz `async` sistemi de eşzamanlılığa başka bir yaklaşım sunar.)

---

## 🆕 `spawn` ile Yeni Bir İş Parçacığı Oluşturma

Yeni bir iş parçacığı oluşturmak için `thread::spawn` fonksiyonunu çağırır ve yeni iş parçacığında çalıştırmak istediğimiz kodu içeren bir kapanış (closure) geçiririz (kapanışlardan Bölüm 13’te bahsetmiştik). Aşağıdaki Örnek 16-1, ana iş parçacığından ve yeni bir iş parçacığından bazı metinleri yazdırmaktadır:

Filename: `src/main.rs`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

Listing 16-1: Ana iş parçacığı başka bir şey yazdırırken yeni bir iş parçacığı oluşturmak

Rust programının ana iş parçacığı tamamlandığında, tüm oluşturulan iş parçacıkları bitirilmese bile kapatılır. Bu programın çıktısı her çalıştırıldığında biraz farklı olabilir, ama genellikle şu şekilde görünür:

```
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

`thread::sleep` çağrıları, bir iş parçacığını kısa süreliğine durdurarak başka bir iş parçacığının çalışmasına izin verir. İş parçacıkları muhtemelen sırayla çalışacaktır, ancak bu garanti edilmez; bu, işletim sisteminizin iş parçacıklarını nasıl planladığına bağlıdır. Bu çalıştırmada, kodda yeni iş parçacığının `println!` satırı önce görünse de ana iş parçacığı önce yazdırdı. Ayrıca, oluşturulan iş parçacığından `i` değişkeni 9’a kadar yazdırmasını istemiş olsak da, ana iş parçacığı kapanmadan önce sadece 5’e kadar yazdırabildi.

Eğer bu kodu çalıştırdığınızda yalnızca ana iş parçacığından çıktı görüyorsanız veya örtüşme yoksa, işletim sistemine iş parçacıkları arasında geçiş yapmak için daha fazla fırsat vermek amacıyla döngü aralıklarındaki sayıları artırmayı deneyin.

---

## ⏳ join Handle Kullanarak Tüm İş Parçacıklarının Bitmesini Beklemek

Örnek 16-1’deki kod, çoğu zaman ana iş parçacığı sona erdiği için oluşturulan iş parçacığını erken bitirir. Ayrıca, iş parçacıklarının hangi sırayla çalışacağı garanti edilmediğinden, oluşturulan iş parçacığının hiç çalışmaması bile mümkündür!

Oluşturulan iş parçacığının erken bitmesini veya hiç çalışmamasını önlemek için, `thread::spawn` fonksiyonunun döndürdüğü değeri bir değişkende saklayabiliriz. `thread::spawn`’ın dönüş türü `JoinHandle<T>`’dir. Bir `JoinHandle<T>`, üzerinde `join` metodunu çağırdığımızda ilgili iş parçacığının bitmesini bekleyen sahipli (owned) bir değerdir. Örnek 16-2, Örnek 16-1’de oluşturulan iş parçacığının `JoinHandle<T>` değerinin nasıl kullanılacağını ve `join` çağrısı ile iş parçacığının bitmeden ana iş parçacığının kapanmamasının nasıl sağlanacağını göstermektedir.

Filename: `src/main.rs`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

Listing 16-2: `thread::spawn` dönüşünden alınan `JoinHandle<T>` kullanarak iş parçacığının tamamlandığından emin olma

Bir `handle` üzerinde `join` çağırmak, o anda çalışan iş parçacığını, `handle`’ın temsil ettiği iş parçacığı sona erene kadar bloke eder. Bir iş parçacığını bloke etmek, onun iş yapmasını veya bitmesini engeller. `join` çağrısını ana iş parçacığındaki `for` döngüsünden sonra koyduğumuz için, Örnek 16-2 şu benzer bir çıktı üretmelidir:

```
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

İki iş parçacığı dönüşümlü olarak çalışmaya devam eder, ancak `handle.join()` çağrısı nedeniyle ana iş parçacığı bekler ve oluşturulan iş parçacığı bitene kadar kapanmaz.

Peki, `handle.join()` çağrısını `main` fonksiyonundaki `for` döngüsünden önce taşırsak ne olur?

Filename: `src/main.rs`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

Bu durumda ana iş parçacığı önce oluşturulan iş parçacığının bitmesini bekler, ardından kendi `for` döngüsünü çalıştırır. Dolayısıyla çıktı artık iç içe geçmiş olmayacak ve şöyle görünecektir:

```
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

Küçük ayrıntılar, örneğin `join` çağrısının nerede yapıldığı, iş parçacıklarınızın aynı anda çalışıp çalışmadığını etkileyebilir.

## 🚚 İş Parçacıkları ile `move` Kapanışları (closures) Kullanma

Çoğunlukla `thread::spawn` fonksiyonuna geçirilen kapanışlarda (closure) `move` anahtar sözcüğünü kullanırız. Çünkü kapanış bu durumda ortamdan (environment) kullandığı değerlerin sahipliğini (ownership) alır ve böylece bu değerlerin sahipliği bir iş parçacığından diğerine aktarılmış olur. Bölüm 13’te, “Kapanışlarla Ortamı Yakalama (Capturing the Environment With Closures)” kısmında `move`’u kapanışlar bağlamında tartışmıştık. Şimdi ise `move` ile `thread::spawn` arasındaki etkileşime odaklanacağız.

Örnek 16-1’de `thread::spawn` fonksiyonuna geçirdiğimiz kapanışın hiçbir argüman almadığına dikkat edin: oluşturulan iş parçacığının kodunda ana iş parçacığından hiçbir veri kullanmıyoruz. Eğer ana iş parçacığındaki verileri oluşturulan iş parçacığında kullanmak istersek, oluşturulan iş parçacığının kapanışı ihtiyaç duyduğu değerleri yakalamalıdır (capture). Örnek 16-3, ana iş parçacığında bir vektör oluşturmayı ve onu oluşturulan iş parçacığında kullanmayı denemektedir. Ancak bu haliyle çalışmayacaktır.

Filename: `src/main.rs`
Bu kod derlenmez!

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```

Listing 16-3: Ana iş parçacığında oluşturulan bir vektörü başka bir iş parçacığında kullanmayı denemek

Kapanış `v`’yi kullandığı için onu yakalar ve kendi ortamının bir parçası haline getirir. `thread::spawn` bu kapanışı yeni bir iş parçacığında çalıştırdığı için `v`’ye erişebilmemiz gerekirdi. Ancak derlemeye çalıştığımızda şu hata ile karşılaşırız:

```
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {v:?}");
  |                                     - `v` is borrowed here
  |
note: function requires argument type to outlive `'static`
...
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
```

Rust `v`’yi nasıl yakalayacağını kendisi çıkarır. `println!` yalnızca `v`’ye bir referans (borrow) ihtiyaç duyduğundan, kapanış `v`’yi ödünç almaya çalışır. Ancak burada bir sorun vardır: Rust, oluşturulan iş parçacığının ne kadar süreceğini bilemez, bu nedenle `v`’nin referansının her zaman geçerli olacağından emin olamaz.

Örnek 16-4 bu sorunu daha açık gösterir:

Filename: `src/main.rs`
Bu kod derlenmez!

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    drop(v); // eyvah!

    handle.join().unwrap();
}
```

Listing 16-4: Ana iş parçacığında `v`’yi düşüren (drop) bir kapanışın `v`’ye referans yakalamaya çalışması

Eğer Rust bu koda izin verseydi, oluşturulan iş parçacığı çalıştırılmadan hemen arka plana alınabilirdi. Bu iş parçacığı içinde `v`’ye bir referans vardır, ama ana iş parçacığı `drop` fonksiyonunu (Bölüm 15’te bahsetmiştik) çağırarak `v`’yi hemen serbest bırakır. Ardından oluşturulan iş parçacığı çalışmaya başlarsa, `v` artık geçerli değildir ve dolayısıyla referansı da geçersiz olurdu.

Bu derleyici hatasını çözmek için hata mesajındaki öneriyi uygulayabiliriz:

```
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
```

Yani kapanışın başına `move` anahtar sözcüğünü ekleyerek, kapanışın kullandığı değerlerin sahipliğini almasını sağlarız. Böylece değerleri ödünç almak yerine doğrudan sahiplenmiş olur. Örnek 16-3’teki kodu değiştiren Örnek 16-5, derlenecek ve beklendiği gibi çalışacaktır:

Filename: `src/main.rs`

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```

Listing 16-5: `move` anahtar sözcüğü ile kapanışın kullandığı değerlerin sahipliğini almasını sağlamak

Burada insanın aklına, Örnek 16-4’teki `drop` sorununu da aynı yöntemle çözmek gelebilir. Ancak bu çözüm işe yaramaz. Çünkü kapanışa `move` eklersek, `v`’nin sahipliği kapanış ortamına taşınır ve ana iş parçacığında artık `drop(v)` çağrısı yapamayız. Bu durumda şu hatayı alırız:

```
error[E0382]: use of moved value: `v`
  --> src/main.rs:10:10
   |
4  |     let v = vec![1, 2, 3];
   |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved into closure here
...
10 |     drop(v); // oh no!
   |          ^ value used here after move
```

Rust’ın sahiplik kuralları yine bizi kurtardı! Örnek 16-3’teki hata, Rust’ın temkinli davranıp `v`’yi yalnızca ödünç almasından kaynaklanıyordu; bu durumda ana iş parçacığı teorik olarak oluşturulan iş parçacığının referansını geçersiz kılabilirdi. Biz `move` ile `v`’nin sahipliğini oluşturulan iş parçacığına taşıdığımızda, Rust’a artık ana iş parçacığının `v`’yi kullanmayacağını garanti etmiş oluyoruz. Örnek 16-4’ü de bu şekilde değiştirseydik, ana iş parçacığında `v`’yi kullanmaya çalıştığımızda sahiplik kurallarını ihlal etmiş olurduk. Yani `move` anahtar sözcüğü Rust’ın temkinli varsayılan davranışı olan ödünç almayı (borrow) geçersiz kılar, fakat sahiplik kurallarını çiğnememize izin vermez.

---

Artık iş parçacıklarının ne olduğunu ve `thread` API’sinin sağladığı yöntemleri gördüğümüze göre, iş parçacıklarını hangi durumlarda kullanabileceğimize bakalım.
