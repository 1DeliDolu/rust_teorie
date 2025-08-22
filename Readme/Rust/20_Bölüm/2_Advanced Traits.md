## 🧩 Gelişmiş Özellikler (advanced traits)

10. Bölümde “Özellikler: Paylaşılan Davranışı Tanımlamak” (Traits: Defining Shared Behavior) kısmında özellikleri (traits) ilk kez ele almıştık, ancak daha ileri detaylara girmemiştik. Artık Rust hakkında daha fazla bilgiye sahip olduğumuz için daha ince ayrıntılara girebiliriz.

---

## 🔗 İlişkili Türler (associated types)

İlişkili türler, bir tür yer tutucusunu (type placeholder) bir trait ile bağlar; böylece trait metod tanımlarında bu yer tutucu türler imzalarında kullanılabilir. Trait’i uygulayan tip (implementor), belirli bir uygulama için bu yer tutucu tür yerine hangi somut türün kullanılacağını belirtir. Bu sayede, türleri tam olarak bilmeden, yalnızca trait uygulandığında belirleyeceğimiz türlerle çalışan bir trait tanımlayabiliriz.

Bu bölümdeki gelişmiş özelliklerin çoğu nadiren gereklidir. İlişkili türler ise orta düzeydedir: kitapta daha önce açıklanan özelliklerden daha seyrek kullanılırlar, ancak bu bölümdeki diğer birçok özelliğe göre daha sık kullanılırlar.

Standart kütüphanede sağlanan `Iterator` trait, ilişkili tür içeren bir trait’e örnektir. İlişkili türün adı `Item`’dır ve `Iterator` trait’ini uygulayan tipin üzerinden geçtiği değerlerin türünü temsil eder. `Iterator` trait’inin tanımı aşağıdaki gibidir (`Listing 20-13`):

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

👉 `Item` türü bir yer tutucudur. `next` metodunun tanımı, `Option<Self::Item>` türünde değer döndüreceğini gösterir. `Iterator` trait’ini uygulayan tip, `Item` için somut türü belirtir ve `next` metodu bu somut türü içeren bir `Option` döndürür.

---

## 🔍 Generics ile Farkı

İlişkili türler, generics’e benzer görünebilir; çünkü generics de bir fonksiyonu belirli türleri belirtmeden tanımlamamıza olanak tanır. Aradaki farkı görmek için `Counter` adında bir tipe `Iterator` trait’ini uygulayalım ve `Item` türünü `u32` olarak belirtelim:

**Filename: src/lib.rs**

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}
```

👉 Bu söz dizimi generics’e benzer görünmektedir. Peki neden `Iterator` trait’i generics ile tanımlanmadı? (`Listing 20-14`)

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

---

## ⚖️ Generics ve İlişkili Türler Arasındaki Fark

* Eğer generics kullansaydık (`Listing 20-14`), her implementasyonda türleri açıkça belirtmemiz gerekirdi.
  Örneğin `Iterator<String>` veya `Iterator<u32>` için `Counter`’a birden fazla uygulama yazabilirdik. Bu, `Counter` için `Iterator`’ın birden fazla versiyonunun olması demektir.
  Dolayısıyla `Counter` üzerinde `next` çağırdığımızda hangi implementasyonu kullanacağımızı belirtmek için tür ek açıklamaları (type annotations) gerekli olurdu.

* İlişkili türlerde ise (`Listing 20-13`) aynı tip için bir trait yalnızca bir kez uygulanabilir. `Counter` için yalnızca **bir tane** `impl Iterator` olabilir ve `Item` türünü yalnızca bir kez seçebiliriz. Bu yüzden `Counter` üzerinde `next` çağırırken her yerde `u32` belirlememize gerek kalmaz.

---

## 📜 İlişkili Türlerin Sözleşmedeki Rolü

İlişkili türler, trait’in sözleşmesinin (contract) bir parçası haline gelir: trait’i uygulayanlar, yer tutucu ilişkili tür için bir somut tür sağlamak zorundadır.

Genellikle ilişkili türlerin adı, türün nasıl kullanılacağını açıklayıcı olur. API dokümantasyonunda ilişkili türü açıkça belgelemek de iyi bir uygulamadır.

## ⚙️ Varsayılan Generic Tür Parametreleri ve Operatör Aşırı Yükleme (default generic type parameters and operator overloading)

Generic tür parametrelerini kullanırken, bu parametreler için **varsayılan somut tür** belirtebiliriz. Böylece trait’i uygulayanların her durumda somut tür belirtmesine gerek kalmaz; eğer varsayılan tür işliyorsa, doğrudan kullanılabilir. Varsayılan tür, `<YerTutucuTür=SomutTür>` sözdizimiyle tanımlanır.

Bu tekniğin faydalı olduğu durumların başında **operatör aşırı yükleme** (operator overloading) gelir. Operatör aşırı yükleme, belirli operatörlerin (örneğin `+`) özel durumlarda davranışını özelleştirmektir.

Rust, kendi operatörlerinizi oluşturmanıza veya rastgele operatörleri aşırı yüklemenize izin vermez. Ancak `std::ops` modülünde listelenen işlemleri, bu işlemlerle ilişkili trait’leri uygulayarak aşırı yükleyebilirsiniz. Örneğin, `Listing 20-15`’te `+` operatörünü iki `Point` örneğini toplamak için aşırı yüklüyoruz. Bunu, `Point` yapısı üzerinde `Add` trait’ini uygulayarak yapıyoruz.

**Filename: src/main.rs**

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

👉 Bu örnekte `add` metodu, iki `Point`’in `x` değerlerini ve `y` değerlerini toplayarak yeni bir `Point` oluşturur. `Add` trait’inde, `Output` adında ilişkili bir tür vardır ve bu, `add` metodunun dönüş türünü belirler.

---

## 📝 `Add` Trait Tanımı ve Varsayılan Tür Parametresi

`Add` trait’inin tanımı şu şekildedir:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

👉 Buradaki yeni sözdizimi `Rhs=Self`’tir. Buna **varsayılan tür parametreleri** (default type parameters) denir.

* `Rhs` (right-hand side), `add` metodundaki `rhs` parametresinin türünü belirler.
* Eğer `Add` trait’ini uygularken `Rhs` için somut tür belirtmezsek, `Rhs` varsayılan olarak `Self` olur.
* Bu da, `Add`’i hangi tip üzerinde uyguluyorsak, `rhs` parametresinin türünün aynı tip olması anlamına gelir.

Örneğin, `Point` için `Add` uyguladığımızda varsayılan `Rhs=Self` kullanılmıştır; yani `Point + Point`.

---

## 📏 Varsayılanı Değiştirme: Millimeters ve Meters Örneği

Bazen varsayılan `Self` yerine farklı bir `Rhs` türü belirtmek isteyebiliriz. Diyelim ki elimizde iki struct var: `Millimeters` ve `Meters`. Bunlar farklı birimleri temsil eder. Amacımız, **milimetreye metre eklemek** ve doğru dönüşümü otomatik yapmaktır. Bunun için `Rhs` türünü `Meters` olarak özelleştiriyoruz (`Listing 20-16`):

**Filename: src/lib.rs**

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

👉 Burada `impl Add<Meters>` ifadesiyle, `Rhs` parametresini `Meters` olarak belirledik. Sonuç türü ise `Millimeters`.

---

## 🎯 Varsayılan Tür Parametrelerinin Kullanım Alanları

Varsayılan tür parametreleri iki ana amaçla kullanılır:

1. **Bir türü mevcut kodu bozmadan genişletmek**

   * Eğer var olan bir trait’e yeni bir tür parametresi eklemek istiyorsanız, bunu varsayılan değerle sağlayarak önceki implementasyonların bozulmasını engelleyebilirsiniz.

2. **Kullanıcıların çoğunun ihtiyaç duymadığı özel durumlarda özelleştirme sağlamak**

   * Standart kütüphanedeki `Add` trait buna örnektir:

     * Çoğu durumda aynı türde iki değer toplanır (`Point + Point`).
     * Ancak özel durumlarda (örneğin `Millimeters + Meters`), parametre özelleştirilerek genişletme yapılabilir.

Bu yaklaşım, çoğu kullanımda ek tür parametresi belirtme zorunluluğunu ortadan kaldırır, yani gereksiz kod tekrarını önler ve trait kullanımını kolaylaştırır.

## 🌀 Aynı İsimdeki Metotlar Arasında Ayrım Yapmak (disambiguating between methods with the same name)

Rust, bir trait’in metodunun başka bir trait’in metoduyla aynı isme sahip olmasını engellemez. Hatta aynı tipe birden fazla trait uygulanabilir ve ayrıca bu tipin kendisine de aynı isimde bir metot yazılabilir.

Bu durumda, hangi metodu çağırmak istediğinizi Rust’a açıkça belirtmeniz gerekir.

---

## ✈️ Örnek: Pilot ve Wizard Trait’leri

Aşağıdaki örnekte (`Listing 20-17`), hem `Pilot` hem de `Wizard` trait’lerinde `fly` metodu tanımlanmıştır. `Human` tipi, bu iki trait’i de uygular ve ayrıca kendi `fly` metoduna sahiptir:

**Filename: src/main.rs**

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

👉 Burada üç farklı `fly` implementasyonu vardır.

`Listing 20-18`’de, `Human` örneği üzerinde doğrudan `fly` çağırıldığında, Rust varsayılan olarak tipin kendi metodunu çağırır:

```rust
fn main() {
    let person = Human;
    person.fly();
}
```

Çıktı:

```
*waving arms furiously*
```

Trait’lerdeki `fly` metodlarını çağırmak için ise trait adını açıkça belirtmek gerekir (`Listing 20-19`):

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

Çıktı:

```
This is your captain speaking.
Up!
*waving arms furiously*
```

👉 Trait adını metodun önünde belirterek hangi implementasyonu istediğimizi açıkça ifade etmiş oluruz.
Ayrıca `Human::fly(&person)` yazmak da `person.fly()` ile aynı anlama gelir.

---

## 🐶 Self Parametresi Olmayan Fonksiyonlarda Çakışma

Eğer trait’teki fonksiyonun `self` parametresi yoksa, yani ilişkili fonksiyon (associated function) ise, Rust hangi implementasyonu kastettiğinizi **tek başına anlayamaz**.

Örneğin (`Listing 20-20`):

**Filename: src/main.rs**

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

Çıktı:

```
A baby dog is called a Spot
```

👉 Burada `Dog::baby_name()` çağrısı doğrudan `Dog` tipi üzerinde tanımlı fonksiyonu çağırmıştır. Biz ise `Animal` trait’inin implementasyonunu çağırmak istiyoruz.

Eğer sadece `Animal::baby_name()` çağırmaya çalışırsak (`Listing 20-21`), Rust hata verir:

```rust
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```

Hata:

```
error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
```

---

## ✅ Çözüm: Fully Qualified Syntax

Rust’a hangi implementasyonu kastettiğimizi söylemek için **fully qualified syntax** kullanırız (`Listing 20-22`):

```rust
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

