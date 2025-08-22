## 🪄 Akıllı İşaretçileri Deref ile Normal Başvurular Gibi Kullanma

`Deref` trait’ini uygulamak, **dereference operatörünün** (`*`) davranışını özelleştirmenizi sağlar (çarpma ya da glob operatörüyle karıştırılmamalıdır). Bir akıllı işaretçiyi normal bir başvuru (reference) gibi davranacak şekilde `Deref` ile uyguladığınızda, başvurular üzerinde çalışan kodu aynı zamanda akıllı işaretçilerle de kullanabilirsiniz.

Önce, dereference operatörünün normal başvurularla nasıl çalıştığına bakalım. Ardından `Box<T>` gibi davranan özel bir tür tanımlamaya çalışacağız ve neden dereference operatörünün bizim tanımladığımız türde normal bir başvuru gibi çalışmadığını göreceğiz. Daha sonra, `Deref` trait’inin akıllı işaretçilerin başvurulara benzer şekilde çalışmasını nasıl mümkün kıldığını inceleyeceğiz. Sonrasında ise Rust’ın **deref coercion** özelliğine bakacağız ve bunun bize hem başvurular hem de akıllı işaretçilerle çalışma imkânı sağladığını göreceğiz.

Not: Birazdan oluşturacağımız `MyBox<T>` türü ile gerçek `Box<T>` arasında büyük bir fark vardır: bizim sürümümüz veriyi heap üzerinde saklamayacaktır. Bu örnekte odak noktamız `Deref` olduğundan, verinin tam olarak nerede saklandığından çok, işaretçi benzeri davranış önemlidir.

---

## 🎯 Değerin Başvurusunu Takip Etmek

Normal bir başvuru bir işaretçi türüdür ve bir işaretçiyi, başka bir yerde saklanan bir değere giden ok gibi düşünebilirsiniz. 15-6 numaralı listede, bir `i32` değerine başvuru oluşturuyoruz ve ardından dereference operatörünü kullanarak başvurunun işaret ettiği değere erişiyoruz.

**Dosya adı: src/main.rs**

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

*Liste 15-6: Bir `i32` değerine başvuruyu takip etmek için dereference operatörünü kullanma*

`x` değişkeni `i32` türünde `5` değerini tutar. `y` değişkenini, `x`’in başvurusu olacak şekilde ayarlarız. `x == 5` olduğunu doğrulayabiliriz. Ancak `y` içindeki değere ilişkin doğrulama yapmak istersek, `*y` kullanmamız gerekir; böylece başvurunun işaret ettiği değeri takip etmiş oluruz (dereference) ve derleyici gerçek değeri karşılaştırabilir. `y`’yi dereference ettiğimizde, artık `y`’nin işaret ettiği tamsayı değerine erişmiş oluruz ve bunu `5` ile karşılaştırabiliriz.

Eğer şu satırı yazmayı deneseydik:

```rust
assert_eq!(5, y);
```

şu derleme hatasını alırdık:

```
$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `deref-example` (bin "deref-example") due to 1 previous error
```

Bir sayı ile bir sayıya başvuruyu karşılaştırmak izinli değildir çünkü bunlar farklı türlerdir. Bu nedenle, işaret edilen değere erişmek için mutlaka dereference operatörünü (`*`) kullanmalıyız.

## 📦 Box<T>’i Bir Başvuru (Reference) Gibi Kullanma

15-6 numaralı listedeki kodu, bir başvuru yerine `Box<T>` kullanacak şekilde yeniden yazabiliriz. 15-7 numaralı listede, `Box<T>` üzerinde kullanılan **dereference operatörü (`*`)**, tıpkı 15-6’daki başvuruda olduğu gibi çalışır:

**Dosya adı: src/main.rs**

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

*Liste 15-7: Bir `Box<i32>` üzerinde dereference operatörünü kullanma*

