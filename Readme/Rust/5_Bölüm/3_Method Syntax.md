## 🛠️ Metot Sözdizimi (method syntax)

Metotlar (methods), fonksiyonlara (functions) benzer: `fn` anahtar sözcüğüyle ve bir isimle bildirilirler, parametreleri ve dönüş değeri olabilir ve başka bir yerden çağrıldıklarında çalıştırılan bir kod bloğu içerirler. Fonksiyonlardan farklı olarak, metotlar bir `struct` (veya `enum` ya da `trait object`, sırasıyla Bölüm 6 ve Bölüm 18’de ele alınacak) bağlamında tanımlanır ve ilk parametreleri her zaman `self` olur; bu parametre, metodun çağrıldığı `struct` örneğini (instance) temsil eder.

## 📐 Metot Tanımlama (defining methods)

Bir `Rectangle` örneğini parametre olarak alan `area` fonksiyonunu değiştirelim ve bunun yerine `Rectangle` yapısında tanımlı bir `area` metodu oluşturalım. Bu, Listeleme 5-13’te gösterilmiştir.

Dosya adı: `src/main.rs`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

Listeleme 5-13: `Rectangle` yapısında bir `area` metodunun tanımlanması

Bir fonksiyonu `Rectangle` bağlamında tanımlamak için `impl` (implementation) bloğu başlatırız. Bu `impl` bloğu içindeki her şey `Rectangle` türüyle ilişkilendirilir. Daha sonra `area` fonksiyonunu bu süslü parantezlerin içine taşırız ve ilk (ve bu durumda tek) parametreyi imzada ve gövdede `self` olacak şekilde değiştiririz. `main` fonksiyonunda, `area` fonksiyonunu çağırıp `rect1`’i argüman olarak vermek yerine, `Rectangle` örneğimizde `area` metodunu doğrudan metot sözdizimiyle çağırabiliriz. Metot sözdizimi bir örneğin ardından gelir: bir nokta ekleriz, ardından metodun adı, parantezler ve varsa argümanlar gelir.

`area` imzasında `rectangle: &Rectangle` yerine `&self` kullandık. `&self` aslında `self: &Self` ifadesinin kısaltmasıdır. Bir `impl` bloğu içinde `Self` türü, o `impl` bloğunun ait olduğu türün takma adıdır (alias). Metotların ilk parametresi olarak `Self` türünde `self` adında bir parametre bulunmalıdır; bu yüzden Rust yalnızca `self` yazarak kısaltmaya izin verir. Ancak burada da, bu metodun `Self` örneğini ödünç aldığını belirtmek için `&` kullanmamız gerekir, tıpkı `rectangle: &Rectangle` örneğinde olduğu gibi. Metotlar, `self`’in sahipliğini (ownership) alabilir, onu değişmez (immutable) ödünç alabilir (burada yaptığımız gibi) veya değişebilir (mutable) ödünç alabilir; tıpkı diğer parametrelerde olduğu gibi.

Burada `&self` seçmemizin nedeni, fonksiyon versiyonunda `&Rectangle` kullandığımızla aynıdır: sahipliği almak istemiyoruz, yalnızca yapının verisini okumak istiyoruz, değiştirmek değil. Eğer metodun yaptığı işin bir parçası olarak çağrıldığı örneği değiştirmek isteseydik, ilk parametre olarak `&mut self` kullanırdık. Sadece `self` kullanarak örneğin sahipliğini alan metotlar nadirdir; bu teknik genellikle metot `self`’i başka bir şeye dönüştürdüğünde ve dönüşümden sonra çağıranın orijinal örneği kullanmasını engellemek istediğimizde kullanılır.

Metotları fonksiyonlar yerine kullanmanın başlıca nedeni, metot sözdizimi sağlamanın yanı sıra her metot imzasında `self` türünü tekrar etmek zorunda kalmamaktır ve organizasyon kolaylığıdır. Bir türün örneğiyle yapılabilecek her şeyi tek bir `impl` bloğu içine koyarız; böylece kütüphanemizi kullananların `Rectangle` için yetenekleri çeşitli yerlerde aramak zorunda kalmalarını önleriz.

## 🏷️ Alanlarla Aynı İsme Sahip Metotlar

Bir metoda, yapının alanlarından biriyle aynı ismi verme seçeneğimiz vardır. Örneğin, `Rectangle` üzerinde `width` adında bir metot tanımlayabiliriz:

Dosya adı: `src/main.rs`

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