Çıktı:

```
A baby dog is called a puppy
```

👉 Burada `<Dog as Animal>::baby_name()` ifadesiyle Rust’a, `Dog` tipini `Animal` olarak ele alıp o implementasyonu çağırmak istediğimizi söylüyoruz.

---

## 📐 Fully Qualified Syntax Genel Biçimi

Genel sözdizimi şöyledir:

```
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

* Eğer fonksiyon bir **metot** ise, ilk argüman `self` olur.
* Eğer fonksiyon bir **ilişkili fonksiyon** ise (`self` parametresi yoksa), yalnızca diğer argümanlar yazılır.

Fully qualified syntax her yerde kullanılabilir, ancak genellikle gerekmez. Rust, çoğu durumda hangi implementasyonu kastettiğinizi tahmin edebilir. Yalnızca **aynı isimli birden fazla implementasyon olduğunda** bu ayrımı yapmanız gerekir.

## 🧭 Supertrait’leri Kullanmak (using supertraits)

Bazen bir trait tanımının başka bir trait’e bağlı olmasını isteyebilirsiniz: yani bir tip ilk trait’i uygulayabilmek için ikinci trait’i de uygulamak zorunda olmalıdır. Bunu yapmamızın nedeni, ilk trait’in ikinci trait’in sağladığı ilişkili öğeleri (associated items) kullanabilmesidir. Bu durumda, ilk trait’in dayandığı trait’e **supertrait** denir.

Örneğin, `OutlinePrint` adında ve `outline_print` metoduna sahip bir trait yazmak isteyelim. Bu metot, verilen değeri yıldız işaretleriyle çerçevelenmiş şekilde yazdıracaktır.

Eğer `Point` struct’ı standart kütüphanedeki `Display` trait’ini uygularsa, `(x, y)` formatında yazılacaktır. Diyelim ki `x=1`, `y=3` olan bir `Point` için `outline_print` çağırdığımızda şu çıktıyı almak istiyoruz:

```
**********
*        *
* (1, 3) *
*        *
**********
```

`outline_print` metodunun uygulanmasında, `Display` trait’inin işlevselliğini kullanmak isteriz. Bu nedenle, `OutlinePrint` yalnızca `Display` trait’ini de uygulayan tipler için geçerli olmalıdır. Bunu trait tanımında `OutlinePrint: Display` şeklinde belirtebiliriz. Bu teknik, bir trait’e trait bound eklemeye benzer.

**Listing 20-23** `OutlinePrint` trait’inin implementasyonunu göstermektedir:

**Filename: src/main.rs**

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

👉 Burada `OutlinePrint`, yalnızca `Display` uygulayan tiplerde geçerlidir. Böylece `to_string` metodunu güvenle kullanabiliriz.

Eğer `Display` olmadan deneseydik, derleyici `to_string` metodunu bulamadığına dair hata verirdi.

---

## 🚫 Display Uygulanmamış Tipte Kullanım

Eğer `Point` struct’ı `Display` uygulamıyorsa, `OutlinePrint` uygulaması derlenmez:

```rust
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

