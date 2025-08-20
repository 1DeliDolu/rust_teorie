## 👋 Giriş (Introduction)

Not: Bu kitabın bu baskısı, **No Starch Press** tarafından basılı ve e-kitap formatında sunulan *The Rust Programming Language* ile aynıdır.

*The Rust Programming Language* kitabına hoş geldiniz; bu kitap Rust hakkında giriş seviyesinde bir kaynaktır. Rust programlama dili, daha hızlı ve daha güvenilir yazılım geliştirmenize yardımcı olur. Programlama dili tasarımında, yüksek seviyeli ergonomi ile düşük seviyeli kontrol genellikle birbiriyle çelişir; Rust ise bu çatışmaya meydan okur. Güçlü teknik kapasite ile harika bir geliştirici deneyimi arasında denge kurarak Rust, size düşük seviyeli ayrıntıları (örneğin bellek kullanımı) kontrol etme imkânı verir — üstelik geleneksel olarak bu tür kontrolle ilişkilendirilen zahmet olmadan.
## 🎯 Rust Kimler İçin (Who Rust Is For)

Rust, birçok kişi için farklı nedenlerle ideal bir dildir. Hadi en önemli gruplardan birkaçına bakalım.

### 👥 Geliştirici Ekipleri (Teams of Developers)

Rust, farklı seviyelerde sistem programlama bilgisine sahip büyük geliştirici ekipleri arasında işbirliği için verimli bir araç olduğunu kanıtlamaktadır. Düşük seviyeli kod, çoğu başka dilde yalnızca kapsamlı testler ve deneyimli geliştiricilerin dikkatli kod incelemeleriyle yakalanabilen çeşitli ince hatalara yatkındır. Rust’ta ise derleyici (compiler), bu zor yakalanan hatalar — eşzamanlılık (concurrency) hataları da dahil — içeren kodu derlemeyi reddederek bir bekçi (gatekeeper) rolü üstlenir. Derleyiciyle birlikte çalışarak ekip, hataların peşinden koşmak yerine zamanını programın mantığına odaklanmaya ayırabilir.

Rust ayrıca, sistem programlama dünyasına çağdaş geliştirici araçları da getirir:

* **Cargo**: Dahili bağımlılık yöneticisi (dependency manager) ve derleme aracı (build tool), Rust ekosistemi genelinde bağımlılık eklemeyi, derlemeyi ve yönetmeyi zahmetsiz ve tutarlı hale getirir.
* **Rustfmt**: Biçimlendirme aracı (formatting tool), geliştiriciler arasında tutarlı bir kodlama stilini garanti eder.
* **rust-analyzer**: Entegre Geliştirme Ortamı (IDE) entegrasyonunu sağlayarak kod tamamlama ve satır içi hata mesajlarını mümkün kılar.

Rust ekosistemindeki bu ve diğer araçlar sayesinde geliştiriciler, sistem düzeyinde (systems-level) kod yazarken de verimli olabilirler.
## 🎓 Öğrenciler (Students)

Rust, öğrenciler ve sistem kavramlarını öğrenmekle ilgilenenler içindir. Rust kullanarak birçok kişi işletim sistemi geliştirme gibi konuları öğrenmiştir. Topluluk oldukça misafirperverdir ve öğrenci sorularını yanıtlamaktan mutluluk duyar. Bu kitap gibi girişimlerle Rust ekipleri, özellikle programlamaya yeni başlayanlar için sistem kavramlarını daha erişilebilir hale getirmeyi amaçlamaktadır.

## 🏢 Şirketler (Companies)

Büyükten küçüğe yüzlerce şirket, üretim ortamında Rust’ı çeşitli görevler için kullanmaktadır. Bunlar arasında komut satırı araçları (command line tools), web servisleri, DevOps araçları, gömülü cihazlar (embedded devices), ses ve video analizi ile kod dönüştürme (transcoding), kripto paralar, biyoinformatik, arama motorları, Nesnelerin İnterneti (Internet of Things) uygulamaları, makine öğrenmesi ve hatta Firefox web tarayıcısının büyük bölümleri yer almaktadır.

## 🛠️ Açık Kaynak Geliştiricileri (Open Source Developers)

Rust, Rust programlama dilini, topluluğunu, geliştirici araçlarını ve kütüphanelerini inşa etmek isteyenler içindir. Rust diline katkıda bulunmanızdan büyük mutluluk duyarız.

## ⚡ Hıza ve Kararlılığa Değer Verenler (People Who Value Speed and Stability)

