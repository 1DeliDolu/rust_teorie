## 🏗️ Yapıları Tanımlama ve Örneklendirme (Defining and Instantiating Structs)

Yapılar (structs), “Demet Türü (The Tuple Type)” bölümünde tartışılan demetlere (tuples) benzer, çünkü her ikisi de birden fazla ilgili değeri tutar. Demetlerde olduğu gibi, bir yapının parçaları farklı türlerde olabilir. Ancak demetlerden farklı olarak, bir yapıda her veri parçasına isim verirsiniz; böylece değerlerin ne anlama geldiği açık olur. Bu isimlerin eklenmesi yapıları demetlerden daha esnek hale getirir: bir örneğin değerlerini belirtmek veya erişmek için verilere sırayla güvenmek zorunda değilsiniz.

Bir yapı tanımlamak için `struct` anahtar sözcüğünü yazar ve tüm yapıya bir ad veririz. Bir yapının adı, bir araya getirilen veri parçalarının önemini açıklamalıdır. Ardından süslü parantezler içinde, alanlar (fields) olarak adlandırdığımız veri parçalarının adlarını ve türlerini tanımlarız. Örneğin, Listeleme 5-1 bir kullanıcı hesabı hakkında bilgi depolayan bir yapı göstermektedir.

Filename: src/main.rs

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Listeleme 5-1: Bir `User` yapısı tanımı

Bir yapıyı tanımladıktan sonra kullanmak için, her alan için somut değerler belirterek bu yapının bir örneğini oluştururuz. Bir örnek oluştururken yapının adını yazar, ardından süslü parantezler içinde `anahtar: değer` çiftleri ekleriz. Buradaki anahtarlar alanların adları, değerler ise bu alanlarda depolamak istediğimiz verilerdir. Alanları, yapıda tanımlandıkları sırayla belirtmek zorunda değiliz. Başka bir deyişle, yapı tanımı tür için genel bir şablon gibidir ve örnekler bu şablonu belirli verilerle doldurarak o türde değerler oluşturur. Örneğin, belirli bir kullanıcıyı Listeleme 5-2’de gösterildiği gibi tanımlayabiliriz.

Filename: src/main.rs

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

Listeleme 5-2: `User` yapısının bir örneğini oluşturma

Bir yapıdan belirli bir değeri almak için nokta gösterimini (dot notation) kullanırız. Örneğin, bu kullanıcının e-posta adresine erişmek için `user1.email` yazarız. Eğer örnek değiştirilebilir (mutable) ise, nokta gösterimini kullanarak belirli bir alana yeni bir değer atayabiliriz. Listeleme 5-3, değiştirilebilir bir `User` örneğinin `email` alanındaki değerin nasıl değiştirileceğini göstermektedir.

Filename: src/main.rs

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

Listeleme 5-3: Bir `User` örneğinde `email` alanındaki değeri değiştirme

Dikkat edin ki tüm örnek değiştirilebilir (mutable) olmalıdır; Rust yalnızca belirli alanları değiştirilebilir olarak işaretlemeye izin vermez. Herhangi bir ifadede olduğu gibi, fonksiyon gövdesindeki son ifade olarak yeni bir yapı örneği oluşturup, bu örneği örtük (implicit) olarak döndürebiliriz.

Listeleme 5-4, verilen `email` ve `username` değerleriyle bir `User` örneği döndüren bir `build_user` fonksiyonunu göstermektedir. `active` alanı `true` değerini, `sign_in_count` ise `1` değerini alır.

Filename: src/main.rs

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

Listeleme 5-4: `email` ve `username` alıp bir `User` örneği döndüren `build_user` fonksiyonu

Fonksiyon parametrelerini yapı alanlarıyla aynı adla adlandırmak mantıklıdır, ancak `email` ve `username` alan adlarını ve değişkenlerini tekrar etmek biraz zahmetli olabilir. Yapının daha fazla alanı olsaydı, her adı tekrar etmek daha da yorucu olurdu. Neyse ki, bunun için kullanışlı bir kısayol vardır!


## 🏗️ Alan Başlatma Kısayolunu Kullanma (Using the Field Init Shorthand)

Listeleme 5-4’te parametre adları ile yapı alan adları tamamen aynı olduğundan, `field init shorthand` sözdizimini kullanarak `build_user` fonksiyonunu tekrar yazabiliriz. Böylece aynı şekilde çalışır, ancak `username` ve `email` ifadelerinin tekrarı ortadan kalkar. Bu durum Listeleme 5-5’te gösterilmektedir.

