## 🛠️ Bir Enum Tanımlama (defining an enum)

Struct’lar (yapılar), bir dikdörtgenin (Rectangle) genişlik (width) ve yükseklik (height) alanlarında olduğu gibi, ilgili alanları ve verileri bir araya getirmenin bir yolunu sunarken; enumlar (enumerations), bir değerin olası değerler kümesinden biri olduğunu ifade etmenizi sağlar. Örneğin, bir dikdörtgenin (Rectangle) aynı zamanda bir daireyi (Circle) veya üçgeni (Triangle) de kapsayan olası şekiller kümesinden biri olduğunu söylemek isteyebiliriz. Bunu yapmak için Rust, bu olasılıkları bir enum olarak kodlamamıza izin verir.

Şimdi, kodda ifade etmek isteyebileceğimiz bir duruma bakalım ve enumların neden yararlı ve bu durumda struct’lardan daha uygun olduğunu görelim. Diyelim ki IP adresleriyle çalışmamız gerekiyor. Günümüzde IP adresleri için kullanılan iki ana standart vardır: sürüm 4 (version four) ve sürüm 6 (version six). Bir IP adresi için programımızın karşılaşacağı tek olasılık bunlar olduğu için, tüm olası varyantları numaralandırabiliriz; işte bu nedenle buna numaralandırma (enumeration) adı verilir.

Herhangi bir IP adresi ya sürüm 4 ya da sürüm 6 olabilir, ancak aynı anda ikisi birden olamaz. IP adreslerinin bu özelliği, enum veri yapısını uygun hale getirir çünkü bir enum değeri yalnızca kendi varyantlarından biri olabilir. Hem sürüm 4 hem de sürüm 6 adresleri temelde IP adresleridir, bu nedenle herhangi bir IP adresi türüne uygulanan durumları işlerken aynı tür olarak ele alınmalıdırlar.

Bu kavramı, `IpAddrKind` adında bir numaralandırma (enum) tanımlayarak ve bir IP adresinin olabileceği olası türleri `V4` ve `V6` olarak listeleyerek kodda ifade edebiliriz. Bunlar enumun varyantlarıdır:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

`IpAddrKind` artık kodumuzun başka yerlerinde kullanabileceğimiz özel bir veri türüdür.

## 🧩 Enum Değerleri (enum values)

`IpAddrKind` enumunun her iki varyantından da örnekler şu şekilde oluşturulabilir:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Enumun varyantlarının, enumun tanımlayıcısı altında ad alanına (namespace) sahip olduğunu ve bunları ayırmak için çift iki nokta (`::`) kullandığımızı unutmayın. Bu, yararlıdır çünkü artık `IpAddrKind::V4` ve `IpAddrKind::V6` değerlerinin her ikisi de aynı türdedir: `IpAddrKind`. Örneğin, herhangi bir `IpAddrKind` alabilen bir fonksiyon tanımlayabiliriz:

```rust
fn route(ip_kind: IpAddrKind) {}
```

Ve bu fonksiyonu her iki varyant ile de çağırabiliriz:

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Enum kullanmanın daha fazla avantajı vardır. IP adresi türümüz hakkında daha fazla düşünürsek, şu anda gerçek IP adresi verisini saklamanın bir yoluna sahip değiliz; yalnızca türünü biliyoruz. Bölüm 5’te struct’ları yeni öğrendiğiniz için, bu sorunu aşağıdaki 6-1 numaralı listede gösterildiği gibi struct’larla çözmek isteyebilirsiniz.

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

*Liste 6-1: Bir IP adresinin verisini ve `IpAddrKind` varyantını bir struct kullanarak saklama*

Burada, iki alanı olan bir `IpAddr` struct’ı tanımladık: `kind` alanı `IpAddrKind` türünde (önceden tanımladığımız enum) ve `address` alanı `String` türünde. Bu struct’ın iki örneğine sahibiz: `home`, `IpAddrKind::V4` türünde olup `127.0.0.1` adresine sahiptir; `loopback` ise `IpAddrKind::V6` türünde olup `::1` adresine sahiptir. Böylece `kind` ve `address` değerlerini birlikte paketlemiş olduk.

Ancak aynı kavramı yalnızca bir enum kullanarak ifade etmek daha kısadır: Bir struct içine enum koymak yerine, veriyi doğrudan her enum varyantının içine koyabiliriz:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

