## 🧩 RefCell<T> ve İçsel Değiştirilebilirlik Deseni (interior mutability pattern)

İçsel değiştirilebilirlik (interior mutability), Rust’ta bir tasarım desenidir (design pattern) ve normalde ödünç alma kuralları (borrowing rules) tarafından yasaklanan bir duruma, yani bir veriye o veri için değiştirilemez referanslar (immutable references) varken mutasyon (mutation) yapabilmenize izin verir. Veriyi değiştirebilmek için bu desen, veri yapısının içinde Rust’ın mutasyon ve ödünç alma kurallarını yöneten olağan kurallarını esnetmek amacıyla `unsafe` kodu (unsafe code) kullanır. `Unsafe` kod, derleyiciye (compiler) kuralları derleyiciye bırakmak yerine bunları elle kontrol ettiğimizi belirtir; `unsafe` kodu 20. bölümde daha ayrıntılı tartışacağız.

İçsel değiştirilebilirlik desenini kullanan türleri ancak, derleyici bunu garanti edemese bile, çalışma zamanında (runtime) ödünç alma kurallarına uyulacağını sağlayabildiğimizde kullanabiliriz. Bu durumda `unsafe` kod, güvenli bir API’nin (safe API) içine sarılır ve dıştaki tür hâlâ değiştirilemez (immutable) kalır.

Bu kavramı, içsel değiştirilebilirlik desenini izleyen `RefCell<T>` türüne bakarak inceleyelim.

## 🛡️ RefCell<T> ile Ödünç Alma Kurallarını Çalışma Zamanında Zorlamak (enforcing borrowing rules at runtime with RefCell<T>)

`Rc<T>`’nin aksine, `RefCell<T>` türü tuttuğu veriler üzerinde tekil sahipliği (single ownership) temsil eder. Peki `RefCell<T>`’i `Box<T>` gibi bir türden ayıran nedir? 4. bölümde öğrendiğiniz ödünç alma kurallarını (borrowing rules) hatırlayın:

* Herhangi bir anda ya bir adet değiştirilebilir (mutable) referansa ya da herhangi sayıda değiştirilemez (immutable) referansa sahip olabilirsiniz (ama ikisine birden değil).
* Referanslar her zaman geçerli olmalıdır (must always be valid).

Referanslar ve `Box<T>` ile, ödünç alma kurallarının değişmezleri derleme zamanında (compile time) zorlanır. `RefCell<T>` ile bu değişmezler çalışma zamanında (runtime) zorlanır. Referanslarla bu kuralları ihlal ederseniz, bir derleyici hatası (compiler error) alırsınız. `RefCell<T>` ile bu kuralları ihlal ederseniz, programınız `panic` eder ve sonlanır.

Ödünç alma kurallarını derleme zamanında denetlemenin avantajı, hataların geliştirme sürecinin daha erken aşamasında yakalanması ve tüm analiz önceden tamamlandığı için çalışma zamanı performansına herhangi bir etkisinin olmamasıdır. Bu nedenlerle, çoğu durumda kuralları derleme zamanında denetlemek en iyi seçimdir ve Rust’ın varsayılan davranışı budur.

Kuralları çalışma zamanında denetlemenin avantajı ise, derleme zamanı denetimlerinin izin vermeyeceği bazı bellek açısından güvenli (memory-safe) senaryoların mümkün olmasıdır. Rust derleyicisi gibi statik analiz (static analysis) doğası gereği tutucudur (conservative). Kodun bazı özelliklerini yalnızca kodu analiz ederek saptamak imkânsızdır: bunun en meşhur örneği, bu kitabın kapsamı dışında kalan ancak araştırması ilginç bir konu olan Durdurma Problemi’dir (Halting Problem).

Bazı analizler imkânsız olduğundan, Rust derleyicisi kodun sahiplik kurallarına (ownership rules) uyduğundan emin olamazsa, doğru bir programı reddedebilir; bu anlamda tutucudur. Rust yanlış bir programı kabul etseydi, kullanıcılar Rust’ın sağladığı garantilere güvenemezdi. Ancak Rust doğru bir programı reddederse, programcı için zahmetli olabilir ama yıkıcı bir şey olmaz. `RefCell<T>` türü, kodunuzun ödünç alma kurallarına uyduğundan emin olduğunuz fakat derleyicinin bunu anlayıp garanti edemediği durumlarda kullanışlıdır.

