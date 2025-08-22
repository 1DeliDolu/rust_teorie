## 📦 Genel Veri Türleri (generic data types)

Genel türleri (generics), birçok farklı somut veri türü (concrete data types) ile kullanabileceğimiz fonksiyon imzaları (function signatures) veya `struct` gibi öğeler için tanımlar oluşturmak amacıyla kullanırız. Önce genel türler kullanarak fonksiyonları (functions), `struct`’ları, `enum`’ları ve yöntemleri (methods) nasıl tanımlayacağımıza bakalım. Ardından genel türlerin kod performansını (performance) nasıl etkilediğini tartışacağız.

## 🧩 Fonksiyon Tanımlarında (in function definitions)

Genel türler kullanan bir fonksiyon tanımlarken, normalde parametrelerin ve dönüş değerinin veri türlerini belirttiğimiz yere, fonksiyonun imzasına (function signature) genel türleri yerleştiririz. Bunu yapmak, kodumuzu daha esnek hale getirir ve fonksiyonumuzun çağıranlarına daha fazla işlev sunarken kod yinelenmesini (duplication) önler.

`largest` fonksiyonumuzla devam ederek, Liste 10-4’te bir dilimde (slice) en büyük değeri bulan iki fonksiyon gösterilmektedir. Daha sonra bunları genel türler kullanan tek bir fonksiyonda birleştireceğiz.

