## 🧹 Drop Trait ile Temizlik Anında Kod Çalıştırma

Akıllı işaretçi deseninde önemli olan ikinci trait, **Drop** trait’idir. Bu trait, bir değer kapsamdan (scope) çıkmak üzereyken ne olacağını özelleştirmenizi sağlar. Herhangi bir tür üzerinde `Drop` trait’ini uygulayabilir ve orada dosyalar veya ağ bağlantıları gibi kaynakları serbest bırakmak için kullanılacak kodu yazabilirsiniz.

Akıllı işaretçilerin çoğunda `Drop` işlevselliği kullanıldığı için burada tanıtıyoruz. Örneğin, bir `Box<T>` düşürüldüğünde (`drop` edildiğinde), kutunun işaret ettiği heap alanı serbest bırakılır.

Bazı dillerde ve bazı türlerde, programcının her seferinde bellek veya kaynakları serbest bırakmak için özel kod çağırması gerekir (örnek: dosya tanıtıcıları, soketler, kilitler). Eğer bunu unuturlarsa sistem aşırı yüklenebilir veya çöker. Rust’ta ise, bir değer kapsamdan çıktığında çalışacak kodu belirtebilirsiniz ve derleyici bu kodu otomatik olarak ekler. Böylece, programda bir türün örneği bittiğinde her yerde temizlik kodu yazmak zorunda kalmazsınız ve yine de kaynak sızıntısı olmaz!

Bir değer kapsamdan çıktığında çalışacak kodu belirtmek için `Drop` trait’ini uygularsınız. `Drop` trait’i sizden, `self`’in mutable referansını alan ve `drop` adlı tek bir metot sağlamanızı ister. Rust’ın `drop` çağrısını ne zaman yaptığını görmek için şimdilik `println!` ifadeleriyle deneyelim.

---

## 📦 Özel Akıllı İşaretçi Örneği

15-14 numaralı listede, `CustomSmartPointer` adlı bir struct tanımlıyoruz. Bu struct’ın tek özelliği, kapsamdan çıktığında *Dropping CustomSmartPointer!* yazdırmasıdır.

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

*Liste 15-14: Temizlik kodunu koyabileceğimiz `Drop` trait’ini uygulayan bir `CustomSmartPointer` struct*

`Drop` trait’i prelude içindedir, bu yüzden ayrıca içeri aktarmamız gerekmez. `CustomSmartPointer` için `Drop` trait’ini uyguladık ve `drop` metodunda `println!` çağrısı yaptık. Burada kendi tipinizin örneği kapsamdan çıktığında çalışmasını istediğiniz herhangi bir kodu koyabilirsiniz.

`main` içinde iki `CustomSmartPointer` örneği oluşturup *CustomSmartPointers created.* yazdırıyoruz. `main` fonksiyonu bittiğinde bu örnekler kapsamdan çıkacak ve Rust otomatik olarak `drop` metodunu çağıracaktır. Dikkat edin, `drop` metodunu kendimiz çağırmadık.

Çalıştırıldığında şu çıktıyı görürüz:

```
$ cargo run
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

Rust, örnekler kapsamdan çıktığında bizim belirttiğimiz `drop` kodunu otomatik çağırdı. Değişkenler, oluşturulma sırasının tersine düşürülür: bu yüzden `d`, `c`’den önce düşürüldü. Bu örnek, `drop` metodunun nasıl çalıştığını görsel olarak göstermek için hazırlandı; gerçek kullanımda ise genellikle temizleme kodu yazarsınız.

---

## 🚫 Drop’u Elle Çağırma Girişimi

Rust’ta `drop` metodunu elle çağırmak mümkün değildir. `Drop`’un amacı otomatik çalışmasıdır. Ancak bazen değeri erken temizlemek isteyebilirsiniz. Örneğin, kilitleri yöneten bir akıllı işaretçi kullanıyorsanız, kilidi bırakmak için `drop`’u erkenden çalıştırmak isteyebilirsiniz. Bunun için Rust, `Drop` trait’inin `drop` metodunu doğrudan çağırmanıza izin vermez; bunun yerine standart kütüphanedeki `std::mem::drop` fonksiyonunu çağırmanız gerekir.

15-15 numaralı listedeki kodda, `drop` metodunu elle çağırmayı denersek hata alırız.

**Dosya adı: src/main.rs**
*Bu kod derlenmez!*

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

*Liste 15-15: `drop` metodunu elle çağırmaya çalışmak*

Derleyici hatası şu şekilde olur:

```
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
```

Bu hata, `drop` metodunu açıkça çağırmamıza izin verilmediğini gösterir. Burada kullanılan *destructor* terimi, genel olarak bir örneği temizleyen fonksiyon anlamına gelir. Rust’taki `drop` da bir destructor’dur. Ancak Rust, otomatik olarak da `drop` çağıracağı için elle çağırmak çift serbest bırakmaya (*double free*) neden olabilir.

---

## ✅ std::mem::drop Kullanımı

Kapsam sonuna gelmeden bir değeri düşürmek (temizlemek) istiyorsak `std::mem::drop` fonksiyonunu kullanırız. Bu fonksiyon `drop` metodundan farklıdır; bir değeri argüman olarak alır ve hemen düşürür.

15-16 numaralı liste, `drop` fonksiyonunu kullanarak değeri erkenden temizlemeyi göstermektedir:

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

*Liste 15-16: `std::mem::drop` kullanarak bir değeri kapsam bitmeden düşürmek*

Çıktı şu şekilde olur:

```
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
```

Görüldüğü gibi `drop` metodu `c` için erkenden çağrıldı.

---

## 🔐 Drop Trait’in Gücü

`Drop` trait’i ve Rust’ın sahiplik sistemiyle:

* Temizliği unutmaktan korkmanıza gerek yoktur çünkü Rust otomatik yapar.
* Kullanılmaya devam eden değerlerin yanlışlıkla temizlenmesi mümkün değildir çünkü sahiplik sistemi başvuruların geçerli olmasını sağlar ve `drop` yalnızca değer gerçekten kullanılmadığında çağrılır.

Hatta `Drop` ile kendi bellek ayırıcılarınızı bile yazabilirsiniz!

---

Artık `Box<T>` ve akıllı işaretçilerin bazı özelliklerini incelediğimize göre, standart kütüphanede tanımlı diğer akıllı işaretçilere göz atalım.
