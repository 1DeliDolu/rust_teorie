## 📂 Bir G/Ç Projesi: Komut Satırı Programı Oluşturma (An I/O Project: Building a Command Line Program)

Bu bölüm, şimdiye kadar öğrendiğiniz birçok becerinin bir tekrarı ve birkaç standart kütüphane özelliğinin keşfidir. Dosya ve komut satırı giriş/çıkışı (input/output) ile etkileşime giren bir komut satırı aracı oluşturacağız; bu sayede Rust’ta edindiğiniz bazı kavramları uygulama fırsatı bulacaksınız.

Rust’un hızı, güvenliği, tek ikili çıktı (single binary output) üretmesi ve çoklu platform desteği onu komut satırı araçları için ideal bir dil haline getiriyor. Bu yüzden, projemizde klasik komut satırı aracı `grep`’in (globally search a regular expression and print) kendi sürümümüzü geliştireceğiz. En basit kullanımda, `grep` belirli bir dosya içinde belirli bir dizeyi (string) arar. Bunu yapmak için `grep`, parametre olarak bir dosya yolu ve bir dize alır. Daha sonra dosyayı okur, bu dosyada verilen dizeyi içeren satırları bulur ve bu satırları yazdırır.

Bu süreçte, komut satırı aracımızı diğer birçok komut satırı aracının kullandığı terminal özelliklerini kullanacak şekilde nasıl tasarlayacağımızı da göstereceğiz. Kullanıcının aracımızın davranışını yapılandırabilmesi için bir ortam değişkeninin (environment variable) değerini okuyacağız. Ayrıca hata mesajlarını standart çıktı (stdout) yerine standart hata akışına (stderr) yazdıracağız. Böylece kullanıcı, başarılı çıktıları bir dosyaya yönlendirirken hata mesajlarını ekranda görmeye devam edebilecek.

Rust topluluğu üyelerinden Andrew Gallant, `ripgrep` adıyla tam özellikli, çok hızlı bir `grep` sürümü oluşturdu. Bizim sürümümüz karşılaştırıldığında oldukça basit olacak, ancak bu bölüm size `ripgrep` gibi gerçek dünya projelerini anlamak için gerekli arka plan bilgisini sağlayacak.

## 🧩 Grep Projesinde Kullanılacak Kavramlar

Grep projemiz, şimdiye kadar öğrendiğiniz bir dizi kavramı bir araya getirecek:

* Kodu düzenleme (Chapter 7)
* Vektörler (vectors) ve string’leri kullanma (Chapter 8)
* Hata yönetimi (Chapter 9)
* Uygun yerlerde trait’ler ve ömürler (lifetimes) kullanma (Chapter 10)
* Test yazma (Chapter 11)

Ayrıca, kısa bir şekilde kapanış fonksiyonlarını (closures), yineleyicileri (iterators) ve trait nesnelerini (trait objects) de tanıtacağız. Bu konuların ayrıntıları ise Chapter 13 ve Chapter 18’de ele alınacak.
