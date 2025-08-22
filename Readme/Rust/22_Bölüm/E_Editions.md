## 📖 Ek E: Sürümler (editions)

1. Bölümde, `cargo new` komutunun `Cargo.toml` dosyanıza bir sürüm (edition) hakkında biraz meta veri eklediğini görmüştünüz. Bu ekte bunun ne anlama geldiğini ele alıyoruz!

Rust dili ve derleyicisi altı haftalık bir sürüm döngüsüne sahiptir; yani kullanıcılar sürekli yeni özellikler alır. Diğer programlama dilleri daha büyük değişiklikleri daha seyrek yayınlarken, Rust daha küçük güncellemeleri daha sık yayınlar. Zamanla bu küçük değişiklikler birikir. Ancak sürümden sürüme bakıldığında, örneğin “Rust 1.10 ile Rust 1.31 arasında çok şey değişti” demek zor olabilir.

Yaklaşık her üç yılda bir, Rust ekibi yeni bir Rust sürümü (edition) çıkarır. Her sürüm, gelen özellikleri tam güncellenmiş belgeler ve araçlarla birlikte açık bir paket haline getirir. Yeni sürümler de normal altı haftalık sürüm sürecinin bir parçası olarak yayınlanır.

---

## 🎯 Sürümlerin Amacı (purpose of editions)

Sürümler farklı insanlar için farklı amaçlara hizmet eder:

* **Aktif Rust kullanıcıları** için: Küçük adımlarla gelen değişiklikler anlaşılması kolay bir paket haline getirilir.
* **Rust kullanmayanlar** için: Yeni sürüm, bazı önemli gelişmelerin olduğunu gösterir ve Rust’a tekrar göz atmaya değer kılar.
* **Rust’ı geliştirenler** için: Yeni sürüm, proje için bir odak noktası sağlar.

---

## 📅 Mevcut Sürümler (available editions)

Bu yazının hazırlandığı sırada dört Rust sürümü bulunmaktadır:

* Rust 2015
* Rust 2018
* Rust 2021
* Rust 2024

Bu kitap, **Rust 2024** sürümünün kullanım tarzıyla yazılmıştır.

---

## ⚙️ Cargo.toml’daki edition Anahtarı (edition key in Cargo.toml)

* `Cargo.toml` içindeki `edition` anahtarı, derleyicinin kodunuz için hangi sürümü kullanması gerektiğini belirtir.
* Eğer bu anahtar yoksa, geriye dönük uyumluluk (backward compatibility) için Rust varsayılan olarak **2015 sürümünü** kullanır.

Her proje, varsayılan 2015 dışında bir sürümü tercih edebilir. Sürümler, kodunuzdaki tanımlayıcılarla çakışabilecek yeni bir anahtar kelime gibi **uyumsuz değişiklikler** içerebilir. Ancak bu değişiklikleri etkinleştirmediğiniz sürece, kullandığınız Rust derleyicisini yükseltirken kodunuz derlenmeye devam eder.

---

## 🔗 Sürümler Arası Uyum (cross-edition compatibility)

* Tüm Rust derleyici sürümleri, yayınlandıkları tarihten önceki tüm sürümleri destekler.
* Ayrıca, desteklenen herhangi iki sürümün crate’leri birlikte bağlanabilir.
* Sürüm değişiklikleri yalnızca derleyicinin kodu **ilk kez ayrıştırma (parsing)** şeklini etkiler.

Örneğin:

* Projeniz Rust 2015 kullanıyorsa ve bağımlılıklarınızdan biri Rust 2018 kullanıyorsa, yine de derleme sorunsuz gerçekleşir.
* Tersi durumda da (Rust 2018 projesi + Rust 2015 bağımlılığı) aynı şekilde çalışır.

---

## 🌐 Özelliklerin Erişilebilirliği (feature availability)

Çoğu özellik, tüm sürümlerde kullanılabilir. Rust’ın herhangi bir sürümünü kullanan geliştiriciler, yeni kararlı (stable) sürümler çıktıkça iyileştirmelerden faydalanmaya devam eder.

Ancak bazı durumlarda, özellikle **yeni anahtar kelimeler** eklendiğinde, bazı özellikler yalnızca sonraki sürümlerde bulunabilir. Bu tür özelliklerden faydalanmak istiyorsanız, sürüm değiştirmeniz gerekir.

---

## 📘 Daha Fazla Bilgi (further details)

Daha fazla ayrıntı için **Edition Guide** adlı kitap, sürümler arasındaki farkları listeler ve `cargo fix` ile kodunuzu yeni bir sürüme otomatik olarak nasıl yükselteceğinizi açıklar.
