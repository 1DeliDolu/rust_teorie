## 📚 Sahiplik (ownership) Nedir?

Sahiplik (ownership), bir Rust programının belleği nasıl yönettiğini belirleyen kurallar bütünüdür. Tüm programlar, çalışırken bilgisayarın belleğini kullanma şeklini yönetmek zorundadır. Bazı diller, program çalışırken artık kullanılmayan belleği düzenli olarak temizleyen bir çöp toplayıcıya (garbage collection) sahiptir; diğer dillerde ise programcı belleği açıkça ayırmak ve serbest bırakmak zorundadır. Rust ise üçüncü bir yaklaşım kullanır: bellek, derleyicinin kontrol ettiği kurallar bütününe sahip bir sahiplik sistemi (ownership system) aracılığıyla yönetilir. Bu kurallardan herhangi biri ihlal edilirse program derlenmez. Sahipliğe ait özelliklerin hiçbiri programınız çalışırken onu yavaşlatmaz.

Sahiplik birçok programcı için yeni bir kavram olduğundan, buna alışmak biraz zaman alır. İyi haber şu ki, Rust ve sahiplik sistemi kuralları konusunda deneyim kazandıkça, güvenli ve verimli kod yazmak sizin için doğal olarak kolaylaşacaktır. Vazgeçmeyin!

Sahipliği anladığınızda, Rust’ı benzersiz kılan özellikleri anlamak için sağlam bir temele sahip olacaksınız. Bu bölümde, sahipliği öğrenmek için çok yaygın bir veri yapısı olan dizgiler (strings) üzerinden örneklerle çalışacağız.

## 🗂️ Yığın (stack) ve Yığın Alanı (heap)

Pek çok programlama dili, yığın (stack) ve yığın alanı (heap) hakkında çok sık düşünmenizi gerektirmez. Ancak Rust gibi bir sistem programlama dilinde, bir değerin yığında mı yoksa yığın alanında mı bulunduğu, dilin nasıl davrandığını ve neden belirli kararları vermeniz gerektiğini etkiler. Sahipliğin (ownership) bazı kısımları bu bölümün ilerleyen kısımlarında yığın ve yığın alanıyla ilişkili olarak açıklanacaktır, bu yüzden hazırlık olarak burada kısa bir açıklama bulunmaktadır.

Hem yığın hem de yığın alanı, çalışma zamanında (runtime) kodunuzun kullanabileceği bellek bölümleridir, ancak farklı şekillerde yapılandırılmışlardır. Yığın, aldığı değerleri sırayla depolar ve değerleri ters sırada kaldırır. Buna “son giren, ilk çıkar” (last in, first out) denir. Bunu tabak yığını gibi düşünebilirsiniz: yeni tabak eklediğinizde onları üstüne koyarsınız, bir tabağa ihtiyacınız olduğunda ise üstten alırsınız. Ortadan ya da alttan tabak almak pek işe yaramaz! Veriyi ekleme işlemine yığına itmek (pushing onto the stack), kaldırma işlemine ise yığından çekmek (popping off the stack) denir. Yığında saklanan tüm verilerin bilinen ve sabit bir boyutu olmalıdır. Derleme zamanında (compile time) boyutu bilinmeyen ya da değişebilecek veriler ise yığın alanında saklanmalıdır.

Yığın alanı (heap) daha az düzenlidir: veriyi yığın alanına koyduğunuzda, belirli miktarda boş yer istersiniz. Bellek ayırıcı (memory allocator), yığın alanında yeterince büyük boş bir alan bulur, onu kullanımda olarak işaretler ve size o konumun adresi olan bir işaretçi (pointer) döndürür. Bu işleme yığın alanına ayırma (allocating on the heap) denir ve bazen kısaca ayırma (allocating) olarak anılır (yığına değer eklemek ayırma sayılmaz). Yığın alanına işaretçi bilinen ve sabit bir boyuta sahip olduğundan, işaretçiyi yığında saklayabilirsiniz; ancak gerçek veriye erişmek istediğinizde işaretçiyi takip etmeniz gerekir. Bunu bir restoranda oturma yerinizin belirlenmesine benzetebilirsiniz: restorana girdiğinizde kaç kişi olduğunuzu söylersiniz, görevli size uygun boş bir masa bulur ve sizi oraya götürür. Eğer grubunuzdan biri geç gelirse, oturduğunuz masayı sorarak sizi bulabilir.

