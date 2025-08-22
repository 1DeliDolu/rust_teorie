## 📊 Veri Türleri (data types)

Rust’teki her değer belirli bir veri türüne (data type) aittir. Bu, Rust’a hangi tür verinin belirtildiğini ve o veriyle nasıl çalışılacağını söyler. İki veri türü alt kümesine bakacağız: skaler (scalar) ve bileşik (compound).

Rust’un statik olarak tür denetimli (statically typed) bir dil olduğunu unutmayın. Bu, derleme (compile) zamanında tüm değişkenlerin türlerinin bilinmesi gerektiği anlamına gelir. Derleyici genellikle, değere ve onu nasıl kullandığımıza bağlı olarak hangi türü kullanmak istediğimizi çıkarabilir. Ancak birden fazla türün mümkün olduğu durumlarda — örneğin, Bölüm 2’deki “Tahmini Gizli Sayıyla Karşılaştırma” kısmında bir `String` değerini sayısal bir türe dönüştürdüğümüzde — tür açıklaması (type annotation) eklememiz gerekir. Örneğin:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Yukarıdaki kodda gösterilen `: u32` tür açıklamasını eklemezsek, Rust şu hatayı gösterecektir. Bu, derleyicinin hangi türü kullanmak istediğimizi bilmek için bizden daha fazla bilgiye ihtiyaç duyduğu anlamına gelir:

```bash
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `guess` an explicit type
  |
2 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
  |              ++++++++++++
```

Daha fazla bilgi için şu komutu çalıştırabilirsiniz:

```bash
rustc --explain E0284
```

Farklı veri türleri için farklı tür açıklamaları göreceksiniz.
 
## 🔢 Skaler Türler (scalar types)

Bir skaler tür (scalar type) tek bir değeri temsil eder. Rust’ta dört temel skaler tür vardır: tamsayılar (integers), kayan noktalı sayılar (floating-point numbers), Boolean değerleri (booleans) ve karakterler (characters). Bunları diğer programlama dillerinden de tanıyor olabilirsiniz. Şimdi bunların Rust’ta nasıl çalıştığına bakalım.

## 🔢 Tamsayı Türleri (integer types)

Tamsayı (integer), kesirli bileşeni olmayan bir sayıdır. Bölüm 2’de bir tamsayı türü kullandık: `u32`. Bu tür bildirimi, değerin işaretsiz (unsigned) bir tamsayı olacağını (işaretli tamsayı türleri `u` yerine `i` ile başlar) ve 32 bitlik bir alan kaplayacağını belirtir. Tablo 3-1, Rust’taki yerleşik tamsayı türlerini göstermektedir. Bir tamsayı değerinin türünü bildirmek için bu varyantların herhangi birini kullanabiliriz.

### Tablo 3-1: Rust’taki Tamsayı Türleri

| Uzunluk        | İşaretli (Signed) | İşaretsiz (Unsigned) |
| -------------- | ----------------- | -------------------- |
| 8-bit          | i8                | u8                   |
| 16-bit         | i16               | u16                  |
| 32-bit         | i32               | u32                  |
| 64-bit         | i64               | u64                  |
| 128-bit        | i128              | u128                 |
| mimariye bağlı | isize             | usize                |

Her varyant işaretli (signed) veya işaretsiz (unsigned) olabilir ve açık bir boyuta sahiptir. İşaretli ve işaretsiz kavramı, sayının negatif olup olamayacağını ifade eder — başka bir deyişle, sayının işarete ihtiyaç duyup duymadığı. Kağıda sayı yazmaya benzer: işaret önemli olduğunda sayı artı veya eksi işaretiyle gösterilir; sayı yalnızca pozitif olacaksa işaretsiz yazılabilir. İşaretli sayılar iki tümleyenli gösterim (two’s complement representation) ile saklanır.

Her işaretli varyant, −(2ⁿ⁻¹) ile 2ⁿ⁻¹ − 1 arasında sayıları depolayabilir; burada n kullanılan bit sayısıdır. Örneğin, `i8` −(2⁷) ile 2⁷ − 1, yani −128 ile 127 arasındaki sayıları saklayabilir. İşaretsiz varyantlar ise 0 ile 2ⁿ − 1 arasındaki sayıları saklayabilir; örneğin `u8` 0 ile 2⁸ − 1, yani 0 ile 255 arasındaki sayıları saklar.

Ayrıca `isize` ve `usize` türleri, programınızın çalıştığı bilgisayarın mimarisine bağlıdır: 64-bit bir mimaride 64 bit, 32-bit bir mimaride 32 bitlik yer kaplar.

