## 🧬 Genel Türler, Özellikler ve Ömürler (generic types, traits, and lifetimes)

Her programlama dili, kavramların yinelenmesini (duplication) etkili biçimde ele almak için araçlara sahiptir. Rust’ta bu araçlardan biri **genel türler (generics)**’tir: somut türlerin (concrete types) veya diğer özelliklerin yerini alan soyut **yer tutucular (placeholders)**. Genel türlerin davranışını (behavior) veya diğer genel türlerle ilişkilerini, kodu derlerken (compile) ve çalıştırırken (run) yerlerine ne geleceğini bilmeden ifade edebiliriz.

Fonksiyonlar (functions), i32 veya String gibi somut bir tür yerine, aynı kodu birden çok somut değer üzerinde çalıştırmak için bilinmeyen değerlere parametre aldıkları gibi, bir **genel tür (generic type)** de alabilir. Aslında, Bölüm 6’da `Option<T>`, Bölüm 8’de `Vec<T>` ve `HashMap<K, V>`, Bölüm 9’da `Result<T, E>` ile genel türleri zaten kullandık. Bu bölümde, kendi türlerinizi (types), fonksiyonlarınızı (functions) ve yöntemlerinizi (methods) genel türlerle nasıl tanımlayacağınızı keşfedeceksiniz!

Önce, kod yinelenmesini azaltmak için bir fonksiyonun nasıl ayıklanacağını (extraction) gözden geçireceğiz. Ardından, yalnızca parametre türleri bakımından farklılık gösteren iki fonksiyondan, aynı tekniği kullanarak genel bir fonksiyon elde edeceğiz. Ayrıca genel türlerin `struct` ve `enum` tanımlarında nasıl kullanılacağını açıklayacağız.

Sonra, **özellikleri (traits)** genel bir şekilde davranış tanımlamak için nasıl kullanacağınızı öğreneceksiniz. Özellikleri genel türlerle birleştirerek, bir genel türü herhangi bir tür yerine belirli bir davranışa sahip türlerle sınırlandırabilirsiniz.

Son olarak, **ömürleri (lifetimes)** ele alacağız: başvuruların (references) birbirleriyle nasıl ilişkili olduğuna dair derleyiciye bilgi veren bir çeşit genel tür. Ömürler, ödünç alınan (borrowed) değerlere ilişkin derleyiciye yeterli bilgiyi vererek, bizim yardımımız olmadan mümkün olandan daha fazla durumda başvuruların geçerli olmasını sağlar.

## ✂️ Bir Fonksiyonu Ayıklayarak Yinelemeyi Gidermek (extracting a function)

Genel türler, yinelenen kodu kaldırmak için belirli türleri birden çok türü temsil eden bir **yer tutucu (placeholder)** ile değiştirmemize olanak tanır. Genel sözdizimine dalmadan önce, belirli değerleri birden çok değeri temsil eden bir yer tutucu ile değiştiren bir fonksiyonu ayıklayarak, genel türleri içermeyen bir yolla yinelenmeyi nasıl kaldıracağımıza bakalım. Sonra aynı tekniği uygulayıp genel bir fonksiyon çıkaracağız! Hangi yinelenen kodu bir fonksiyona ayıklayabileceğinizi görerek, genel türler kullanabilecek yinelenen kodu da fark etmeye başlayacaksınız.

Liste 10-1’de, bir listedeki en büyük sayıyı bulan kısa bir programla başlayacağız.

**Dosya adı: src/main.rs**

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}
```

Liste 10-1: Bir sayı listesindeki en büyük sayıyı bulma

Tamsayılar listesini `number_list` değişkeninde saklıyor ve listedeki ilk sayıya olan bir başvuruyu (reference) `largest` adlı değişkende tutuyoruz. Ardından listedeki tüm sayıları dolaşıyoruz ve mevcut sayı `largest`’ta saklanan sayıdan büyükse, bu değişkendeki başvuruyu değiştiriyoruz. Ancak mevcut sayı, şimdiye kadar görülen en büyük sayıdan küçük veya ona eşitse, değişken değişmez ve kod listedeki bir sonraki sayıya geçer. Listedeki tüm sayılar değerlendirildikten sonra, `largest` bu durumda 100 olan en büyük sayıyı işaret etmelidir.

Artık iki farklı sayı listesinde en büyük sayıyı bulma görevi verilmiş olsun. Bunu yapmak için, Liste 10-1’deki kodu çoğaltmayı ve programın iki farklı yerinde aynı mantığı kullanmayı seçebiliriz; bu, Liste 10-2’de gösterilmiştir.

**Dosya adı: src/main.rs**

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}
```

Liste 10-2: İki sayı listesinde en büyük sayıyı bulmak için kod

Bu kod çalışsa da, kodu çoğaltmak zahmetli ve hataya açıktır. Ayrıca, bir değişiklik yapmak istediğimizde birden çok yerdeki kodu güncellemeyi hatırlamamız gerekir.

Bu yinelemeyi ortadan kaldırmak için, parametre olarak geçirilen herhangi bir tamsayı listesi üzerinde çalışan bir fonksiyon tanımlayarak soyutlama (abstraction) oluşturacağız. Bu çözüm, kodumuzu daha anlaşılır kılar ve bir listede en büyük sayıyı bulma kavramını soyut şekilde ifade etmemizi sağlar.

Liste 10-3’te, en büyük sayıyı bulan kodu `largest` adlı bir fonksiyona ayıklıyoruz. Ardından, Liste 10-2’deki iki listedeki en büyük sayıyı bulmak için bu fonksiyonu çağırıyoruz. Ayrıca bu fonksiyonu gelecekte sahip olabileceğimiz herhangi bir `i32` değer listesinden oluşan dilim (slice) üzerinde de kullanabiliriz.

**Dosya adı: src/main.rs**

```rust
fn largest(list: &[i32]) -> &i32 {
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

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");
}
```

Liste 10-3: İki listede en büyük sayıyı bulmak için ayıklanmış (abstracted) kod

`largest` fonksiyonunun, fonksiyona geçirebileceğimiz herhangi bir somut `i32` değer dilimini temsil eden `list` adlı bir parametresi vardır. Sonuç olarak, fonksiyonu çağırdığımızda, kod aktardığımız somut değerler üzerinde çalışır.

Özetle, Liste 10-2’deki kodu Liste 10-3’e dönüştürmek için attığımız adımlar şunlardır:

* Yinelenen kodu belirleyin.
* Yinelenen kodu bir fonksiyonun gövdesine ayıklayın ve o kodun girdilerini ile dönüş değerlerini fonksiyon imzasında belirtin.
* Yinelenen kodun iki örneğini de fonksiyonu çağıracak şekilde güncelleyin.

Sırada, genel türlerle aynı adımları kullanarak kod yinelenmesini azaltmak var. Fonksiyon gövdesinin belirli değerler yerine soyut bir liste üzerinde çalışabilmesi gibi, genel türler de kodun soyut türler (abstract types) üzerinde çalışmasına izin verir.

Örneğin, elimizde iki fonksiyon olduğunu varsayalım: biri `i32` değerlerinden oluşan bir dilimdeki en büyük öğeyi bulan, diğeri ise `char` değerlerinden oluşan bir dilimdeki en büyük öğeyi bulan. Bu yinelenmeyi nasıl ortadan kaldırırdık? Hadi keşfedelim!
