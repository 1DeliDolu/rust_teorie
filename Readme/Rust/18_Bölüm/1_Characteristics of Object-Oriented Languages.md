## ⚙️ Nesne Yönelimli Dillerin Özellikleri (Characteristics of Object-Oriented Languages)

Bir dilin nesne yönelimli sayılması için hangi özelliklere sahip olması gerektiği konusunda programlama topluluğunda bir fikir birliği yoktur. Rust, OOP dâhil olmak üzere birçok programlama paradigmasından etkilenmiştir; örneğin, 13. bölümde fonksiyonel programlamadan gelen özellikleri inceledik. Tartışmaya açık olsa da, OOP dilleri genellikle bazı ortak özellikleri paylaşır: **nesneler (objects)**, **kapsülleme (encapsulation)** ve **kalıtım (inheritance)**. Şimdi bu özelliklerin her birinin ne anlama geldiğine ve Rust’ın bunları destekleyip desteklemediğine bakalım.

---

## 🧩 Nesneler Veri ve Davranış İçerir (Objects Contain Data and Behavior)

Erich Gamma, Richard Helm, Ralph Johnson ve John Vlissides tarafından yazılan *Design Patterns: Elements of Reusable Object-Oriented Software* (Addison-Wesley, 1994), yaygın olarak **Gang of Four** kitabı olarak bilinir ve nesne yönelimli tasarım desenlerinin bir kataloğudur. Bu kitap, OOP’yi şu şekilde tanımlar:

> Nesne yönelimli programlar, nesnelerden oluşur. Bir nesne hem veriyi hem de o veriyi işleyen prosedürleri bir araya getirir. Bu prosedürler genellikle metotlar (methods) veya işlemler (operations) olarak adlandırılır.

Bu tanıma göre Rust nesne yönelimlidir: `struct` ve `enum` veri içerir, `impl` blokları ise bu `struct` ve `enum`’lara metotlar sağlar. Her ne kadar metotları olan `struct` ve `enum`’lar nesne olarak adlandırılmasa da, Gang of Four’un nesne tanımına göre aynı işlevselliği sunarlar.

---

## 🔒 Uygulama Detaylarını Gizleyen Kapsülleme (Encapsulation That Hides Implementation Details)

OOP ile genellikle ilişkilendirilen bir diğer özellik kapsülleme (encapsulation) kavramıdır. Bu, bir nesnenin uygulama detaylarının o nesneyi kullanan kod tarafından erişilemez olması anlamına gelir. Dolayısıyla, bir nesneyle etkileşim kurmanın tek yolu onun **public API**’sidir; nesneyi kullanan kod, nesnenin iç detaylarına ulaşıp doğrudan veriyi veya davranışı değiştirememelidir. Bu sayede, programcı nesnenin iç yapısını değiştirebilir veya yeniden düzenleyebilir, nesneyi kullanan kodu değiştirmeden.

Kapsüllemeyi nasıl kontrol edeceğimizi 7. bölümde tartışmıştık: `pub` anahtar sözcüğünü kullanarak hangi modüllerin, tiplerin, fonksiyonların ve metotların public olacağına karar verebiliriz; varsayılan olarak her şey private’tır. Örneğin, `i32` değerlerinden oluşan bir `vector` içeren bir alanı bulunan `AveragedCollection` adında bir `struct` tanımlayabiliriz. Bu `struct`, ayrıca `vector` içindeki değerlerin ortalamasını tutan bir alana da sahip olabilir; böylece ortalama her ihtiyaç duyulduğunda yeniden hesaplanmak zorunda kalmaz. Yani, `AveragedCollection` bizim için hesaplanan ortalamayı önbelleğe alır. Listeleme 18-1’de `AveragedCollection` struct’ının tanımı verilmiştir:

**Dosya Adı:** src/lib.rs

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

Listeleme 18-1: Tamsayı listesini ve listedeki öğelerin ortalamasını tutan `AveragedCollection` struct’ı

`struct`, başka kodların da kullanabilmesi için `pub` olarak işaretlenmiştir, fakat içindeki alanlar private kalır. Bu önemlidir çünkü listeye bir değer eklendiğinde veya çıkarıldığında ortalamanın da güncellenmesini garanti etmek isteriz. Bunu, Listeleme 18-2’de gösterildiği gibi `add`, `remove` ve `average` metotlarını uygulayarak yaparız:

**Dosya Adı:** src/lib.rs

