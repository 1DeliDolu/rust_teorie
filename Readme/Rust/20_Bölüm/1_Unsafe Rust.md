## ⚠️ Unsafe Rust

Şimdiye kadar ele aldığımız tüm kodlarda, Rust’ın bellek güvenliği garantileri derleme zamanında uygulanıyordu. Ancak Rust’ın içinde, bu bellek güvenliği garantilerini uygulamayan ikinci bir dil vardır: buna `unsafe Rust` denir ve normal Rust gibi çalışır, fakat bize ek süper güçler sağlar.

`Unsafe Rust`, doğası gereği statik analiz muhafazakâr olduğundan vardır. Derleyici, kodun garantileri koruyup korumadığını belirlemeye çalışırken, bazı geçerli programları reddetmesi, geçersiz programları kabul etmesinden daha iyidir. Kod aslında doğru olsa bile, Rust derleyicisi yeterli bilgiye sahip değilse onu reddeder. Bu durumlarda `unsafe` kod kullanarak derleyiciye şu mesajı verebilirsiniz: “Bana güven, ne yaptığımı biliyorum.” Ancak dikkat edin: `unsafe` Rust’ı kendi sorumluluğunuzda kullanırsınız; yanlış kullanırsanız, null işaretçi (null pointer) dereferansı gibi bellek güvenliği sorunları ortaya çıkabilir.

Rust’ın `unsafe` yanına sahip olmasının bir başka nedeni, altta yatan bilgisayar donanımının doğası gereği güvensiz olmasıdır. Eğer Rust size güvensiz işlemler yapma izni vermeseydi, bazı görevleri yerine getiremezdiniz. Rust, işletim sistemiyle doğrudan etkileşim veya kendi işletim sisteminizi yazma gibi düşük seviyeli sistem programlamasına izin vermelidir. Düşük seviyeli sistem programlamasıyla çalışmak, dilin hedeflerinden biridir. Şimdi `unsafe Rust` ile neler yapabileceğimize ve bunu nasıl yapacağımıza bakalım.

---

## 🦸 Unsafe Süper Güçler (unsafe superpowers)

`Unsafe Rust`a geçmek için `unsafe` anahtar sözcüğünü kullanır ve `unsafe` kodu barındıran yeni bir blok başlatırsınız. `Unsafe Rust`ta güvenli Rust’ta yapamayacağınız beş işlem yapabilirsiniz; bunlara `unsafe` süper güçler denir. Bunlar şunlardır:

* Ham bir işaretçiyi (raw pointer) dereferans etmek
* Bir `unsafe` fonksiyon veya metodu çağırmak
* Değiştirilebilir bir statik değişkene (`mutable static variable`) erişmek veya onu değiştirmek
* Bir `unsafe trait` uygulamak
* Bir `union`un alanlarına erişmek

`Unsafe` sözcüğünün, borç denetleyicisini (borrow checker) kapatmadığını veya Rust’ın diğer güvenlik kontrollerini devre dışı bırakmadığını anlamak önemlidir: eğer `unsafe` kod içinde bir referans kullanırsanız, bu hâlâ denetlenecektir. `Unsafe` anahtar sözcüğü sadece bu beş özelliğe erişim sağlar ve derleyici bu özellikler için bellek güvenliği kontrolü yapmaz. Yine de bir `unsafe` blok içinde belirli bir güvenlik düzeyi elde edersiniz.

Ayrıca `unsafe`, blok içindeki kodun mutlaka tehlikeli olduğu veya kesinlikle bellek güvenliği sorunları çıkaracağı anlamına gelmez: amaç, programcı olarak `unsafe` blok içindeki kodun belleğe geçerli bir şekilde erişmesini sağlamanızdır.

İnsanlar hata yapabilir, ancak bu beş `unsafe` işlemin `unsafe` ile işaretlenmiş bloklarda bulunması gerektiğinden, bellek güvenliğiyle ilgili hataların yalnızca bu bloklarda olduğunu bilirsiniz. `Unsafe` blokları küçük tutun; bellek hatalarını araştırırken size faydası olacaktır.