Yığına veri eklemek (pushing) yığın alanında ayırma yapmaktan (allocating) daha hızlıdır çünkü ayırıcı, yeni veriyi saklayacağı yeri aramak zorunda değildir; o yer her zaman yığının en üstüdür. Buna kıyasla, yığın alanında yer ayırmak daha çok iş gerektirir çünkü ayırıcı önce veriyi barındıracak kadar büyük bir alan bulmalı, ardından bir sonraki ayırma için kayıt işlemleri yapmalıdır.

Yığın alanındaki veriye erişmek genelde yığındaki veriye erişmekten daha yavaştır çünkü oraya ulaşmak için bir işaretçiyi takip etmeniz gerekir. Günümüz işlemcileri, bellekte daha az sıçrama yaptıklarında daha hızlıdır. Aynı benzetmeyle, bir garsonun restorandaki masalardan sipariş almasını düşünün: en verimli yöntem, bir masadaki tüm siparişleri alıp sonra diğer masaya geçmektir. A masasında bir sipariş, sonra B masasında bir sipariş, ardından tekrar A, sonra tekrar B şeklinde sipariş almak çok daha yavaş olurdu. Benzer şekilde, işlemci de genelde işini, birbirine yakın verilerle (yığındaki gibi) daha iyi yapar; birbirinden uzak verilerle (yığın alanındaki gibi) çalışmak daha yavaş olabilir.

Kodunuz bir fonksiyonu çağırdığında, fonksiyona geçirilen değerler (yığın alanındaki verilere ait işaretçiler de olabilir) ve fonksiyonun yerel değişkenleri yığına eklenir. Fonksiyon bittiğinde, bu değerler yığından kaldırılır.

Kodun hangi kısımlarının yığın alanındaki hangi verileri kullandığını takip etmek, yığın alanındaki yinelenen veri miktarını en aza indirmek ve kullanılmayan verileri temizleyerek yerin dolmasını engellemek, sahipliğin (ownership) çözdüğü problemlerdir. Sahipliği anladığınızda, yığın ve yığın alanı hakkında çok sık düşünmenize gerek kalmayacaktır. Ancak sahipliğin asıl amacının yığın alanındaki verileri yönetmek olduğunu bilmek, onun neden bu şekilde çalıştığını anlamanıza yardımcı olur.

## 📏 Sahiplik Kuralları (ownership rules)

Öncelikle sahiplik kurallarına bir göz atalım. Bu kuralları açıklayan örnekler üzerinde çalışırken aklınızda bulundurun:

* Rust’taki her değerin bir sahibi vardır.
* Aynı anda yalnızca bir sahibi olabilir.
* Sahibi kapsam (scope) dışına çıktığında, değer düşürülür (dropped).

## 📦 Değişken Kapsamı (variable scope)

Artık temel Rust sözdizimini geçtiğimize göre, örneklerde `fn main() {` kodunu dahil etmeyeceğiz. Eğer siz de takip ediyorsanız, aşağıdaki örnekleri manuel olarak `main` fonksiyonu içine yerleştirdiğinizden emin olun. Böylece örneklerimiz daha kısa olacak ve ayrıntılara odaklanabileceğiz.

Sahipliğin ilk örneği olarak, bazı değişkenlerin kapsamına (scope) bakalım. Kapsam, bir öğenin programda geçerli olduğu aralığı ifade eder. Aşağıdaki değişkeni ele alalım:

```rust
let s = "hello";
```

Değişken `s`, değerinin programın metnine sabitlenmiş olduğu bir dizge (string literal) ifade eder. Bu değişken, tanımlandığı noktadan itibaren geçerli olur ve mevcut kapsamın sonuna kadar geçerli kalır. Listeleme 4-1, `s` değişkeninin geçerli olduğu yerleri açıklayan yorumlar içeren bir programı göstermektedir.