```rust
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

Listeleme 18-2: `AveragedCollection` üzerinde public `add`, `remove` ve `average` metotlarının uygulanması

`add`, `remove` ve `average` metotları, `AveragedCollection` içindeki verilere erişmenin veya onları değiştirmenin tek yoludur. Bir öğe `add` ile eklendiğinde veya `remove` ile çıkarıldığında, her çağrı ortalamayı güncellemekten sorumlu private `update_average` metodunu çalıştırır.

`list` ve `average` alanlarını private bıraktığımız için dışarıdaki kodun doğrudan `list` üzerinde öğe eklemesi veya çıkarması mümkün değildir; aksi halde `average` alanı listeyle senkronize olmayabilirdi. `average` metodu, dış kodun ortalama değerini okumasına izin verir fakat onu değiştirmesine izin vermez.

Bu kapsülleme sayesinde, ileride `AveragedCollection`’ın uygulama detaylarını kolayca değiştirebiliriz. Örneğin, `list` alanı için `Vec<i32>` yerine `HashSet<i32>` kullanabiliriz. `add`, `remove` ve `average` metotlarının imzaları aynı kaldığı sürece, `AveragedCollection`’ı kullanan kodun değiştirilmesine gerek olmaz. Ancak eğer `list` public olsaydı, bu durumda `HashSet<i32>` ve `Vec<i32>` farklı metotlara sahip olduğundan dışarıdaki kodun da değişmesi gerekirdi.

Eğer kapsülleme bir dilin nesne yönelimli sayılması için gerekli bir özellikse, Rust bu gerekliliği karşılar. Koddaki farklı parçalar için `pub` kullanma veya kullanmama seçeneği, uygulama detaylarının kapsüllenmesini sağlar.

## 🧬 Kalıtımın Tip Sistemi ve Kod Paylaşımı Olarak Kullanımı (Inheritance as a Type System and as Code Sharing)

Kalıtım (inheritance), bir nesnenin başka bir nesnenin tanımından öğeleri devralmasına imkân tanıyan bir mekanizmadır. Böylece, üst (parent) nesnenin verilerini ve davranışlarını yeniden tanımlamak zorunda kalmadan alt (child) nesneye kazandırabilirsiniz.

Eğer bir dilin nesne yönelimli sayılabilmesi için mutlaka kalıtım desteklemesi gerekiyorsa, Rust böyle bir dil değildir. Makro (macro) kullanmadan bir `struct`’ın, üst `struct`’ın alanlarını ve metot uygulamalarını devralmasını sağlamanın bir yolu yoktur.

Bununla birlikte, programlama araç kutusunda kalıtıma alışkınsanız, Rust’ta neden kalıtımı kullanmak istediğinize bağlı olarak farklı çözümlerden yararlanabilirsiniz.

---

## 🔄 Kalıtımı Seçmenin İki Ana Nedeni (Two Main Reasons to Choose Inheritance)

Kalıtımı seçmenin iki ana nedeni vardır:

1. **Kodun yeniden kullanımı (code reuse):**
   Bir tür (type) için belirli bir davranışı uygulayabilir ve kalıtım sayesinde bu uygulamayı başka bir tür için tekrar kullanabilirsiniz.
   Rust’ta bu, **varsayılan trait metot uygulamaları (default trait method implementations)** ile sınırlı bir şekilde yapılabilir. Örneğin, 10-14 numaralı listede `Summary` trait’ine `summarize` metodunun varsayılan bir uygulamasını eklediğimizde, `Summary` trait’ini uygulayan her tür bu metoda otomatik olarak sahip oluyordu. Bu, bir üst sınıfta (parent class) tanımlanan metodun, onu miras alan alt sınıfta (child class) da mevcut olmasına benzer. Ayrıca, `Summary` trait’ini uygularken `summarize` metodunun varsayılan uygulamasını geçersiz kılabiliriz (override). Bu da, alt sınıfın üst sınıftan devraldığı bir metodun uygulamasını değiştirmesine benzer.

2. **Tip sistemi (type system):**
   Kalıtımı kullanmanın diğer nedeni, alt türün (child type), üst türün (parent type) kullanıldığı yerlerde de kullanılabilmesini sağlamaktır. Bu aynı zamanda **polimorfizm (polymorphism)** olarak adlandırılır. Yani, belirli özellikleri paylaşan nesneleri çalışma zamanında (runtime) birbirinin yerine kullanabilmek anlamına gelir.

---

## 🔀 Polimorfizm (Polymorphism)

Birçok kişi için polimorfizm kalıtım ile eş anlamlıdır. Ancak aslında daha genel bir kavramdır ve çoklu türlerle (multiple types) çalışabilen kodu ifade eder. Kalıtım bağlamında, bu türler genellikle alt sınıflardır (subclasses).

Rust ise bunun yerine **jenerikler (generics)** kullanarak farklı türler üzerinde soyutlama yapar ve **trait sınırları (trait bounds)** ile bu türlerin hangi işlevleri sağlaması gerektiğini belirler. Bu bazen **sınırlı parametrik polimorfizm (bounded parametric polymorphism)** olarak adlandırılır.

Kalıtım, son yıllarda birçok programlama dilinde tasarım çözümü olarak gözden düşmüştür. Çünkü çoğu zaman gerekenden fazla kod paylaşımına yol açma riski vardır. Alt sınıflar her zaman üst sınıfın tüm özelliklerini paylaşmamalıdır, ancak kalıtım ile bunu yaparlar. Bu da bir programın tasarımını daha az esnek hale getirebilir. Ayrıca, alt sınıflarda mantıklı olmayan veya hata oluşturan metotların çağrılmasına neden olabilir. Bunun yanında, bazı diller yalnızca tekli kalıtıma (single inheritance) izin verir, yani bir alt sınıf yalnızca bir üst sınıftan kalıtım alabilir, bu da esnekliği daha da kısıtlar.

Bu nedenlerden dolayı, Rust kalıtım yerine farklı bir yaklaşım benimser: **trait nesneleri (trait objects)** kullanır. Şimdi, trait nesnelerinin Rust’ta polimorfizmi nasıl sağladığına bakalım.