Burada, `width` metodunun, örnekteki `width` alanının değeri 0’dan büyükse `true`, aksi halde `false` döndürmesini seçtik. Aynı ada sahip bir metodun içinde, bu alanı istediğimiz şekilde kullanabiliriz. `main` fonksiyonunda, `rect1.width` ifadesini parantezlerle kullandığımızda Rust bunun `width` metodu olduğunu anlar. Parantez kullanmadığımızda ise bunun `width` alanı olduğunu anlar.

Çoğu zaman (ama her zaman değil) bir metoda alanla aynı isim verdiğimizde, metodun yalnızca bu alandaki değeri döndürmesini ve başka bir şey yapmamasını isteriz. Bu tür metotlara getter denir ve Rust, bazı diğer dillerde olduğu gibi getter’ları `struct` alanları için otomatik olarak uygulamaz. Getter’lar faydalıdır çünkü alanı özel (private) yapabilir, ama metodu herkese açık (public) yaparak bu alana yalnızca-okuma (read-only) erişimi sağlayabilirsiniz. Bir alan veya metodu herkese açık (public) veya özel (private) yapmanın nasıl olacağını Bölüm 7’de tartışacağız.

## ❓ `->` Operatörü Nerede? (`-> operator`)

C ve C++’ta, metot çağırmak için iki farklı operatör kullanılır: bir nesne (object) üzerinde doğrudan metot çağırıyorsanız `.` kullanırsınız, eğer nesneye işaret eden bir gösterici (pointer) üzerinden metot çağırıyorsanız ve önce o göstericiyi açmanız gerekiyorsa `->` kullanırsınız. Başka bir deyişle, eğer `object` bir gösterici ise, `object->something()` ifadesi `(*object).something()` ifadesine benzer.

Rust’ta `->` operatörünün bir karşılığı yoktur; bunun yerine Rust’ta otomatik referanslama ve dereferanslama (automatic referencing and dereferencing) adı verilen bir özellik vardır. Metot çağırma, Rust’ta bu davranışın geçerli olduğu birkaç yerden biridir.

## ⚙️ Otomatik Referanslama ve Dereferanslama (automatic referencing and dereferencing)

Bu özellik şöyle çalışır: `object.something()` şeklinde bir metot çağırdığınızda, Rust otomatik olarak `&`, `&mut` veya `*` ekler; böylece `object`, metodun imzasıyla eşleşir. Başka bir deyişle, aşağıdaki iki ifade aynıdır:

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

İlk yazım çok daha temiz görünür. Bu otomatik referanslama davranışı, metotların net bir alıcıya (receiver) sahip olması sayesinde çalışır — yani `self` türü. Bir alıcı (receiver) ve metot adı verildiğinde, Rust kesin olarak metodun okuma (`&self`), değiştirme (`&mut self`) veya sahipliği alma (`self`) işlemi yaptığını çözümleyebilir. Rust’ın metot alıcılarında ödünç almayı (borrowing) örtük hale getirmesi, sahiplik (ownership) modelinin pratikte ergonomik olmasının önemli bir parçasıdır.
## 📏 Daha Fazla Parametre Alan Metotlar (methods with more parameters)

Metot kullanımını pratik edelim ve `Rectangle` yapısında ikinci bir metot tanımlayalım. Bu sefer bir `Rectangle` örneğinin başka bir `Rectangle` örneğini almasını ve eğer ikinci `Rectangle` tamamen `self` (ilk `Rectangle`) içine sığabiliyorsa `true`, aksi durumda `false` döndürmesini istiyoruz. Yani `can_hold` metodunu tanımladıktan sonra, Listeleme 5-14’te gösterilen programı yazabilmek istiyoruz.

Dosya adı: `src/main.rs`

```rust
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

Listeleme 5-14: Henüz yazılmamış `can_hold` metodunun kullanılması

Beklenen çıktı şu şekilde olacaktır, çünkü `rect2`’nin her iki boyutu da `rect1`’inkinden küçüktür, fakat `rect3`, `rect1`’den daha geniştir:

```
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

Bir metot tanımlamak istediğimizi biliyoruz, dolayısıyla bu metot `impl Rectangle` bloğu içinde olacak. Metodun adı `can_hold` olacak ve başka bir `Rectangle` örneğini değişmez ödünçleme (immutable borrow) yoluyla parametre olarak alacak. Parametrenin türünü, metodu çağıran koda bakarak anlayabiliriz: `rect1.can_hold(&rect2)` ifadesinde `&rect2` geçiriliyor; bu, `Rectangle` türünde bir örnek olan `rect2`’ye değişmez ödünçlemedir. Bu mantıklıdır çünkü `rect2`’yi yalnızca okumamız gerekir (yazmamız gerekseydi değişebilir ödünçleme olurdu), ayrıca `main` fonksiyonunun `rect2`’nin sahipliğini korumasını ve `can_hold` çağrısından sonra da onu kullanabilmesini isteriz.