`Unsafe` kodu mümkün olduğunca izole etmek için en iyisi onu güvenli bir soyutlama (safe abstraction) içine almak ve güvenli bir API sunmaktır. Bu konuyu ilerleyen kısımda, `unsafe` fonksiyonları ve metotları incelerken tartışacağız. Standart kütüphanenin bazı kısımları, denetlenmiş `unsafe` kod üzerine inşa edilmiş güvenli soyutlamalar olarak uygulanmıştır. `Unsafe` kodu güvenli bir soyutlama ile sarmak, `unsafe` kullanımının sizin veya kullanıcılarınızın her yerde karşılaşabileceği alanlara sızmasını engeller; çünkü güvenli soyutlama kullanmak güvenlidir.

Şimdi, beş `unsafe` süper gücü sırayla inceleyelim. Ayrıca `unsafe` kod için güvenli arayüz sağlayan bazı soyutlamalara da bakacağız.

---

## 🖇️ Ham İşaretçiyi Dereferans Etmek (dereferencing a raw pointer)

4. Bölümdeki “Asılı Referanslar” (dangling references) kısmında, derleyicinin referansların her zaman geçerli olmasını sağladığından bahsetmiştik. `Unsafe Rust`, referanslara benzer iki yeni türe sahiptir: ham işaretçiler (raw pointers). Referanslarda olduğu gibi, ham işaretçiler değiştirilemez (`*const T`) veya değiştirilebilir (`*mut T`) olabilir. Yıldız (`*`) burada dereferans operatörü değildir; tür adının bir parçasıdır. Ham işaretçiler bağlamında `immutable`, işaretçinin dereferans edildikten sonra doğrudan yeniden atanamayacağı anlamına gelir.

Referanslar ve akıllı işaretçilerden (`smart pointers`) farklı olarak ham işaretçiler:

* Hem değiştirilemez hem değiştirilebilir işaretçilere veya aynı konuma birden fazla değiştirilebilir işaretçiye sahip olarak ödünç alma kurallarını yok sayabilir.
* Geçerli belleği işaret edeceklerinin garantisi yoktur.
* Null olabilirler.
* Otomatik temizleme işlemi uygulamazlar.

Rust’ın bu garantilerini uygulamaktan vazgeçerek, daha yüksek performans veya Rust garantilerinin geçerli olmadığı başka bir dil veya donanımla arayüz kurma yeteneği kazanabilirsiniz.

Aşağıda, değiştirilemez ve değiştirilebilir bir ham işaretçi oluşturma örneği verilmiştir:

```rust
let mut num = 5;

let r1 = &raw const num;
let r2 = &raw mut num;
```

👉 Bu kodda `raw borrow` operatörleri kullanılarak ham işaretçiler oluşturulur.

Bu kodda `unsafe` sözcüğünü kullanmadığımıza dikkat edin. Güvenli kod içinde ham işaretçiler oluşturabiliriz; ancak bunları `unsafe` blok dışında dereferans edemeyiz.

`&raw const num`, `*const i32` türünde değiştirilemez bir ham işaretçi oluşturur; `&raw mut num`, `*mut i32` türünde değiştirilebilir bir ham işaretçi oluşturur. Bunları doğrudan yerel bir değişkenden türettiğimiz için geçerli olduklarını biliyoruz, ancak bu durum tüm ham işaretçiler için geçerli değildir.

Aşağıdaki örnekte `as` ile tür dönüştürme (cast) kullanarak belirsiz bir bellek adresine ham işaretçi oluşturuyoruz:

```rust
let address = 0x012345usize;
let r = address as *const i32;
```

👉 Bu kod, bellek üzerinde rastgele bir adrese işaretçi oluşturur. Bu adres geçerli olmayabilir ve program bellek hatasıyla sonlanabilir.

Belleği okumak için bu işaretçiyi dereferans etmek gerektiğinde `unsafe` blok zorunludur:

```rust
let mut num = 5;

let r1 = &raw const num;
let r2 = &raw mut num;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

👉 Bu örnekte ham işaretçiler `unsafe` blok içinde dereferans edilmiştir.

Bu tür işaretçilerle çalışırken dikkatli olun: aynı bellek konumuna hem `*const i32` hem `*mut i32` işaretçileri oluşturabilir ve değiştirilebilir işaretçi üzerinden veriyi değiştirebilirsiniz. Bu durum veri yarışına (data race) yol açabilir.

Tüm bu tehlikelere rağmen, ham işaretçiler neden kullanılır? En önemli kullanım senaryolarından biri C koduyla arayüz oluşturmak, bir diğeri ise borç denetleyicisinin anlamadığı güvenli soyutlamalar geliştirmektir. Bir sonraki bölümde `unsafe` fonksiyonları ele alacağız ve `unsafe` kod kullanan güvenli bir soyutlama örneğine bakacağız.

## 📞 Unsafe Bir Fonksiyon veya Metodu Çağırmak (calling an unsafe function or method)

`Unsafe` blok içinde yapabileceğiniz ikinci işlem, `unsafe` fonksiyonları çağırmaktır. `Unsafe` fonksiyonlar ve metotlar, normal fonksiyon ve metotlarla aynı görünüme sahiptir; ancak tanımın başında fazladan bir `unsafe` bulunur. Bu bağlamda `unsafe` anahtar sözcüğü, fonksiyon çağrıldığında karşılanması gereken gereklilikler olduğunu belirtir; çünkü Rust bu gerekliliklerin yerine getirildiğini garanti edemez. Bir `unsafe` fonksiyonu `unsafe` blok içinde çağırarak, fonksiyonun belgelerini okuduğumuzu, onu doğru şekilde kullanmayı anladığımızı ve fonksiyonun koşullarını yerine getirdiğimizi kabul etmiş oluruz.

Aşağıda, gövdesinde hiçbir şey yapmayan `dangerous` adlı bir `unsafe` fonksiyon örneği verilmiştir:

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

👉 `dangerous` fonksiyonu, yalnızca `unsafe` blok içinde çağrılabilir.

Eğer `unsafe` blok olmadan `dangerous` fonksiyonunu çağırmaya çalışırsak hata alırız:

```
$ cargo run
   Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
error[E0133]: call to unsafe function `dangerous` is unsafe and requires unsafe block
 --> src/main.rs:4:5
  |
4 |     dangerous();
  |     ^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior
```

👉 Bu hata, `unsafe` fonksiyonların yalnızca `unsafe` blok içinde çağrılabileceğini belirtir.

Bir `unsafe` fonksiyonun gövdesinde `unsafe` işlemler yapmak gerektiğinde, tıpkı normal bir fonksiyon içinde olduğu gibi yine `unsafe` blok kullanmak zorunludur. Bu, `unsafe` blokların mümkün olduğunca küçük tutulmasına yardımcı olur.

---

## 🛡️ Unsafe Kodu Güvenli Bir Soyutlama ile Sarmak (creating a safe abstraction over unsafe code)

Bir fonksiyonun içinde `unsafe` kod bulunması, tüm fonksiyonun `unsafe` olarak işaretlenmesi gerektiği anlamına gelmez. Aksine, `unsafe` kodu güvenli bir fonksiyonla sarmak yaygın bir soyutlamadır. Bunun örneği, standart kütüphanede yer alan `split_at_mut` fonksiyonudur. Bu fonksiyon, bir slice’ı (dilim) verilen indeks noktasından iki parçaya ayırır. Aşağıda, `split_at_mut` fonksiyonunun nasıl kullanılacağını görebilirsiniz:

```rust
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

👉 Bu örnek, `split_at_mut` fonksiyonunun güvenli kullanımını gösterir.

Bu fonksiyonu yalnızca güvenli Rust kullanarak yazmaya çalışırsak, kod derlenmez. Aşağıdaki örnek (`Listing 20-5`), başarısız bir denemeyi gösterir:

```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}
```

👉 Bu kod derlenmez; çünkü Rust aynı slice üzerinde iki kere değiştirilebilir (mutable) borçlanmaya izin vermez.

Derleyicinin hata mesajı şunu söyler:

```
error[E0499]: cannot borrow `*values` as mutable more than once at a time
```

Bu durumda `unsafe` kod kullanmamız gerekir. Aşağıda `unsafe` blok, ham işaretçi (`raw pointer`) ve `unsafe` fonksiyon çağrılarıyla çalışan bir çözüm verilmiştir:

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

👉 Burada `slice::from_raw_parts_mut` ve `ptr.add` fonksiyonları `unsafe` olduğundan `unsafe` blok içinde kullanılmalıdır.

* `as_mut_ptr` metodu, slice’ın ham işaretçisini döndürür (`*mut i32`).
* `slice::from_raw_parts_mut`, bir ham işaretçiden ve uzunluktan slice oluşturur.
* `ptr.add(mid)`, işaretçiyi belirtilen offset kadar ilerletir.

Bu kodda yaptığımız doğrulama (`assert!(mid <= len)`), ham işaretçilerin slice’ın geçerli belleğini işaret ettiğini garanti eder. Bu nedenle `unsafe` kod güvenli bir soyutlamaya dönüştürülmüştür.

Bu fonksiyon `unsafe` olarak işaretlenmek zorunda değildir ve güvenli Rust kodundan çağrılabilir.

---

## 💥 Yanlış Kullanım Örneği

Aşağıdaki örnekte (`Listing 20-7`), geçersiz bir bellek adresinden slice oluşturulmaktadır:

```rust
use std::slice;

let address = 0x01234usize;
let r = address as *mut i32;

let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
```

👉 Bu kod, belirsiz bir bellek alanını 10.000 öğelik bir slice gibi kullanmaya çalışır. Bu, **tanımsız davranış**a yol açar ve programın çökmesine sebep olabilir.
## 🌐 `extern` Fonksiyonları Kullanarak Harici Kodu Çağırmak (using extern functions to call external code)

Bazen Rust kodunuzun başka bir dilde yazılmış kodla etkileşime girmesi gerekebilir. Bunun için Rust, **yabancı fonksiyon arayüzü** (Foreign Function Interface – FFI) oluşturmayı ve kullanmayı kolaylaştıran `extern` anahtar sözcüğüne sahiptir. FFI, bir programlama dilinin fonksiyonları tanımlamasına ve farklı (yabancı) bir programlama dilinin bu fonksiyonları çağırabilmesine olanak tanır.

Aşağıdaki örnekte (`Listing 20-8`), C standart kütüphanesindeki `abs` fonksiyonunu Rust’a entegre etme gösterilmektedir. `extern` blokları genellikle `unsafe` olarak işaretlenmelidir, çünkü diğer diller Rust’ın kurallarını ve garantilerini uygulamaz, Rust da bunları denetleyemez. Bu nedenle sorumluluk programcıya düşer.

**Filename: src/main.rs**

```rust
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

👉 Bu kodda, `abs` fonksiyonu C dilindeki ABI’ye göre `extern` blokta tanımlanmış ve `unsafe` blok içinde çağrılmıştır.

`unsafe extern "C"` bloğu içinde, çağırmak istediğimiz harici fonksiyonların adlarını ve imzalarını listeleriz. Buradaki `"C"` kısmı, harici fonksiyonun hangi **uygulama ikili arayüzünü** (ABI – Application Binary Interface) kullandığını belirtir. ABI, fonksiyonun assembly seviyesinde nasıl çağrılacağını tanımlar. En yaygın ABI `"C"` olup, C programlama dilinin ABI’sini takip eder. Rust’ın desteklediği tüm ABI’ler hakkında bilgi Rust Reference’ta bulunabilir.

---

## ✅ `unsafe extern` Blok İçinde Güvenli Fonksiyon Belirlemek

`unsafe extern` bloğu içindeki tüm öğeler örtük olarak `unsafe` kabul edilir. Ancak bazı FFI fonksiyonları çağrılmak için güvenlidir. Örneğin, C standart kütüphanesindeki `abs` fonksiyonu bellek güvenliği açısından bir sorun oluşturmaz ve her `i32` değeriyle çağrılabilir. Bu gibi durumlarda, `safe` anahtar sözcüğüyle bu fonksiyonun güvenli olduğunu belirtebiliriz. Böylece artık onu çağırmak için `unsafe` blok gerekmez.

**Filename: src/main.rs**

```rust
unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

