## 🧬 Ömürlerle Başvuruları Doğrulama (lifetimes)

Ömürler (lifetimes), zaten kullandığımız başka bir tür geneldir (generic). Bir türün istediğimiz davranışa sahip olmasını sağlamaktan ziyade, ömürler (lifetimes), başvuruların (references) ihtiyaç duyduğumuz süre boyunca geçerli olmasını sağlar.

Bölüm 4’teki “Referanslar ve Ödünç Alma” (References and Borrowing) bölümünde değinmediğimiz bir ayrıntı, Rust’taki her başvurunun (reference) bir ömrü (lifetime) olduğudur; bu, söz konusu başvurunun geçerli olduğu kapsamı (scope) ifade eder. Çoğu zaman, ömürler (lifetimes) tıpkı türler (types) gibi örtüktür ve çıkarımsanır. Yalnızca birden çok türün mümkün olduğu durumlarda türleri açıklamamız gerekir. Benzer bir şekilde, başvuruların (references) ömürlerinin (lifetimes) birkaç farklı şekilde ilişkili olabileceği durumlarda ömürleri açıklamamız gerekir. Rust, çalışma zamanında kullanılacak gerçek başvuruların kesinlikle geçerli olacağından emin olmak için bu ilişkileri genel ömür parametrelerini (generic lifetime parameters) kullanarak açıklamamızı ister.

Ömürleri açıklamak (annotating lifetimes), çoğu programlama dilinde bulunan bir kavram değildir, bu yüzden başlangıçta alışılmadık gelebilir. Bu bölümde ömürlerin (lifetimes) tamamını ele almayacak olsak da, kavrama alışmanız için karşılaşabileceğiniz yaygın ömür sözdizimi (lifetime syntax) biçimlerini tartışacağız.

## 🛡️ Ömürlerle Sallantıda Kalan Başvuruları Önleme (dangling references)

Ömürlerin (lifetimes) temel amacı, bir programın amaçladığı veriden farklı bir veriye başvurmasına yol açan sallantıda kalan başvuruları (dangling references) önlemektir. 10-16 numaralı listede, dış bir kapsam (outer scope) ve iç bir kapsam (inner scope) içeren programı ele alın.

Bu kod derlenmez!

```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}");
}
```

Liste 10-16: Değeri kapsam dışına çıkmış bir başvuruyu (reference) kullanma girişimi

Not: 10-16, 10-17 ve 10-23 numaralı listelerdeki örnekler, değişkenleri başlangıç değeri vermeden bildirir; böylece değişken adı dış kapsamda (outer scope) var olur. İlk bakışta bu durum Rust’ın boş (null) değerleri olmamasıyla çelişiyor gibi görünebilir. Ancak bir değişkeni ona değer vermeden kullanmaya çalışırsak, derleme zamanı hatası (compile-time error) alırız; bu da Rust’ın gerçekten boş (null) değerlere izin vermediğini gösterir.

Dış kapsam, başlangıç değeri olmadan r adlı bir değişken bildirir ve iç kapsam, başlangıç değeri 5 olan x adlı bir değişken bildirir. İç kapsamda, r’nin değerini x’e bir başvuru (reference) olacak şekilde atamaya çalışırız. Sonra iç kapsam biter ve r’deki değeri yazdırmayı deneriz. Bu kod, r’nin başvurduğu değerin, r’yi kullanmaya çalışmadan önce kapsam dışına çıkmış olması nedeniyle derlenmez. Hata iletisi şöyledir:

```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
5 |         let x = 5;
  |             - binding `x` declared here
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {r}");
  |                  --- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

Hata iletisi, x değişkeninin “yeterince uzun süre yaşamadığını” söyler. Bunun nedeni, x’in satır 7’de iç kapsam sona erdiğinde kapsam dışına çıkacak olmasıdır. Ancak r dış kapsam için hâlâ geçerlidir; kapsamı daha geniş olduğu için “daha uzun yaşar” deriz. Rust bu koda izin verseydi, r, x kapsam dışına çıktığında serbest bırakılmış belleğe başvuruyor olacaktı ve r ile yapmayı denediğimiz hiçbir şey düzgün çalışmayacaktı. Peki Rust, bu kodun geçersiz olduğuna nasıl karar verir? Bir ödünç alma denetleyicisi (borrow checker) kullanır.

## 🔍 Ödünç Alma Denetleyicisi (borrow checker)

Rust derleyicisi, tüm ödünç almaların (borrows) geçerli olup olmadığını belirlemek için kapsamları (scopes) karşılaştıran bir ödünç alma denetleyicisine (borrow checker) sahiptir. Liste 10-17, değişkenlerin ömürlerini (lifetimes) gösteren açıklamalarla birlikte, Liste 10-16’daki kodun aynısını göstermektedir.

Bu kod derlenmez!

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
```