Veriyi doğrudan her enum varyantına ekliyoruz, bu yüzden ek bir struct’a gerek yok. Burada enumların nasıl çalıştığına dair başka bir ayrıntıyı da görebiliyoruz: tanımladığımız her enum varyantının adı, enumun bir örneğini oluşturan bir fonksiyon haline gelir. Yani `IpAddr::V4()` bir `String` argümanı alır ve bir `IpAddr` örneği döndürür. Enum tanımı yaptığımızda bu kurucu fonksiyon otomatik olarak tanımlanır.

Enum kullanmanın bir başka avantajı da her varyantın farklı tür ve miktarda veri içerebilmesidir. Sürüm 4 IP adresleri her zaman 0 ile 255 arasında dört sayısal bileşenden oluşur. Eğer `V4` adreslerini dört `u8` değeri olarak saklamak, `V6` adreslerini ise bir `String` değeri olarak ifade etmek istersek, bunu struct ile yapamayız. Enumlar bu durumu kolayca işler:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

Sürüm 4 ve sürüm 6 IP adreslerini saklamak için veri yapıları tanımlamanın birkaç farklı yolunu gösterdik. Ancak IP adreslerini saklamak ve türünü belirtmek o kadar yaygındır ki, standart kütüphane bunun için bir tanım sunar! Standart kütüphanedeki `IpAddr` tanımı, bizim tanımladığımız enumun aynısıdır; ancak adres verisini, her varyant için farklı tanımlanmış iki struct biçiminde gömer:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Bu kod, bir enum varyantının içine herhangi bir türde veri koyabileceğinizi gösterir: örneğin stringler, sayısal türler veya struct’lar. Hatta başka bir enum bile koyabilirsiniz! Ayrıca, standart kütüphanedeki türler, genellikle sizin aklınıza gelebileceklerden çok daha karmaşık değildir.

Dikkat edilmesi gereken bir nokta da şudur: Standart kütüphanede `IpAddr` tanımı olsa da, onu kapsamımıza (scope) getirmediğimiz sürece kendi tanımımızı oluşturabilir ve kullanabiliriz. Türleri kapsamımıza getirme konusunu Bölüm 7’de daha detaylı ele alacağız.

Şimdi 6-2 numaralı listede başka bir enum örneğine bakalım: bu enumun varyantlarının içine çok farklı türler gömülüdür.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

*Liste 6-2: Varyantlarının her biri farklı miktarda ve türde veri saklayan bir `Message` enumu*

Bu enumun dört varyantı vardır:

* `Quit`: Hiçbir veri içermez.
* `Move`: Struct gibi adlandırılmış alanlara sahiptir.
* `Write`: Tek bir `String` içerir.
* `ChangeColor`: Üç `i32` değeri içerir.

Böyle varyantlara sahip bir enum tanımlamak, farklı türde struct’lar tanımlamaya benzer; ancak enum `struct` anahtar sözcüğünü kullanmaz ve tüm varyantlar `Message` türü altında gruplanır. Aşağıdaki struct’lar, yukarıdaki enum varyantlarının tuttuğu veriyi tutabilir:

```rust
struct QuitMessage; // birim struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

Ancak farklı struct’lar kullanırsak, her biri kendi türüne sahip olur ve bunları işleyebilecek tek bir fonksiyonu kolayca tanımlayamayız. Oysa 6-2 numaralı listede tanımlanan `Message` enumu tek bir türdür.

Enumlar ile struct’lar arasındaki başka bir benzerlik de şudur: `impl` kullanarak struct’lara metot tanımlayabildiğimiz gibi, enumlara da metot tanımlayabiliriz. Örneğin, `Message` enumuna tanımlayabileceğimiz `call` adlı bir metot şöyledir:

```rust
impl Message {
    fn call(&self) {
        // metodun gövdesi buraya yazılır
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

Metodun gövdesi, `self` aracılığıyla üzerinde çağrıldığı değere erişir. Bu örnekte `m`, `Message::Write(String::from("hello"))` değerine sahiptir ve `m.call()` çalıştırıldığında `self` bu değeri temsil eder.

Şimdi standart kütüphanede çok yaygın ve kullanışlı olan başka bir enumu inceleyelim: `Option`.


## 🛡️ Option Enum’u ve Null Değerler Üzerindeki Avantajları (the Option enum and its advantages over null values)

Bu bölümde, standart kütüphane tarafından tanımlanmış başka bir enum olan `Option` üzerine bir vaka çalışması inceleyeceğiz. `Option` türü, bir değerin ya bir şey olabileceği ya da hiçbir şey olamayacağı çok yaygın senaryoyu kodlar.

Örneğin, boş olmayan bir listedeki ilk öğeyi isterseniz bir değer elde edersiniz. Ancak boş bir listedeki ilk öğeyi isterseniz hiçbir şey elde etmezsiniz. Bu kavramı tür sistemi (type system) üzerinden ifade etmek, derleyicinin ele almanız gereken tüm durumları gerçekten ele alıp almadığınızı kontrol etmesine olanak tanır; bu işlevsellik, diğer programlama dillerinde son derece yaygın olan hataları önleyebilir.

Programlama dili tasarımı genellikle hangi özelliklerin dahil edildiği açısından düşünülür, ancak hariç tutulan özellikler de önemlidir. Rust, birçok dilde bulunan `null` özelliğine sahip değildir. `Null`, orada bir değer olmadığını ifade eden bir değerdir. `Null` destekleyen dillerde değişkenler her zaman iki durumdan birinde olabilir: `null` ya da `not-null`.

`Null` değerinin mucidi Tony Hoare, 2009’daki *“Null References: The Billion Dollar Mistake”* başlıklı sunumunda şöyle demiştir:

> Bunu milyar dolarlık hatam olarak adlandırıyorum. O dönemde, nesne yönelimli bir dilde referanslar için kapsamlı ilk tür sistemini tasarlıyordum. Amacım, referansların tüm kullanımının tamamen güvenli olmasını ve denetimin otomatik olarak derleyici tarafından yapılmasını sağlamaktı. Ancak, uygulanması çok kolay olduğu için `null` referans ekleme cazibesine karşı koyamadım. Bu, son kırk yılda sayısız hataya, güvenlik açıklarına ve sistem çökmelerine yol açtı; muhtemelen milyar dolarlık acıya ve zarara sebep oldu.

Null değerlerle ilgili sorun şudur: bir null değeri, sanki null olmayan (not-null) bir değer gibi kullanmaya çalışırsanız, bir tür hata alırsınız. Bu null ya da null olmayan özelliği her yerde bulunduğundan, bu tür bir hata yapmak son derece kolaydır.

Ancak null’un ifade etmeye çalıştığı kavram yine de yararlıdır: null, herhangi bir nedenle şu anda geçersiz ya da bulunmayan bir değerdir.

Sorun aslında kavramda değil, belirli uygulama biçimindedir. Bu nedenle Rust’ta null yoktur; fakat bir değerin mevcut ya da yok olma kavramını kodlayabilen bir enum (enum) vardır. Bu enum `Option<T>` (Option) olup standart kütüphane tarafından şu şekilde tanımlanmıştır:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` o kadar kullanışlıdır ki, önyüklemeye (prelude) dâhildir; onu kapsamınıza (scope) açıkça getirmeniz gerekmez. Varyantları (variants) da önyüklemeye dâhildir: `Some` ve `None`’u `Option::` öneki olmadan doğrudan kullanabilirsiniz. `Option<T>` hâlâ sıradan bir enumdur ve `Some(T)` ile `None`, `Option<T>` türünün varyantlarıdır.

`<T>` söz dizimi, henüz ele almadığımız bir Rust özelliğidir. Bu, bir generic tür parametresidir (generic type parameter) ve generics konusunu Bölüm 10’da daha ayrıntılı inceleyeceğiz. Şimdilik bilmeniz gereken, `<T>`’nin `Option` enumunun `Some` varyantının herhangi bir türden tek bir veri tutabileceği anlamına geldiğidir ve `T` yerine kullanılan her somut tür, genel `Option<T>` türünü farklı bir tür yapar. Sayısal türleri ve karakter türlerini tutmak için `Option` değerlerini kullanmaya dair bazı örnekler:

```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

`some_number`’ın türü `Option<i32>`’dir. `some_char`’ın türü ise farklı bir tür olan `Option<char>`’dır. Rust, `Some` varyantının içine bir değer koyduğumuz için bu türleri çıkarımlayabilir (type inference). `absent_number` için ise Rust, genel `Option` türünü belirtmemizi ister: derleyici (compiler), yalnızca bir `None` değerine bakarak ilgili `Some` varyantının hangi türü tutacağını çıkarımlayamaz. Burada, `absent_number`’ın `Option<i32>` türünde olmasını istediğimizi Rust’a bildiriyoruz.

Bir `Some` değeri olduğunda, bir değerin mevcut olduğunu biliriz ve değer `Some` içinde tutulur. `None` değeri olduğunda ise bir bakıma null ile aynı şeyi ifade eder: geçerli bir değerimiz yoktur. Peki `Option<T>`’ye sahip olmak, null’a sahip olmaktan neden daha iyidir?

Kısacası, `Option<T>` ile `T` (burada `T` herhangi bir tür olabilir) farklı türler olduğundan, derleyici `Option<T>` değerini sanki kesinlikle geçerli bir değer varmış gibi kullanmamıza izin vermez. Örneğin, aşağıdaki kod derlenmez; çünkü bir `i8` ile bir `Option<i8>` toplamaya çalışmaktadır:

```rust
// Bu kod derlenmez!
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

Bu kodu çalıştırırsak, aşağıdakine benzer bir hata iletisi alırız:

```
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            `&i8` implements `Add<i8>`
            `&i8` implements `Add`
            `i8` implements `Add<&i8>`
            `i8` implements `Add`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `enums` (bin "enums") due to 1 previous error
```
Yoğun! Özünde, bu hata iletisi Rust’ın bir `i8` ile bir `Option<i8>`’i nasıl toplayacağını bilmediği anlamına gelir; çünkü bunlar farklı türlerdir (type). Rust’ta `i8` gibi bir türe ait bir değere sahip olduğumuzda, derleyici (compiler) her zaman geçerli bir değere sahip olduğumuzu garanti eder. Bu sayede, o değeri kullanmadan önce `null` (null) olup olmadığını kontrol etmek zorunda kalmadan güvenle ilerleyebiliriz. Yalnızca bir `Option<i8>` (veya üzerinde çalıştığımız her ne tür değer ise) olduğunda, bir değere sahip olmama olasılığı konusunda endişelenmemiz gerekir ve derleyici, o değeri kullanmadan önce bu durumu ele aldığımızdan emin olur.

Başka bir deyişle, `Option<T>`’yi `T` türüne dönüştürmeden `T` ile ilgili işlemleri gerçekleştiremezsiniz. Genel olarak bu, `null` ile ilgili en yaygın sorunlardan birini yakalamaya yardımcı olur: bir şeyin `null` olmadığı varsayımına kapılmak, oysa gerçekte `null` olması.

Yanlış şekilde `null` olmayan (not-null) bir değeri varsayma riskini ortadan kaldırmak, kodunuzda daha özgüvenli olmanıza yardımcı olur. `Null` olabilen bir değere sahip olmak için, o değerin türünü açıkça `Option<T>` yaparak bu duruma bilinçli olarak izin vermelisiniz. Ardından, bu değeri kullandığınızda, değerin `null` olduğu durumu açıkça ele almanız gerekir. Türü `Option<T>` olmayan her yerde, değerin `null` olmadığı varsayımını güvenle yapabilirsiniz. Bu, `null`’ın her yere yayılmasını sınırlamak ve Rust kodunun güvenliğini artırmak için Rust’ta kasıtlı bir tasarım kararıdır.

Peki, `Option<T>` türünde bir değere sahip olduğunuzda, içindeki `Some` varyantından (variant) `T` değerini nasıl çıkarıp kullanırsınız? `Option<T>` enumunun (enum) farklı durumlarda faydalı çok sayıda metodu vardır; bunları dokümantasyonunda inceleyebilirsiniz. `Option<T>` üzerindeki metodlara aşina olmak, Rust yolculuğunuzda son derece yararlı olacaktır.

Genel olarak, bir `Option<T>` değerini kullanmak için her varyantı ele alacak koda ihtiyaç duyarsınız. Yalnızca `Some(T)` değeri olduğunda çalışacak ve içteki `T`’yi kullanmasına izin verilen bir kodunuz olmalı. Yalnızca `None` değeri olduğunda çalışacak ve `T` değerinin mevcut olmadığı başka bir kodunuz daha olmalı. `match` ifadesi (match expression), enumlarla birlikte kullanıldığında tam da bunu yapan bir denetim akışı (control flow) yapısıdır: hangi enum varyantına sahip olduğuna bağlı olarak farklı kodları çalıştırır ve eşleşen değerin içindeki veriyi bu koda sağlar.