15-7 ve 15-6 arasındaki temel fark şudur: burada `y`, `x`’in değerine işaret eden bir başvuru değil, `x`’in kopyalanmış değerine işaret eden bir kutu (`Box`) örneği olarak ayarlanmıştır. Son doğrulamada, `y` bir başvuru olduğunda yaptığımız gibi kutunun işaretçisini takip etmek için dereference operatörünü kullanabiliriz. Şimdi, `Box<T>`’yi özel kılan şeyin ne olduğunu görmek için kendi türümüzü tanımlayarak başlayalım.

---

## 🛠️ Kendi Akıllı İşaretçimizi Tanımlamak

Standart kütüphanedeki `Box<T>` türüne benzer bir akıllı işaretçi inşa ederek, akıllı işaretçilerin başvurulardan varsayılan olarak nasıl farklı davrandığını deneyimleyeceğiz. Ardından, dereference operatörünü kullanma yeteneğini nasıl ekleyebileceğimize bakacağız.

`Box<T>` türü, aslında tek bir öğeye sahip tuple `struct` olarak tanımlanır. Bu nedenle 15-8 numaralı listede `MyBox<T>` türünü aynı şekilde tanımlıyoruz. Ayrıca `Box<T>` üzerinde tanımlanan `new` fonksiyonuna karşılık gelen yeni bir fonksiyon da ekliyoruz.

**Dosya adı: src/main.rs**

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

*Liste 15-8: `MyBox<T>` türünü tanımlama*

Burada `MyBox` adında bir `struct` tanımlıyor ve herhangi bir türde değer saklamak istediğimiz için jenerik parametre `T`’yi bildiriyoruz. `MyBox` türü, tek öğesi `T` türünde olan bir tuple `struct`’tur. `MyBox::new` fonksiyonu bir `T` parametresi alır ve verilen değeri tutan bir `MyBox` örneği döndürür.

---

Şimdi, 15-7 numaralı listedeki `main` fonksiyonunu 15-8’e ekleyelim ve `Box<T>` yerine tanımladığımız `MyBox<T>` türünü kullanalım. 15-9 numaralı listedeki kod derlenmeyecektir çünkü Rust, `MyBox`’ın nasıl dereference edileceğini bilmez.

**Dosya adı: src/main.rs**
*Bu kod derlenmez!*

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

*Liste 15-9: `MyBox<T>`’i başvurular ve `Box<T>` ile aynı şekilde kullanmaya çalışma*

Bu durumda derleme hatası şu şekilde olur:

```
$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^

For more information about this error, try `rustc --explain E0614`.
error: could not compile `deref-example` (bin "deref-example") due to 1 previous error
```

`MyBox<T>` türümüz dereference edilemez çünkü bu yeteneği türümüze uygulamadık. `*` operatörüyle dereference edebilmeyi sağlamak için `Deref` trait’ini uygulamamız gerekir.

## 🛠️ Deref Trait’ini Uygulamak

10. bölümdeki *“Bir Tür Üzerinde Trait Uygulamak”* kısmında tartıştığımız gibi, bir trait’i uygulamak için o trait’in gerekli metotlarını sağlamamız gerekir. Standart kütüphanede sağlanan `Deref` trait’i, bizden `deref` adında tek bir metot uygulamamızı ister. Bu metot `self`’i ödünç alır ve iç veriye bir başvuru döndürür. 15-10 numaralı liste, `MyBox<T>` tanımına eklenebilecek `Deref` uygulamasını göstermektedir.

**Dosya adı: src/main.rs**

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

*Liste 15-10: `MyBox<T>` üzerinde `Deref` uygulamak*

`type Target = T;` sözdizimi, `Deref` trait’inin kullanacağı ilişkili bir türü (associated type) tanımlar. İlişkili türler, jenerik parametre tanımlamanın biraz farklı bir yoludur; şimdilik bunlarla fazla ilgilenmenize gerek yok, 20. bölümde ayrıntılı şekilde ele alacağız.