Liste 10-17: Sırasıyla r ve x değişkenlerinin 'a ve 'b olarak adlandırılan ömürlerine (lifetimes) ilişkin açıklamalar

Burada r’nin ömrünü 'a, x’in ömrünü ise 'b ile açıkladık. Görüldüğü gibi, içteki 'b bloğu dıştaki 'a ömür bloğuna göre çok daha küçüktür. Derleme zamanında Rust, bu iki ömrün boyutunu karşılaştırır ve r’nin 'a ömrüne sahip olduğunu, ancak 'b ömrüne sahip bir belleğe başvurduğunu görür. Program reddedilir çünkü 'b, 'a’dan daha kısadır: başvurunun (reference) konusu, başvurunun kendisi kadar uzun yaşamaz.

Liste 10-18, sallantıda kalan başvuruyu (dangling reference) ortadan kaldıracak şekilde kodu düzeltir ve hatasız derlenir.

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+
```

Liste 10-18: Verinin ömrü referanstan daha uzun olduğu için geçerli bir başvuru (reference)

Burada x, 'b ömrüne sahiptir ve bu durumda 'a’dan daha uzundur. Bu, r’nin x’e başvurabileceği anlamına gelir; çünkü Rust, r içindeki başvurunun, x geçerli olduğu sürece her zaman geçerli olacağını bilir.

Artık başvuruların (references) ömürlerinin (lifetimes) nerede olduğunu ve Rust’ın başvuruların daima geçerli olmasını sağlamak için ömürleri nasıl analiz ettiğini bildiğinize göre, fonksiyonlar bağlamında parametrelerin ve dönüş değerlerinin genel ömürlerini (generic lifetimes) inceleyelim.

## 🧩 Fonksiyonlarda Genel Ömürler (generic lifetimes in functions)

İki `string dilimi`nin (string slice) daha uzun olanını döndüren bir fonksiyon yazacağız. Bu fonksiyon iki `string dilimi` (string slice) alacak ve tek bir `string dilimi` (string slice) döndürecek. `longest` fonksiyonunu uyguladıktan sonra, Liste 10-19’daki kodun “The longest string is abcd” yazdırması gerekir.

Filename: src/main.rs

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
```

Liste 10-19: İki `string dilimi`nin (string slice) daha uzununu bulmak için `longest` fonksiyonunu çağıran bir `main` fonksiyonu

Fonksiyonun, `string`lerin kendilerini değil, `string dilimleri`ni (string slices) almasını istediğimize dikkat edin; çünkü `longest` fonksiyonunun parametrelerin sahipliğini (ownership) almasını istemiyoruz. Neden Liste 10-19’da kullandığımız parametrelerin istediğimiz türde olduğunu tartışmak için Bölüm 4’teki “Parametreler Olarak String Dilimleri” (String Slices as Parameters) bölümüne bakın.

Liste 10-20’de gösterildiği gibi `longest` fonksiyonunu uygulamayı denersek, derlenmeyecektir.

Filename: src/main.rs
Bu kod derlenmez!

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

Liste 10-20: İki `string dilimi`nin (string slice) daha uzununu döndüren fakat henüz derlenmeyen `longest` uygulaması

Bunun yerine, ömürler (lifetimes) hakkında konuşan aşağıdaki hatayı alırız:

```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

Yardım metni, dönüş türünün üzerine genel bir ömür parametresi (generic lifetime parameter) eklenmesi gerektiğini ortaya koyar; çünkü Rust, döndürülen başvurunun (reference) `x`’e mi, yoksa `y`’ye mi ait olduğunu bilemez. Aslında biz de bilemeyiz; çünkü bu fonksiyonun gövdesindeki `if` bloğu `x`’e, `else` bloğu ise `y`’ye bir başvuru (reference) döndürür!

Bu fonksiyonu tanımlarken, ona geçirilecek somut değerleri bilmiyoruz; dolayısıyla `if` durumunun mu, yoksa `else` durumunun mu çalışacağını da bilmiyoruz. Ayrıca, geçirilecek başvuruların (references) somut ömürlerini (lifetimes) de bilmediğimiz için, Liste 10-17 ve 10-18’de yaptığımız gibi kapsamları (scopes) inceleyerek döndüreceğimiz başvurunun her zaman geçerli olup olmayacağını belirleyemeyiz. Ödünç alma denetleyicisi (borrow checker) de bunu belirleyemez; çünkü `x` ve `y`’nin ömürlerinin, dönüş değerinin ömrüyle nasıl ilişkili olduğunu bilemez. Bu hatayı düzeltmek için, başvurular arasındaki ilişkiyi tanımlayan genel ömür parametreleri (generic lifetime parameters) ekleyeceğiz ki ödünç alma denetleyicisi (borrow checker) analizini yapabilsin.

## ✒️ Ömür Açıklama Sözdizimi (lifetime annotation syntax)

Ömür açıklamaları (lifetime annotations), herhangi bir başvurunun (reference) ne kadar süre yaşayacağını değiştirmez. Bunun yerine, ömürleri etkilemeden birden fazla başvurunun ömürlerinin birbirleriyle olan ilişkilerini tanımlar. Nasıl ki fonksiyon imzasında (signature) genel bir tür parametresi (generic type parameter) belirtildiğinde fonksiyon herhangi bir türü kabul edebiliyorsa, genel bir ömür parametresi (generic lifetime parameter) belirterek de fonksiyon herhangi bir ömre sahip başvuruları kabul edebilir.

Ömür açıklamaları biraz alışılmadık bir sözdizimine sahiptir: ömür parametrelerinin (lifetime parameters) adları bir kesme işareti (') ile başlamalıdır ve genellikle küçük harfli ve çok kısa olur; tıpkı genel türler (generic types) gibi. Çoğu kişi ilk ömür açıklaması için `'a` adını kullanır. Ömür parametresi açıklamalarını, başvurunun (reference) türünden bir boşlukla ayırarak `&` işaretinden sonra yazarız.

İşte bazı örnekler: bir `i32`’ye ömür parametresi olmadan yapılan başvuru, `'a` adında ömür parametresi bulunan bir `i32` başvurusu ve aynı ömre `'a` sahip olan değiştirilebilir (mutable) bir `i32` başvurusu.

```rust
&i32        // bir başvuru
&'a i32     // açıkça belirtilmiş ömürle bir başvuru
&'a mut i32 // açıkça belirtilmiş ömürle değiştirilebilir bir başvuru
```

Tek başına bir ömür açıklaması çok fazla anlam ifade etmez; çünkü açıklamalar, Rust’a birden fazla başvurunun genel ömür parametrelerinin birbirleriyle nasıl ilişkili olduğunu anlatmak için vardır. Şimdi ömür açıklamalarının `longest` fonksiyonunun bağlamında birbirleriyle nasıl ilişkili olduğunu inceleyelim.

## 📝 Fonksiyon İmzalarında Ömür Açıklamaları (lifetime annotations in function signatures)

Fonksiyon imzalarında (function signatures) ömür açıklamaları (lifetime annotations) kullanmak için, tıpkı genel tür parametrelerinde (generic type parameters) yaptığımız gibi, fonksiyon adı ile parametre listesi arasına açılı ayraçlar (`<>`) içinde genel ömür parametrelerini (generic lifetime parameters) bildirmemiz gerekir.

İmzanın şu kısıtlamayı ifade etmesini istiyoruz: döndürülen başvuru (reference), her iki parametre de geçerli olduğu sürece geçerli olacaktır. Bu, parametrelerin ömürleri ile dönüş değerinin ömrü arasındaki ilişkidir. Ömre `'a` adını vereceğiz ve bunu her başvuruya ekleyeceğiz; Liste 10-21’de gösterildiği gibi.

Filename: src/main.rs

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Liste 10-21: İmzada geçen tüm başvuruların aynı `'a` ömrüne sahip olması gerektiğini belirten `longest` fonksiyonu tanımı

Bu kod, Liste 10-19’daki `main` fonksiyonuyla kullanıldığında derlenmeli ve istediğimiz sonucu üretmelidir.

Fonksiyon imzası artık Rust’a şunu söylüyor: `'a` adında bir ömür için, fonksiyon iki parametre alır; bu parametrelerin ikisi de en az `'a` kadar yaşayan `string dilimleri`dir (string slices). Fonksiyon imzası ayrıca, fonksiyondan döndürülen `string diliminin` de en az `'a` kadar yaşayacağını belirtir. Pratikte bu, `longest` fonksiyonunun döndürdüğü başvurunun ömrünün, fonksiyon argümanlarının başvurduğu değerlerin ömürlerinin daha küçük olanına eşit olduğu anlamına gelir. Bu ilişkiler, Rust’ın bu kodu analiz ederken kullanmasını istediğimiz şeydir.

