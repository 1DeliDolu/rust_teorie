## 📋 Vektörlerle (vectors) Değer Listeleri Saklamak

İlk ele alacağımız koleksiyon tipi, vektör (vector) olarak da bilinen `Vec<T>`’dir. Vektörler, bellekte tüm değerleri yan yana yerleştiren tek bir veri yapısı içinde birden fazla değer saklamanıza olanak tanır. Vektörler yalnızca aynı türde değerleri saklayabilir. Bir dosyadaki metin satırları veya bir alışveriş sepetindeki ürünlerin fiyatları gibi öğe listelerine sahip olduğunuzda oldukça kullanışlıdırlar.

### 🆕 Yeni Bir Vektör Oluşturma (creating a new vector)

Yeni boş bir vektör oluşturmak için `Vec::new` fonksiyonunu çağırırız. Liste 8-1’de gösterildiği gibi:

```rust
let v: Vec<i32> = Vec::new();
```

Liste 8-1: `i32` türünde değerler tutacak yeni, boş bir vektör oluşturma

Burada bir tür açıklaması (type annotation) eklediğimize dikkat edin. Bu vektöre herhangi bir değer eklemediğimiz için, Rust hangi türde öğeleri saklamak istediğimizi bilemez. Bu önemli bir noktadır. Vektörler generic’ler (generics) kullanılarak uygulanır; kendi türlerinizle generics kullanmayı Bölüm 10’da ele alacağız. Şimdilik, standart kütüphanenin sağladığı `Vec<T>` tipinin herhangi bir türü tutabileceğini bilmeniz yeterlidir. Belirli bir türü tutacak bir vektör oluşturduğumuzda, türü köşeli parantezler içinde belirtebiliriz. Liste 8-1’de `v` içindeki `Vec<T>`’nin `i32` türünde öğeler tutacağını Rust’a belirtmiş olduk.

Çoğu zaman, `Vec<T>`’yi başlangıç değerleriyle oluşturursunuz ve Rust saklamak istediğiniz değer türünü kendisi çıkarır, bu yüzden tür açıklaması yapmanız nadiren gerekir. Rust bu iş için `vec!` makrosunu sağlar; bu makro kendisine verdiğiniz değerleri tutacak yeni bir vektör oluşturur. Liste 8-2, 1, 2 ve 3 değerlerini tutan yeni bir `Vec<i32>` oluşturur. Tamsayı türü `i32`’dir çünkü bu varsayılan tamsayı türüdür; bunu Bölüm 3’teki “Veri Tipleri” (Data Types) kısmında tartışmıştık.

```rust
let v = vec![1, 2, 3];
```

Liste 8-2: Değerler içeren yeni bir vektör oluşturma

Başlangıçta `i32` değerleri verdiğimiz için, Rust `v`’nin türünün `Vec<i32>` olduğunu çıkarır ve tür açıklaması gerekmez. Şimdi bir vektörün nasıl güncelleneceğine bakalım.

### ✏️ Bir Vektörü Güncellemek (updating a vector)

Bir vektör oluşturmak ve ardından içine öğeler eklemek için `push` metodunu kullanabiliriz. Liste 8-3’te gösterildiği gibi:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

Liste 8-3: Bir vektöre değer eklemek için `push` metodunu kullanma

Her değişkende olduğu gibi, değerini değiştirmek istiyorsak, Bölüm 3’te tartışıldığı gibi `mut` anahtar kelimesini kullanarak onu değiştirilebilir (mutable) yapmamız gerekir. İçine koyduğumuz sayılar `i32` türündedir ve Rust bunu verilerden çıkarır, dolayısıyla `Vec<i32>` açıklamasına gerek yoktur.
## 📖 Vektörlerin (vectors) Elemanlarını Okuma

Bir vektörde saklanan bir değere başvurmanın iki yolu vardır: indeksleme (indexing) veya `get` metodunu kullanmak. Aşağıdaki örneklerde, bu fonksiyonların döndürdüğü değerlerin türlerini açıklık için belirttik.

Liste 8-4, bir vektördeki bir elemana hem indeksleme sözdizimiyle hem de `get` metoduyla erişmenin iki yolunu gösteriyor:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

Liste 8-4: Bir vektördeki öğeye erişmek için indeksleme sözdizimini ve `get` metodunu kullanma

Burada birkaç ayrıntıya dikkat edin. Üçüncü öğeyi almak için 2 indeks değerini kullanıyoruz, çünkü vektörler sıfırdan başlayan sayılarla indekslenir. `&` ve `[]` kullanmak, belirtilen indeks değerindeki öğeye bir referans döndürür. `get` metodunu bir indeks ile kullandığımızda ise `Option<&T>` döner ve bunu `match` ile kullanabiliriz.