Rust, bir dilde hız ve kararlılık arayan kişiler içindir. Hızdan kastımız hem Rust kodunun ne kadar hızlı çalışabileceği hem de Rust’ın program yazmanıza ne kadar hızlı olanak tanıdığıdır. Rust derleyicisinin (compiler) kontrolleri, özellik eklemeleri ve yeniden düzenlemeler (refactoring) boyunca kararlılığı garanti eder. Bu, bu tür kontrollerin olmadığı dillerdeki kırılgan eski kodların (legacy code) aksine, geliştiricilerin genellikle değiştirmekten korktuğu bir durumdur.

Rust, sıfır maliyetli soyutlamalar (zero-cost abstractions) — yani yüksek seviyeli özelliklerin, elle yazılmış kod kadar hızlı çalışan düşük seviyeli koda derlenmesi — için çabalayarak güvenli kodun aynı zamanda hızlı olmasını da amaçlar.

Rust dili, burada bahsedilenlerin ötesinde birçok farklı kullanıcıyı da desteklemeyi umuyor; burada anılanlar yalnızca en büyük paydaşlardan birkaçıdır. Genel olarak Rust’ın en büyük hedefi, programcıların onlarca yıldır kabul etmek zorunda kaldıkları ödünleri ortadan kaldırmaktır: güvenlik ve verimlilik, hız ve ergonomi bir arada. Rust’ı deneyin ve sunduğu seçeneklerin sizin için işe yarayıp yaramadığını görün.

## 📗 Bu Kitap Kimler İçin (Who This Book Is For)

Bu kitap, başka bir programlama dilinde kod yazmış olduğunuzu varsayar fakat bunun hangi dil olduğu hakkında herhangi bir varsayımda bulunmaz. İçeriği, çok çeşitli programlama geçmişlerinden gelenler için geniş ölçüde erişilebilir kılmaya çalıştık. Programlamanın ne olduğu ya da nasıl düşünülmesi gerektiği hakkında fazla zaman harcamıyoruz. Eğer programlamaya tamamen yeniyseniz, programlamaya özel olarak giriş sağlayan bir kitap okumanız sizin için daha faydalı olacaktır.
## 📚 Bu Kitap Nasıl Kullanılır (How to Use This Book)

Genel olarak, bu kitap baştan sona sırayla okunacak şekilde hazırlanmıştır. İlerleyen bölümler, önceki bölümlerdeki kavramlar üzerine inşa edilir ve önceki bölümler bir konuyu ayrıntılı ele almasa bile daha sonra o konuya geri dönülür.

Bu kitapta iki tür bölüm bulacaksınız: **kavram bölümleri (concept chapters)** ve **proje bölümleri (project chapters)**. Kavram bölümlerinde Rust’ın bir yönünü öğreneceksiniz. Proje bölümlerinde ise, şimdiye kadar öğrendiklerinizi uygulayarak küçük programlar geliştireceğiz. 2., 12. ve 21. bölümler proje bölümleridir; geri kalanlar kavram bölümleridir.

* **Bölüm 1**: Rust’ın nasıl kurulacağını, bir “Hello, world!” programının nasıl yazılacağını ve Rust’ın paket yöneticisi ve derleme aracı olan Cargo’nun nasıl kullanılacağını açıklar.

* **Bölüm 2**: Rust ile bir program yazmaya uygulamalı bir giriş sunar ve bir sayı tahmin oyunu geliştirmenizi sağlar. Burada kavramları genel hatlarıyla ele alıyoruz, daha sonraki bölümlerde ayrıntılara gireceğiz. Eğer hemen uygulamalı çalışmaya başlamak isterseniz, Bölüm 2 sizin için uygundur.

* **Bölüm 3**: Diğer programlama dillerinde bulunan Rust özelliklerini ele alır.

* **Bölüm 4**: Rust’ın **ownership sistemi**ni inceler. Eğer her ayrıntıyı öğrenmeden sonraki konuya geçmeyi sevmeyen dikkatli bir öğreniciyseniz, Bölüm 2’yi atlayıp doğrudan Bölüm 3’e geçebilir, daha sonra öğrendiklerinizi uygulamak için tekrar Bölüm 2’ye dönebilirsiniz.

* **Bölüm 5**: `struct` ve metotları ele alır.

* **Bölüm 6**: `enum`, `match` ifadeleri ve `if let` kontrol akışı yapısını kapsar. Bu yapılar Rust’ta özel tipler oluşturmak için kullanılır.

* **Bölüm 7**: Rust’ın modül sistemini ve kodunuzu organize etmek için kullanılan gizlilik (privacy) kurallarını, ayrıca kamuya açık **API (Application Programming Interface)** yapısını ele alır.

* **Bölüm 8**: Standart kütüphanede bulunan yaygın koleksiyon veri yapılarından bazılarını — `vector`, `string`, `hash map` — tartışır.

* **Bölüm 9**: Rust’ın hata yönetimi (error-handling) felsefesini ve tekniklerini inceler.