Unutmayın: Bu fonksiyon imzasında ömür parametrelerini belirtirken, geçirilmiş veya döndürülmüş herhangi bir değerin ömrünü değiştirmiyoruz. Bunun yerine, ödünç alma denetleyicisinin (borrow checker), bu kısıtlamalara uymayan değerleri reddetmesini belirtiyoruz. Dikkat edin ki, `longest` fonksiyonu `x` ve `y`’nin ne kadar yaşayacağını tam olarak bilmek zorunda değildir; yalnızca `'a` yerine geçebilecek ve bu imzayı tatmin edecek bir kapsam (scope) olduğunu bilmesi yeterlidir.

Fonksiyonlarda ömür açıklamaları yaparken, açıklamalar fonksiyon gövdesine değil, fonksiyon imzasına yazılır. Ömür açıklamaları, tıpkı türler gibi imzanın bir parçası olur. Fonksiyon imzalarının ömür sözleşmesini (lifetime contract) içermesi, Rust derleyicisinin analizini daha basit hale getirir. Bir fonksiyonun ömür açıklaması ya da çağrılma biçiminde bir sorun varsa, derleyici hataları kodumuzun ilgili kısmına ve kısıtlamalara daha kesin şekilde işaret edebilir. Eğer Rust derleyicisi ömürler arasındaki ilişkilere dair daha fazla çıkarım yapsaydı, yalnızca sorunun kaynağından çok uzaktaki bir kullanım noktasına işaret edebilirdi.

Somut başvuruları `longest` fonksiyonuna geçirdiğimizde, `'a` yerine geçirilen somut ömür, `x` ve `y` kapsamlarının kesişen kısmı olur. Başka bir deyişle, genel `'a` ömrü, `x` ve `y`’nin ömürlerinin daha küçük olanına eşit olan somut ömrü alır. Döndürülen başvuruyu da aynı `'a` ömrüyle açıklamış olduğumuz için, döndürülen başvuru da `x` ve `y`’nin ömürlerinden daha küçük olanı kadar geçerli olacaktır.

Şimdi, farklı somut ömürlere sahip başvuruları geçirerek `longest` fonksiyonunun nasıl sınırlandığını inceleyelim. Liste 10-22 basit bir örnektir.

Filename: src/main.rs

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}
```

Liste 10-22: Farklı somut ömürlere sahip `String` başvurularıyla `longest` fonksiyonunu kullanma

Bu örnekte, `string1` dış kapsamın sonuna kadar geçerlidir, `string2` iç kapsamın sonuna kadar geçerlidir ve `result`, iç kapsamın sonuna kadar geçerli olan bir şeye başvurur. Bu kodu çalıştırırsanız, ödünç alma denetleyicisinin (borrow checker) onayladığını göreceksiniz; kod derlenecek ve “The longest string is long string is long” çıktısını verecektir.

Şimdi de, `result` içindeki başvurunun ömrünün iki argümandan daha kısa olanın ömrü olması gerektiğini gösteren bir örneği deneyelim. `result` değişkeninin bildirimini iç kapsamın dışına taşıyacağız, ancak değer atamasını `string2` ile birlikte iç kapsamda bırakacağız. Daha sonra, `result`’u kullanan `println!` ifadesini iç kapsam bittikten sonraki yere taşıyacağız. Liste 10-23’teki kod derlenmeyecektir.

Filename: src/main.rs
Bu kod derlenmez!

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}
```

Liste 10-23: `string2` kapsam dışına çıktıktan sonra `result` değişkenini kullanma girişimi

Bu kodu derlemeye çalıştığımızda şu hatayı alırız:

```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
5 |         let string2 = String::from("xyz");
  |             ------- binding `string2` declared here
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {result}");
  |                                     -------- borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

Hata, `result`’un `println!` ifadesi için geçerli olabilmesi adına `string2`’nin dış kapsamın sonuna kadar geçerli olması gerektiğini gösterir. Rust bunu bilir; çünkü fonksiyon parametrelerinin ve dönüş değerinin ömürlerini aynı `'a` parametresiyle açıklamıştık.