Rust, bir öğeye başvurmak için iki yol sunar, böylece mevcut öğe aralığının dışında bir indeks değeri kullanıldığında programın nasıl davranacağını seçebilirsiniz. Örneğin, beş öğeli bir vektörümüz olduğunu ve ardından 100. indeksteki öğeye erişmeye çalıştığımızı düşünelim. Liste 8-5’te gösterildiği gibi:

```rust
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

Liste 8-5: Beş öğe içeren bir vektörde 100. indeksteki öğeye erişmeye çalışmak

Bu kodu çalıştırdığımızda, ilk `[]` yöntemi programın panic etmesine neden olur çünkü var olmayan bir öğeye başvurur. Bu yöntem, vektörün sonunu aşan bir öğeye erişilmeye çalışıldığında programınızın çökmesini istediğiniz durumlarda en uygunudur.

Öte yandan, `get` metoduna vektörün dışında bir indeks verildiğinde, panic etmek yerine `None` döndürür. Eğer normal koşullar altında arada sırada vektör sınırlarını aşan erişimlerin olabileceğini düşünüyorsanız bu yöntemi kullanabilirsiniz. Kodunuz, ya `Some(&element)` ya da `None` sonucunu işlemek üzere mantık içerir (Bölüm 6’da tartışıldığı gibi). Örneğin, indeks bir kişinin girdiği sayıdan geliyor olabilir. Eğer kullanıcı yanlışlıkla çok büyük bir sayı girerse ve program `None` döndürürse, ona vektörde kaç öğe olduğunu söyleyebilir ve geçerli bir değer girmesi için tekrar şans verebilirsiniz. Bu, programı bir yazım hatası yüzünden çökertmekten çok daha kullanıcı dostudur.

Program geçerli bir referansa sahip olduğunda, borrow checker (ödünç alma denetleyicisi) sahiplik ve ödünç alma kurallarını (Bölüm 4’te ele alındı) uygulayarak bu referansın ve vektörün içeriğine olan diğer referansların geçerliliğini garanti eder. Aynı kapsamda hem değiştirilebilir (mutable) hem de değiştirilemez (immutable) referansların olamayacağını belirten kuralı hatırlayın. Bu kural Liste 8-6’da geçerlidir: bir vektörün ilk öğesine değiştirilemez bir referans tuttuğumuzda ve aynı zamanda sonuna yeni bir öğe eklemeye çalıştığımızda. Eğer daha sonra o ilk öğeye tekrar başvurmaya çalışırsak program çalışmaz.

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {first}");
```

Liste 8-6: Bir öğeye referans tutarken vektöre yeni öğe eklemeye çalışmak

Bu kodu derlemek şu hatayla sonuçlanır:

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {first}");
  |                                     ------- immutable borrow later used here
```

Daha fazla bilgi için şu komutu deneyin: `rustc --explain E0502`.

Liste 8-6’daki kod ilk bakışta çalışmalı gibi görünebilir: neden ilk öğeye bir referans, vektörün sonuna yapılan değişiklikten etkilensin ki? Bu hata, vektörlerin çalışma şekliyle ilgilidir: vektörler değerleri bellekte yan yana koyar. Vektörün sonuna yeni bir öğe eklemek, eğer mevcut yerde tüm öğeleri yan yana yerleştirmek için yeterli alan yoksa, yeni bir bellek alanı ayırmayı ve eski öğeleri yeni alana kopyalamayı gerektirebilir. Böyle bir durumda, ilk öğeye olan referans aslında artık serbest bırakılmış (deallocated) bir belleğe işaret ediyor olurdu. Ödünç alma kuralları programların böyle bir duruma düşmesini engeller.

Not: `Vec<T>` tipinin uygulama detayları hakkında daha fazla bilgi için “The Rustonomicon”’a bakın.
## 🔄 Bir Vektördeki (vector) Değerler Üzerinde Yineleme (iteration)

Bir vektördeki her öğeye sırayla erişmek için, tek tek indekslerle erişmek yerine tüm öğeler üzerinde yineleme (iteration) yaparız. Liste 8-7, `i32` değerlerinden oluşan bir vektördeki her öğeye değiştirilemez (immutable) referanslar almak ve bunları yazdırmak için `for` döngüsünün nasıl kullanılacağını gösterir:

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```

Liste 8-7: Bir `for` döngüsü kullanarak vektördeki her öğeyi yineleme ile yazdırma

Ayrıca, değiştirilebilir (mutable) bir vektördeki her öğeye değiştirilebilir referanslarla yineleme yapabilir ve tüm öğeleri değiştirebiliriz. Liste 8-8’deki `for` döngüsü, her öğeye 50 ekleyecektir:

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