`can_hold` metodunun dönüş değeri `bool` olacak ve uygulama, `self`’in `width` ve `height` değerlerinin diğer `Rectangle`’ın `width` ve `height` değerlerinden büyük olup olmadığını kontrol edecektir. Yeni `can_hold` metodunu, Listeleme 5-13’teki `impl` bloğuna ekleyelim; bu, Listeleme 5-15’te gösterilmiştir.

Dosya adı: `src/main.rs`

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

Listeleme 5-15: Başka bir `Rectangle` örneğini parametre olarak alan `can_hold` metodunun `Rectangle` üzerinde uygulanması

Bu kodu Listeleme 5-14’teki `main` fonksiyonuyla çalıştırdığımızda istediğimiz çıktıyı elde ederiz. Metotlar, `self` parametresinden sonra imzaya eklediğimiz birden fazla parametre alabilirler ve bu parametreler fonksiyonlardaki parametrelerle aynı şekilde çalışır.

## 🧩 İlişkili Fonksiyonlar (associated functions)

Bir `impl` bloğu içinde tanımlanan tüm fonksiyonlara, `impl` ifadesinden sonra gelen türe bağlı oldukları için ilişkili fonksiyonlar denir. İlk parametreleri `self` olmayan ilişkili fonksiyonlar (yani metot olmayanlar) tanımlayabiliriz; çünkü bu fonksiyonların çalışmak için türün bir örneğine ihtiyaçları yoktur. Daha önce bu tür bir fonksiyon kullandık: `String` türünde tanımlı olan `String::from` fonksiyonu.

Metot olmayan ilişkili fonksiyonlar genellikle `struct` için yeni bir örnek döndüren yapıcılar (constructors) olarak kullanılır. Bunlar çoğunlukla `new` olarak adlandırılır, fakat `new` özel bir isim değildir ve dile gömülü değildir. Örneğin, yalnızca tek bir boyut parametresi alıp onu hem `width` hem de `height` olarak kullanan ve böylece aynı değeri iki kez belirtmek zorunda kalmadan kare bir `Rectangle` oluşturmayı kolaylaştıran `square` adında bir ilişkili fonksiyon tanımlayabiliriz:

Dosya adı: `src/main.rs`

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

Dönüş türünde ve fonksiyon gövdesinde kullanılan `Self` anahtar kelimeleri, `impl` ifadesinden sonra görünen türün takma adıdır; bu durumda `Rectangle`.

Bu ilişkili fonksiyonu çağırmak için, `struct` adıyla birlikte `::` sözdizimini kullanırız. Örneğin:

```rust
let sq = Rectangle::square(3);
```

Bu fonksiyon, `struct` tarafından ad alanına (namespace) alınmıştır: `::` sözdizimi hem ilişkili fonksiyonlarda hem de modüller tarafından oluşturulan ad alanlarında kullanılır. Modülleri Bölüm 7’de ele alacağız.

## 📦 Birden Fazla `impl` Bloğu (multiple impl blocks)

Her `struct`, birden fazla `impl` bloğuna sahip olabilir. Örneğin, Listeleme 5-15, her metodun kendi `impl` bloğunda bulunduğu Listeleme 5-16’daki koda eşdeğerdir:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

Listeleme 5-16: Listeleme 5-15’in birden fazla `impl` bloğu kullanılarak yeniden yazılması

Bu örnekte metotları birden fazla `impl` bloğuna ayırmak için bir neden yok, ancak bu geçerli bir sözdizimidir. Birden fazla `impl` bloğunun faydalı olduğu bir durumu Bölüm 10’da, jenerik (generic) türler ve trait’ler konusunu işlerken göreceğiz.

## 📝 Özet (summary)

`Struct`’lar, alanınıza (domain) anlam katan özel türler (custom types) oluşturmanıza olanak tanır. `Struct` kullanarak ilişkili veri parçalarını birbirine bağlı tutabilir ve her parçayı adlandırarak kodunuzu anlaşılır hale getirebilirsiniz. `impl` bloklarında, türünüzle ilişkili fonksiyonlar tanımlayabilirsiniz ve metotlar, `struct` örneklerinin davranışlarını belirtmenizi sağlayan bir tür ilişkili fonksiyondur.

Fakat özel türler oluşturmanın tek yolu `struct` değildir: şimdi araç kutunuza başka bir araç eklemek için Rust’ın `enum` özelliğine geçelim.