İnsan olarak biz bu koda bakıp `string1`’in `string2`’den uzun olduğunu, dolayısıyla `result`’un `string1`’e başvuru içereceğini görebiliriz. `string1` hâlâ kapsam dışına çıkmadığından, `string1`’e olan başvuru `println!` ifadesi için hâlâ geçerli olur. Ancak derleyici bu durumda başvurunun geçerli olduğunu göremez. Biz Rust’a, `longest` fonksiyonunun döndürdüğü başvurunun ömrünün, geçirilen başvuruların ömürlerinden daha kısa olanıyla aynı olduğunu söyledik. Bu yüzden, ödünç alma denetleyicisi (borrow checker), 10-23 numaralı listedeki kodu geçersiz bir başvuru ihtimali nedeniyle reddeder.

Şimdi, `longest` fonksiyonuna geçirilen başvuruların değerlerini ve ömürlerini değiştirerek farklı deneyler tasarlayın ve döndürülen başvurunun nasıl kullanıldığını inceleyin. Derleyiciyi çalıştırmadan önce deneylerinizin ödünç alma denetleyicisinden geçip geçmeyeceğine dair tahminlerde bulunun; ardından gerçekten haklı olup olmadığınızı kontrol edin!


## 🧠 Ömürler Açısından Düşünmek (thinking in terms of lifetimes)

Bir fonksiyonun ömür parametrelerini (lifetime parameters) nasıl belirtmeniz gerektiği, fonksiyonun ne yaptığına bağlıdır. Örneğin, `longest` fonksiyonunun uygulamasını her zaman ilk parametreyi döndürecek şekilde değiştirirsek, `y` parametresine ömür belirtmemize gerek kalmaz. Aşağıdaki kod derlenecektir:

Filename: src/main.rs

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

Burada `x` parametresi ve dönüş türü için `'a` ömür parametresi belirttik, fakat `y` için belirtmedik; çünkü `y`’nin ömrünün, ne `x`’in ömrüyle ne de dönüş değerinin ömrüyle herhangi bir ilişkisi yoktur.

Bir fonksiyondan bir başvuru (reference) döndürürken, dönüş türü için ömür parametresi mutlaka parametrelerden birinin ömür parametresiyle eşleşmelidir. Eğer döndürülen başvuru parametrelerden birine ait değilse, fonksiyon içinde oluşturulmuş bir değere ait olmalıdır. Ancak bu durumda, değer fonksiyon sonunda kapsam dışına çıkacağı için sallantıda kalan bir başvuru (dangling reference) oluşur. Derlenmeyen aşağıdaki `longest` uygulamasını inceleyin:

Filename: src/main.rs
Bu kod derlenmez!

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Burada, dönüş türü için `'a` ömür parametresi belirtmiş olsak bile, bu uygulama derlenmeyecektir; çünkü dönüş değerinin ömrü parametrelerin ömrüyle hiçbir şekilde ilişkili değildir. Alınan hata mesajı şöyledir:

```
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0515]: cannot return value referencing local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here

For more information about this error, try `rustc --explain E0515`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

Sorun şudur: `result` değişkeni `longest` fonksiyonunun sonunda kapsam dışına çıkar ve temizlenir. Biz ise fonksiyondan `result`’a bir başvuru döndürmeye çalışıyoruz. Sallantıda kalan bu başvuruyu değiştirecek şekilde ömür parametreleri belirtmemizin bir yolu yoktur ve Rust da sallantıda kalan başvuru oluşturulmasına izin vermez. Bu durumda en iyi çözüm, başvuru yerine sahipli (owned) bir veri türü döndürmektir; böylece değerin temizlenmesinden çağıran fonksiyon sorumlu olur.

Sonuçta, ömür sözdizimi (lifetime syntax), bir fonksiyonun çeşitli parametrelerinin ve dönüş değerlerinin ömürlerini birbirine bağlamaktan ibarettir. Bu bağlantılar kurulduğunda, Rust belleğin güvenli bir şekilde kullanılmasını sağlayacak kadar bilgiye sahip olur ve sallantıda kalan işaretçiler (dangling pointers) veya bellek güvenliğini ihlal edecek işlemlere izin vermez.


## 🏗️ Struct Tanımlarında Ömür Açıklamaları (lifetime annotations in struct definitions)

Şimdiye kadar tanımladığımız `struct`ların hepsi sahipli (owned) türler tuttu. Ancak `struct`ların başvuru (reference) tutmasını da tanımlayabiliriz; bu durumda, `struct` tanımındaki her başvuruya bir ömür açıklaması (lifetime annotation) eklememiz gerekir. Liste 10-24’te, bir `string dilimi` (string slice) tutan `ImportantExcerpt` adında bir `struct` vardır.

Filename: src/main.rs

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

Liste 10-24: Bir başvuru tutan ve ömür açıklaması gerektiren bir `struct`

Bu `struct`, `part` adında tek bir alana sahiptir ve bu alan bir `string dilimi` (string slice) yani bir başvuru tutar. Genel veri türlerinde (generic data types) olduğu gibi, ömür parametresinin (lifetime parameter) adını `struct` adından sonra açılı ayraçlar (`<>`) içinde bildiririz; böylece ömür parametresini `struct` tanımının gövdesinde kullanabiliriz. Bu açıklama, `ImportantExcerpt` örneğinin `part` alanında tuttuğu başvurudan daha uzun yaşamayacağını ifade eder.

Buradaki `main` fonksiyonu, `novel` değişkenine ait `String` içindeki ilk cümleye başvuru tutan bir `ImportantExcerpt` örneği oluşturur. `novel` içindeki veri, `ImportantExcerpt` örneği oluşturulmadan önce zaten vardır. Ayrıca, `novel`, `ImportantExcerpt` kapsam dışına çıkana kadar kapsamda kalır; bu yüzden `ImportantExcerpt` içindeki başvuru geçerlidir.


## ✂️ Ömür Gizleme (lifetime elision)

Artık her başvurunun (reference) bir ömrü (lifetime) olduğunu ve başvuru kullanan fonksiyonlar veya `struct`lar için ömür parametreleri belirtmeniz gerektiğini öğrendiniz. Ancak, Liste 4-9’da tanımladığımız ve aşağıda Liste 10-25’te tekrar gösterilen bir fonksiyon, ömür açıklamaları (lifetime annotations) olmadan derlenmişti.

Filename: src/lib.rs

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Liste 10-25: Parametresi ve dönüş türü başvurular (references) olmasına rağmen ömür açıklamaları olmadan derlenen bir fonksiyon

Bu fonksiyonun ömür açıklamaları olmadan derlenmesinin nedeni tarihseldir: Rust’ın erken sürümlerinde (1.0 öncesi) bu kod derlenmezdi; çünkü her başvurunun açık bir ömre sahip olması gerekiyordu. O zamanlardaki fonksiyon imzası şöyle yazılırdı:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

Rust ekibi çok sayıda kod yazdıktan sonra, programcıların belirli durumlarda aynı ömür açıklamalarını tekrar tekrar yazdıklarını fark etti. Bu durumlar öngörülebilirdi ve birkaç deterministik kurala uyuyordu. Geliştiriciler bu kuralları derleyiciye programladılar; böylece ödünç alma denetleyicisi (borrow checker), bu durumlarda ömürleri çıkarımlayabiliyordu ve açık açıklamalara gerek kalmıyordu.

Rust tarihinin bu kısmı önemlidir; çünkü gelecekte daha fazla deterministik desen keşfedilip derleyiciye eklenebilir. Bu da daha az ömür açıklaması yazmayı gerektirebilir.

Rust’ın başvuruların analizine programladığı bu desenlere **ömür gizleme kuralları (lifetime elision rules)** denir. Bunlar, programcıların uyması gereken kurallar değildir; derleyicinin dikkate aldığı özel durumlardır. Kodunuz bu durumlara uyuyorsa, ömürleri açıkça yazmanıza gerek yoktur.

Ömür gizleme kuralları tam çıkarım sağlamaz. Eğer Rust bu kuralları uyguladıktan sonra başvuruların ömürleri konusunda hâlâ belirsizlik kalırsa, derleyici ömürleri tahmin etmez. Bunun yerine, eklemeniz gereken ömür açıklamalarıyla çözebileceğiniz bir hata verir.

Fonksiyon veya metot parametrelerindeki ömürlere **girdi ömürleri (input lifetimes)**, dönüş değerlerindeki ömürlere ise **çıktı ömürleri (output lifetimes)** denir.

Derleyici, ömür açıklamaları yazılmadığında başvuruların ömürlerini belirlemek için üç kural kullanır. İlk kural girdi ömürleri için, ikinci ve üçüncü kurallar çıktı ömürleri için geçerlidir. Derleyici üç kuralı uyguladıktan sonra hâlâ ömrü çözülemeyen başvurular varsa, hata verir. Bu kurallar hem `fn` tanımlarına hem de `impl` bloklarına uygulanır.

1. **Kural 1:** Derleyici, başvuru olan her parametreye bir ömür parametresi atar.

   * Örneğin, tek parametreli bir fonksiyon tek ömür parametresi alır:
     `fn foo<'a>(x: &'a i32);`
   * İki parametreli fonksiyon iki ayrı ömür parametresi alır:
     `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);`

2. **Kural 2:** Eğer yalnızca bir girdi ömrü parametresi varsa, bu ömür tüm çıktı ömrü parametrelerine atanır.

   * Örneğin: `fn foo<'a>(x: &'a i32) -> &'a i32;`