Filename: src/main.rs

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

Listeleme 5-5: `username` ve `email` parametreleri yapı alanlarıyla aynı ada sahip olduğundan `field init shorthand` kullanan bir `build_user` fonksiyonu

Burada, `User` yapısının `email` adlı bir alanına sahip yeni bir örnek oluşturuyoruz. Bu alanın değerini, `build_user` fonksiyonunun `email` parametresindeki değere ayarlamak istiyoruz. `email` alanı ile `email` parametresi aynı ada sahip olduğundan, `email: email` yazmak yerine sadece `email` yazmamız yeterlidir.


## 🏗️ Yapı Güncelleme Sözdizimi ile Diğer Örneklerden Yeni Örnekler Oluşturma (Creating Instances from Other Instances with Struct Update Syntax)

Aynı türden başka bir örnekteki değerlerin çoğunu alıp yalnızca bazılarını değiştiren yeni bir yapı örneği oluşturmak genellikle faydalıdır. Bunu `struct update syntax` kullanarak yapabilirsiniz.

Öncelikle, Listeleme 5-6’da `user2` için yeni bir `User` örneğini güncelleme sözdizimi olmadan nasıl oluşturacağımızı gösteriyoruz. Burada yalnızca `email` için yeni bir değer atıyoruz, diğer tüm değerler için ise Listeleme 5-2’de oluşturduğumuz `user1`’den alıyoruz.

Filename: src/main.rs

