## 📚 Ek C: Türetilen Trait’ler (derivable traits)

Kitap boyunca, `derive` özniteliğini (attribute) tartıştık; bunu bir `struct` veya `enum` tanımına uygulayabilirsiniz. `derive` özniteliği, üzerine `derive` sözdizimiyle açıklama eklediğiniz tür için, ilgili trait’i kendi varsayılan (default) implementasyonu ile uygulayacak kodu üretir.

Bu ekte, standart kütüphanede `derive` ile kullanılabilen tüm trait’lere bir başvuru sunulmaktadır. Her bölüm şunları kapsar:

* Bu trait türetildiğinde (derive) hangi operatör ve metotların etkinleşeceği
* Türetilen trait’in sağladığı implementasyonun ne yaptığı
* Trait’i uygulamanın tür hakkında ne ifade ettiği
* Hangi koşullar altında trait’i uygulamanın mümkün olup olmadığı
* Trait gerektiren işlem örnekleri

Eğer `derive` özniteliğinin sağladığından farklı bir davranış isterseniz, bu trait’leri manuel olarak nasıl uygulayabileceğinize dair ayrıntılar için standart kütüphane belgelerine bakınız.

Burada listelenen trait’ler, standart kütüphane tarafından tanımlanmış ve `derive` ile uygulanabilen tek trait’lerdir. Diğer trait’lerin makul bir varsayılan davranışı yoktur; bunları neyi başarmaya çalıştığınıza göre sizin tanımlamanız gerekir.

Türetilmesi mümkün olmayan bir trait örneği `Display`’dir; bu trait son kullanıcılar için biçimlendirme sağlar. Bir türün son kullanıcıya nasıl gösterileceğini daima dikkatle düşünmelisiniz. Kullanıcı türün hangi kısımlarını görmelidir? Hangileri onun için ilgilidir? Verilerin hangi biçimi en anlamlıdır? Rust derleyicisinin bu bilgiye sahip olmaması nedeniyle sizin için uygun bir varsayılan davranış üretemez.

Bu ekte sunulan türetilebilir trait listesi kapsamlı değildir: kütüphaneler kendi trait’leri için `derive` implementasyonu sağlayabilir, bu nedenle `derive` ile kullanılabilecek trait listesi aslında açık uçludur. `derive` uygulamak, 20. Bölümdeki “Makrolar (macros)” bölümünde ele alınan yordam makrolarını (procedural macros) kullanmayı içerir.

---

## 🐞 Programcı Çıktısı için Debug

`Debug` trait’i, biçimlendirme (format) dizgilerinde `{:?}` gösterimiyle hata ayıklama biçimlendirmesini (debug formatting) etkinleştirir.

* `Debug` trait’i, bir türün örneklerini hata ayıklama amacıyla yazdırmanıza olanak sağlar; böylece siz ve türünüzü kullanan diğer programcılar, bir programın yürütülmesinin belirli bir noktasındaki örneği inceleyebilirsiniz.
* Örneğin, `assert_eq!` makrosu `Debug` gerektirir; çünkü eşitlik doğrulaması başarısız olduğunda makro, neden eşit olmadıklarını görmek için verilen örneklerin değerlerini yazdırır.

---

## ⚖️ Eşitlik Karşılaştırmaları için PartialEq ve Eq

* `PartialEq` trait’i, `==` ve `!=` operatörlerini etkinleştirerek, bir türün örneklerinin eşit olup olmadığını karşılaştırmanıza olanak tanır.
* `derive(PartialEq)` çağrıldığında `eq` metodu uygulanır.

  * Struct’larda: İki örnek, tüm alanlar eşitse eşit kabul edilir; herhangi bir alan farklıysa eşit değildir.
  * Enum’larda: Her varyant yalnızca kendisiyle eşittir; diğer varyantlarla eşit değildir.
* `Eq` trait’inin hiçbir metodu yoktur. Amacı, açıklama eklenen türün her değeri için değerin kendisine eşit olduğunu garanti etmektir.

  * `Eq` yalnızca `PartialEq` uygulayan türlere uygulanabilir.
  * Ancak her `PartialEq` türü `Eq` olamaz. Örneğin, `NaN` değerine sahip kayan nokta türlerinde `NaN != NaN` olduğu için `Eq` uygulanamaz.