3. **Kural 3:** Eğer birden fazla girdi ömrü parametresi varsa, ama bunlardan biri `&self` veya `&mut self` ise (yani bir metot söz konusuysa), `self`’in ömrü tüm çıktı ömrü parametrelerine atanır.

   * Bu kural, metotların okunmasını ve yazılmasını daha kolay hale getirir çünkü daha az sembol gerekir.

Şimdi kendimizi derleyici yerine koyalım. Liste 10-25’teki `first_word` fonksiyonu imzasındaki başvuruların ömürlerini bu kuralları uygulayarak çıkaralım. Başlangıçta ömürsüz imza şudur:

```rust
fn first_word(s: &str) -> &str {
```

Derleyici birinci kuralı uygular: her parametreye bir ömür atanır. Adını her zamanki gibi `'a` koyarsak imza şöyle olur:

```rust
fn first_word<'a>(s: &'a str) -> &str {
```

İkinci kural uygulanır çünkü yalnızca bir girdi ömrü vardır. Bu durumda, giriş ömrü çıkış ömrüne atanır. Böylece imza şu hale gelir:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

Artık bu fonksiyon imzasındaki tüm başvuruların ömürleri vardır ve derleyici, programcıdan ömür açıklamaları istemeden analizine devam edebilir.

Şimdi başka bir örneğe bakalım: Liste 10-20’de ömür parametresi olmayan `longest` fonksiyonunu hatırlayın:

```rust
fn longest(x: &str, y: &str) -> &str {
```

Birinci kuralı uygulayalım: her parametre kendi ömrünü alır. İki parametre olduğu için iki ömür parametresi elde ederiz:

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Burada ikinci kural uygulanmaz; çünkü birden fazla girdi ömrü vardır. Üçüncü kural da uygulanmaz; çünkü `longest` bir metot değil, dolayısıyla parametrelerden hiçbiri `self` değildir. Tüm üç kuralı uyguladıktan sonra dönüş türünün ömrünü hâlâ belirleyemedik. Bu yüzden, Liste 10-20’deki kodu derlemeye çalışırken hata aldık: derleyici ömür gizleme kurallarını uyguladı, ancak imzadaki tüm başvuruların ömürlerini çıkarımlayamamış oldu.

Üçüncü kural yalnızca metot imzalarında geçerli olduğundan, neden metot imzalarında genellikle ömür açıklamalarını yazmamıza gerek kalmadığını görmek için bir sonraki adımda ömürleri bu bağlamda inceleyeceğiz.

## 🛠️ Metot Tanımlarında Ömür Açıklamaları (lifetime annotations in method definitions)

Ömürlere sahip bir `struct` üzerinde metotlar uygularsak, aynı genel tür parametrelerinde (generic type parameters) olduğu gibi sözdizimini kullanırız. Bunun örneğini Liste 10-11’de görmüştük. Ömür parametrelerini (lifetime parameters) nerede bildirip nerede kullandığımız, bunların `struct` alanlarıyla mı yoksa metot parametreleri ve dönüş değerleriyle mi ilişkili olduğuna bağlıdır.

`struct` alanları için ömür adları her zaman `impl` anahtar kelimesinden sonra bildirilmelidir ve `struct` adından sonra kullanılmalıdır; çünkü bu ömürler `struct`’ın tipinin bir parçasıdır.

`impl` bloğu içindeki metot imzalarında ise başvurular (references), `struct`’ın alanlarındaki başvuruların ömrüne bağlı olabilir ya da bağımsız olabilir. Ayrıca, ömür gizleme kuralları (lifetime elision rules) genellikle metot imzalarında ömür açıklamalarını gereksiz kılar. Liste 10-24’te tanımladığımız `ImportantExcerpt` adlı `struct` ile bazı örneklere bakalım.

Önce, tek parametresi `self`’e başvuru olan ve dönüş değeri bir başvuru içermeyen `i32` olan `level` adlı metodu kullanalım:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

Burada `impl`’den sonra yapılan ömür parametresi bildirimi ve tür adından sonra kullanımı zorunludur. Ancak, `self`’e yapılan başvurunun ömrünü açıklamamız gerekmez; çünkü birinci gizleme kuralı (first elision rule) bunu sağlar.