Tamsayı sabitlerini (integer literals) Tablo 3-2’de gösterilen biçimlerde yazabilirsiniz. Birden fazla sayısal tür olabilen sabitler için tür son ekleri (type suffix) kullanılabilir. Örneğin `57u8` değeri `u8` türünü açıkça belirtir. Ayrıca sayıları daha okunabilir yapmak için `_` ayırıcı kullanılabilir: `1_000` ifadesi `1000` ile aynı değeri taşır.

### Tablo 3-2: Rust’taki Tamsayı Sabitleri (integer literals)

| Sayı sabitleri (Number literals) | Örnek        |
| -------------------------------- | ------------ |
| Ondalık (Decimal)                | 98\_222      |
| Onaltılık (Hex)                  | 0xff         |
| Sekizlik (Octal)                 | 0o77         |
| İkilik (Binary)                  | 0b1111\_0000 |
| Bayt (yalnızca `u8`) (Byte)      | b'A'         |

Peki hangi tamsayı türünü kullanmanız gerektiğini nasıl bileceksiniz? Emin değilseniz, Rust’un varsayılanları genellikle iyi bir başlangıçtır: tamsayı türleri varsayılan olarak `i32` olur. `isize` veya `usize` kullanacağınız temel durum, bir koleksiyonda dizinleme (indexing) yaparken ortaya çıkar.
## ⚠️ Tamsayı Taşması (integer overflow)

Diyelim ki `u8` türünde bir değişkeniniz var ve bu değişken 0 ile 255 arasındaki değerleri tutabiliyor. Eğer değişkene bu aralık dışındaki bir değer, örneğin 256, atamaya çalışırsanız tamsayı taşması (integer overflow) meydana gelir ve bu iki davranıştan biri gerçekleşebilir.

Programınızı **debug modunda** derlediğinizde, Rust tamsayı taşmasını kontrol eder ve bu durum oluştuğunda programınızın çalışma zamanında hata vererek paniklemesine (panic) neden olur. Rust, bir program hatayla sonlandığında bu durumu “panicking” olarak adlandırır. Panik konusunu, Bölüm 9’daki “panic! ile Geri Döndürülemez Hatalar” kısmında ayrıntılı inceleyeceğiz.

Programınızı **release modunda** (`--release` bayrağı ile) derlediğinizde, Rust tamsayı taşması için panik kontrolü eklemez. Bunun yerine Rust, **iki tümleyenli sarmalama** (two’s complement wrapping) uygular. Kısaca, türün tutabileceği en büyük değeri aşan sayılar, türün alabileceği en küçük değere “sarmalanır”. Örneğin:

* `u8` için 256 değeri `0` olur,
* 257 değeri `1` olur,
* ve bu şekilde devam eder.

Bu durumda program paniklemez, ancak değişken beklediğinizden farklı bir değer alır. Tamsayı taşmasının bu sarmalama davranışına güvenmek hata olarak kabul edilir.

Tamsayı taşması ihtimalini açıkça yönetmek için standart kütüphanenin ilkel sayısal türler için sağladığı şu yöntem ailelerini kullanabilirsiniz:

* Tüm modlarda sarmalama yapmak için `wrapping_*` yöntemleri (örneğin `wrapping_add`).
* Taşma durumunda `None` döndürmek için `checked_*` yöntemleri.
* Değer ve taşma olup olmadığını belirten bir Boolean döndürmek için `overflowing_*` yöntemleri.
* Taşma durumunda değeri minimum veya maksimumda sabitlemek için `saturating_*` yöntemleri.


## 🔢 Kayan Noktalı Türler (floating-point types)

Rust, ondalık noktalı sayılar için iki ilkel tür (primitive type) sağlar. Bunlar `f32` ve `f64` olup sırasıyla 32 bit ve 64 bit uzunluğundadır. Varsayılan tür `f64`’tür çünkü modern CPU’larda `f32` kadar hızlıdır ancak daha yüksek hassasiyet (precision) sunar. Tüm kayan noktalı türler işaretlidir (signed).

Aşağıdaki örnek, kayan noktalı sayıların nasıl kullanılacağını göstermektedir:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Kayan noktalı sayılar **IEEE-754 standardına** göre temsil edilir.

---

## ➕ Sayısal İşlemler (numeric operations)

Rust, tüm sayısal türler için temel matematiksel işlemleri destekler: toplama, çıkarma, çarpma, bölme ve kalanı bulma.
Tamsayı bölme (integer division), sıfıra doğru yuvarlanarak kesilir.

