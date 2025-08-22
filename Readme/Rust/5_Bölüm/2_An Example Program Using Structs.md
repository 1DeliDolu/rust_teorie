## 🖥️ Yapılarla (struct) Bir Örnek Program

Yapıları (struct) ne zaman kullanmak isteyebileceğimizi anlamak için bir dikdörtgenin alanını hesaplayan bir program yazalım. Önce tekil değişkenleri kullanarak başlayacağız, ardından programı yeniden düzenleyerek yapılar (struct) kullanana kadar ilerleyeceğiz.

`rectangles` adında yeni bir ikili (binary) Cargo projesi oluşturalım. Bu proje, piksellerle belirtilen bir dikdörtgenin genişliğini ve yüksekliğini alacak ve dikdörtgenin alanını hesaplayacak. Listeleme 5-8, proje `src/main.rs` dosyamızda bunun nasıl yapılabileceğini gösteren kısa bir programı göstermektedir.

### 📂 Dosya Adı: src/main.rs

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

Listeleme 5-8: Ayrı genişlik (`width`) ve yükseklik (`height`) değişkenleriyle belirtilen bir dikdörtgenin alanını hesaplama

Şimdi bu programı `cargo run` ile çalıştıralım:

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/rectangles`
The area of the rectangle is 1500 square pixels.
```

Bu kod, her boyutu `area` fonksiyonuna geçirerek dikdörtgenin alanını doğru bir şekilde hesaplamaktadır, fakat bu kodu daha açık ve okunabilir hale getirebiliriz.

Bu koddaki sorun `area` fonksiyonunun imzasında (signature) belirgindir:

```rust
fn area(width: u32, height: u32) -> u32 {
```

`area` fonksiyonunun amacı bir dikdörtgenin alanını hesaplamaktır. Ancak yazdığımız fonksiyon iki parametre almaktadır ve programımızın hiçbir yerinde bu parametrelerin birbiriyle ilişkili olduğu açıkça belirtilmemektedir. Genişliği ve yüksekliği bir araya getirmek daha okunabilir ve yönetilebilir olacaktır. Bunu yapmanın bir yolunu Bölüm 3’teki “Tuple (demet) Tipi” kısmında zaten tartışmıştık: tuple kullanmak.
## 🔄 Tuple (demet) ile Yeniden Düzenleme

Listeleme 5-9, programımızın tuple kullanan başka bir sürümünü göstermektedir.

### 📂 Dosya Adı: src/main.rs

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

Listeleme 5-9: Dikdörtgenin genişlik (`width`) ve yüksekliğini (`height`) bir tuple ile belirtme

Bir açıdan bu program daha iyidir. Tuple yapısı bize biraz düzen sağlar ve artık yalnızca tek bir argüman geçiyoruz. Ancak diğer yandan, bu sürüm daha az açıktır: tuple elemanlarına ad verilmediği için hesaplama sırasında tuple’ın parçalarına indeksle erişmemiz gerekir, bu da hesaplamayı daha az anlaşılır hale getirir.

Genişlik ve yüksekliği karıştırmak alan hesaplaması için fark yaratmaz, ancak dikdörtgeni ekrana çizmek istediğimizde önemli olur! Genişliğin tuple indeksinde `0`, yüksekliğin ise `1` olduğunu akılda tutmamız gerekir. Başka biri kodumuzu kullanacak olursa bunu anlamak ve akılda tutmak daha da zor olurdu. Çünkü verimizin anlamını kodda aktarmadık; bu nedenle hata yapma olasılığı artar.

---

## 🏗️ Yapılarla (struct) Yeniden Düzenleme: Daha Fazla Anlam Katmak

Verilere etiket vererek onlara anlam katmak için yapıları (struct) kullanırız. Tuple kullandığımız yapıyı, hem bütününe hem de parçalarına ad verdiğimiz bir struct’a dönüştürebiliriz. Listeleme 5-10’da gösterildiği gibi.

### 📂 Dosya Adı: src/main.rs

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

Listeleme 5-10: Bir `Rectangle` yapısı (struct) tanımlama

Burada bir yapı (struct) tanımladık ve adına `Rectangle` dedik. Süslü parantezler içinde alanları (`fields`) `width` ve `height` olarak tanımladık, her ikisinin de türü `u32`. Daha sonra `main` fonksiyonunda, genişliği 30 ve yüksekliği 50 olan özel bir `Rectangle` örneği oluşturduk.

