## 🧹 Drop Trait ile Temizlik Anında Kod Çalıştırma

Akıllı işaretçi deseninde önemli olan ikinci trait, **Drop** trait’idir. Bu trait, bir değer kapsamdan (scope) çıkmak üzereyken ne olacağını özelleştirmenizi sağlar. Herhangi bir tür üzerinde `Drop` trait’ini uygulayabilir ve oraya dosya veya ağ bağlantıları gibi kaynakları serbest bırakacak kodu koyabilirsiniz.

Akıllı işaretçiler bağlamında `Drop` trait’ini tanıtıyoruz çünkü akıllı işaretçilerin uygulanmasında neredeyse her zaman `Drop` işlevselliği kullanılır. Örneğin, bir `Box<T>` düşürüldüğünde (`drop` edildiğinde) kutunun işaret ettiği heap alanı serbest bırakılır.

Bazı dillerde ve türlerde, programcı her seferinde bellek veya kaynakları serbest bırakacak kodu kendisi çağırmak zorundadır (örnek: dosya tanıtıcıları, soketler, kilitler). Eğer unutulursa sistem aşırı yüklenebilir ve çöker. Rust’ta ise, bir değer kapsamdan çıktığında çalışacak kodu belirtebilirsiniz; derleyici bu kodu otomatik olarak ekler. Böylece programın farklı yerlerinde örnekler kullanıldıktan sonra temizlik kodunu koymayı unutsanız bile kaynak sızıntısı olmaz!

Bir değer kapsamdan çıktığında çalışacak kodu belirtmek için `Drop` trait’ini uygularsınız. `Drop` trait’i, `self`’in mutable referansını alan `drop` adlı tek bir metot sağlamanızı ister. Rust’ın `drop` metodunu ne zaman çağırdığını görmek için şimdilik `println!` ifadeleri kullanalım.

---

## 📦 Özel Akıllı İşaretçi Örneği

15-14 numaralı listede, yalnızca kapsamdan çıktığında mesaj yazdıran bir `CustomSmartPointer` struct tanımlıyoruz.

**Dosya adı: src/main.rs**

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

*Liste 15-14: Temizlik kodunun yerleştirilebileceği `Drop` trait’ini uygulayan `CustomSmartPointer`*

`Drop` trait’i prelude içindedir, yani ayrıca içeri aktarmamız gerekmez. `CustomSmartPointer` üzerinde `Drop` uyguladık ve `drop` metodunda `println!` çağırdık. Bu metodun gövdesi, örneğiniz kapsamdan çıktığında çalıştırmak istediğiniz temizlik kodunu koyacağınız yerdir.

`main` içinde iki `CustomSmartPointer` oluşturduk ve *CustomSmartPointers created.* yazdırdık. `main` bittiğinde örnekler kapsamdan çıkacak ve Rust otomatik olarak `drop` metodunu çağırarak belirttiğimiz mesajları yazdıracaktır. Burada `drop` metodunu biz çağırmadık.

Program çalıştırıldığında şu çıktıyı görürüz:

```
$ cargo run
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

Rust, örnekler kapsamdan çıktığında bizim belirttiğimiz `drop` kodunu çağırdı. Değişkenler oluşturulma sırasının tersine düşürülür: bu yüzden `d`, `c`’den önce düşürüldü. Bu örnek yalnızca `drop` metodunun nasıl çalıştığını görsel olarak göstermek için yazıldı; gerçek kullanımda genellikle temizlik kodu koyarsınız.

---

## 🚫 Drop’u Elle Çağırmaya Çalışmak

Otomatik `drop` işlevini devre dışı bırakmak kolay değildir ve genellikle gerek de yoktur; zaten `Drop` trait’inin amacı otomatik çalışmasıdır. Ama bazen değeri erkenden temizlemek isteyebilirsiniz. Örneğin, kilit yöneten akıllı işaretçilerde, kilidi serbest bırakmak için `drop`’un erken çağrılmasını isteyebilirsiniz.

Rust, `Drop` trait’inin `drop` metodunu elle çağırmanıza izin vermez. Bunun yerine, bir değeri kapsam sonuna gelmeden düşürmek için standart kütüphanede sağlanan `std::mem::drop` fonksiyonunu çağırmanız gerekir.

Eğer 15-14’teki `main` fonksiyonunu değiştirip `drop` metodunu elle çağırmaya kalkarsak (15-15), derleme hatası alırız.

**Dosya adı: src/main.rs**
*Bu kod derlenmez!*

## 🧹 Temizlikte Kod Çalıştırma: Drop Trait (Drop trait)

Aşağıdaki 15-15 numaralı listedeki gibi, `Drop` trait’inin (Drop trait) `drop` metodunu el ile çağırmayı denersek, derleyici hata verir:

**Dosya adı: src/main.rs**

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
```