Aşağıdaki örnek, her işlemin bir `let` ifadesinde nasıl kullanılacağını göstermektedir:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    // toplama (addition)
    let sum = 5 + 10;

    // çıkarma (subtraction)
    let difference = 95.5 - 4.3;

    // çarpma (multiplication)
    let product = 4 * 30;

    // bölme (division)
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Sonuç: -1

    // kalan (remainder)
    let remainder = 43 % 5;
}
```

Bu ifadelerdeki her işlem bir matematiksel operatör kullanır ve tek bir değere değerlendirilir; ardından bu değer bir değişkene bağlanır. Ek B’de Rust’un sağladığı tüm operatörlerin listesi bulunur.

---

## ✅ Boolean Türü (boolean type)

Çoğu programlama dilinde olduğu gibi, Rust’ta da Boolean türünün iki olası değeri vardır: `true` ve `false`. Boolean değerleri **bir bayt** uzunluğundadır. Rust’ta Boolean türü `bool` ile belirtilir.

Örnek:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let t = true;

    let f: bool = false; // açık tür bildirimi ile
}
```

Boolean değerlerini kullanmanın temel yolu, koşullu ifadeler (conditionals) içindedir; örneğin `if` ifadesinde. `if` ifadelerinin Rust’ta nasıl çalıştığını “Kontrol Akışı (Control Flow)” bölümünde inceleyeceğiz.

---

## 🔤 Karakter Türü (character type)

Rust’un `char` türü, dildeki en temel alfabetik türdür. İşte `char` değerlerinin bildirimine dair bazı örnekler:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // açık tür bildirimi ile
    let heart_eyed_cat = '😻';
}
```

`char` sabitleri tek tırnak (`'`) içinde belirtilir; çift tırnak (`"`) ise string sabitleri içindir.
Rust’un `char` türü **4 bayt** uzunluğundadır ve bir **Unicode skaler değeri** (Unicode scalar value) temsil eder. Bu sayede sadece ASCII değil; aksanlı harfler, Çince, Japonca ve Korece karakterler, emoji ve genişliği sıfır olan boşluklar (zero-width spaces) da geçerli `char` değerleridir.

Unicode skaler değer aralıkları:

* `U+0000` ile `U+D7FF`
* `U+E000` ile `U+10FFFF`

Ancak, Unicode’da “karakter” kavramı aslında açık bir şekilde tanımlı değildir. Bu nedenle, insani sezginizle “karakter” olarak düşündüğünüz şey her zaman Rust’taki `char` ile örtüşmeyebilir. Bu konuyu Bölüm 8’deki **“UTF-8 Kodlu Metni Stringlerle Saklamak”** kısmında ayrıntılı olarak inceleyeceğiz.

## 🧩 Bileşik Türler (compound types)

Bileşik türler (compound types), birden fazla değeri tek bir tür altında gruplamanıza olanak tanır. Rust’ta iki temel bileşik tür vardır: `tuple` ve `array`.

---

## 🧮 Tuple Türü (tuple type)

Tuple, farklı türlerden oluşan birden fazla değeri tek bir bileşik tür içinde gruplamanın genel bir yoludur. Tuple’lar **sabit uzunluktadır**: bir kez tanımlandıktan sonra boyutları değiştirilemez.

Bir tuple, parantez içine yazılmış virgülle ayrılmış değerler listesiyle oluşturulur. Tuple’daki her konumun bir türü vardır ve tuple içindeki değerlerin türleri aynı olmak zorunda değildir. Bu örnekte isteğe bağlı tür açıklamaları eklenmiştir:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

Değişken `tup`, tüm tuple’a bağlanır çünkü tuple tek bir bileşik öğe olarak kabul edilir. Tuple’daki bireysel değerlere erişmek için, tuple’ı **parçalama (destructuring)** yöntemiyle açabiliriz:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

Bu program önce bir tuple oluşturur ve `tup` değişkenine bağlar. Ardından `let` ifadesiyle tuple üç ayrı değişkene (`x`, `y`, `z`) ayrılır. Bu işleme **destructuring** denir çünkü tek bir tuple üç parçaya bölünür. Son olarak program `y` değerini (6.4) ekrana basar.

Tuple öğelerine doğrudan erişmek için, değerin indeksini belirten bir nokta (`.`) kullanılır:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

Bu program bir `x` tuple’ı oluşturur ve ardından her bir öğesine sırasıyla indeksleriyle (`0`, `1`, `2`) erişir. Çoğu programlama dilinde olduğu gibi, tuple’daki ilk indeks `0`’dır.

Değeri olmayan tuple’a özel bir isim verilir: **unit**. Bu değer ve karşılık gelen tür `()` şeklinde yazılır ve boş bir değer veya boş bir dönüş türünü temsil eder. Eğer ifadeler başka bir değer döndürmezse, dolaylı olarak unit değeri döndürürler.

---

## 📦 Array Türü (array type)