Örnek: `HashMap<K, V>`’de anahtar olarak kullanılabilmek için türün `Eq` (ve `Hash`) uygulaması gerekir.

---

## 📊 Sıralama Karşılaştırmaları için PartialOrd ve Ord

* `PartialOrd` trait’i, bir türün örneklerini `<`, `>`, `<=`, `>=` operatörleriyle karşılaştırarak sıralamanıza olanak sağlar.
* `derive(PartialOrd)`, `partial_cmp` metodunu uygular. Bu metod `Option<Ordering>` döndürür ve değerler sıralanamazsa `None` döner.

  * Örneğin, `NaN` ile herhangi bir sayı karşılaştırıldığında sonuç `None` olur.
* Struct’larda: Alanlar tanım sırasına göre karşılaştırılır.
* Enum’larda: Enum’da daha önce tanımlanan varyantlar, daha sonra tanımlananlardan küçüktür.

`Ord` trait’i, her iki değer arasında daima geçerli bir sıralamanın var olduğunu garanti eder.

* `cmp` metodu uygular ve `Ordering` döndürür.
* `Ord`, yalnızca `PartialOrd` ve `Eq` (dolayısıyla `PartialEq`) uygulayan türlerde kullanılabilir.

Örnek: `BTreeSet<T>` yapısında değerler, sıralamaya göre depolandığından `Ord` gereklidir.

---

## 🌀 Değerleri Kopyalamak için Clone ve Copy

* `Clone` trait’i, bir değerin derin kopyasını (deep copy) açıkça oluşturmanıza olanak tanır.

  * `derive(Clone)` çağrıldığında `clone` metodu uygulanır ve türün tüm alanlarına ayrı ayrı `clone` çağrılır.
  * Bu nedenle, türün tüm alanları `Clone` olmalıdır.
  * Örnek: `slice.to_vec()` çağrısı. Slice, verilerin sahibi değildir; `Vec` dönerken öğelerin sahibi olmalıdır, bu yüzden her öğe `clone` edilir.

* `Copy` trait’i, yalnızca yığıtta (stack) saklanan bitleri kopyalayarak bir değeri kopyalamanıza olanak tanır; ek kod çalıştırılmaz.

  * `Copy` hiçbir metot tanımlamaz, böylece kopyalamanın daima çok hızlı olacağı garanti edilir.
  * `Copy` olan her tür ayrıca `Clone` da olmalıdır. `Copy`, `Clone`’un önemsiz (trivial) bir versiyonunu uygular.
  * Örnek: Tamsayılar (`i32`, `u64`), `Copy`’dir.

Not: `Copy` ile yapılabilen her şey `Clone` ile de yapılabilir, ancak kod daha yavaş olabilir veya `clone` çağrısı gerekebilir.

---

## 🔑 Sabit Boyutlu Değere Eşleme için Hash

* `Hash` trait’i, keyfi boyuttaki bir tür örneğini alıp sabit boyutlu bir değere eşlemenize olanak tanır.
* `derive(Hash)` çağrıldığında `hash` metodu uygulanır ve türün her parçasına `hash` çağrılır.
* Örnek: `HashMap<K, V>`’de anahtarlar verimli depolama için `Hash` implementasyonu gerektirir.

---

## 🏷️ Varsayılan Değerler için Default

* `Default` trait’i, bir tür için varsayılan değer oluşturmanızı sağlar.
* `derive(Default)` çağrıldığında `default` fonksiyonu uygulanır ve tüm alanlara `default` çağrılır.
* `Default::default` sıkça struct güncelleme sözdizimi (struct update syntax) ile birlikte kullanılır:

```rust
let s = StructType { x: 1, ..Default::default() };
```

* `unwrap_or_default` metodunda `Default` gereklidir. Örneğin, bir `Option<T>` `None` ise, `unwrap_or_default` çağrısı `T::default()` döndürür.