`area` fonksiyonu artık bir parametreyle tanımlandı, adına `rectangle` dedik ve türü `Rectangle` yapısının değiştirilemez ödünç alınmış (`immutable borrow`) bir örneği. Bölüm 4’te de belirtildiği gibi, yapının sahipliğini almak yerine onu ödünç almak isteriz. Böylece `main` sahipliği elinde tutar ve `rect1`’i kullanmaya devam edebilir. Bu nedenle fonksiyon imzasında ve fonksiyon çağrısında `&` işaretini kullanıyoruz.

`area` fonksiyonu, `Rectangle` örneğinin `width` ve `height` alanlarına erişir (bir struct örneğini ödünç alarak alanlarına erişmek, değerleri taşımadığı için bu şekilde struct’ların ödünç alınması yaygın bir durumdur). Artık `area` fonksiyon imzamız tam olarak kastettiğimizi söylüyor: `Rectangle`’ın genişlik ve yükseklik alanlarını kullanarak alanını hesapla. Bu, genişlik ve yüksekliğin birbirleriyle ilişkili olduğunu açıkça ifade eder ve tuple’daki `0` ve `1` indeks değerleri yerine açıklayıcı isimler verir. Bu da kodun anlaşılırlığını artırır.

## 🛠️ Türetilmiş Özelliklerle (derived traits) Faydalı İşlevsellik Ekleme

Programımızı hata ayıklarken bir `Rectangle` örneğini yazdırabilmek ve tüm alanlarının değerlerini görebilmek faydalı olurdu. Listeleme 5-11, önceki bölümlerde yaptığımız gibi `println!` makrosunu (println! macro) kullanmayı deniyor. Ancak bu çalışmayacaktır.

### 📂 Dosya Adı: src/main.rs

Bu kod derlenmez!

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1}");
}
```

Listeleme 5-11: Bir `Rectangle` örneğini yazdırmaya çalışma

Bu kodu derlediğimizde, şu temel mesajla bir hata alırız:

```
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

`println!` makrosu pek çok türde biçimlendirme (formatting) yapabilir ve varsayılan olarak süslü parantezler, `println!`’a `Display` (Display) olarak bilinen biçimlendirmeyi kullanmasını söyler: doğrudan son kullanıcı tüketimine yönelik çıktı. Şu ana kadar gördüğümüz ilkel türler (primitive types) varsayılan olarak `Display` uygular, çünkü bir kullanıcıya bir `1` ya da başka bir ilkel türün nasıl gösterileceği konusunda yalnızca tek bir mantıklı yol vardır. Ancak yapılarda (struct) `println!`’ın çıktıyı nasıl biçimlendirmesi gerektiği daha belirsizdir; çünkü daha fazla görüntüleme olanağı vardır: Virgüller olsun mu? Süslü parantezler yazdırılsın mı? Tüm alanlar gösterilsin mi? Bu belirsizlikten dolayı Rust ne istediğimizi tahmin etmeye çalışmaz ve yapılar için `println!` ile `{}` yer tutucusunu kullanacak bir `Display` (Display) uygulaması sağlamaz.

Hataları okumaya devam edersek, şu yararlı notu buluruz:

```
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Hadi deneyelim! `println!` makrosu çağrısı artık `println!("rect1 is {rect1:?}");` şeklinde olacaktır. Süslü parantezlerin içine `:?` belirticisini koymak, `println!`’a `Debug` (Debug) adı verilen bir çıktı biçimini kullanmak istediğimizi söyler. `Debug` özelliği (trait) geliştiriciler için yararlı bir şekilde yapımızı yazdırmamızı sağlar; böylece kodumuzu hata ayıklarken değerini görebiliriz.

Bu değişiklikle kodu derleyin. Tüh! Yine de bir hata alıyoruz:

```
error[E0277]: `Rectangle` doesn't implement `Debug`
```

Ancak derleyici yine yararlı bir not verir:

```
= help: the trait `Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

Rust, hata ayıklama bilgilerini yazdırma işlevselliğini içerir; fakat bu işlevselliği yapımız için kullanılabilir hale getirmek üzere açıkça katılmamız (opt in) gerekir. Bunu yapmak için, Listeleme 5-12’de gösterildiği gibi, yapı tanımından hemen önce dış öznitelik (attribute) `#[derive(Debug)]` ekleriz.

### 📂 Dosya Adı: src/main.rs

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}
```

Listeleme 5-12: `Debug` özelliğinin (Debug) türetilmesi için öznitelik ekleme ve `Rectangle` örneğini hata ayıklama biçimlendirmesiyle yazdırma

Artık programı çalıştırdığımızda hata almayız ve aşağıdaki çıktıyı görürüz:

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle { width: 30, height: 50 }
```