`Rc<T>`’ye benzer biçimde, `RefCell<T>` yalnızca tek iş parçacıklı (single-threaded) senaryolarda kullanılır ve onu çok iş parçacıklı (multithreaded) bir bağlamda kullanmaya çalışırsanız derleme zamanı hatası verilir. `RefCell<T>` işlevselliğini çok iş parçacıklı bir programda nasıl elde edebileceğimizi 16. bölümde konuşacağız.

`Box<T>`, `Rc<T>` veya `RefCell<T>` seçmek için gerekçelerin özeti:

* `Rc<T>` aynı verinin birden çok sahibini (multiple owners) mümkün kılar; `Box<T>` ve `RefCell<T>` tekil sahiptir (single owners).
* `Box<T>`, derleme zamanında denetlenen değiştirilemez veya değiştirilebilir ödünçlere (immutable/mutable borrows checked at compile time) izin verir; `Rc<T>` yalnızca derleme zamanında denetlenen değiştirilemez ödünçlere izin verir; `RefCell<T>` ise çalışma zamanında denetlenen değiştirilemez veya değiştirilebilir ödünçlere (borrows checked at runtime) izin verir.
* `RefCell<T>` çalışma zamanında denetlenen değiştirilebilir ödünçlere izin verdiği için, `RefCell<T>` değiştirilemez (immutable) olsa bile içindeki değeri değiştirebilirsiniz (mutate).

Değiştirilemez bir değerin içindeki değeri değiştirmek içsel değiştirilebilirlik desenidir (interior mutability pattern). Bu desenin faydalı olduğu bir duruma bakalım ve bunun nasıl mümkün olduğuna göz atalım.

## 🔄 İçsel Değiştirilebilirlik: Değiştirilemez Bir Değere Değiştirilebilir Ödünç (interior mutability: a mutable borrow to an immutable value)

Ödünç alma kurallarının bir sonucu olarak, elinizde değiştirilemez (immutable) bir değer varken onu değiştirilebilir (mutable) olarak ödünç alamazsınız. Örneğin, aşağıdaki kod derlenmez:

```
This code does not compile!
fn main() {
    let x = 5;
    let y = &mut x;
}
```

Bu kodu derlemeye çalışırsanız aşağıdaki hatayı alırsınız:

```
$ cargo run
   Compiling borrowing v0.1.0 (file:///projects/borrowing)
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> src/main.rs:3:13
  |
3 |     let y = &mut x;
  |             ^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut x = 5;
  |         +++
```

```
For more information about this error, try `rustc --explain E0596`.
error: could not compile `borrowing` (bin "borrowing") due to 1 previous error
```

Bununla birlikte, bir değerin kendi yöntemlerinde (methods) kendini mutasyona uğratması ancak diğer koda değiştirilemez (immutable) görünmesi yararlı olabilecek durumlar vardır. Değerin yöntemleri dışındaki kod, değeri değiştiremez. `RefCell<T>` kullanmak, içsel değiştirilebilirlik (interior mutability) yeteneğini elde etmenin bir yoludur; ancak `RefCell<T>` kuralları bütünüyle aşmaz: derleyicideki ödünç denetleyici (borrow checker) bu içsel değiştirilebilirliğe izin verir ve ödünç alma kuralları bunun yerine çalışma zamanında (runtime) denetlenir. Kuralları ihlal ederseniz, derleyici hatası yerine bir `panic!` alırsınız.

Şimdi `RefCell<T>` kullanarak değiştirilemez bir değeri nasıl mutasyona uğratabileceğimizi gösteren pratik bir örnek üzerinden ilerleyelim ve bunun neden yararlı olduğunu görelim.


## 🧪 İçsel Değiştirilebilirlik için Bir Kullanım Senaryosu: Sahte Nesneler (mock objects)

Bazen test sırasında bir programcı belirli bir davranışı gözlemlemek ve doğru bir şekilde uygulanıp uygulanmadığını doğrulamak için bir türü başka bir tür yerine kullanır. Bu geçici tür, **test double** (test dublörü) olarak adlandırılır. Bunu film yapımındaki **stunt double** (dublör) kavramı gibi düşünün; bir kişi oyuncunun yerine geçerek zor bir sahneyi gerçekleştirir. Test dublörleri, testleri çalıştırırken başka türlerin yerine geçer. **Mock objects** (sahte nesneler), test sırasında gerçekleşenleri kaydeden ve doğru eylemlerin gerçekleştiğini doğrulamanızı sağlayan belirli türde test dublörleridir.

Rust, diğer dillerin sahip olduğu anlamda nesnelere sahip değildir ve standart kütüphanesinde sahte nesne işlevselliği bulunmaz. Ancak, bir **struct** tanımlayarak sahte nesneyle aynı amaca hizmet edebilirsiniz.

