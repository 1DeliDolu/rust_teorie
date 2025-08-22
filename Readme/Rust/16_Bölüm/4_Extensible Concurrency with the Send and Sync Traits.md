## 🧩 `Send` ve `Sync` Öznitelikleriyle Genişletilebilir Eşzamanlılık (extensible concurrency with the Send and Sync traits)

Dikkat çekici biçimde, bu bölümde şimdiye dek konuştuğumuz neredeyse tüm eşzamanlılık (concurrency) özellikleri dilin değil, standart kütüphanenin (standard library) parçasıydı. Eşzamanlılığı ele alma seçenekleriniz dil ya da standart kütüphane ile sınırlı değildir; kendi eşzamanlılık özelliklerinizi yazabilir veya başkalarının yazdıklarını kullanabilirsiniz.

Bununla birlikte, dilin içine gömülü olan temel eşzamanlılık kavramları arasında standart kütüphaneden ziyade `std::marker` öznitelikleri (traits) olan `Send` ve `Sync` yer alır.

---

## 🔁 `Send` ile İş Parçacıkları Arasında Sahiplik Aktarımına İzin Vermek (allowing transference of ownership between threads with Send)

`Send` **işaretleyici özniteliği** (marker trait), `Send` uygulayan (implementing) türlerin değerlerinin sahipliğinin (ownership) iş parçacıkları (threads) arasında aktarılabileceğini belirtir. Neredeyse her Rust türü `Send`’dir, ancak bazı istisnalar vardır; örneğin `Rc<T>`: Bir `Rc<T>` değerini klonlayıp kopyalardan birinin sahipliğini başka bir iş parçacığına aktarmaya çalışırsanız, her iki iş parçacığı da referans sayacını aynı anda güncelleyebilir. Bu nedenle `Rc<T>`, iş parçacığı güvenliğinin getireceği performans maliyetini ödemek istemediğiniz tek iş parçacıklı durumlar için uygulanmıştır.

Dolayısıyla Rust’ın tür sistemi ve **öznitelik sınırları** (trait bounds), bir `Rc<T>` değerini iş parçacıkları arasında kazara ve güvensiz biçimde gönderemeyeceğinizden emin olur. Bunu Listing 16-14’te yapmayı denediğimizde, `Rc<Mutex<i32>>` için `Send` özniteliğinin uygulanmadığına dair hatayı almıştık. `Send` uygulayan `Arc<T>`’ye geçtiğimizde ise kod derlendi.

Tamamıyla `Send` türlerinden oluşan herhangi bir tür de otomatik olarak `Send` olarak işaretlenir. Hemen hemen tüm ilkel türler (primitive types) `Send`’dir; istisna olarak **ham işaretçiler** (raw pointers) bulunur; bunlardan Bölüm 20’de bahsedeceğiz.

---

## 👥 `Sync` ile Birden Fazla İş Parçacığından Erişime İzin Vermek (allowing access from multiple threads with Sync)

`Sync` **işaretleyici özniteliği** (marker trait), `Sync` uygulayan türün birden fazla iş parçacığından güvenli biçimde referans alınabileceğini belirtir. Başka bir deyişle, herhangi bir tür `T`, eğer `&T` (T’ye değiştirilemez referans — immutable reference) `Send` ise `Sync`’tir; yani referans başka bir iş parçacığına güvenle gönderilebilir. `Send`’e benzer şekilde, ilkel türlerin tümü `Sync`’tir ve tamamen `Sync` türlerinden oluşan türler de `Sync` olur.

Akıllı işaretçi `Rc<T>`, `Send` uygulamadığı nedenlerle `Sync` de uygulamaz. Bölüm 15’te ele aldığımız `RefCell<T>` türü ve ilişkili `Cell<T>` ailesi de `Sync` değildir. `RefCell<T>`’nin çalışma zamanında (runtime) yaptığı ödünç alma denetimi (borrow checking) iş parçacığı güvenli değildir. Akıllı işaretçi `Mutex<T>` ise `Sync` uygular ve “Bir `Mutex<T>`’yi Birden Fazla İş Parçacığı Arasında Paylaşmak” (sharing a `Mutex<T>` between multiple threads) bölümünde gördüğünüz gibi birden fazla iş parçacığıyla paylaşarak erişim sağlamada kullanılabilir.


## ⚠️ `Send` ve `Sync` Özniteliklerini Elle Uygulamak Güvensizdir (implementing Send and Sync manually is unsafe)

Tamamen `Send` ve `Sync` özniteliklerini uygulayan türlerden oluşan türler, otomatik olarak `Send` ve `Sync` olur. Bu yüzden bu öznitelikleri (traits) manuel olarak uygulamamız gerekmez. İşaretleyici öznitelikler (marker traits) oldukları için herhangi bir metodları bile yoktur. Sadece eşzamanlılık (concurrency) ile ilgili değişmezleri (invariants) zorunlu kılmak için kullanışlıdırlar.

Bu öznitelikleri manuel olarak uygulamak, **güvensiz Rust kodu** (unsafe Rust code) yazmayı gerektirir. Bölüm 20’de güvensiz Rust kodunun kullanımını ele alacağız; şimdilik önemli bilgi şudur: `Send` ve `Sync` parçalarından oluşmayan yeni eşzamanlı türler oluşturmak, güvenlik garantilerini korumak için çok dikkatli düşünmeyi gerektirir. Bu garantiler ve onları nasıl koruyacağınız hakkında daha fazla bilgiyi *The Rustonomicon* kaynağında bulabilirsiniz.

---

## 📋 Özet (summary)

Bu kitapta eşzamanlılığı son kez görmüyoruz: bir sonraki bölüm, **async programlama**ya odaklanacak ve Bölüm 21’deki proje, burada tartıştığımız küçük örneklerden daha gerçekçi bir durumda bu kavramları kullanacak.

Daha önce de belirtildiği gibi, Rust’ın eşzamanlılığı ele alma yöntemlerinin çok azı dilin parçasıdır. Birçok eşzamanlılık çözümü `crate`ler olarak uygulanmıştır. Bunlar standart kütüphaneden daha hızlı evrim geçirir, bu yüzden çok iş parçacıklı (multithreaded) durumlarda kullanmak için güncel, en ileri düzey `crate`leri araştırdığınızdan emin olun.

Rust standart kütüphanesi, mesaj geçişi (message passing) için **kanallar (channels)** ve eşzamanlı bağlamlarda güvenle kullanılabilen **akıllı işaretçi türleri** (smart pointer types) sağlar: örneğin `Mutex<T>` ve `Arc<T>`. Tür sistemi ve ödünç alma denetleyicisi (borrow checker), bu çözümleri kullanan kodun veri yarışlarına (data races) veya geçersiz referanslara yol açmamasını garanti eder. Kodunuzu derlemeyi başardığınızda, başka dillerde yaygın olan zor izlenebilir hatalar olmadan, çok iş parçacığında sorunsuz çalışacağından emin olabilirsiniz.

Artık eşzamanlı programlama korkulacak bir kavram değil: cesurca (fearlessly) programlarınızı eşzamanlı hale getirin! 🚀