Şimdi de üçüncü ömür gizleme kuralının uygulandığı bir örneğe bakalım:

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```

Burada iki girdi ömrü vardır, bu yüzden Rust birinci ömür gizleme kuralını uygular ve hem `&self` hem de `announcement` kendi ömürlerini alır. Sonra, parametrelerden biri `&self` olduğu için, dönüş türü `&self`’in ömrünü alır ve tüm ömürler belirlenmiş olur.

---

## ⏳ Statik Ömür (the static lifetime)

Özel olarak ele almamız gereken bir ömür `'static`tir. Bu ömür, ilgili başvurunun programın tüm süresi boyunca yaşayabileceğini belirtir. Tüm string literal’ler `'static` ömrüne sahiptir ve şu şekilde açıklanabilir:

```rust
let s: &'static str = "I have a static lifetime.";
```

Bu string’in metni doğrudan programın ikili dosyasında (binary) saklanır ve her zaman kullanılabilir. Bu nedenle, tüm string literal’lerin ömrü `'static`tir.

Hata mesajlarında `'static` ömrünü kullanmanız önerilebilir. Ancak, bir başvuruya `'static` ömrü vermeden önce gerçekten programınızın tüm süresi boyunca yaşayıp yaşamadığını ve yaşamasını isteyip istemediğinizi düşünün. Çoğu durumda, `'static` ömrü öneren hata mesajı, sallantıda kalan bir başvuru (dangling reference) oluşturmaya çalışmanızdan veya mevcut ömürler arasında uyumsuzluk olmasından kaynaklanır. Bu tür durumlarda çözüm, `'static` ömrünü belirtmek değil, bu sorunları düzeltmektir.

---

## ⚙️ Genel Tür Parametreleri, Trait Sınırları ve Ömürler Birlikte (generic type parameters, trait bounds, and lifetimes together)

Şimdi de tek bir fonksiyonda genel tür parametrelerini (generic type parameters), trait sınırlarını (trait bounds) ve ömürleri (lifetimes) birlikte belirtmenin sözdizimine kısaca bakalım!

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
```

Bu fonksiyon, Liste 10-21’deki iki `string dilimi`nden (string slices) daha uzun olanını döndüren `longest` fonksiyonudur. Ancak şimdi `T` adında genel bir tür parametresine sahip ek bir `ann` parametresi vardır. Bu tür parametresi, `where` ifadesinde belirtildiği gibi `Display` trait’ini uygulayan herhangi bir tür olabilir. Bu ek parametre `{}` ile yazdırılacağı için `Display` trait sınırı gereklidir. Ömürler de bir tür genel olduğu için, `'a` ömür parametresinin bildirimi ile `T` genel tür parametresi aynı açılı ayraçlar listesinde, fonksiyon adından sonra yer alır.

## 📋 Özet (summary)

Bu bölümde birçok şey ele aldık! Artık genel tür parametreleri (generic type parameters), trait’ler ve trait sınırları (trait bounds) ile genel ömür parametrelerini (generic lifetime parameters) bildiğinize göre, tekrara düşmeden ve birçok farklı durumda çalışabilen kodlar yazmaya hazırsınız.

Genel tür parametreleri (generic type parameters), kodu farklı türlere uygulamanızı sağlar. Trait’ler ve trait sınırları (trait bounds), türler genel olsa bile kodun ihtiyaç duyduğu davranışlara sahip olmalarını garanti eder. Ömür açıklamalarını (lifetime annotations) kullanarak, bu esnek kodun sallantıda kalan başvurulara (dangling references) sahip olmamasını nasıl sağlayacağınızı öğrendiniz. Ve bütün bu analizler derleme zamanında (compile time) gerçekleşir, yani çalışma zamanı (runtime) performansını etkilemez!

İnanması zor olabilir ama bu bölümde ele aldığımız konular hakkında öğrenilecek çok daha fazla şey var: Bölüm 18’de, trait’leri kullanmanın başka bir yolu olan **trait nesneleri (trait objects)** anlatılmaktadır. Ayrıca, yalnızca çok ileri düzey senaryolarda ihtiyaç duyacağınız daha karmaşık ömür açıklaması durumları da vardır; bunlar için Rust Reference’a göz atmalısınız.

Ama sıradaki adımda, kodunuzun olması gerektiği gibi çalıştığından emin olmak için Rust’ta test yazmayı öğreneceksiniz.
