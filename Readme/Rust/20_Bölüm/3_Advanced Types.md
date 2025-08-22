## 🧩 Gelişmiş Türler (advanced types)

Rust tür sistemi, şimdiye kadar bahsettiğimiz ancak henüz ayrıntılı olarak incelemediğimiz bazı özelliklere sahiptir. Öncelikle **newtype (yeni tür)** kavramını ve neden faydalı olduğunu ele alacağız. Daha sonra, `newtype` ile benzer bir özellik olan ancak biraz farklı anlamlara sahip **type alias (tür takma adları)** konusuna geçeceğiz. Ayrıca **! türü** ve **dinamik boyutlu türler (dynamically sized types)** hakkında da konuşacağız.

---

## 🛡️ Tür Güvenliği ve Soyutlama için Newtype Deseni (newtype pattern)

Bu bölüm, daha önce ele aldığımız “Harici Türler Üzerinde Harici Trait’leri Uygulamak için Newtype Deseni Kullanımı” başlıklı bölümü okuduğunuzu varsayar. **Newtype deseni**, şimdiye kadar bahsettiğimiz görevlerin ötesinde de faydalıdır. Örneğin:

* Değerlerin yanlış kullanılmasını derleme zamanında önler.
* Bir değerin **birimlerini (units)** belirtmek için kullanılabilir.

Örneğin, daha önce `Millimeters` ve `Meters` yapılarının (`struct`) bir `u32` değerini sarmaladığını görmüştük. Eğer parametre tipi `Millimeters` olan bir fonksiyon yazarsak, bu fonksiyon `Meters` veya düz bir `u32` ile çağrılamaz.

`Newtype` deseni aynı zamanda soyutlama sağlar: yeni tür, içteki özel (`private`) türden farklı bir **kamuya açık API (public API)** sunabilir.

Ek olarak, `newtype` içsel uygulamayı gizleyebilir. Örneğin, `People` adında bir tür tanımlayıp içine `HashMap<i32, String>` sarmalayabiliriz. Bu, kişilerin kimlik (ID) numarasını isimle eşleştirir. `People` kullanan kod, yalnızca bizim sunduğumuz API ile etkileşime girer (örneğin, isim ekleme metodu). İçeride `i32` ID’ler atadığımızı bilmesine gerek kalmaz. Bu yöntem, kapsülleme (encapsulation) için hafif bir yaklaşımdır.

---

## 📝 Tür Takma Adları ile Tür Eşanlamlıları Oluşturmak (type aliases)

Rust, var olan bir türe başka bir ad vermemizi sağlayan **type alias** özelliğini sunar. Bunun için `type` anahtar kelimesi kullanılır. Örneğin:

```rust
type Kilometers = i32;
```

Burada `Kilometers`, `i32` için bir eşanlamlıdır. `Millimeters` ve `Meters` örneklerinden farklı olarak `Kilometers`, yeni bir tür değildir. `Kilometers` tipindeki değerler, `i32` ile aynı şekilde işlenir:

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

Çünkü `Kilometers` ve `i32` aynı türdür, birbirleriyle toplanabilir ve `i32` parametresi alan fonksiyonlara `Kilometers` geçirilebilir. Ancak bu yöntem, `newtype` deseninde olduğu gibi tür denetimi avantajı sağlamaz.

---

## 🔄 Tekrarlardan Kaçınmak için Tür Takma Adları

Tür takma adlarının en yaygın kullanım amacı, tekrarları azaltmaktır. Örneğin, şu uzun türü düşünelim:

```
Box<dyn Fn() + Send + 'static>
```

Bunu tekrar tekrar yazmak hem yorucu hem de hataya açıktır. Örneğin:

```rust
let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // --snip--
}
```

Bunun yerine bir takma ad tanımlayabiliriz:

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
```

Bu, hem okunabilirliği hem de yazımı kolaylaştırır. Ayrıca anlamlı bir isim seçmek niyetinizi açıkça belirtir. Burada `Thunk`, ileri bir zamanda çalıştırılacak kodu temsil ettiği için uygun bir isimdir.

---

## 📦 Result ile Tür Takma Adlarının Kullanımı

`Type alias` kullanımı, `Result<T, E>` türüyle de oldukça yaygındır. Standart kütüphanedeki `std::io` modülünü düşünelim. Giriş/çıkış (I/O) işlemleri genellikle başarısız olabilir, bu yüzden `Result<T, E>` dönerler. `std::io::Error` tüm olası I/O hatalarını temsil eden bir yapı (`struct`)tır.

Örneğin `Write` trait’inde bazı fonksiyonlar şöyle tanımlanır:

```rust
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

Burada `Result<..., Error>` çokça tekrar eder. Bunu önlemek için `std::io` şu tür takma adını tanımlar:

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

Böylece fonksiyon imzaları şu hale gelir:

```rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

Bu takma ad iki fayda sağlar:

1. Kodun yazımı ve okunması kolaylaşır.
2. Tüm `std::io` genelinde tutarlı bir arayüz sunar.

Ayrıca bu bir alias olduğu için hâlâ `Result<T, E>` türüdür; dolayısıyla `?` operatörü gibi özel sözdizimini ve `Result` üzerindeki tüm metotları kullanmaya devam edebiliriz.

## 🚫 Asla Dönmeyen Tür (never type)

Rust’ta özel bir tür vardır: `!`. Tür kuramında (type theory) bu **boş tür (empty type)** olarak bilinir çünkü hiçbir değeri yoktur. Biz buna **never type (asla dönmeyen tür)** diyoruz çünkü bir fonksiyonun asla dönmeyeceği durumda dönüş türü olarak kullanılır. Örneğin:

```rust
fn bar() -> ! {
    // --snip--
}
```

Bu kod “`bar` fonksiyonu asla dönmez” şeklinde okunur. Bu tür fonksiyonlara **diverging functions (sapma yapan fonksiyonlar)** denir. `!` türünün değeri oluşturulamayacağı için `bar` asla bir değer döndüremez.

Peki, hiç değer üretemeyen bir tür ne işe yarar? Hatırlarsanız, Sayı Tahmin Oyunu’nda (Listing 2-5) şu kod vardı (Listing 20-27):

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

`match` kolları aynı türü döndürmek zorundadır. Bu yüzden aşağıdaki kod derlenmez:

```rust
let guess = match guess.trim().parse() {
    Ok(_) => 5,
    Err(_) => "hello",
};
```

Çünkü `guess` hem `i32` hem de `&str` olamaz. Peki, `continue` ne döndürür? Aslında `continue` ifadesi `!` değerine sahiptir. Yani Rust bu durumda `guess` değişkeninin türünü `u32` olarak belirler, çünkü `!` herhangi bir türle uyuşabilir.

Resmi olarak, `!` türüne sahip ifadeler **herhangi bir türe dönüştürülebilir (coerce edilebilir)**. Bu yüzden `continue` ifadesi `u32` ile aynı blokta yer alabilir. Çünkü `continue` asla bir değer döndürmez, kontrolü döngünün başına gönderir.

Aynı şey `panic!` makrosunda da görülür. Örneğin `Option<T>::unwrap` fonksiyonunda:

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

Burada `val` türü `T`’dir, `panic!` türü ise `!`’dir. Dolayısıyla tüm `match` ifadesi `T` türünde olur. `panic!` programı sonlandırdığı için dönüş değeri gerekmez.

Bir diğer örnek: bitmeyen döngüler de `!` türüne sahiptir:

```rust
print!("forever ");

loop {
    print!("and ever ");
}
```

Bu döngü hiç bitmeyeceği için `!` türüne sahiptir. Ancak eğer içinde `break` olsaydı, döngü sonlanacağı için türü `!` olmazdı.

---

## 📏 Dinamik Boyutlu Türler ve Sized Trait (dynamically sized types, Sized trait)

Rust, bir türün bellekte ne kadar yer kapladığını bilmek zorundadır. Ancak **dinamik boyutlu türler (DST, dynamically sized types)** bu konuda bir istisnadır. Bu türlerin boyutu yalnızca çalışma zamanında bilinebilir.

Örneğin `str` türünü ele alalım. `&str`’i sıkça kullandık, ancak `str` tek başına bir DST’dir. Bir `str` değişkeni oluşturamayız:

```rust
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

Bu kod derlenmez çünkü `s1` 12 bayt, `s2` ise 15 bayt uzunluğundadır. Rust, aynı türe ait değerlerin aynı boyutta olmasını ister.

Çözüm olarak `&str` kullanırız. Bir `&str`, iki bilgiyi saklar: `str`’in başlangıç adresi ve uzunluğu. Böylece `&str`’in boyutu derleme zamanında bilinir (her zaman iki `usize`). Genel kural: **DST değerleri her zaman bir işaretçinin (pointer) arkasında saklanmalıdır.**

`str` yalnızca `&str` ile değil, `Box<str>` veya `Rc<str>` gibi yapılarla da kullanılabilir. Aynı durum trait’ler için de geçerlidir: trait’ler de DST’dir. Bu nedenle onları `&dyn Trait`, `Box<dyn Trait>` veya `Rc<dyn Trait>` gibi işaretçiler aracılığıyla kullanırız.

---

## 📐 Sized Trait ile Çalışmak

DST’lerle çalışabilmek için Rust, `Sized` trait’ini sunar.

* Boyutu derleme zamanında bilinen tüm türler otomatik olarak `Sized`’i uygular.
* Varsayılan olarak, tüm generic fonksiyonlara `Sized` kısıtı eklenir.

Yani şu tanım:

```rust
fn generic<T>(t: T) {
    // --snip--
}
```

aslında şöyle değerlendirilir:

```rust
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

Eğer türün boyutunun derleme zamanında bilinmesi zorunluluğunu kaldırmak istersek `?Sized` kullanırız:

```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

* `?Sized` demek, “`T` boyutu bilinen de olabilir, bilinmeyen de.”
* Bu sözdizimi sadece `Sized` trait’i için geçerlidir.
* Parametre `&T` olarak yazılmıştır çünkü `T` boyutu bilinmeyebilir; işaretçi arkasında tutulması gerekir.

---

## ⏭️ Sonraki Konu

Bir sonraki bölümde **fonksiyonlar ve kapanışlar (functions and closures)** konusuna geçiyoruz!