`deref` metodunun gövdesini `&self.0` ile doldurduk; böylece `deref`, `*` operatörüyle erişmek istediğimiz değere bir başvuru döndürür. 5. bölümdeki *“Farklı Türler Yaratmak için Alan İsimleri Olmayan Tuple Struct’lar Kullanmak”* kısmından hatırlayacağınız gibi `.0`, tuple struct içindeki ilk değeri erişmek için kullanılır. Artık 15-9 numaralı listedeki `main` fonksiyonunda `*` ile çağrılan `MyBox<T>` değeri derlenir ve doğrulamalar başarılı olur!

`Deref` trait’i olmadan, derleyici yalnızca `&` başvurularını dereference edebilir. `deref` metodu, derleyiciye `Deref` uygulayan herhangi bir türün değerini alıp `deref` metodunu çağırarak bir `&` başvuru elde etme imkânı verir; derleyici de bu başvuruyu dereference etmeyi bilir.

15-9 numaralı listede `*y` yazdığımızda, aslında Rust perde arkasında şu kodu çalıştırır:

```rust
*(y.deref())
```

Rust, `*` operatörünü bir `deref` çağrısıyla ve ardından normal bir dereference ile değiştirir. Böylece `deref` metodunu doğrudan çağırıp çağırmamamız gerektiğini düşünmek zorunda kalmayız. Bu Rust özelliği sayesinde, elimizde normal bir başvuru olsun ya da `Deref` uygulayan özel bir tür olsun, aynı şekilde çalışan kod yazabiliriz.

`deref` metodunun bir değere başvuru döndürmesinin ve `*(y.deref())` ifadesinde parantez dışındaki normal dereference’in hâlâ gerekli olmasının sebebi, sahiplik (ownership) sistemidir. Eğer `deref` metodu değeri doğrudan döndürseydi, değer `self`’ten taşınmış (move) olurdu. Bu durumda (ve çoğu dereference kullanımında) `MyBox<T>` içindeki değerin sahipliğini almak istemeyiz.

Son olarak, `*` operatörü her kullanıldığında yalnızca bir kez `deref` çağrısıyla değiştirilir. Bu işlem sonsuz özyineleme yapmaz, böylece elimizde `i32` türünde bir veri olur ve bu da 15-9 numaralı listedeki `assert_eq!` içindeki `5` ile eşleşir.

## 🔄 Fonksiyonlar ve Metotlarda Örtük Deref Zorlama (Implicit Deref Coercions)

**Deref zorlama (deref coercion)**, `Deref` trait’ini uygulayan bir türün başvurusunu başka bir türün başvurusuna dönüştürür. Örneğin, `String` türü `Deref` trait’ini `&str` döndürecek şekilde uyguladığı için, `&String` → `&str` dönüşümü yapılabilir. Bu, Rust’ın fonksiyon ve metotlara argüman geçirirken sağladığı bir kolaylıktır ve yalnızca `Deref` trait’ini uygulayan türlerde çalışır. Bu işlem, elimizdeki türün referansını hedef parametre türüne dönüştürene kadar `deref` metodunu otomatik çağırarak gerçekleşir.

`Deref coercion`, programcıların fonksiyon ve metot çağrılarında fazladan `&` ve `*` eklememek için Rust’a eklenmiştir. Ayrıca bu özellik, hem başvurularla hem de akıllı işaretçilerle çalışabilen daha fazla kod yazmamıza imkân tanır.

---

## 👋 Deref Zorlamayı Örnekte Görmek

15-8’de tanımladığımız `MyBox<T>` türünü ve 15-10’da eklediğimiz `Deref` uygulamasını kullanalım. 15-11 numaralı listede, parametresi string slice olan bir fonksiyon tanımlanmıştır.

**Dosya adı: src/main.rs**

```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}
```

