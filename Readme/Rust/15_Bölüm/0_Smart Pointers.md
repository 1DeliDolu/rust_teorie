## 📦 Akıllı İşaretçiler (smart pointers)

Bir işaretçi (pointer), bellekteki bir adresi içeren değişken için kullanılan genel bir kavramdır. Bu adres başka bir veriye başvurur ya da “işaret eder”. Rust’taki en yaygın işaretçi türü, 4. bölümde öğrendiğiniz başvurudur (reference). Başvurular `&` sembolü ile gösterilir ve işaret ettikleri değeri ödünç alırlar (borrow). Veriye başvurmaktan başka özel bir yetenekleri yoktur ve ek bir maliyetleri bulunmaz.

Akıllı işaretçiler (smart pointers) ise bir işaretçi gibi davranan ama ek meta veriler ve yetenekler barındıran veri yapılarıdır. Akıllı işaretçi kavramı Rust’a özgü değildir: akıllı işaretçiler C++’ta ortaya çıkmıştır ve başka dillerde de vardır. Rust’ın standart kütüphanesinde, başvuruların sunduğunun ötesinde işlevsellik sağlayan çeşitli akıllı işaretçiler tanımlanmıştır. Genel kavramı keşfetmek için, aralarında bir başvuru sayma (reference counting) türünün de olduğu birkaç akıllı işaretçi örneğine bakacağız. Bu işaretçi, sahiplerin sayısını takip ederek verinin birden fazla sahip tarafından kullanılmasına imkân verir ve hiçbir sahip kalmadığında veriyi temizler.

Rust, sahiplik (ownership) ve ödünç alma (borrowing) kavramlarıyla birlikte, başvurular ve akıllı işaretçiler arasında ek bir farklılığa sahiptir: başvurular yalnızca veriyi ödünç alırken, birçok durumda akıllı işaretçiler işaret ettikleri verinin sahibidir.

Her ne kadar o anda bu şekilde adlandırmamış olsak da, bu kitapta daha önce birkaç akıllı işaretçiyle karşılaştık: 8. bölümdeki `String` ve `Vec<T>`. Bu türlerin ikisi de bir miktar belleğe sahip oldukları ve onu yönetmenize izin verdikleri için akıllı işaretçi sayılır. Ayrıca meta verileri ve ek yetenekleri veya garantileri vardır. Örneğin `String`, kapasitesini meta veri olarak saklar ve verisinin her zaman geçerli UTF-8 olacağını garanti eder.

Akıllı işaretçiler genellikle `struct` kullanılarak uygulanır. Normal bir `struct`’tan farklı olarak, akıllı işaretçiler `Deref` ve `Drop` trait’lerini uygular. `Deref` trait’i, akıllı işaretçi `struct` örneğinin bir başvuru gibi davranmasını sağlar; böylece kodunuzu hem başvurularla hem de akıllı işaretçilerle çalışacak şekilde yazabilirsiniz. `Drop` trait’i ise, akıllı işaretçi örneği kapsam (scope) dışına çıktığında çalışacak kodu özelleştirmenize olanak tanır. Bu bölümde bu iki trait’i tartışacak ve akıllı işaretçiler için neden önemli olduklarını göstereceğiz.

Akıllı işaretçi deseni (smart pointer pattern), Rust’ta sıkça kullanılan genel bir tasarım deseni olduğundan, bu bölümde var olan tüm akıllı işaretçileri kapsamayacağız. Birçok kütüphanenin kendi akıllı işaretçileri vardır ve siz de kendi akıllı işaretçinizi yazabilirsiniz. Biz standart kütüphanedeki en yaygın akıllı işaretçileri ele alacağız:

* `Box<T>`, değerleri heap üzerinde ayırmak için
* `Rc<T>`, birden fazla sahipliğe imkân veren başvuru sayma türü
* `Ref<T>` ve `RefMut<T>`, `RefCell<T>` aracılığıyla erişilen ve ödünç alma kurallarını derleme zamanında değil çalışma zamanında uygulayan tür

Buna ek olarak, değiştirilemez (immutable) bir türün içsel (interior) bir değeri değiştirmek için API sunduğu içsel değiştirilebilirlik (interior mutability) desenini ele alacağız. Ayrıca başvuru döngülerini (reference cycles), bunların belleği nasıl sızdırabileceğini (memory leak) ve nasıl önlenebileceğini tartışacağız.

Haydi başlayalım!