fn main() {
    println!("Absolute value of -3 according to C: {}", abs(-3));
}
```

👉 Bu örnekte, `abs` fonksiyonu `safe` olarak işaretlendiği için güvenli Rust kodu içinden doğrudan çağrılabilir.

⚠️ Ancak unutmayın: Bir fonksiyonu `safe` olarak işaretlemek onu otomatikman güvenli yapmaz! Bu, Rust’a verdiğiniz bir **söz**dür; bu sözün yerine getirildiğinden emin olmak hâlâ sizin sorumluluğunuzdadır.

---

## 🔄 Rust Fonksiyonlarını Başka Dillerden Çağırmak (calling Rust functions from other languages)

`extern` aynı zamanda başka dillerin Rust fonksiyonlarını çağırmasına da izin verir. Bunun için, ilgili fonksiyonun başına `extern` anahtar sözcüğünü ve kullanılacak ABI’yi ekleriz. Ayrıca Rust derleyicisine, fonksiyonun adını değiştirmemesi için `#[unsafe(no_mangle)]` özniteliğini eklememiz gerekir.

**Mangling**, derleyicinin verdiğimiz fonksiyon adını, derleme sürecinde daha fazla bilgi içerecek şekilde değiştirmesi işlemidir. Ancak farklı dillerdeki derleyiciler farklı kurallarla mangling yapar. Rust fonksiyonunun başka dillerden erişilebilir olması için ad mangling işlemini devre dışı bırakmalıyız. Bu ise `unsafe` kabul edilir; çünkü mangling olmadan farklı kütüphanelerde isim çakışması olabilir. Bu yüzden, seçtiğimiz ismin güvenli olduğundan emin olmak bizim sorumluluğumuzdadır.

Aşağıdaki örnekte, `call_from_c` fonksiyonunu C kodundan erişilebilir hale getiriyoruz. Bu fonksiyon, derlendikten sonra paylaşımlı kütüphane olarak bağlanabilir:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

👉 Bu kullanımda `unsafe`, yalnızca öznitelikte (`#[unsafe(no_mangle)]`) gereklidir; `extern` blokta ayrıca `unsafe` gerekmez.

## 📊 Değiştirilebilir Statik Bir Değişkene Erişim veya Onu Değiştirme (accessing or modifying a mutable static variable)

Bu kitapta şimdiye kadar küresel değişkenlerden (global variables) bahsetmedik; Rust bunları destekler, ancak sahiplik kurallarıyla çelişebileceği için sorunlu olabilir. İki iş parçacığının (thread) aynı değiştirilebilir (mutable) küresel değişkene erişmesi, veri yarışına (data race) yol açabilir.

Rust’ta küresel değişkenlere **statik değişkenler** (`static variables`) denir. Aşağıdaki örnek (`Listing 20-10`), bir `&str` değeriyle statik değişken tanımı ve kullanımı göstermektedir:

**Filename: src/main.rs**

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {HELLO_WORLD}");
}
```

👉 Bu örnekte `HELLO_WORLD` adında değiştirilemez bir statik değişken tanımlanmış ve kullanılmıştır.

Statik değişkenler, 3. Bölümde incelediğimiz **sabitlere** (constants) benzer. İsimleri genellikle `SCREAMING_SNAKE_CASE` ile yazılır. Statik değişkenler yalnızca `'static` ömrüne (lifetime) sahip referansları saklayabilir. Bu ömrü derleyici çıkarabildiği için açıkça belirtmemiz gerekmez. Değiştirilemez (immutable) statik değişkene erişim güvenlidir.