```rust
{                      // s burada geçerli değil, çünkü henüz tanımlanmadı
    let s = "hello";   // s bu noktadan itibaren geçerlidir

    // s ile işlemler yap
}                      // bu kapsam bitti, s artık geçerli değil
```

**Listeleme 4-1: Bir değişken ve onun geçerli olduğu kapsam (scope)**

Başka bir deyişle, burada iki önemli zaman noktası vardır:

1. `s` kapsam içine girdiğinde geçerli hale gelir.
2. Kapsamdan çıkana kadar geçerliliğini korur.

Bu noktada, kapsamlar ile değişkenlerin ne zaman geçerli olduğu arasındaki ilişki, diğer programlama dillerindekine benzerdir. Şimdi bu anlayışı temel alarak `String` türünü tanıtacağız.

## 🔤 String Türü (String type)

Sahiplik kurallarını gösterebilmek için, 3. Bölümdeki “Veri Türleri” (Data Types) kısmında ele aldıklarımızdan daha karmaşık bir veri türüne ihtiyacımız var. Daha önce ele alınan türlerin boyutu bilinir, yığında (stack) saklanabilir ve kapsamları (scope) sona erdiğinde yığından çıkarılabilirler. Ayrıca başka bir kod parçasının aynı değeri farklı bir kapsamda kullanması gerektiğinde hızlı ve kolayca kopyalanarak bağımsız yeni bir örnek oluşturulabilir. Ancak biz, yığın alanında (heap) saklanan verilere bakmak ve Rust’ın bu verileri ne zaman temizleyeceğini nasıl bildiğini keşfetmek istiyoruz. Bunun için en iyi örnek `String` türüdür.

Biz burada `String` türünün sahiplikle (ownership) ilgili kısımlarına odaklanacağız. Bu özellikler, standart kütüphane tarafından sağlanan ya da sizin oluşturduğunuz diğer karmaşık veri türleri için de geçerlidir. `String` türünü daha ayrıntılı olarak 8. Bölümde ele alacağız.

Daha önce dizge sabitlerini (string literals) gördük; bunlarda dizge değeri programımıza sabitlenmiştir. Dizge sabitleri kullanışlıdır, ancak her durumda metin (text) kullanmak için uygun değildir. Bunun bir nedeni, değiştirilemez (immutable) olmalarıdır. Bir diğer nedeni ise, yazdığımız anda her dizge değerinin bilinememesidir. Örneğin, kullanıcıdan giriş almak ve onu saklamak istersek ne olur? İşte bu gibi durumlar için Rust, ikinci bir dizge türü olan `String` türünü sağlar. Bu tür, yığın alanına (heap) ayrılmış veriyi yönetir ve bu nedenle derleme zamanında bilinmeyen miktarda metni saklayabilir. Bir dizge sabitinden `String` oluşturmak için `from` fonksiyonunu kullanabilirsiniz:

```rust
let s = String::from("hello");
```

👉 Burada `::` (çift iki nokta) operatörü, `from` fonksiyonunu `String` türü altında ad alanına (namespace) yerleştirmemizi sağlar; yoksa `string_from` gibi farklı bir ad kullanmamız gerekirdi. Bu sözdizimini (syntax) daha ayrıntılı olarak 5. Bölümdeki “Yöntem Sözdizimi” (Method Syntax) kısmında ve 7. Bölümdeki “Modül Ağacında Bir Öğeye Başvurmak için Yollar” (Paths for Referring to an Item in the Module Tree) kısmında tartışacağız.

Bu tür dizgeler değiştirilebilir (mutable) olabilir:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() bir sabiti String sonuna ekler