Liste 8-8: Vektördeki öğelere değiştirilebilir referanslarla yineleme yapma

Değiştirilebilir referansın işaret ettiği değeri değiştirmek için, `+=` operatörünü kullanmadan önce `*` dereference (değer çözme) operatörünü kullanarak `i` içindeki değere ulaşmamız gerekir. Dereference operatörü hakkında daha fazla bilgiyi Bölüm 15’teki “Referanstan Değere Ulaşma” kısmında ele alacağız.

Bir vektör üzerinde, ister değiştirilemez ister değiştirilebilir şekilde yineleme yapmak, borrow checker kuralları sayesinde güvenlidir. Eğer Liste 8-7 veya Liste 8-8’deki `for` döngülerinin gövdesinde öğe eklemeye ya da çıkarmaya çalışsaydık, Liste 8-6’daki kodda olduğu gibi bir derleyici hatası alırdık. Bunun nedeni, `for` döngüsünün tuttuğu referansın, aynı anda tüm vektörün değiştirilmesini engellemesidir.
## 🧩 Farklı Türleri Saklamak için Enum Kullanma (using an enum to store multiple types)

Vektörler yalnızca aynı türden değerleri saklayabilir. Bu bazen elverişsiz olabilir; farklı türlerden öğeleri bir listede saklamamız gereken kullanım durumları vardır. Neyse ki, bir `enum`’un varyantları (variants) aynı `enum` türü altında tanımlanır, bu yüzden farklı türleri temsil edecek tek bir türe ihtiyaç duyduğumuzda bir `enum` tanımlayıp kullanabiliriz!

Örneğin, bir hesap tablosundaki (spreadsheet) bir satırdan değerler almak istediğimizi varsayalım. Bu satırın bazı sütunları tamsayılar (integers), bazıları kayan noktalı sayılar (floating-point numbers), bazıları ise string’ler içerebilir. Farklı değer türlerini tutacak `enum` varyantları tanımlayabiliriz ve tüm varyantlar `enum` türü olarak kabul edilir. Böylece, bu `enum`’u tutacak bir vektör oluşturabilir ve sonuçta farklı türleri aynı koleksiyonda saklayabiliriz. Bunu Liste 8-9’da gösterdik:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

Liste 8-9: Farklı türden değerleri tek bir vektörde saklamak için bir `enum` tanımlama

Rust, derleme zamanında vektörde hangi türlerin bulunacağını bilmek zorundadır; böylece her öğeyi saklamak için heap üzerinde ne kadar bellek gerektiğini hesaplar. Ayrıca, bu vektörde hangi türlere izin verileceğini açıkça belirtmemiz gerekir. Eğer Rust vektörlerin herhangi bir türü saklamasına izin verseydi, bir veya daha fazla türün vektör öğeleri üzerinde yapılan işlemlerde hataya yol açma ihtimali olurdu. `Enum` ve `match` ifadesini birlikte kullanmak, her olasılığın derleme zamanında ele alındığını garanti eder (Bölüm 6’da tartışıldığı gibi).

Eğer bir programın çalışma zamanında vektöre hangi türleri alacağını kapsamlı bir şekilde bilmiyorsanız, `enum` tekniği işe yaramaz. Bunun yerine, Bölüm 18’de ele alacağımız trait nesnelerini (trait objects) kullanabilirsiniz.

Artık vektörleri kullanmanın en yaygın yollarından bazılarını tartıştığımıza göre, standart kütüphanenin `Vec<T>` üzerinde tanımladığı birçok faydalı metodun API dokümantasyonunu gözden geçirdiğinizden emin olun. Örneğin, `push` metoduna ek olarak, `pop` metodu son öğeyi kaldırır ve döndürür.
## 🗑️ Bir Vektörün (vector) Bırakılmasıyla Elemanlarının da Bırakılması

Herhangi bir başka struct gibi, bir vektör de kapsam (scope) dışına çıktığında serbest bırakılır. Bu durum Liste 8-10’da gösterilmiştir:

```rust
{
    let v = vec![1, 2, 3, 4];

    // v ile ilgili işlemler yapılır
} // <- v kapsam dışına çıkar ve burada serbest bırakılır
```

Liste 8-10: Vektörün ve elemanlarının bırakıldığı (drop edildiği) yerin gösterilmesi

Bir vektör bırakıldığında, içindeki tüm içerikler de bırakılır; yani vektörün tuttuğu tamsayılar da temizlenir. Borrow checker, bir vektörün içeriğine yapılan referansların yalnızca vektörün kendisi geçerli olduğu sürece kullanılmasını garanti eder.

Haydi şimdi bir sonraki koleksiyon türüne geçelim: **String!**