Statik değişkenlerle sabitler arasındaki ince bir fark şudur: statik değişkenler bellekte sabit bir adrese sahiptir, her kullanımda aynı veriye erişilir. Sabitler ise kullanıldıkça verilerini çoğaltabilir. Ayrıca statik değişkenler **mutable** olabilir. Ancak değiştirilebilir statik değişkenlere erişmek veya onları değiştirmek `unsafe` kabul edilir.

Aşağıda (`Listing 20-11`) `COUNTER` adında değiştirilebilir bir statik değişkenin tanımı, erişimi ve değiştirilmesi gösterilmiştir:

**Filename: src/main.rs**

```rust
static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
```

👉 Bu örnekte, `COUNTER` değiştirilebilir statik değişkenine yalnızca `unsafe` blok içinde erişilebilir.

* `mut` anahtar sözcüğü değiştirilebilirliği belirtir.
* `COUNTER`a erişen veya onu değiştiren her kod `unsafe` blok içinde olmalıdır.
* Bu örnek tek iş parçacıklı olduğu için güvenlidir ve çıktısı `COUNTER: 3` olur.

Çoklu iş parçacığında ise bu durum **tanımsız davranış** doğurur. Bu yüzden fonksiyonun tamamı `unsafe` olarak işaretlenmiş ve güvenlik koşulu bir **SAFETY yorumu** ile belgelenmiştir.

Rust’ta alışkanlık olarak:

* `unsafe` fonksiyon yazarken **SAFETY** ile başlayan yorum eklemek, çağıranın hangi güvenlik kurallarını sağlaması gerektiğini açıklamak için kullanılır.
* `unsafe` bir işlem yapılırken de yine **SAFETY** yorumu eklemek, güvenlik kurallarının nasıl korunduğunu açıklamak için tercih edilir.

Derleyici, değiştirilebilir statik değişkene referans oluşturmanıza izin vermez. Sadece `raw borrow` operatörleriyle oluşturulan ham işaretçiler (`raw pointers`) üzerinden erişim mümkündür. Bu, güvenlik gerekliliklerini daha açık hale getirmek için zorunludur.

Genel erişime açık değiştirilebilir verilerde veri yarışını önlemek zordur. Bu yüzden Rust, değiştirilebilir statik değişkenleri `unsafe` kabul eder. Mümkünse, 16. Bölümde incelediğimiz eşzamanlılık tekniklerini ve iş parçacığı güvenli akıllı işaretçileri (`thread-safe smart pointers`) kullanmak tercih edilmelidir.

---

## 🛠️ Unsafe Bir Özelliği (Trait) Uygulamak (implementing an unsafe trait)