### 📊 Senaryo

Test edeceğimiz senaryo şu: Bir değeri maksimum değere göre izleyen ve mevcut değer maksimuma ne kadar yakınsa buna bağlı olarak mesaj gönderen bir kütüphane oluşturacağız. Bu kütüphane, örneğin bir kullanıcının yapmasına izin verilen API çağrısı kotasını takip etmek için kullanılabilir.

Kütüphane yalnızca bir değerin maksimuma ne kadar yakın olduğunu izleme işlevini ve hangi zamanlarda hangi mesajların gönderilmesi gerektiğini sağlayacak. Kütüphaneyi kullanan uygulamalar, mesajları göndermek için mekanizmayı kendileri sağlayacak: uygulama mesajı kendi içinde gösterebilir, e-posta gönderebilir, SMS atabilir veya başka bir şey yapabilir. Kütüphanenin bu ayrıntıyı bilmesine gerek yok. Tek ihtiyacı, bizim sağlayacağımız `Messenger` adlı bir trait’i uygulayan bir şeydir. 15-20 numaralı listede kütüphane kodunu görebilirsiniz.

```
Filename: src/lib.rs
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

Listing 15-20: Bir değerin maksimum değere ne kadar yaklaştığını takip eden ve belirli seviyelerde uyarılar veren kütüphane

Bu koddaki önemli kısımlardan biri, `Messenger` trait’inin yalnızca bir metodu olmasıdır: `send`. Bu metodun ilk parametresi `&self` yani değiştirilemez bir referanstır ve ikinci parametresi mesajın metnidir. Bu trait, sahte nesnemizin gerçek nesne gibi kullanılabilmesi için uygulaması gereken arayüzdür. Diğer önemli kısım, `LimitTracker` içindeki `set_value` metodunun davranışını test etmek istememizdir. `value` parametresine ne geçeceğimizi değiştirebiliriz, ancak `set_value` bize doğrudan bir sonuç döndürmez. Bizim istediğimiz, eğer `LimitTracker` bir `Messenger` implementasyonu ve belirli bir `max` değeriyle oluşturulursa, farklı `value` değerleri verdiğimizde doğru mesajların gönderilip gönderilmediğini kontrol edebilmektir.

Bunun için, `send` çağrıldığında e-posta veya SMS göndermek yerine yalnızca hangi mesajların gönderilmesi gerektiğini kaydeden bir sahte nesneye ihtiyacımız var. Yeni bir sahte nesne örneği oluşturabilir, onu kullanan bir `LimitTracker` yaratabilir, `set_value` metodunu çağırabilir ve ardından sahte nesnede beklenen mesajların kaydedilip kaydedilmediğini kontrol edebiliriz. 15-21 numaralı liste bu denemenin bir versiyonunu göstermektedir, ancak ödünç alma denetleyicisi (borrow checker) buna izin vermeyecektir.

```
Filename: src/lib.rs
This code does not compile!
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

Listing 15-21: Borrow checker’ın izin vermediği bir `MockMessenger` uygulama girişimi

Bu test kodu, `sent_messages` alanında `Vec<String>` tutan bir `MockMessenger` struct’ı tanımlar. Bu alan, gönderilmesi istenen mesajların kaydını tutar. Ayrıca, boş mesaj listesiyle başlayan yeni `MockMessenger` değerlerini kolayca oluşturmak için bir `new` fonksiyonu tanımlarız. Ardından `Messenger` trait’ini `MockMessenger` için uygularız. `send` metodunda parametre olarak iletilen mesajı alır ve `sent_messages` listesine ekleriz.

Testte, `LimitTracker`’ın değeri maksimumun %75’inden fazla bir değere ayarlaması durumunda ne olacağını test ediyoruz. Önce yeni bir `MockMessenger` oluştururuz. Ardından ona referans verilen bir `LimitTracker` oluşturur ve `max` değerini 100 yaparız. `set_value` metodunu 80 değeriyle çağırırız, bu da 100’ün %75’inden fazladır. Son olarak, `MockMessenger`’ın tuttuğu mesaj listesinde bir mesaj bulunması gerektiğini doğrularız.

Ancak burada bir sorun vardır:

```
$ cargo test
   Compiling limit-tracker v0.1.0 (file:///projects/limit-tracker)
error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
  --> src/lib.rs:58:13
   |
58 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

Bu hatanın önerdiği gibi `&mut self` kullanamayız çünkü `Messenger` trait’ini sırf test için değiştirmek istemiyoruz. Bunun yerine, mevcut tasarımımızla uyumlu olacak bir çözüm bulmamız gerekiyor.

İşte bu noktada **içsel değiştirilebilirlik** (interior mutability) devreye girer! `sent_messages`’ı bir `RefCell<T>` içinde saklayarak, `send` metodunun mesajları kaydetmek için bu alanı değiştirmesine izin verebiliriz. 15-22 numaralı liste bunun nasıl göründüğünü göstermektedir.

```
Filename: src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

Listing 15-22: Dış değerin değiştirilemez kabul edilmesine rağmen `RefCell<T>` kullanarak iç değeri değiştirme

`sent_messages` alanı artık `Vec<String>` yerine `RefCell<Vec<String>>` türündedir. `new` fonksiyonunda, boş bir vektör etrafında yeni bir `RefCell<Vec<String>>` örneği oluştururuz.

`send` metodunun implementasyonunda, ilk parametre hâlâ trait tanımıyla uyumlu olarak `&self` yani değiştirilemez bir ödünçtür. Ancak, `self.sent_messages` içindeki `RefCell<Vec<String>>` üzerinde `borrow_mut` çağırarak iç değere (vektör) değiştirilebilir bir referans elde ederiz. Daha sonra bu değiştirilebilir referans üzerinde `push` çağırarak test sırasında gönderilen mesajların kaydını tutabiliriz.

Son yapmamız gereken değişiklik, assert ifadesindedir: iç vektörde kaç öğe olduğunu görmek için `borrow` çağırarak değiştirilemez bir referans elde ederiz.

Artık `RefCell<T>`’yi nasıl kullanacağınızı gördüğünüze göre, onun nasıl çalıştığını inceleyelim!


## 📌 Çalışma Zamanında Ödünçleri Takip Etmek: RefCell<T> (keeping track of borrows at runtime with RefCell<T>)

Değiştirilemez (immutable) ve değiştirilebilir (mutable) referanslar oluştururken sırasıyla `&` ve `&mut` sözdizimini kullanırız. `RefCell<T>` ile bunun yerine güvenli API’sinin (safe API) parçası olan `borrow` ve `borrow_mut` metodlarını kullanırız. `borrow` metodu akıllı işaretçi türü (smart pointer type) `Ref<T>` döndürür, `borrow_mut` ise `RefMut<T>` döndürür. Her iki tür de `Deref` uygular, bu yüzden onları normal referanslar gibi kullanabiliriz.

`RefCell<T>`, aktif olan `Ref<T>` ve `RefMut<T>` akıllı işaretçi sayısını takip eder. Her `borrow` çağrısında, `RefCell<T>` kaç tane değiştirilemez ödünç almanın aktif olduğunu sayısını 1 artırır. Bir `Ref<T>` değeri kapsam dışına çıktığında, değiştirilemez ödünç sayısı 1 azalır. Derleme zamanı ödünç alma kurallarıyla aynı şekilde, `RefCell<T>` de her an birçok değiştirilemez ödünç ya da yalnızca bir değiştirilebilir ödünç olmasına izin verir.

Eğer bu kuralları ihlal etmeye çalışırsak, referanslarda olduğu gibi bir derleyici hatası almak yerine, `RefCell<T>`’nin implementasyonu çalışma zamanında `panic` eder. 15-23 numaralı liste, 15-22’deki `send` implementasyonunun değiştirilmiş bir versiyonunu gösteriyor. Burada özellikle aynı kapsamda iki değiştirilebilir ödünç oluşturmayı deneyerek, `RefCell<T>`’nin çalışma zamanında buna izin vermediğini göstereceğiz.

```
Filename: src/lib.rs
This code panics!
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }
```

Listing 15-23: Aynı kapsamda iki değiştirilebilir referans oluşturma girişimi – `RefCell<T>` `panic` eder

İlk olarak `borrow_mut` tarafından döndürülen `RefMut<T>` akıllı işaretçisini `one_borrow` değişkenine atarız. Sonra aynı şekilde başka bir değiştirilebilir ödünç alıp `two_borrow`’ya atarız. Bu da aynı kapsamda iki mutable referans oluşturur ve buna izin verilmez. Bu kodu testlerde çalıştırdığımızda 15-23’teki kod hatasız derlenecektir, ancak test başarısız olacaktır:

```
$ cargo test
running 1 test
test tests::it_sends_an_over_75_percent_warning_message ... FAILED

thread 'tests::it_sends_an_over_75_percent_warning_message' panicked at src/lib.rs:60:53:
already borrowed: BorrowMutError
```