*Liste 15-15: Erken temizlik yapmak için Drop trait’inden `drop` metodunu elle çağırmaya çalışma*

Bu kodu derlemeye çalıştığımızda şu hatayı alırız:

```
$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
error[E0040]: explicit use of destructor method
  --> src/main.rs:16:7
   |
16 |     c.drop();
   |       ^^^^ explicit destructor calls not allowed
   |
help: consider using `drop` function
   |
16 |     drop(c);
   |     +++++ ~

For more information about this error, try `rustc --explain E0040`.
error: could not compile `drop-example` (bin "drop-example") due to 1 previous error
```

Bu hata, `drop`’u açıkça çağırmamıza izin verilmediğini belirtir. Hata iletisinde kullanılan *destructor* (destructor) terimi, bir örneğin temizliğini yapan fonksiyon için genel programlama terimidir. *Destructor*, bir örnek oluşturan *constructor*’a benzetilebilir. Rust’taki `drop` fonksiyonu belirli bir destructordur.

Rust, `drop`’u açıkça çağırmamıza izin vermez, çünkü `main`’in sonunda değerin üzerinde yine otomatik olarak `drop` çağrılacaktır. Bu durum, aynı değerin iki kez temizlenmeye çalışılması nedeniyle *double free* hatasına yol açar.

Bir değer kapsamdan (scope) çıktığında otomatik `drop` eklenmesini devre dışı bırakamayız ve `drop` metodunu açıkça çağıramayız. Bu nedenle, bir değerin erken temizlenmesini zorunlu kılmamız gerekiyorsa, standart kütüphane tarafından sağlanan `std::mem::drop` fonksiyonunu (prelude) kullanırız.

`std::mem::drop` fonksiyonu, Drop trait’inin `drop` metodundan farklıdır. Zorla düşürmek (force-drop) istediğimiz değeri argüman olarak vererek çağırırız. Bu fonksiyon prelude içindedir; bu yüzden 15-15 numaralı listedeki `main` fonksiyonunu, 15-16’da gösterildiği gibi `drop` fonksiyonunu çağıracak şekilde değiştirebiliriz.

**Dosya adı: src/main.rs**

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

*Liste 15-16: Bir değer kapsamdan çıkmadan önce açıkça düşürmek için `std::mem::drop` çağırma*

Bu kod çalıştırıldığında aşağıdaki çıktı alınır:

```
$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/drop-example`
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
```

*Dropping CustomSmartPointer with data `some data`!* metni, *CustomSmartPointer created.* ile *CustomSmartPointer dropped before the end of main.* metinleri arasında yazdırılır; bu da `drop` metodundaki kodun o noktada `c`’yi düşürmek için çağrıldığını gösterir.

Drop trait’inde (Drop trait) belirtilen kodu, temizliği kolay ve güvenli kılmak için pek çok şekilde kullanabilirsiniz: örneğin, kendi bellek ayırıcınızı bile oluşturabilirsiniz! Drop trait ve Rust’ın sahiplik (ownership) sistemi sayesinde, temizliği hatırlamanıza gerek kalmaz çünkü Rust bunu otomatik olarak yapar.

Ayrıca hâlâ kullanılmakta olan değerleri yanlışlıkla temizlemekten de endişe etmenize gerek yoktur: başvuruların her zaman geçerli olmasını sağlayan sahiplik sistemi, `drop`’un yalnızca değer artık kullanılmadığında ve yalnızca bir kez çağrılacağını da garanti eder.

Artık `Box<T>`’i ve akıllı işaretçilerin bazı özelliklerini incelediğimize göre, standart kütüphanede tanımlı birkaç başka akıllı işaretçiye bakalım.
