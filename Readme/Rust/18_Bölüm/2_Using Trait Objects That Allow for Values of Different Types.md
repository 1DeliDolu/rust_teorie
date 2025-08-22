## 🧰 Farklı Türlerdeki Değerleri Destekleyen Trait Nesnelerinin Kullanımı (Using Trait Objects That Allow for Values of Different Types)

8. bölümde, `vector`’ların yalnızca tek bir türde öğe depolayabildiğini belirtmiştik. Bunu aşmak için Listeleme 8-9’da `SpreadsheetCell` adında bir `enum` tanımlamış ve bu `enum`’da tamsayı, ondalık ve metin tutan varyantlar oluşturmuştuk. Böylece her hücreye farklı türde veri koyabiliyor ve yine de bir satırı temsil eden `vector` kullanabiliyorduk. Kodumuz derlendiğinde türler sabitse bu oldukça iyi bir çözümdür.

Ancak, bazen kütüphanemizi kullanan kişinin belirli bir durumda geçerli olabilecek tür kümesini genişletebilmesini isteriz. Bunu göstermek için, öğelerin listesini dolaşıp her birine `draw` metodunu çağırarak ekrana çizen örnek bir grafik kullanıcı arayüzü (GUI) aracı oluşturacağız. Bu, GUI araçlarında yaygın bir tekniktir. `gui` adında bir kütüphane crate’i oluşturup, bunun içinde GUI kütüphanesinin yapısını tanımlayacağız. Bu crate, `Button` veya `TextField` gibi kullanılabilecek bazı türleri içerebilir. Ayrıca, `gui` kullanıcıları kendi türlerini de tanımlamak isteyeceklerdir; örneğin bir programcı `Image`, diğeri `SelectBox` ekleyebilir.

Bu örnekte tam teşekküllü bir GUI kütüphanesi uygulamayacağız, ancak parçaların nasıl bir araya geldiğini göstereceğiz. Kütüphaneyi yazarken, başka programcıların hangi türleri oluşturmak isteyeceğini bilemeyiz. Ancak şunu biliyoruz: `gui` farklı türlerde birçok değeri takip etmeli ve her biri için `draw` metodunu çağırabilmelidir. Bu çağrının sonucunda ne olacağını bilmesine gerek yoktur; sadece bu metodun çağrılabilir olduğundan emin olması yeterlidir.

Kalıtımı olan bir dilde, `Component` adında bir sınıf tanımlar, bu sınıfta `draw` metodunu uygularız. Diğer sınıflar (`Button`, `Image`, `SelectBox`) ise `Component`’ten kalıtım alır, `draw` metodunu miras edinir ve kendi özel davranışlarını tanımlamak için bu metodu geçersiz kılabilirlerdi. Çatı kütüphane ise bu türlerin hepsini `Component` örnekleri gibi görebilir ve `draw` metodunu çağırabilirdi. Ancak Rust’ta kalıtım olmadığından, `gui` kütüphanesini kullanıcıların yeni türlerle genişletebilmesi için farklı bir yol izlememiz gerekir.

---

## 📝 Ortak Davranış için Trait Tanımlamak (Defining a Trait for Common Behavior)

`gui`’da istediğimiz davranışı uygulamak için, `Draw` adında bir trait tanımlayacağız. Bu trait’in `draw` adında tek bir metodu olacak. Daha sonra, trait nesnesi (trait object) içeren bir `vector` tanımlayabiliriz. Bir **trait nesnesi**, hem belirli bir trait’i uygulayan türün örneğine hem de o tür üzerindeki trait metotlarını çalışma zamanında (runtime) aramak için kullanılan tabloya işaret eder.

Bir trait nesnesi oluşturmak için `&` referans veya `Box<T>` gibi bir işaretçi (pointer) kullanırız, ardından `dyn` anahtar sözcüğünü ve ilgili trait’i belirtiriz. (Trait nesnelerinin neden mutlaka bir işaretçi kullanması gerektiğini 20. bölümde “Dynamically Sized Types and the Sized Trait” kısmında tartışacağız.) Trait nesnelerini, jenerik (generic) veya somut (concrete) türlerin yerine kullanabiliriz. Bir yerde trait nesnesi kullanıldığında, Rust’ın tip sistemi derleme zamanında oradaki tüm değerlerin belirtilen trait’i uyguladığından emin olur. Böylece, tüm olası türleri derleme zamanında bilmemiz gerekmez.

Rust’ta, `struct` ve `enum`’ları “nesne (object)” olarak adlandırmaktan kaçındığımızı belirtmiştik. Bunun nedeni, Rust’ta veri (`struct` alanları) ve davranış (`impl` blokları) ayrı tutulurken, diğer dillerde bu ikisi tek bir kavram içinde birleştirilip nesne denmesidir. Ancak, **trait nesneleri**, veri ve davranışı birleştirdikleri için diğer dillerdeki nesnelere daha çok benzer. Yine de fark şudur: trait nesnelerine veri ekleyemeyiz. Trait nesnelerinin özel amacı, ortak davranışlar üzerinde soyutlama sağlamaktır.