println!("{s}"); // bu `hello, world!` yazdırır
```

👉 Buradaki fark nedir? Neden `String` değiştirilebilirken (mutable), sabit dizgeler değiştirilemez (immutable)? Fark, bu iki türün belleği nasıl ele almasında yatmaktadır.

## 🧾 Bellek ve Ayırma (memory and allocation)

Bir dizge sabiti (string literal) durumunda, içeriği derleme zamanında biliriz; bu nedenle metin doğrudan nihai çalıştırılabilir dosyaya gömülür. İşte bu yüzden dizge sabitleri hızlı ve verimlidir. Ancak bu özellikler yalnızca dizge sabitlerinin değiştirilemezliğinden (immutability) gelir. Ne yazık ki, boyutu derleme zamanında bilinmeyen ve program çalışırken değişebilecek her metin parçası için belleğe gömülü bir blok koyamayız.

`String` türünde ise, değiştirilebilir (mutable) ve büyüyebilir bir metin parçasını desteklemek için, içeriği saklamak üzere derleme zamanında bilinmeyen bir miktarda belleği yığın alanına (heap) ayırmamız gerekir. Bu da şu anlama gelir:

* Bellek, çalışma zamanında bellek ayırıcıdan (memory allocator) istenmelidir.
* `String` işimiz bittiğinde bu belleği ayırıcıya geri vermenin bir yoluna ihtiyaç duyarız.

İlk kısmı biz yaparız: `String::from` çağırdığımızda, onun implementasyonu ihtiyaç duyduğu belleği ister. Bu, neredeyse tüm programlama dillerinde ortaktır.

Ancak ikinci kısım farklıdır. Çöp toplayıcılı (garbage collector - GC) dillerde, GC kullanılmayan belleği takip eder ve temizler; bizim bu konuda düşünmemize gerek yoktur. Çöp toplayıcısı olmayan dillerin çoğunda ise, belleğin artık kullanılmadığını bizim belirlememiz ve onu serbest bırakacak (free) kodu açıkça çağırmamız gerekir, tıpkı onu talep ederken yaptığımız gibi. Bunu doğru yapmak tarihsel olarak zor bir programlama problemidir:

* Eğer unutursak, belleği boşa harcarız.
* Eğer çok erken yaparsak, geçersiz (invalid) bir değişkenimiz olur.
* Eğer iki kez yaparsak, bu da bir hatadır.

Yani tam olarak bir `allocate` işlemini bir `free` işlemiyle eşleştirmemiz gerekir.

Rust ise farklı bir yol izler: sahip olan değişken kapsam (scope) dışına çıktığında bellek otomatik olarak geri verilir. İşte Listeleme 4-1’in, dizge sabiti yerine `String` kullanan bir sürümü:

```rust
{
    let s = String::from("hello"); // s bu noktadan itibaren geçerlidir

    // s ile işlemler yap
}                                  // bu kapsam bitti ve s artık geçerli değil
```

👉 Burada `String` için ayrılan belleği ayırıcıya geri verebileceğimiz doğal bir nokta vardır: `s` kapsam dışına çıktığında. Bir değişken kapsam dışına çıktığında Rust bizim için özel bir fonksiyon çağırır. Bu fonksiyon `drop` olarak adlandırılır ve `String` türünün yazarı belleği geri verme kodunu buraya koyar. Rust, kapanan süslü parantezde `drop` fonksiyonunu otomatik olarak çağırır.

Not: C++ dilinde, bir öğenin ömrünün sonunda kaynakların serbest bırakılması desenine bazen **Resource Acquisition Is Initialization (RAII)** denir. Daha önce RAII desenini kullandıysanız, Rust’taki `drop` fonksiyonu size tanıdık gelecektir.

Bu desen, Rust kodunun yazılma şeklini derinden etkiler. Şu anda basit görünebilir, ancak yığın alanında ayırdığımız veriyi birden fazla değişkenin kullanmasını istediğimiz daha karmaşık durumlarda, kodun davranışı beklenmedik olabilir. Şimdi bu durumların bazılarını inceleyelim.


## 🔄 Değişkenler ve Verilerin Taşınarak (move) Etkileşimi

Rust’ta birden fazla değişken aynı veriyle farklı şekillerde etkileşebilir. Listeleme 4-2’de bir tamsayı (integer) üzerinden örneğe bakalım:

```rust
let x = 5;
let y = x;
```

**Listeleme 4-2: x değişkeninin tamsayı değerini y’ye atamak**

Muhtemelen bunun ne yaptığını tahmin edebiliriz: “5 değerini `x`’e bağla; sonra `x`’teki değerin bir kopyasını oluştur ve `y`’ye bağla.” Artık elimizde `x` ve `y` olmak üzere iki değişken var ve her ikisi de 5’e eşit. Bu gerçekten de böyledir, çünkü tamsayılar bilinen ve sabit boyutlu basit değerlerdir ve bu iki `5` değeri yığına (stack) eklenmiştir.

Şimdi `String` versiyonuna bakalım:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Bu çok benzer görünüyor, dolayısıyla aynı şekilde çalışacağını varsayabiliriz: yani ikinci satır, `s1`’deki değerin bir kopyasını oluşturup `s2`’ye bağlayacak gibi. Ancak tam olarak böyle olmaz.

Bir `String`, bellekte üç parçadan oluşur: dizgenin içeriğini tutan belleğe bir işaretçi (pointer), uzunluk (length) ve kapasite (capacity). Bu üçlü yığında saklanır. Dizgenin içeriği ise yığın alanında (heap) tutulur.

### 📊 Bellek Temsilleri

* **Şekil 4-1:** `s1` değişkeninin bellekteki temsili — yığında işaretçi, uzunluk (5), kapasite (5); yığın alanında ise `"hello"` baytları.
* **Şekil 4-2:** `s1` `s2`’ye atandığında, işaretçi, uzunluk ve kapasite yığında kopyalanır; ancak işaretçinin gösterdiği yığın alanındaki veri kopyalanmaz. Yani hem `s1` hem de `s2` aynı yığın alanını işaret eder.
* **Şekil 4-3:** Eğer Rust yığın alanındaki veriyi de kopyalasaydı, her değişkenin kendi `"hello"` kopyası olurdu. Ancak bu, büyük verilerde performans açısından pahalı olurdu.
* **Şekil 4-4:** `s1`, `s2`’ye taşındıktan sonra geçersiz (invalid) hale gelir. Artık yalnızca `s2` geçerlidir ve bellek serbest bırakıldığında yalnızca `s2` bunu yapar.

### ⚠️ Çifte Serbest Bırakma Sorunu (double free)

Daha önce belirtildiği gibi, bir değişken kapsam dışına çıktığında Rust otomatik olarak `drop` fonksiyonunu çağırır ve belleği temizler. Ancak Şekil 4-2’de hem `s1` hem de `s2` aynı yığın alanını işaret eder. Eğer ikisi de kapsam dışına çıktığında aynı belleği serbest bırakmaya çalışırsa, bu **çifte serbest bırakma hatası (double free error)** olur. Bu tür hatalar bellek bozulmasına (memory corruption) ve güvenlik açıklarına yol açabilir.

Bunu önlemek için Rust, `let s2 = s1;` satırından sonra `s1`’i artık geçerli saymaz. Böylece `s1` kapsam dışına çıktığında serbest bırakılacak bir şey yoktur. Bu nedenle `s1`’i kullanmaya çalışırsanız, program derlenmez:

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}, world!"); // Bu kod derlenmez!
```