```rust
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

Listeleme 5-6: `user1`’in tüm değerlerini (bir tanesi hariç) kullanarak yeni bir `User` örneği oluşturma

`struct update syntax` kullanarak aynı işlemi daha az kodla gerçekleştirebiliriz. Listeleme 5-7’de gösterildiği gibi, `..` sözdizimi açıkça belirtilmeyen alanların verilen örnekteki alanlarla aynı değere sahip olmasını sağlar.

Filename: src/main.rs

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

Listeleme 5-7: `User` örneği için yeni bir `email` değeri atarken, geri kalan değerleri `user1`’den almak için `struct update syntax` kullanma

Listeleme 5-7’deki kod da `user2` için `email` alanına farklı bir değer verir, ancak `username`, `active` ve `sign_in_count` alanlarını `user1` ile aynı tutar. `..user1` ifadesi, kalan tüm alanların değerlerinin `user1`’deki karşılık gelen alanlardan alınması gerektiğini belirtmek için en sonda yer almalıdır. Bunun dışında, yapının tanımındaki alanların sırasından bağımsız olarak istediğimiz kadar alanı, istediğimiz sırayla açıkça belirtebiliriz.

Dikkat edilmesi gereken nokta, `struct update syntax`’ın bir atama (`=`) gibi çalışmasıdır; bunun nedeni, verilerin taşınmasıdır (move). “Değişkenler ve Taşıma ile Veri Etkileşimi (Variables and Data Interacting with Move)” bölümünde gördüğümüz gibi. Bu örnekte, `user2` oluşturulduktan sonra `user1` artık kullanılamaz, çünkü `user1`’in `username` alanındaki `String`, `user2`’ye taşınmıştır. Eğer `user2` için hem `email` hem de `username` alanlarına yeni `String` değerleri verseydik ve sadece `active` ile `sign_in_count` değerlerini `user1`’den kullansaydık, `user1` hala geçerli olurdu. Çünkü hem `active` hem de `sign_in_count`, `Copy` özniteliğini (trait) uygulayan türlerdir ve bu durumda “Yalnızca Yığında Veri: Copy (Stack-Only Data: Copy)” bölümünde tartıştığımız davranış geçerli olurdu. Ayrıca bu örnekte `user1.email` hâlâ kullanılabilir, çünkü bu değer `user1`’den taşınmamıştır.
## 🏗️ Alanları Adlandırılmamış Demet Yapılarını Kullanarak Farklı Türler Oluşturma (Using Tuple Structs Without Named Fields to Create Different Types)

Rust, demetlere (tuples) benzeyen ancak alanları adlandırılmamış yapıları da destekler; bunlara **demet yapıları (tuple structs)** denir. Demet yapıları, yapının adı sayesinde ek bir anlam kazanır, ancak alanlarla ilişkili adları yoktur; yalnızca alanların türleri vardır. Demet yapıları, tüm demete bir ad vermek, onu diğer demetlerden farklı bir tür yapmak istediğinizde ve her alanı normal bir yapıda olduğu gibi adlandırmanın gereksiz veya uzun olacağı durumlarda faydalıdır.

Bir demet yapısı tanımlamak için `struct` anahtar sözcüğü, ardından yapı adı ve parantez içinde alan türleri yazılır. Örneğin, burada `Color` ve `Point` adlı iki demet yapısını tanımlıyor ve kullanıyoruz:

Filename: src/main.rs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Burada `black` ve `origin` değerlerinin farklı türler olduğuna dikkat edin; çünkü bunlar farklı demet yapılarının örnekleridir. Tanımladığınız her yapı kendi türüdür, alanlar aynı türden bile olsa. Örneğin, `Color` türünde bir parametre alan bir fonksiyon, argüman olarak `Point` kabul edemez; her iki tür de üç `i32` değerinden oluşmasına rağmen.

Bunun dışında, demet yapıları, tıpkı demetlerde olduğu gibi, içlerindeki parçalara ayrıştırılabilir (destructure) ve belirli bir değere erişmek için `.` ardından indeks kullanılabilir. Ancak demetlerden farklı olarak, demet yapılarını ayrıştırırken yapının türünü belirtmek gerekir. Örneğin, `origin` noktasındaki değerleri `x`, `y` ve `z` adında değişkenlere ayırmak için şu şekilde yazmalıyız:

```rust
let Point(x, y, z) = origin;
```
## 🏗️ Alanı Olmayan Birim Benzeri Yapılar (Unit-Like Structs Without Any Fields)

Alanları olmayan yapılar da tanımlayabilirsiniz! Bunlara **birim benzeri yapılar (unit-like structs)** denir, çünkü “Demet Türü (The Tuple Type)” bölümünde bahsettiğimiz `()` birim türüne benzer şekilde davranırlar. Birim benzeri yapılar, bir türün üzerinde bir öznitelik (trait) uygulamanız gerektiğinde, ancak o türün içinde saklamak istediğiniz herhangi bir veriniz olmadığında faydalıdır. Öznitelikleri (traits) 10. bölümde tartışacağız. İşte `AlwaysEqual` adlı bir birim yapının tanımlanmasına ve örneklendirilmesine dair bir örnek:

Filename: src/main.rs

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

`AlwaysEqual` tanımlamak için `struct` anahtar sözcüğünü, istediğimiz adı ve ardından bir noktalı virgül kullanırız. Süslü parantez veya parantez gerekmez! Daha sonra aynı şekilde, tanımladığımız adı kullanarak `AlwaysEqual`’in bir örneğini `subject` değişkeninde oluşturabiliriz; yine süslü parantez veya parantez olmadan.

Hayal edin ki ileride bu tür için bir davranış uygularız; öyle ki her `AlwaysEqual` örneği, diğer herhangi bir türün her örneğiyle daima eşit olsun. Bu, test amaçları için bilinen bir sonuç elde etmek üzere yapılabilir. Bu davranışı uygulamak için herhangi bir veriye ihtiyacımız olmaz! 10. bölümde, özniteliklerin (traits) nasıl tanımlandığını ve bunların, birim benzeri yapılar dahil, herhangi bir tür üzerinde nasıl uygulanabileceğini göreceksiniz.
## 🏗️ Yapı Verilerinin Sahipliği (Ownership of Struct Data)

Listeleme 5-1’deki `User` yapı tanımında, `&str` dize dilimi (string slice) türü yerine sahip olunan `String` türünü kullandık. Bu bilinçli bir tercihtir, çünkü her yapı örneğinin tüm verilerinin sahibi olmasını ve bu verilerin, tüm yapı geçerli olduğu sürece geçerli olmasını isteriz.

Yapıların, başka bir şeye ait olan verilerin referanslarını saklaması da mümkündür; ancak bunu yapmak ömürler (lifetimes) kullanmayı gerektirir. Ömürler, bir yapının referans verdiği verilerin, yapının kendisi geçerli olduğu sürece geçerli olmasını garanti eden bir Rust özelliğidir. Diyelim ki ömürleri belirtmeden bir yapıda referans saklamaya çalıştınız, aşağıdaki gibi; bu çalışmaz:

Filename: src/main.rs
This code does not compile!

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```

Derleyici, ömür belirleyicilerinin (lifetime specifiers) eksik olduğundan şikayet edecektir:

```
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs` (bin "structs") due to 2 previous errors
```

10. bölümde, bu hataları nasıl düzelteceğimizi ve yapılar içinde referansları nasıl saklayabileceğimizi tartışacağız. Ancak şimdilik bu tür hataları, `&str` gibi referanslar yerine `String` gibi sahip olunan türler kullanarak düzelteceğiz.