**Filename: src/main.rs**

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}
```

Listing 10-4: Yalnızca adları ve imzalarındaki türleri farklı olan iki fonksiyon

`largest_i32` fonksiyonu, bir dilimdeki (slice) en büyük `i32` değerini bulan ve Liste 10-3’te ayıkladığımız fonksiyondur. `largest_char` fonksiyonu, bir dilimdeki en büyük `char` değerini bulur. Fonksiyon gövdeleri aynı koda sahiptir; o hâlde tek bir fonksiyonda genel tür parametresi (generic type parameter) tanıtarak bu yinelenmeyi ortadan kaldıralım.

Yeni tekil bir fonksiyonda türleri parametreleştirmek (parameterize) için, tıpkı bir fonksiyonun değer parametrelerinde yaptığımız gibi, tür parametresine (type parameter) bir ad vermemiz gerekir. Tür parametresi adı olarak herhangi bir tanımlayıcıyı kullanabilirsiniz. Ancak biz `T` kullanacağız; çünkü Rust’ta tür parametresi adları gelenek gereği kısadır, sıklıkla tek harften oluşur ve Rust’ın tür adlandırma geleneği `CamelCase`’tir. “type”ın kısaltması olan `T`, çoğu Rust programcısının varsayılan tercihidir.

Bir parametreyi fonksiyon gövdesinde kullanırken, derleyicinin bu adın ne anlama geldiğini bilmesi için parametre adını imzada bildirmemiz gerekir. Benzer şekilde, bir fonksiyon imzasında tür parametresi adını kullandığımızda, onu kullanmadan önce tür parametresi adını bildirmeliyiz. Genel `largest` fonksiyonunu tanımlamak için, tür adı bildirimlerini fonksiyon adı ile parametre listesi arasındaki sivri ayraçların (angle brackets) içine, `<>`, şu şekilde yerleştiririz:

```
fn largest<T>(list: &[T]) -> &T {
```

Bu tanımı şöyle okuruz: `largest` fonksiyonu, bir `T` türü (type) üzerinde geneldir (generic over). Bu fonksiyonun `list` adında bir parametresi vardır ve bu, `T` türünden değerlerin bir dilimidir (slice). `largest` fonksiyonu, aynı `T` türünden bir değere başvuru (reference) döndürecektir.

Liste 10-5, imzasında genel veri türü (generic data type) kullanan birleştirilmiş `largest` fonksiyon tanımını gösterir. Liste ayrıca fonksiyonun hem `i32` değerlerinden hem de `char` değerlerinden oluşan bir dilimle nasıl çağrılabileceğini de gösterir. Dikkat: bu kod henüz derlenmeyecektir (compile).

**Filename: src/main.rs**
This code does not compile!

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
```

Listing 10-5: Genel tür parametreleri (generic type parameters) kullanan `largest` fonksiyonu; bu hâliyle henüz derlenmez

Bu kodu hemen derlersek, şu hatayı alırız:

```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T` with trait `PartialOrd`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

Yardım metni `std::cmp::PartialOrd`’dan bahsediyor; bu bir **özelliktir (trait)** ve özellikleri bir sonraki bölümde ele alacağız. Şimdilik şunu bilmek yeterli: Bu hata, `largest` fonksiyonunun gövdesinin `T`’nin alabileceği tüm olası türler için çalışmayacağını söyler. Fonksiyon gövdesinde `T` türündeki değerleri karşılaştırmak istediğimizden, yalnızca değerleri sıralanabilen (ordered) türleri kullanabiliriz. Karşılaştırmaları etkinleştirmek için standart kitaplıkta (standard library) türler üzerinde uygulanabilen `std::cmp::PartialOrd` özelliği vardır (bu özellikle ilgili daha fazlası için Ek C’ye bakın). Liste 10-5’i düzeltmek için yardım metninin önerisini izleyip, `T` için geçerli türleri yalnızca `PartialOrd`’u uygulayanlarla sınırlandırabiliriz (restrict). Bu durumda liste derlenecektir; çünkü standart kitaplık hem `i32` hem de `char` üzerinde `PartialOrd`’u uygular.

## 🏗️ Struct Tanımlarında (in struct definitions)

Bir `struct`’ı, alanlarının (fields) bir veya daha fazlasında genel tür parametresi (generic type parameter) kullanacak şekilde `<>` sözdizimiyle tanımlayabiliriz. Liste 10-6, herhangi bir türden `x` ve `y` koordinat değerlerini tutan bir `Point<T>` `struct`’ını tanımlar.

**Filename: src/main.rs**

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

Listing 10-6: `T` türünden `x` ve `y` değerleri tutan bir `Point<T>` struct’ı

Genel türleri `struct` tanımlarında kullanma sözdizimi, fonksiyon tanımlarında kullanılanla benzerdir. Önce tür parametresinin adını `struct` adının hemen ardından sivri ayraçlar (angle brackets) içinde bildiririz. Sonra `struct` tanımında somut türler yerine genel türü kullanırız.

Dikkat edin: `Point<T>` tanımında yalnızca bir genel tür kullandığımız için bu tanım, `Point<T>` `struct`’ının bir `T` türü üzerinde genel olduğunu söyler ve hem `x` hem de `y` alanları aynı türdedir, bu tür her ne olursa olsun. Eğer Liste 10-7’de olduğu gibi farklı türlerden değerler içeren bir `Point<T>` örneği oluşturursak, kodumuz derlenmez.

**Filename: src/main.rs**
This code does not compile!

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

Listing 10-7: `x` ve `y` alanları aynı genel tür `T`’yi kullandığı için aynı türde olmalıdır

Bu örnekte, `x`’e tamsayı değeri 5 atadığımızda, derleyiciye bu `Point<T>` örneği için `T`’nin bir tamsayı olacağını bildirmiş oluruz. Sonra `y` için 4.0 belirttiğimizde —ki onu `x` ile aynı türde tanımladık— şu tür uyuşmazlığı (type mismatch) hatasını alırız:

```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

Hem `x` hem de `y` genel türlerden olsun ama farklı türler olabilsin diye, birden fazla genel tür parametresi kullanabiliriz. Örneğin, Liste 10-8’de `Point` tanımını `T` ve `U` türleri üzerinde genel olacak şekilde değiştiriyoruz; burada `x`, `T` türünden ve `y`, `U` türündendir.

**Filename: src/main.rs**

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

Listing 10-8: `x` ve `y` alanlarının farklı türlerde olmasına izin veren, iki tür üzerinde genel olan (`T`, `U`) bir `Point<T, U>` struct’ı

Artık gösterilen tüm `Point` örnekleri geçerlidir! Bir tanımda istediğiniz kadar genel tür parametresi kullanabilirsiniz; ancak birkaçı fazlası, kodunuzu okunması zor hâle getirir. Kodunuzda çok fazla genel tür kullanmanız gerekiyorsa, bu, kodunuzun daha küçük parçalara yeniden yapılandırılması (restructuring) gerektiğinin bir işareti olabilir.

## 🎭 Enum Tanımlarında (in enum definitions)

`struct`’larda yaptığımız gibi, `enum`’ları da varyantlarında (variants) genel veri türlerini (generic data types) tutacak şekilde tanımlayabiliriz. Standart kitaplığın (standard library) sağladığı ve Bölüm 6’da kullandığımız `Option<T>` enum’una tekrar bakalım:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Bu tanım artık sizin için daha anlamlı olmalı. Gördüğünüz gibi, `Option<T>` enum’u `T` türü üzerinde geneldir (generic over type T) ve iki varyanta sahiptir: `Some`, `T` türünden bir değer tutar; `None` ise hiç değer tutmaz. `Option<T>` enum’unu kullanarak, isteğe bağlı bir değer (optional value) soyut kavramını ifade edebiliriz ve `Option<T>` genel olduğu için, bu soyutlamayı değer türü ne olursa olsun kullanabiliriz.

Enum’lar birden fazla genel tür de kullanabilir. Bölüm 9’da kullandığımız `Result` enum’unun tanımı buna bir örnektir:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` enum’u iki tür (`T` ve `E`) üzerinde geneldir ve iki varyanta sahiptir: `Ok`, `T` türünden bir değer tutar; `Err`, `E` türünden bir değer tutar. Bu tanım, `Result` enum’unu bir işlemin başarılı (bir `T` türünden değer döndürür) veya hatalı (bir `E` türünden hata döndürür) olabileceği her yerde kullanışlı kılar. Aslında bu yapıyı, Liste 9-3’te bir dosya açarken kullandık; dosya başarıyla açıldığında `T`, `std::fs::File` türüyle dolduruldu, açma sırasında hata oluştuğunda ise `E`, `std::io::Error` türüyle dolduruldu.

Kodunuzda yalnızca tuttukları değer türleri açısından farklılık gösteren birden fazla `struct` veya `enum` tanımı fark ettiğinizde, yinelenmeyi önlemek için genel türleri kullanabilirsiniz.

---

## 🛠️ Yöntem Tanımlarında (in method definitions)

Bölüm 5’te yaptığımız gibi, `struct` ve `enum`’lar üzerinde yöntemler (methods) tanımlayabiliriz ve bu tanımlarda da genel türler kullanabiliriz. Liste 10-9, Liste 10-6’da tanımladığımız `Point<T>` struct’ını, `x` adlı bir yöntem ile göstermektedir.

**Filename: src/main.rs**

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

Listing 10-9: `Point<T>` struct’ı üzerinde `x` alanına başvuru döndüren `x` adlı yöntemi uygulamak

Burada, `Point<T>` üzerinde `x` adlı bir yöntem tanımladık; bu yöntem, `x` alanındaki veriye bir başvuru döndürür.

Dikkat edin: `impl`’den hemen sonra `T`’yi bildirmemiz gerekir; böylece `T`’yi `Point<T>` türü üzerinde yöntemler tanımladığımızı belirtmek için kullanabiliriz. `impl`’den sonra `T`’yi genel tür olarak bildirerek, Rust `Point` içindeki sivri ayraçlardaki türün somut değil, genel olduğunu anlar. Bu genel parametreye farklı bir ad seçebilirdik; ancak `struct` tanımında bildirilenle aynı adı kullanmak gelenekseldir. Eğer bir `impl` bloğunda genel tür bildiren bir yöntem yazarsanız, bu yöntem o türün hangi somut türle kullanıldığına bakılmaksızın tanımlanır.

Bir tür üzerinde yöntemler tanımlarken, genel türlere kısıtlamalar (constraints) da koyabiliriz. Örneğin, yöntemleri yalnızca `Point<f32>` örnekleri üzerinde tanımlayabiliriz; herhangi bir `Point<T>` üzerinde değil. Liste 10-10’da, somut tür `f32` kullanıyoruz, yani `impl`’den sonra tür bildirmiyoruz.

**Filename: src/main.rs**

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Listing 10-10: Genel tür parametresi `T` yerine yalnızca belirli bir somut tür (`f32`) için geçerli olan bir `impl` bloğu

Bu kod, `Point<f32>` türünün bir `distance_from_origin` yöntemine sahip olacağı anlamına gelir; `T`’si `f32` olmayan diğer `Point<T>` örneklerinde bu yöntem tanımlanmaz. Bu yöntem, noktanın `(0.0, 0.0)` koordinatındaki noktadan ne kadar uzak olduğunu ölçer ve yalnızca kayan nokta (floating-point) türleri için mevcut matematiksel işlemleri kullanır.

Bir `struct` tanımındaki genel tür parametreleri, o `struct`’ın yöntem imzalarında kullandıklarınızla her zaman aynı olmak zorunda değildir. Liste 10-11’de, örneği daha net kılmak için `Point` `struct`’ı için `X1` ve `Y1` genel türleri ve `mixup` yöntem imzası için `X2` ve `Y2` genel türleri kullanılmıştır. Bu yöntem, `self` `Point`’inden `x` değerini (`X1` türünde) ve geçirilen `Point`’ten `y` değerini (`Y2` türünde) alarak yeni bir `Point` örneği oluşturur.

**Filename: src/main.rs**

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

Listing 10-11: Struct tanımındakinden farklı genel türler kullanan bir yöntem

`main` içinde, `x` değeri `i32` (5) ve `y` değeri `f64` (10.4) olan bir `Point` tanımladık. `p2` değişkeni ise, `x` değeri `"Hello"` (string slice) ve `y` değeri `'c'` (char) olan bir `Point`. `p1` üzerinde `mixup` yöntemini çağırıp `p2`’yi argüman olarak verdiğimizde `p3` değişkenini elde ederiz; bu, `x` değeri `p1`’den geldiği için `i32` türünde olur. `p3`’ün `y` değeri ise `p2`’den geldiği için `char` türünde olur. `println!` makrosu şu çıktıyı verir:
`p3.x = 5, p3.y = c`

Bu örneğin amacı, bazı genel parametrelerin `impl` ile, bazılarının ise yöntem tanımıyla bildirildiği bir durumu göstermektir. Burada `X1` ve `Y1` genel parametreleri `impl`’den sonra bildirilmiştir çünkü `struct` tanımıyla ilişkilidir. `X2` ve `Y2` ise yalnızca metoda özgü oldukları için `fn mixup`’tan sonra bildirilmiştir.


## ⚡ Genel Türler Kullanan Kodun Performansı (performance of code using generics)

Genel tür parametrelerini (generic type parameters) kullanmanın çalışma zamanında (runtime) bir maliyeti olup olmadığını merak edebilirsiniz. İyi haber şu ki, genel türler kullanmak, programınızın somut türlerle (concrete types) çalıştırılmasına göre daha yavaş olmasına neden olmaz.

Rust bunu, derleme (compile) zamanında genel türleri kullanan kodun **tekbiçimleştirilmesi (monomorphization)** yoluyla başarır. Tekbiçimleştirme, genel kodu (generic code) kullanılan somut türleri doldurarak özel (specific) koda dönüştürme işlemidir. Bu süreçte derleyici, Liste 10-5’te genel fonksiyon oluşturmak için yaptığımız adımların tersini yapar: derleyici, genel kodun çağrıldığı tüm yerleri inceler ve her çağrıda kullanılan somut türler için özel kod üretir.

Bunun nasıl çalıştığını görmek için standart kitaplığın (standard library) genel `Option<T>` enum’unu kullanalım:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Rust bu kodu derlediğinde, tekbiçimleştirme işlemi yapar. Bu süreçte derleyici, `Option<T>` örneklerinde kullanılan değerlere bakar ve iki tür `Option<T>` belirler: biri `i32`, diğeri `f64`. Böylece `Option<T>`’nin genel tanımını `i32` ve `f64` için özelleştirilmiş iki ayrı tanıma genişletir ve genel tanımı bunlarla değiştirir.

Tekbiçimleştirilmiş kod aşağıdakine benzer (derleyici, burada gösterdiğimizden farklı adlar kullanır; biz sadece örnekliyoruz):

**Filename: src/main.rs**

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Genel `Option<T>`, derleyici tarafından oluşturulan somut tanımlarla değiştirilmiştir. Rust, genel kodu her örnekte türü belirlenmiş koda derlediği için, genel türleri kullanmanın çalışma zamanında hiçbir maliyeti yoktur. Kod çalıştığında, sanki her tanımı elle çoğaltmışız gibi çalışır.

**Tekbiçimleştirme (monomorphization) süreci, Rust’ın genel türlerini çalışma zamanında son derece verimli hale getirir.**
