## 🧭 Paketler, Crate’ler ve Modüller ile Büyüyen Projeleri Yönetme (managing growing projects with packages, crates, and modules)

Büyük programlar yazdıkça, kodunuzu organize etmek giderek daha önemli hale gelecektir. İlgili işlevsellikleri gruplandırarak ve farklı özelliklere sahip kodu ayırarak, belirli bir özelliği uygulayan kodun nerede bulunduğunu ve bir özelliğin nasıl çalıştığını değiştirmek için nereye gitmeniz gerektiğini netleştirirsiniz.

Şimdiye kadar yazdığımız programlar tek bir dosyada tek bir modül (module) içindeydi. Bir proje büyüdükçe, kodu birden çok modüle (module) ve ardından birden çok dosyaya bölerek organize etmelisiniz. Bir paket (package), birden çok ikili crate (binary crate) ve isteğe bağlı olarak bir kütüphane crate’i (library crate) içerebilir. Bir paket (package) büyüdükçe, bölümleri harici bağımlılıklar (external dependencies) haline gelen ayrı crate’lere (crate) ayırabilirsiniz. Bu bölüm tüm bu teknikleri kapsar. Birlikte evrilen, birbiriyle ilişkili paketlerden (package) oluşan çok büyük projeler için Cargo, çalışma alanları (workspaces) sağlar; bunları Bölüm 14’te “Cargo Çalışma Alanları (Cargo Workspaces)” bölümünde ele alacağız.

Ayrıca, uygulama ayrıntılarının kapsüllenmesini (encapsulating implementation details) de tartışacağız; bu, kodu daha yüksek bir düzeyde yeniden kullanmanızı sağlar: bir işlemi uyguladıktan sonra, diğer kodlar uygulamanın nasıl çalıştığını bilmek zorunda kalmadan herkese açık arayüzü (public interface) üzerinden kodunuzu çağırabilir. Yazdığınız kod, hangi bölümlerin diğer kodların kullanımı için herkese açık (public) olduğunu ve hangi bölümlerin değişiklik hakkını saklı tuttuğunuz özel uygulama ayrıntıları (private implementation details) olduğunu tanımlar. Bu, aklınızda tutmanız gereken ayrıntı miktarını sınırlamanın başka bir yoludur.

İlgili bir kavram da kapsam (scope) kavramıdır: kodun yazıldığı iç içe bağlam, “kapsam içinde” tanımlı bir dizi isme sahiptir. Kod okurken, yazarken ve derlerken, programcıların ve derleyicilerin belirli bir noktadaki belirli bir ismin bir değişkene, fonksiyona, struct’a, enum’a, modüle (module), sabite veya başka bir öğeye atıfta bulunup bulunmadığını ve bu öğenin ne anlama geldiğini bilmesi gerekir. Kapsamlar (scope) oluşturabilir ve hangi isimlerin kapsam içinde veya dışında olduğunu değiştirebilirsiniz. Aynı kapsam (scope) içinde aynı ada sahip iki öğeye sahip olamazsınız; ad çakışmalarını çözmek için araçlar mevcuttur.

Rust, kodunuzun organizasyonunu yönetmenize—hangi ayrıntıların ortaya çıkarılacağını, hangilerinin özel kalacağını ve programlarınızdaki her kapsamda (scope) hangi isimlerin bulunacağını—olanak tanıyan çok sayıda özelliğe sahiptir. Bazen topluca modül sistemi (module system) olarak anılan bu özellikler şunları içerir:

* Paketler (packages): Crate’leri (crate) derlemenizi, test etmenizi ve paylaşmanızı sağlayan bir Cargo özelliği
* Crate’ler (crates): Bir kütüphane veya çalıştırılabilir üreten modül ağacı (tree of modules)
* Modüller ve `use` (modules and use): Yolların (paths) organizasyonunu, kapsamını (scope) ve gizliliğini kontrol etmenizi sağlar
* Yollar (paths): Bir struct, fonksiyon veya modül (module) gibi bir öğeyi adlandırmanın bir yolu

Bu bölümde, tüm bu özellikleri ele alacak, etkileşimlerini tartışacak ve kapsamı (scope) yönetmek için nasıl kullanılacaklarını açıklayacağız. Bölümün sonunda, modül sistemi (module system) hakkında sağlam bir anlayışa sahip olacak ve kapsamlarla (scope) bir profesyonel gibi çalışabileceksiniz!