* **Bölüm 10**: `generic`, `trait` ve `lifetime` konularına odaklanır; bunlar birden fazla tipe uygulanabilen kod yazmanıza imkân verir.

* **Bölüm 11**: Rust’ın güvenlik garantilerine rağmen programın mantığının doğruluğunu sağlamak için gerekli olan testlere ayrılmıştır.

* **Bölüm 12**: Dosyalarda metin araması yapan `grep` komut satırı aracının bir alt kümesini kendi başımıza uygularız. Bu bölümde, önceki bölümlerde öğrendiğimiz birçok kavramı kullanırız.

* **Bölüm 13**: Fonksiyonel programlama dillerinden gelen Rust özellikleri olan `closure` ve `iterator` konularını inceler.

* **Bölüm 14**: Cargo’yu daha derinlemesine ele alır ve kütüphanelerinizi başkalarıyla paylaşmanın en iyi uygulamalarını tartışır.

* **Bölüm 15**: Standart kütüphanede bulunan **akıllı işaretçileri (smart pointers)** ve bunların işlevselliğini mümkün kılan `trait`leri ele alır.

* **Bölüm 16**: Farklı eşzamanlı programlama (concurrent programming) modellerini inceler ve Rust’ın çoklu iş parçacıkları (multithreading) ile güvenle programlama yapmanıza nasıl yardımcı olduğunu açıklar.

* **Bölüm 17**: Bunun üzerine inşa ederek Rust’ın `async` ve `await` sözdizimini, görevler (tasks), gelecekler (futures) ve akışlar (streams) ile etkin kıldıkları hafif eşzamanlılık modelini ele alır.

* **Bölüm 18**: Rust deyimlerinin nesne yönelimli programlama ilkeleriyle nasıl karşılaştırıldığını tartışır.

* **Bölüm 19**: Desenler (patterns) ve desen eşleştirme (pattern matching) üzerine bir başvuru niteliği taşır.

* **Bölüm 20**: İleri seviye konulardan oluşan bir seçki sunar; `unsafe Rust`, makrolar ve `lifetime`, `trait`, `type`, `function`, `closure` hakkında daha fazlası burada yer alır.

* **Bölüm 21**: Çok iş parçacıklı (multithreaded) düşük seviyeli bir web sunucusu uyguladığımız bir proje içerir.

Son olarak, bazı ek bölümler (appendix) dili daha çok başvuru (reference) formatında ele alır:

* **Ek A**: Rust’ın anahtar kelimelerini kapsar.
* **Ek B**: Rust’ın operatörleri ve sembollerini kapsar.
* **Ek C**: Standart kütüphanenin sağladığı türetilebilir `trait`leri kapsar.
* **Ek D**: Yararlı geliştirme araçlarını açıklar.
* **Ek E**: Rust sürümlerini (editions) açıklar.
* **Ek F**: Kitabın çevirilerini içerir.
* **Ek G**: Rust’ın nasıl geliştirildiğini ve `nightly Rust`ın ne olduğunu açıklar.

Bu kitabı okumanın yanlış bir yolu yoktur: ileriye atlamak isterseniz, buyurun yapın! Bir karışıklık yaşarsanız önceki bölümlere dönmeniz gerekebilir. Ama size en uygun olan şekilde ilerleyin.

### ❗ Hata Mesajlarını Okumayı Öğrenmek

Rust öğrenme sürecinin önemli bir kısmı, derleyicinin gösterdiği hata mesajlarını okumayı öğrenmektir: bu mesajlar sizi çalışan koda yönlendirecektir. Bu nedenle, kitap boyunca derlenmeyen örnekler ve bunlara karşılık derleyicinin göstereceği hata mesajlarını da paylaşacağız. Rastgele bir örneği girip çalıştırırsanız, derlenmeyebilir! Örneği çalıştırmadan önce çevresindeki metni okuyarak, o örneğin hatalı olması amaçlanıp amaçlanmadığını kontrol edin.

Ferris de size çalışması amaçlanmayan kodu ayırt etmede yardımcı olacak:

| Ferris                                    | Anlamı                              |
| ----------------------------------------- | ----------------------------------- |
| Soru işareti olan Ferris                  | Bu kod derlenmez!                   |
| Ellerini kaldıran Ferris                  | Bu kod panik oluşturur!             |
| Tek pençesini kaldırıp omuz silken Ferris | Bu kod istenilen davranışı üretmez. |

Çoğu durumda, derlenmeyen kodun doğru sürümüne sizi biz yönlendireceğiz.

### 💾 Kaynak Kod (Source Code)

Bu kitabın oluşturulduğu kaynak dosyalar GitHub’da bulunabilir.