Listeleme 18-3’te `Draw` trait’inin tanımı gösterilmiştir:

**Dosya adı:** src/lib.rs

```rust
pub trait Draw {
    fn draw(&self);
}
```

Listeleme 18-3: `Draw` trait’inin tanımı

Bu sözdizimi, 10. bölümde trait tanımlamayı tartıştığımız kısımdan tanıdık gelecektir. Şimdi yeni bir sözdizimi geliyor: Listeleme 18-4’te, `components` adında bir `vector` tutan `Screen` struct’ı tanımlanıyor. Bu `vector`’un türü `Box<dyn Draw>` olup, bir trait nesnesidir.

**Dosya adı:** src/lib.rs

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

Listeleme 18-4: `Draw` trait’ini uygulayan trait nesnelerini tutan bir `components` alanına sahip `Screen` struct’ı

`Screen` struct’ı üzerinde, her bir bileşenin `draw` metodunu çağıran `run` metodunu tanımlıyoruz (Listeleme 18-5).

**Dosya adı:** src/lib.rs

```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

Listeleme 18-5: `Screen` üzerindeki `run` metodu her bileşenin `draw` metodunu çağırıyor

Bu yaklaşım, trait sınırları (trait bounds) olan jenerik tür kullanan bir `struct` tanımlamaktan farklıdır. Jenerik tür parametresi yalnızca tek bir somut türle doldurulabilirken, trait nesneleri çalışma zamanında birden fazla somut türün aynı trait’i paylaşarak kullanılmasına imkân tanır. Örneğin, `Screen` struct’ını jenerikler ve trait sınırları ile de tanımlayabilirdik (Listeleme 18-6):

**Dosya adı:** src/lib.rs

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

Listeleme 18-6: Jenerikler ve trait sınırları kullanılarak tanımlanmış `Screen` struct’ı ve `run` metodu

Bu durumda, yalnızca tek bir `Screen` örneği oluşturabiliriz ve o örnekteki tüm bileşenler ya tamamen `Button`, ya da tamamen `TextField` olmalıdır. Eğer yalnızca homojen koleksiyonlara ihtiyacımız varsa, jenerikler ve trait sınırlarını kullanmak tercih edilir çünkü derleme zamanında monomorfize edilerek somut türler üzerinden çalışırlar.

Diğer taraftan, trait nesneleri ile yazılmış yöntem sayesinde bir `Screen` örneği, aynı `Vec` içinde hem `Box<Button>` hem de `Box<TextField>` tutabilir. Şimdi bunun nasıl çalıştığını görelim ve ardından çalışma zamanı performansı üzerindeki etkilerini tartışalım.


## 🛠️ Trait’in Uygulanması (Implementing the Trait)

Şimdi `Draw` trait’ini uygulayan bazı türler ekleyeceğiz. Burada `Button` türünü sağlayacağız. Aslında bir GUI kütüphanesi uygulamak bu kitabın kapsamı dışında olduğundan, `draw` metodunun gövdesinde gerçek bir işlevsel kod olmayacak. Ama nasıl olabileceğini hayal etmek için, bir `Button` struct’ı genişlik (`width`), yükseklik (`height`) ve etiket (`label`) alanlarına sahip olabilir (Listeleme 18-7).

**Dosya adı:** src/lib.rs

```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

Listeleme 18-7: `Draw` trait’ini uygulayan `Button` struct’ı

`Button` üzerindeki `width`, `height` ve `label` alanları, başka bileşen türlerindekilerden farklı olabilir. Örneğin, bir `TextField` türü aynı alanlara ek olarak `placeholder` alanına sahip olabilir. Ekrana çizmek istediğimiz her tür `Draw` trait’ini uygular, fakat her biri kendi `draw` metodunda farklı kod kullanarak o türün nasıl çizileceğini tanımlar. Örneğin, `Button` türü ayrıca, kullanıcı butona tıkladığında ne olacağını tanımlayan metotlar içeren bir `impl` bloğuna sahip olabilir. Bu tür metotlar `TextField` gibi bileşenlere uygun olmayabilir.

Kütüphanemizi kullanan biri `width`, `height` ve `options` alanlarına sahip `SelectBox` struct’ı tanımlamak isterse, `SelectBox` üzerinde de `Draw` trait’ini uygular (Listeleme 18-8).

**Dosya adı:** src/main.rs

```rust
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
```

Listeleme 18-8: Başka bir crate içinde `SelectBox` struct’ı üzerinde `Draw` trait’inin uygulanması

Artık kütüphane kullanıcımız, `Screen` örneği oluşturmak için `main` fonksiyonunu yazabilir. `Screen` örneğine, her birini `Box<T>` içine koyarak trait nesnesine dönüştürmek koşuluyla, hem `SelectBox` hem de `Button` ekleyebilir. Sonra `Screen` üzerindeki `run` metodunu çağırabilir; bu da her bileşenin `draw` metodunu çağırır (Listeleme 18-9).