Güzel! En süslü çıktı olmasa da bu örneğin tüm alanlarının değerlerini gösterir; bu da kesinlikle hata ayıklama sırasında yardımcı olur. Daha büyük yapılarımız olduğunda, okunması biraz daha kolay bir çıktı faydalı olur; bu durumlarda `{:?}` yerine `{:#?}` kullanabiliriz (pretty-print (pretty-print)). Bu örnekte `{:#?}` stili şu çıktıyı verir:

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

Bir değeri `Debug` biçimiyle yazdırmanın başka bir yolu da `dbg!` makrosunu (dbg! macro) kullanmaktır; bu makro bir ifadenin sahipliğini (ownership) alır (referans alan `println!`’ın tersine), kodunuzda `dbg!` çağrısının geçtiği dosya ve satır numarasını, ayrıca bu ifadenin ortaya çıkan değerini yazdırır ve değerin sahipliğini geri döndürür.

Not: `dbg!` makrosunu çağırmak, standart hata konsol akışına (`stderr`) (stderr) yazdırır; `println!` ise standart çıktı konsol akışına (`stdout`) (stdout) yazdırır. `stderr` ve `stdout` hakkında daha fazla bilgiyi Bölüm 12’deki “Hata Mesajlarını Standart Çıktı Yerine Standart Hataya Yazma” bölümünde konuşacağız.

Aşağıda hem `width` alanına atanan değerle hem de `rect1` içindeki tüm yapının değeriyle ilgilendiğimiz bir örnek vardır:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

`30 * scale` ifadesinin etrafına `dbg!` yerleştirebiliriz ve `dbg!` ifadenin değerinin sahipliğini döndürdüğü için, `width` alanı, orada `dbg!` çağrısı olmasaydı sahip olacağı değeri alır. `rect1`’in sahipliğini `dbg!`’ın almasını istemediğimiz için bir sonraki çağrıda `rect1`’e bir referans kullanıyoruz. Bu örneğin çıktısı şöyle görünür:

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10:16] 30 * scale = 60
[src/main.rs:14:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```
## 🔍 dbg! Çıktısını Yorumlamak (dbg! macro)

Çıktının ilk kısmının, `src/main.rs` dosyasının 10. satırından geldiğini görüyoruz; burada `30 * scale` ifadesini (expression) hata ayıklıyoruz ve ortaya çıkan değer 60’tır (tamsayılar için (integers) uygulanan `Debug` biçimlendirme (Debug formatting), yalnızca değerlerini yazdırır). `src/main.rs`’in 14. satırındaki `dbg!` çağrısı, `&rect1`’in değerini, yani `Rectangle` yapısını yazdırır. Bu çıktı, `Rectangle` türünün (type) güzel `Debug` biçimlendirmesini (pretty Debug formatting) kullanır. `dbg!` makrosu (macro), kodunuzun ne yaptığını anlamaya çalışırken gerçekten yardımcı olabilir!

## 🧪 Türetilen Özellikler ve Öznitelikler (derived traits, derive attribute)

`Debug` özelliğine (Debug trait) ek olarak, Rust, özel türlerimizde (custom types) `derive` özniteliği (derive attribute) ile kullanabileceğimiz bir dizi özellik (traits) sağlamıştır; bunlar faydalı davranışlar ekleyebilir. Bu özellikler ve davranışları, Ek C’de (Appendix C) listelenmiştir. Bölüm 10’da, bu özellikleri özel davranışlarla nasıl uygulayacağımızı (implement) ve kendi özelliklerinizi (traits) nasıl oluşturacağınızı ele alacağız. `derive` dışındaki pek çok öznitelik (attributes) de vardır; daha fazla bilgi için Rust Referansı’ndaki “Attributes” bölümüne (Rust Reference “Attributes” section) bakın.

## 🛠️ Fonksiyondan Yönteme Geçiş: `area`’yı `Rectangle` Üzerinde Tanımlama (method)

`area` fonksiyonumuz çok özeldir: yalnızca dikdörtgenlerin alanını hesaplar. Bu davranışı, başka hiçbir türle çalışmayacağı için `Rectangle` yapımıza daha yakından bağlamak faydalı olacaktır. Kodumuzu, `area` fonksiyonunu `Rectangle` türü (type) üzerinde tanımlanmış bir `area` yöntemi (method) haline dönüştürerek nasıl yeniden düzenlemeye devam edebileceğimize bakalım.