Kodun `already borrowed: BorrowMutError` mesajıyla `panic` ettiğini görüyoruz. Bu, `RefCell<T>`’nin çalışma zamanında ödünç alma kurallarının ihlalini nasıl ele aldığıdır.

Ödünç alma hatalarını derleme zamanı yerine çalışma zamanında yakalamayı seçmek, hataların geliştirme sürecinde daha geç bulunabileceği anlamına gelir; belki de kodunuz üretime (production) dağıtılana kadar fark edilmeyebilir. Ayrıca, ödünçleri derleme zamanı yerine çalışma zamanında takip etmenin küçük bir performans maliyeti vardır. Ancak `RefCell<T>` kullanarak, yalnızca değiştirilemez değerlerin izin verildiği bir bağlamda sahte nesnenin kendini değiştirerek gördüğü mesajları kaydetmesini sağlayabilirsiniz. Bu, normal referansların sunduğundan daha fazla işlevsellik elde etmenize imkân verir.

---

## 🔄 Rc<T> ve RefCell<T> ile Değiştirilebilir Verilere Çoklu Sahiplik (allowing multiple owners of mutable data with Rc<T> and RefCell<T>)

`RefCell<T>`’nin yaygın bir kullanım şekli, `Rc<T>` ile birlikte kullanılmasıdır. Hatırlarsanız, `Rc<T>` bir verinin birden fazla sahibi olmasını sağlar, fakat bu veriye yalnızca değiştirilemez erişim sunar. Eğer bir `Rc<T>` içinde bir `RefCell<T>` tutarsak, hem çoklu sahipliği olan hem de değiştirilebilir bir değer elde edebiliriz!

Örneğin, 15-18 numaralı listede bir `cons` listesi örneğinde `Rc<T>` kullanarak bir listenin başka bir listeyle sahipliği paylaşmasına izin vermiştik. Ancak `Rc<T>` yalnızca değiştirilemez değerler tuttuğu için, liste oluşturulduktan sonra değerlerini değiştiremeyiz. Buraya `RefCell<T>` ekleyerek listedeki değerleri değiştirebiliriz. 15-24 numaralı liste, `Cons` tanımında `RefCell<T>` kullanarak tüm listelerde saklanan değeri nasıl değiştirebileceğimizi göstermektedir.

```
Filename: src/main.rs
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
```

Listing 15-24: `Rc<RefCell<i32>>` kullanarak değiştirilebilir bir `List` oluşturma

Öncelikle `Rc<RefCell<i32>>` örneğini oluşturur ve `value` adlı değişkende saklarız. Böylece ona daha sonra doğrudan erişebiliriz. Daha sonra `a` listesinde `Cons` varyantı kullanarak bu değeri tutarız. `value`’yu klonlamamız gerekir; böylece hem `a` hem de `value`, içteki `5` değerinin sahibi olur. Aksi halde sahiplik aktarılır veya yalnızca ödünç alma gerçekleşirdi.

Liste `a`’yı bir `Rc<T>` içine sararız; böylece `b` ve `c` listeleri de ona referans verebilir, tıpkı 15-18’de yaptığımız gibi.

Sonrasında `a`, `b` ve `c` listeleri oluşturulduktan sonra, `value` içindeki değere 10 eklemek isteriz. Bunun için `borrow_mut` çağırırız. Bu çağrı, `Rc<T>`’yi otomatik dereference ederek içteki `RefCell<T>` değerine erişmemizi sağlar. `borrow_mut`, bir `RefMut<T>` döndürür ve dereference operatörünü kullanarak içteki değeri değiştiririz.

Sonuçta, `a`, `b` ve `c`’yi yazdırdığımızda hepsinde `5` yerine `15` değerini görürüz:

```
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

Bu teknik oldukça kullanışlıdır! `RefCell<T>` kullanarak dışarıdan bakıldığında değiştirilemez bir `List` değeri elde ederiz. Ama `RefCell<T>`’nin içsel değiştirilebilirlik metodları sayesinde gerektiğinde verimizi değiştirebiliriz. Çalışma zamanı denetimleri, veri yarışlarından (data races) bizi korur ve bazen bu esnekliği sağlamak için biraz hızdan ödün vermek değerlidir.

Unutmayın: `RefCell<T>` çok iş parçacıklı (multithreaded) kodda çalışmaz! Bunun için `RefCell<T>`’nin iş parçacığı güvenli versiyonu `Mutex<T>` kullanılır; `Mutex<T>`’yi 16. bölümde tartışacağız.