Hata mesajı şuna benzer olacaktır:

```
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:15
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1}, world!");
  |               ^^^^ value borrowed here after move
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++
```

### 🔀 Move Kavramı

Diğer dillerde duymuş olabileceğiniz **yüzeysel kopya (shallow copy)** ve **derin kopya (deep copy)** terimleriyle karşılaştırırsak: işaretçi, uzunluk ve kapasitenin kopyalanıp verinin kopyalanmaması yüzeysel kopya gibi görünür. Ancak Rust, ilk değişkeni (`s1`) geçersiz hale getirdiği için buna yüzeysel kopya denmez, bunun yerine **move (taşıma)** denir. Bu örnekte `s1`, `s2`’ye taşınmıştır.

Bu tasarım sayesinde:

* Yalnızca `s2` geçerli olur ve kapsam dışına çıktığında belleği tek başına serbest bırakır.
* Rust, verilerinizin **derin kopyalarını (deep copies)** asla otomatik olarak oluşturmaz. Bu nedenle, Rust’taki tüm otomatik kopyalamalar çalışma zamanı performansı açısından ucuz (inexpensive) kabul edilebilir.

## 📐 Kapsam ve Atama (scope and assignment)

Kapsam (scope), sahiplik (ownership) ve belleğin `drop` fonksiyonu ile serbest bırakılması arasındaki ilişki ters yönde de geçerlidir. Bir değişkene tamamen yeni bir değer atadığınızda, Rust eski değerin belleğini hemen serbest bırakır. Örneğin:

```rust
let mut s = String::from("hello");
s = String::from("ahoy");

println!("{s}, world!");
```

Başlangıçta `s` değişkenini `"hello"` değerine sahip bir `String` ile tanımlarız. Ardından hemen `"ahoy"` değerine sahip yeni bir `String` oluşturup `s`’ye atarız. Bu noktada, yığın alanındaki (heap) orijinal değer artık hiçbir şey tarafından işaretlenmez.

👉 İlk dizge böylece hemen kapsam dışına çıkar. Rust, onun üzerinde `drop` fonksiyonunu çalıştırır ve bellek hemen serbest bırakılır. En sonda yazdırılan değer `"ahoy, world!"` olur.

---

## 🧬 Değişkenler ve Verilerin Clone ile Etkileşimi

Eğer `String` türünün yalnızca yığında (stack) saklanan kısmını değil, yığın alanındaki veriyi de kopyalamak istersek, `clone` adında yaygın bir yöntemi kullanabiliriz:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

👉 Bu, sorunsuz çalışır ve açıkça Şekil 4-3’teki davranışı üretir: yığın alanındaki veri de kopyalanır.

`clone` çağrısı gördüğünüzde, bunun arka planda bazı ek kod çalıştırdığını ve performans açısından pahalı olabileceğini bilirsiniz. Bu, farklı bir şey olduğuna dair görsel bir göstergedir.

---

## 🗂️ Yalnızca Yığında Saklanan Veriler: Copy

Şimdiye kadar bahsetmediğimiz bir durum daha var. Listeleme 4-2’de gördüğümüz tamsayı örneği şu şekilde geçerlidir:

```rust
let x = 5;
let y = x;

println!("x = {x}, y = {y}");
```

Bu, az önce öğrendiklerimizle çelişiyor gibi görünebilir: `clone` çağrısı yok ama `x` hâlâ geçerli ve `y`’ye taşınmadı.

Bunun nedeni, tamsayı gibi derleme zamanında boyutu bilinen türlerin tamamen yığında saklanmasıdır. Gerçek değeri kopyalamak hızlı ve ucuzdur. Dolayısıyla `y` oluşturulduktan sonra `x`’in geçerli olmasını engellemek için bir sebep yoktur. Başka bir deyişle, burada derin kopya (deep copy) ile yüzeysel kopya (shallow copy) arasında fark yoktur. Bu nedenle `clone` çağrısı yapmak da farklı bir şey yapmayacak, bu yüzden gerek yoktur.

Rust’ta bu türler için özel bir ek açıklama vardır: `Copy` özelliği (trait). Yığında saklanan türler (`i32`, `bool`, `char` vb.) `Copy` özelliğini uygular. Bir tür `Copy` özelliğini uyguluyorsa, bu türden değişkenler taşınmaz, aksine kolayca kopyalanır ve atamadan sonra hâlâ geçerlidir.

Rust, `Drop` özelliğini (trait) uygulamış bir türde `Copy` ek açıklamasına izin vermez. Çünkü tür kapsam dışına çıktığında özel bir şey yapılması gerekiyorsa, `Copy` uygulanması çelişki doğurur.

### `Copy` özelliğini uygulayan bazı türler:

* Tüm tamsayı türleri (`u32` gibi)
* Mantıksal tür `bool` (`true` ve `false`)
* Tüm kayan nokta türleri (`f64` gibi)
* Karakter türü `char`
* Yalnızca `Copy` özelliğini uygulayan türlerden oluşan demetler (tuples). Örneğin, `(i32, i32)` `Copy` uygular, ancak `(i32, String)` uygulamaz.

---

## 🛠️ Sahiplik ve Fonksiyonlar (ownership and functions)

Bir değeri fonksiyona geçirmek, tıpkı bir değişkene atama yapmak gibidir: değer ya taşınır (move) ya da kopyalanır (copy). Listeleme 4-3’te bununla ilgili açıklamalı bir örnek verilmiştir:

```rust
fn main() {
    let s = String::from("hello");  // s kapsam içine girer

    takes_ownership(s);             // s’nin değeri fonksiyona taşınır
                                    // artık burada geçerli değildir

    let x = 5;                      // x kapsam içine girer

    makes_copy(x);                  // i32 Copy özelliğini uyguladığı için
                                    // x fonksiyona taşınmaz,
                                    // bu yüzden x’i sonradan da kullanabiliriz.
} // Burada önce x, sonra s kapsam dışına çıkar. Ancak s taşındığı için
  // onun için özel bir şey yapılmaz.

fn takes_ownership(some_string: String) { // some_string kapsam içine girer
    println!("{some_string}");
} // Burada some_string kapsam dışına çıkar ve drop çağrılır.
  // Yığın alanındaki bellek serbest bırakılır.

fn makes_copy(some_integer: i32) { // some_integer kapsam içine girer
    println!("{some_integer}");
} // Burada some_integer kapsam dışına çıkar. Özel bir şey olmaz.
```

👉 **Listeleme 4-3: Sahiplik ve kapsam ile açıklamalı fonksiyonlar**

`takes_ownership` fonksiyonundan sonra `s`’yi kullanmaya çalışırsanız, Rust derleme hatası verir. Bu statik kontroller bizi hatalardan korur.

---

## ↩️ Dönüş Değerleri ve Kapsam (return values and scope)

Dönüş değerleri de sahipliği transfer edebilir. Listeleme 4-4’te bununla ilgili bir örnek vardır:

```rust
fn main() {
    let s1 = gives_ownership();        // gives_ownership dönüş değerini s1’e taşır

    let s2 = String::from("hello");    // s2 kapsam içine girer

    let s3 = takes_and_gives_back(s2); // s2 taşınır ve dönüş değeri s3’e taşınır
} // Burada s3 kapsam dışına çıkar ve düşürülür. s2 taşındığı için özel bir şey
  // olmaz. s1 kapsam dışına çıkar ve düşürülür.

fn gives_ownership() -> String {
    let some_string = String::from("yours"); 
    some_string // dönüş yapılır ve sahiplik çağıran fonksiyona taşınır
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // dönüş yapılır ve sahiplik çağıran fonksiyona taşınır
}
```

👉 **Listeleme 4-4: Dönüş değerleriyle sahipliğin aktarılması**

Kural hep aynıdır: bir değeri başka bir değişkene atamak sahipliği taşır. Yığın alanında veri içeren bir değişken kapsam dışına çıktığında, eğer sahiplik başka bir değişkene aktarılmadıysa, `drop` çağrılır ve bellek serbest bırakılır.

Ancak sahipliği her fonksiyona geçirip sonra geri döndürmek zahmetli olur. Bir fonksiyonun bir değeri kullanmasını, ama sahipliğini almamasını isteriz. Bunun için Rust bize **referanslar (references)** adında bir özellik sağlar.

Bunu açıklamadan önce, Rust’ın birden fazla değeri tuple kullanarak döndürebileceğini görelim (Listeleme 4-5):

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); 
    (s, length)
}
```

👉 **Listeleme 4-5: Parametrelerin sahipliğini geri döndürmek**

Ancak bu yöntem, oldukça sık karşılaşacağımız bir kavram için fazla zahmetlidir. Neyse ki Rust, sahipliği aktarmadan değeri kullanmamızı sağlayan **referanslar (references)** özelliğine sahiptir.
