## 🔗 Her Şeyi Bir Araya Getirmek: Futures (gelecekler), Tasks (görevler) ve Threads (iş parçacıkları)

Bölüm 16’da gördüğümüz gibi, iş parçacıkları (threads) eşzamanlılık için bir yaklaşımdır. Bu bölümde ise başka bir yaklaşımı gördük: `async` ile `Future` (gelecekler) ve `Stream` (akışlar) kullanmak.
“Hangisini seçmeliyim?” diye soruyorsanız, cevap: **duruma bağlıdır!** Hatta çoğu zaman seçim yalnızca *threads veya async* değil, *threads ve async birlikte* olur.

---

Birçok işletim sistemi onlarca yıldır iş parçacığına dayalı eşzamanlılık modelleri sağlamaktadır ve bu nedenle birçok programlama dili onları destekler. Ancak bu modellerin de bazı dezavantajları vardır:

* Birçok işletim sisteminde her iş parçacığı için **önemli miktarda bellek** kullanılır.
* Başlatma ve kapatma için **ek yük (overhead)** vardır.
* İş parçacıkları yalnızca işletim sistemi ve donanım tarafından destekleniyorsa kullanılabilir.
* Bazı gömülü sistemlerde işletim sistemi bile olmadığı için iş parçacıkları da yoktur.

---

Async modeli ise farklı (ve aslında tamamlayıcı) bir dizi denge sunar.

* Async modelde, eşzamanlı işlemler kendi iş parçacıklarını gerektirmez.
* Bunun yerine **görevler (tasks)** üzerinde çalışırlar.
* Örneğin, akışlar bölümünde senkron bir fonksiyondan iş başlatmak için `trpl::spawn_task` kullandık.
* Bir görev (task), iş parçacığına benzer, ancak işletim sistemi yerine **kütüphane seviyesindeki kod (runtime)** tarafından yönetilir.

---

Önceki bölümde, bir async kanal kullanarak ve senkron koddan çağırabileceğimiz bir async görev oluşturarak nasıl bir stream (akış) inşa edebileceğimizi gördük.
Aynı şeyi bir iş parçacığı (thread) ile de yapabiliriz.

Bölüm 17-40’ta `trpl::spawn_task` ve `trpl::sleep` kullandık.
Bölüm 17-41’de ise bunların yerine standart kütüphaneden `thread::spawn` ve `thread::sleep` kullanıyoruz:

```rust
// Filename: src/main.rs
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    // Bu `trpl::spawn` değil, `std::thread::spawn`!
    thread::spawn(move || {
        let mut count = 0;
        loop {
            // Bu da `trpl::sleep` değil, `std::thread::sleep`!
            thread::sleep(Duration::from_millis(1));
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

Listing 17-41: `get_intervals` fonksiyonu için async `trpl` API’leri yerine `std::thread` API’lerini kullanmak.

---

Bu kodu çalıştırdığınızda, çıktısı Listing 17-40 ile tamamen aynı olur. Üstelik çağıran kod açısından çok az şey değişmiştir. Yani bir fonksiyon async runtime’da görev (task) başlatmış, diğeri ise işletim sistemi iş parçacığı başlatmış olsa da, sonuçta elde edilen stream bu farklardan etkilenmez.

---

Buna rağmen, bu iki yaklaşım **çok farklı davranır**.

* Basit örnekte farkı ölçmek zor olsa da, modern bir bilgisayarda milyonlarca async görev başlatabiliriz.
* Aynısını iş parçacıkları ile yapmaya kalksak, **belleğimiz tükenirdi!**

Bunun nedeni API’lerin bu kadar benzer tasarlanmasıdır:

* İş parçacıkları → **senkron işlemler için sınır** oluşturur; eşzamanlılık iş parçacıkları arasında mümkündür.
* Görevler (tasks) → **asenkron işlemler için sınır** oluşturur; eşzamanlılık hem görevler arasında hem de görevlerin içindeki future’lar arasında mümkündür.
* Future’lar → Rust’taki **en küçük eşzamanlılık birimi**dir ve her future başka future’ların ağacını temsil edebilir.

Runtime (özellikle executor):

* Görevleri yönetir.
* Görevler de future’ları yönetir.

Bu anlamda görevler, işletim sistemi yerine runtime tarafından yönetilen **hafif iş parçacıkları** gibidir; ek olarak async özellikleri sayesinde daha fazla yetenekleri vardır.

---

Bu, async görevlerin her zaman iş parçacıklarından daha iyi olduğu anlamına gelmez (ya da tersi).

* İş parçacıkları ile eşzamanlılık bazı açılardan async’e göre daha basit bir programlama modelidir.
* Bu hem güçlü hem de zayıf bir yan olabilir.
* İş parçacıkları biraz “fire and forget” şeklindedir: bir future’ın async’deki gibi eşleniği yoktur; başladıklarında, işletim sistemi dışında kesintiye uğramadan sona kadar çalışırlar.
* Yani future’ların sahip olduğu **görev içi eşzamanlılık** desteğine sahip değillerdir.
* Rust’ta iş parçacıkları için bir **iptal (cancellation)** mekanizması yoktur. Async future’larda ise future bittiğinde state otomatik olarak temizlenir.

---

İstersen bu noktada ben sana bu kısmın **özet şemasını** da çıkarabilirim: Future → Task → Thread ilişkisi görselleştirilmiş şekilde. İstiyor musun?

## 🔄 Threads ve Futures’i Birleştirmek

İş parçacıklarının (threads) bu sınırlamaları, onları **birleştirmeyi (compose etmeyi)** future’lara göre daha zor hale getirir.
Örneğin, bu bölümde oluşturduğumuz `timeout` ve `throttle` yardımcılarını iş parçacıklarıyla yazmak çok daha karmaşık olurdu.

Bunun nedeni, **future**’ların aslında çok daha zengin veri yapıları olmasıdır.
Bu sayede, onları birbirine **doğal bir şekilde bileştirmek** mümkündür.

---

## 🧩 Görevlerin (Tasks) Rolü

* **Tasks**, future’lar üzerinde daha fazla kontrol sağlar.
* Hangi future’ların nasıl gruplandırılacağını seçmemize izin verir.
* Ve çoğu zaman **iş parçacıkları ile birlikte çok iyi çalışırlar**:

  * Çünkü görevler, bazı runtimelarda, iş parçacıkları arasında taşınabilir.
  * Örneğin, kullandığımız runtime (içinde `spawn_blocking` ve `spawn_task` fonksiyonları var) aslında **varsayılan olarak çok iş parçacıklı (multithreaded)** çalışır.
  * Birçok runtime, **iş çalma (work stealing)** yaklaşımını kullanır.

    * Görevler, hangi iş parçacıkları daha az meşgulse oraya kaydırılır.
    * Böylece sistemin genel performansı iyileştirilir.
  * Bu yaklaşım, hem iş parçacıklarını hem görevleri (ve dolayısıyla future’ları) gerektirir.

---

## 📌 Ne Zaman Hangisini Kullanmalı?

* Eğer iş **yüksek paralellik** (parallelism) içeriyorsa → **threads** daha iyi bir seçimdir.

  * Örnek: Çok sayıda veriyi parçalara ayırıp bağımsız olarak işlemek.

* Eğer iş **yüksek eşzamanlılık** (concurrency) içeriyorsa → **async** daha iyi bir seçimdir.

  * Örnek: Farklı hız ve aralıklarla gelen mesajları aynı anda işlemek.

* Eğer hem paralellik hem de eşzamanlılık gerekiyorsa → **threads ve async’i birlikte** kullanabilirsiniz.

  * Her biri, kendi güçlü olduğu alanlarda görev alır.

---

## 📖 Örnek: Threads + Async Birlikte

```rust
// Filename: src/main.rs
use std::{thread, time::Duration};

fn main() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
```

📌 Bu örnekte:

* Önce bir **async channel** oluşturuluyor.
* Ardından bir iş parçacığı başlatılıyor (`thread::spawn`).

  * Bu iş parçacığı gönderici (sender) tarafını sahipleniyor.
  * İçinde 1’den 10’a kadar sayıları gönderiyor, her bir gönderim arasında 1 saniye bekliyor.
* Son olarak, `trpl::run` içine geçirilen bir async block çalıştırılıyor.

  * Bu async block, alıcı (receiver) tarafında mesajları `await` ile bekliyor ve ekrana yazdırıyor.

---

## 🎬 Gerçek Dünya Senaryosu

Düşünün:

* Bir dizi **video kodlama işlemi** için özel bir iş parçacığı kullanıyorsunuz (çünkü bu işlem **CPU’ya bağlıdır**).
* Ama bu işlemler tamamlandığında kullanıcı arayüzüne haber vermek için **async kanal** kullanıyorsunuz.

Rust’ta bu tür **threads + async kombinasyonlarını** gerçek dünyada sıkça görebilirsiniz.

---

## ✅ Özet

* Threads ve async farklı ama tamamlayıcıdır.
* Threads → **paralellik** için daha uygundur.
* Async → **eşzamanlılık** için daha uygundur.
* İkisini birlikte kullanmak mümkündür ve çoğu durumda en iyi çözümdür.
* Rust size **her iki modeli de güvenli ve hızlı** şekilde kullanmanız için araçlar sunar.

---

İstersen ben sana bu son bölümü bir tablo şeklinde **Threads vs Async karşılaştırması** olarak özetleyebilirim. Görsel öğrenmeye daha uygun olur. İstiyor musun?