Birden fazla değeri bir arada tutmanın bir diğer yolu `array` kullanmaktır. Tuple’dan farklı olarak, bir array içindeki **her öğe aynı türde olmalıdır**. Rust’taki array’ler de **sabit uzunluktadır**.

Bir array, köşeli parantezler içinde virgülle ayrılmış değerler listesiyle tanımlanır:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Array’ler, verilerinizi yığında (stack) depolamak istediğinizde faydalıdır (Bölüm 4’te stack ve heap konularını detaylı ele alacağız). Ayrıca, her zaman sabit sayıda elemana sahip olmak istediğinizde array kullanılır.

Array, **vector** kadar esnek değildir. `vector`, standart kütüphane tarafından sağlanan ve boyutu değişebilen (büyüyüp küçülebilen) bir koleksiyon türüdür çünkü içeriği heap üzerinde tutulur. Eğer array mi vector mü kullanmanız gerektiğinden emin değilseniz, genellikle `vector` tercih etmelisiniz. Vector’leri Bölüm 8’de ayrıntılı ele alacağız.

Bununla birlikte, öğe sayısının değişmeyeceğini biliyorsanız array daha kullanışlıdır. Örneğin, bir programda ayların isimlerini kullanıyorsanız, bunların her zaman 12 olacağını bildiğinizden array tercih edebilirsiniz:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

Bir array’in türünü yazarken, köşeli parantezler içinde eleman türü, ardından noktalı virgül, ardından da eleman sayısı belirtilir:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Burada `i32`, her bir elemanın türüdür. Noktalı virgülden sonra gelen `5`, dizinin beş elemandan oluştuğunu belirtir.

Ayrıca, tüm elemanları aynı değerle başlatmak için, başlangıç değerini yazıp noktalı virgül ile uzunluğu belirtebilirsiniz:

```rust
let a = [3; 5];
```

Bu durumda `a` isimli array, her biri başlangıçta `3` olan 5 eleman içerecektir. Bu ifade, `let a = [3, 3, 3, 3, 3];` ile aynı sonucu verir, ancak daha kısa bir yazım şeklidir.

## 📥 Dizi Elemanlarına Erişim (accessing array elements)

Bir dizi (array), yığında (stack) ayrılabilen, bilinen ve sabit boyutlu tek bir bellek bloğudur. Bir dizinin elemanlarına indeksleme (indexing) kullanarak erişebilirsiniz:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

Bu örnekte `first` değişkeni `1` değerini alır çünkü bu değer dizinin `[0]` indeksinde bulunur. `second` değişkeni ise `2` değerini alır çünkü bu değer dizinin `[1]` indeksinde bulunur.

---

## ⚠️ Geçersiz Dizi Elemanı Erişimi (invalid array element access)

Peki, dizinin sonunu aşan bir elemana erişmeye çalışırsanız ne olur? Bölüm 2’deki tahmin oyunu örneğine benzer şekilde, kullanıcıdan bir dizi indeksi alan şu kodu çalıştırdığınızı düşünün:

**Dosya adı:** `src/main.rs`

```rust
// Bu kod panic ile sonlanır!
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

Bu kod başarıyla derlenir. Eğer `cargo run` ile çalıştırıp 0, 1, 2, 3 veya 4 girerseniz, program dizideki ilgili indeksteki değeri ekrana yazar. Ancak örneğin `10` girerseniz, şu çıktıyı görürsünüz:

```
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Bu durumda program, geçersiz bir indeksleme işlemi yapıldığı noktada **çalışma zamanı hatası (runtime error)** ile karşılaşır. Hata mesajı basılır ve program sonlandırılır; son `println!` ifadesi çalışmaz.

Rust, indeksleme yaparken belirtilen indeksin dizi uzunluğundan küçük olduğunu kontrol eder. Eğer indeks, uzunluktan büyük ya da eşitse, Rust `panic` ile programı sonlandırır. Bu kontrolün çalışma zamanında yapılması gerekir çünkü derleyici, kullanıcının daha sonra hangi değeri gireceğini bilemez.

---

Bu durum, Rust’un **bellek güvenliği (memory safety)** ilkelerinin bir örneğidir. Düşük seviyeli birçok dilde bu kontrol yapılmaz ve hatalı bir indeks verildiğinde geçersiz belleğe erişilebilir. Rust ise böyle bir durumda belleğe erişime izin vermek yerine programı hemen sonlandırarak sizi bu tür hatalardan korur.

Bölüm 9’da Rust’un hata işleme (error handling) mekanizmalarını ve hem okunabilir hem de güvenli kod yazmayı, yani ne panikleyen ne de geçersiz bellek erişimine izin veren çözümleri daha ayrıntılı inceleyeceğiz.