**Dosya adı:** src/main.rs

```rust
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

Listeleme 18-9: Aynı trait’i uygulayan farklı türlerdeki değerleri saklamak için trait nesnelerinin kullanımı

Biz kütüphaneyi yazarken `SelectBox` türünün eklenebileceğini bilmiyorduk, ama `Screen` implementasyonu yeni tür üzerinde çalışabildi ve onu çizebildi. Çünkü `SelectBox`, `Draw` trait’ini uyguladığı için `draw` metoduna sahipti.

Bu kavram, bir değerin somut türünden ziyade yalnızca hangi mesajlara cevap verdiğiyle ilgilenmek anlamına gelir. Bu da dinamik olarak tür kontrol eden dillerdeki **duck typing** kavramına benzer: *“ördek gibi yürüyorsa ve ördek gibi vaklıyorsa, o zaman ördektir!”* Listeleme 18-5’teki `run` metodunda, her bileşenin somut türünü bilmemiz gerekmez. `Button` mı yoksa `SelectBox` mı olduğuna bakmadan sadece `draw` metodunu çağırır. `components` vektöründeki değerlerin türünü `Box<dyn Draw>` olarak belirleyerek, `Screen`’in `draw` metodunu çağırabileceğimiz değerlere ihtiyaç duyduğunu ifade etmiş olduk.

Trait nesneleri ve Rust’ın tip sistemini kullanarak duck typing’e benzer kod yazmanın avantajı, çalışma zamanında bir değerin belirli bir metodu uygulayıp uygulamadığını kontrol etmek zorunda olmamamızdır. Bir değer `Draw` trait’ini uygulamıyorsa, Rust kodumuzu derlemeyecektir.

Örneğin, Listeleme 18-10’da bir `Screen` örneğini `String` ile oluşturmaya çalışırsak:

**Dosya adı:** src/main.rs

```rust
// Bu kod derlenmez!
use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
}
```

Listeleme 18-10: Trait nesnesinin trait’ini uygulamayan bir türü kullanmaya çalışmak

Şu hatayı alırız çünkü `String`, `Draw` trait’ini uygulamamaktadır:

```
$ cargo run
   Compiling gui v0.1.0 (file:///projects/gui)
error[E0277]: the trait bound `String: Draw` is not satisfied
 --> src/main.rs:5:26
  |
5 |         components: vec![Box::new(String::from("Hi"))],
  |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
  |
  = help: the trait `Draw` is implemented for `Button`
  = note: required for the cast from `Box<String>` to `Box<dyn Draw>`
```

Bu hata, `Screen`’e yanlışlıkla istemediğimiz bir şey göndermiş olabileceğimizi veya `String` türüne `Draw` trait’ini uygulamamız gerektiğini söyler.

---

## ⚡ Trait Nesneleri Dinamik Yönlendirme Yapar (Trait Objects Perform Dynamic Dispatch)

10. bölümde, jenerikler için derleyici tarafından gerçekleştirilen **monomorfizasyon (monomorphization)** sürecini tartışmıştık: Derleyici, jenerik tür parametrelerinin yerine kullanılan her somut tür için jenerik olmayan implementasyonlar üretir. Bu türden kod **statik yönlendirme (static dispatch)** yapar, yani derleyici hangi metodun çağrıldığını derleme zamanında bilir.

Buna karşılık, **dinamik yönlendirme (dynamic dispatch)** derleyicinin çağrılacak metodu derleme zamanında bilmediği durumdur. Bu durumda derleyici, çalışma zamanında hangi metodun çağrılacağını çözecek kod üretir.

Trait nesneleri kullandığımızda, Rust dinamik yönlendirme yapmak zorundadır. Çünkü derleyici, trait nesneleriyle kullanılan tüm olası türleri bilemez. Bunun yerine, çalışma zamanında trait nesnesi içindeki işaretçiler sayesinde hangi metodun çağrılacağını belirler. Bu arama, statik yönlendirmede olmayan bir **çalışma zamanı maliyeti** getirir. Ayrıca, dinamik yönlendirme derleyicinin metodun kodunu inline etmesini engeller ve bazı optimizasyonların yapılamamasına yol açar. Rust, **dyn uyumluluğu (dyn compatibility)** adı verilen bazı kurallarla dinamik yönlendirmeyi nerede kullanıp kullanamayacağınızı belirler. Bu kurallar bu tartışmanın kapsamı dışındadır, ancak Rust referansında daha fazla bilgi bulabilirsiniz.

Yine de, Listeleme 18-5 ve 18-9’da gördüğümüz gibi, yazdığımız koda esneklik kazandırır. Bu da bir **esneklik ile performans arasında denge** (trade-off) meselesidir.