*Liste 15-11: Parametresi `&str` olan bir `hello` fonksiyonu*

Bu fonksiyonu `"Rust"` gibi bir string slice ile çağırabiliriz:

```rust
hello("Rust");
```

Deref zorlama sayesinde, bu fonksiyonu `MyBox<String>` değerine bir başvuruyla da çağırabiliriz. (15-12)

**Dosya adı: src/main.rs**

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

*Liste 15-12: `MyBox<String>` başvurusu ile `hello` çağırmak (deref coercion sayesinde çalışır)*

Burada `hello` fonksiyonunu `&m` argümanıyla çağırıyoruz; bu, bir `MyBox<String>` değerine başvurudur. 15-10’daki `Deref` uygulaması sayesinde Rust, `&MyBox<String>` → `&String` dönüşümünü `deref` çağrısıyla yapar. Standart kütüphanedeki `String` türü de `Deref`’i `&str` döndürecek şekilde uygular. Rust, bir kez daha `deref` çağırarak `&String` → `&str` dönüşümünü yapar ve bu da `hello` fonksiyonunun tanımıyla eşleşir.

---

## ❌ Eğer Deref Coercion Olmasaydı

Rust’ta `deref coercion` olmasaydı, 15-12’deki çağrı yerine 15-13’teki gibi daha karmaşık bir kod yazmamız gerekirdi:

**Dosya adı: src/main.rs**

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

*Liste 15-13: Rust’ta deref coercion olmasa yazmamız gereken kod*

Burada `(*m)` ifadesi `MyBox<String>`’i dereference ederek bir `String` üretir. Ardından `&` ve `[..]` ile bu string’in tamamını kapsayan bir string slice (`&str`) alırız. Bu kadar sembol arasında, kodu okumak ve yazmak çok daha zor hale gelir. Deref zorlama, bu dönüşümleri bizim için otomatik olarak yaparak kodu sadeleştirir.

---

## ⚙️ Rust’ın Deref Coercion Mekanizması

`Deref` trait’i tanımlandığında, Rust ilgili türleri analiz eder ve `Deref::deref`’i gerektiği kadar kez çağırarak parametrenin türüyle eşleşen bir başvuru üretir. Kaç kez `deref` çağrılması gerektiği **derleme zamanında** çözülür, bu yüzden deref coercion kullanmanın **çalışma zamanı maliyeti yoktur**.

---

## 🔐 Mutabilite ile Deref Coercion İlişkisi

Nasıl ki `Deref` trait’i ile immutable başvurular için `*` operatörünü özelleştirebiliyorsak, mutable başvurular için de `DerefMut` trait’ini kullanabiliriz.

Rust üç durumda deref coercion yapar:

1. `&T` → `&U` dönüşümü (eğer `T: Deref<Target=U>`)
2. `&mut T` → `&mut U` dönüşümü (eğer `T: DerefMut<Target=U>`)
3. `&mut T` → `&U` dönüşümü (eğer `T: Deref<Target=U>`)

İlk iki durum benzerdir; ikinci durumda mutabilite vardır.

* 1. durumda: `&T` varsa ve `T`, `U`’ya deref yapıyorsa, `&U` elde edilebilir.
* 2. durumda: aynı şey mutable referanslar için geçerlidir.

3. durum daha karmaşıktır: Rust bir mutable başvuruyu immutable’a çevirebilir. Ancak tersi **mümkün değildir**: immutable başvurular mutable’a dönüştürülemez.

Bunun nedeni **ödünç alma kurallarıdır**: eğer bir mutable başvurunuz varsa, o veri için başka hiçbir başvuru olamaz. Bir mutable başvuruyu immutable’a dönüştürmek bu kuralları bozmaz. Ancak immutable → mutable dönüşümü, o veriye başka immutable başvuruların olmadığını garanti etmez. Bu garanti yoksa Rust, immutable → mutable dönüşümüne izin vermez.