Hata mesajı:

```
error[E0277]: `Point` doesn't implement `std::fmt::Display`
```

Bunu düzeltmek için `Point`’e `Display` implementasyonu ekleriz:

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

Artık `OutlinePrint` trait’i `Point` üzerinde uygulanabilir ve `outline_print` çağrıldığında yıldızlı çerçeveyle birlikte yazdırılacaktır.

---

## 📦 Newtype Deseni ile Harici Trait’leri Harici Tiplere Uygulamak (using the newtype pattern to implement external traits on external types)

10. Bölümde “Bir Trait’i Bir Tipte Uygulamak” (Implementing a Trait on a Type) kısmında **orphan rule**’dan bahsetmiştik. Bu kurala göre bir trait’i bir tip üzerinde yalnızca şu koşullarda uygulayabiliriz:

* Trait veya tip (veya ikisi) crate’imizde tanımlı olmalı.

Harici bir trait’i harici bir tipe doğrudan uygulamak mümkün değildir. Bunun etrafından dolaşmak için **newtype deseni** kullanılır.

Newtype deseni, bir tuple struct oluşturarak var olan bir tipi sarmalamaktır. Struct yalnızca tek bir alan içerir ve sarmaladığı tipin etrafında ince bir kaplama görevi görür. Bu yeni tip crate’imize aittir, bu yüzden üzerinde istediğimiz trait’i uygulayabiliriz.

Bu desenin adı Haskell dilinden gelir. Çalışma zamanında hiçbir performans kaybı yoktur; sarmalayıcı tip derleme sırasında yok sayılır.

---

## ✨ Örnek: Vec<T> Üzerinde Display Uygulamak

Diyelim ki `Vec<T>` üzerinde `Display` trait’ini uygulamak istiyoruz. Ancak hem `Vec<T>` hem `Display` harici tanımlandığından orphan rule buna izin vermez. Bunun yerine `Wrapper` adında bir struct tanımlarız ve `Vec<String>` değerini sarmalarız. Ardından `Display`’i `Wrapper` üzerinde uygularız.

**Listing 20-24** bunu göstermektedir:

**Filename: src/main.rs**

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
```

👉 `Wrapper`, `Vec<String>` etrafında bir tuple struct’tır. İçteki `Vec<T>`’ye `self.0` üzerinden erişiyoruz.

Böylece `Display`, `Wrapper` üzerinden kullanılabilir.

---

## ⚠️ Newtype Deseninin Dezavantajı

`Wrapper` yeni bir tip olduğundan, içteki `Vec<T>`’nin metotlarına sahip değildir. Eğer aynı davranışı istiyorsak:

* Tüm `Vec<T>` metotlarını `Wrapper` üzerinde tek tek uygulayıp `self.0`’a yönlendirmemiz gerekir.
* Veya `Deref` trait’ini `Wrapper` için uygulayarak `Vec<T>`’ye otomatik yönlendirme sağlayabiliriz (15. Bölümde ele alındı).

Ancak eğer iç tipin tüm işlevlerini sunmak istemiyorsak (örneğin davranışı kısıtlamak için), yalnızca istediğimiz metotları manuel uygulayabiliriz.

---

## 🔍 Özet

* **Supertrait**: Bir trait’in başka bir trait’e bağlı olmasını sağlar.
* **Newtype deseni**: Harici trait’leri harici tiplere uygulamak veya tip davranışını kısıtlayarak sarmalamak için kullanılır.
* Performans kaybı yoktur, ancak metodları yeniden tanımlama veya `Deref` ile yönlendirme gerektirebilir.