`Unsafe` ile **unsafe trait** uygulamak mümkündür. Bir trait’in en az bir metodunun derleyici tarafından doğrulanamayan bir kuralı (invariant) varsa, bu trait `unsafe` olarak işaretlenir. Böyle bir trait’i uygularken de `unsafe impl` kullanılır:

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```

👉 Bu durumda, `unsafe impl` ile derleyiciye doğrulamadığı kuralları bizim sağlayacağımıza söz vermiş oluruz.

Örneğin, 16. Bölümde incelediğimiz `Send` ve `Sync` işaretçi (marker) trait’lerini hatırlayın. Bir tip tamamen `Send` ve `Sync` olan türlerden oluşuyorsa, derleyici bunları otomatik olarak uygular. Ancak `raw pointer` gibi `Send` veya `Sync` olmayan türler içeriyorsa ve biz bu tipi `Send` ya da `Sync` yapmak istiyorsak, `unsafe` kullanmamız gerekir. Rust, bu tipin iş parçacıkları arasında güvenle gönderilebileceğini garanti edemez, bu yüzden bu kontrolleri bizim manuel yapmamız gerekir.

---

## 🧩 Bir Union’ın Alanlarına Erişim (accessing fields of a union)

`Unsafe` ile yapılabilen son işlem, bir `union`un alanlarına erişmektir. `Union`, `struct`e benzer; fakat her seferinde yalnızca bir alan kullanılır. `Union`lar genellikle C kodundaki `union`larla arayüz kurmak için kullanılır. Alanlara erişim `unsafe` kabul edilir; çünkü Rust, bir `union` örneğinde şu anda hangi türde veri tutulduğunu garanti edemez. `Union` hakkında daha fazla bilgi Rust Reference’ta bulunabilir.

---

## 🔍 Unsafe Kodu Denetlemek için Miri Kullanmak (using Miri to check unsafe code)

`Unsafe` kod yazarken, gerçekten güvenli ve doğru olup olmadığını kontrol etmek isteyebilirsiniz. Bunun için en iyi yollarından biri, tanımsız davranışları tespit eden resmi Rust aracı **Miri**’yi kullanmaktır.

* **Borrow checker** derleme zamanında çalışan statik bir araçtır.
* **Miri** ise çalışma zamanında (runtime) çalışan dinamik bir araçtır.

Miri, programınızı veya testlerinizi çalıştırarak Rust kurallarını ihlal ettiğinizde uyarı verir.

Miri kullanmak için Rust’ın **nightly** sürümü gerekir. Şu komutlarla kurulabilir:

```
rustup +nightly component add miri
```

👉 Bu işlem projenizin Rust sürümünü değiştirmez; sadece sisteme aracı ekler.

Miri’yi çalıştırmak için:

```
cargo +nightly miri run
cargo +nightly miri test
```

Örneğin, `Listing 20-11` kodunu Miri ile çalıştırırsak:

```
$ cargo +nightly miri run
COUNTER: 3
```

Miri, değiştirilebilir verilere paylaşımlı referansların bulunduğunu uyarı olarak bildirir. Bu durumda uyarı, kesinlikle tanımsız davranış garantisi olmadığı için sadece riskin altını çizer. Bazı durumlarda Miri doğrudan hatalı kalıpları da saptayabilir ve çözüm önerileri sunabilir.

⚠️ Ancak Miri her hatayı yakalayamaz. Dinamik analiz aracı olduğu için yalnızca gerçekten çalıştırılan kod parçalarındaki sorunları yakalar. Bu nedenle güvenilirlik için iyi test teknikleriyle birlikte kullanılmalıdır.

Özetle: Eğer Miri bir hata bulursa kesinlikle bir sorun vardır. Ancak Miri bir hata bulmazsa, bu kodun tamamen güvenli olduğu anlamına gelmez. Yine de çoğu durumda çok faydalıdır. Bu bölümdeki diğer `unsafe` örnekleri de Miri ile çalıştırıp çıktısını incelemeyi deneyebilirsiniz.

Daha fazla bilgi için Miri’nin GitHub deposuna bakabilirsiniz.


## ⏳ Ne Zaman Unsafe Kod Kullanmalı (when to use unsafe code)

Beş `unsafe` süper gücünden birini kullanmak için `unsafe` yazmak yanlış değildir, hatta olumsuz karşılanmaz; ancak `unsafe` kodu doğru yazmak daha zordur, çünkü derleyici bellek güvenliğini korumaya yardımcı olamaz. `Unsafe` kod kullanmak için bir nedeniniz olduğunda, bunu yapabilirsiniz ve açık `unsafe` ek açıklaması (annotation), sorunlar ortaya çıktığında kaynağını takip etmeyi kolaylaştırır.

`Unsafe` kod yazdığınızda, yazdığınız kodun Rust’ın kurallarına uyduğundan daha emin olmak için **Miri** aracını kullanabilirsiniz.

`Unsafe Rust` ile etkili bir şekilde çalışmayı çok daha derinlemesine keşfetmek için, bu konudaki resmi Rust rehberi olan **Rustonomicon**’u okuyun.
