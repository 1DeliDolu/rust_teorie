## 📚 Ortak Koleksiyonlar (common collections)

Rust’un standart kütüphanesi, koleksiyonlar (collections) adı verilen birçok faydalı veri yapısı içerir. Diğer veri tipleri tek bir değeri temsil ederken, koleksiyonlar birden fazla değeri tutabilir. Dahili dizi (array) ve tuple tiplerinden farklı olarak, bu koleksiyonların işaret ettiği veriler heap üzerinde depolanır; bu da verinin miktarının derleme zamanında bilinmesine gerek olmadığı ve program çalışırken büyüyüp küçülebileceği anlamına gelir. Her koleksiyon türünün farklı yetenekleri ve maliyetleri vardır, ve mevcut durumunuza uygun olanı seçmek zamanla geliştireceğiniz bir beceridir. Bu bölümde Rust programlarında çok sık kullanılan üç koleksiyonu ele alacağız:

* Bir vektör (vector), değişken sayıda değeri yan yana depolamanıza olanak tanır.
* Bir string (string), karakterlerin bir koleksiyonudur. Daha önce String tipinden bahsetmiştik, ancak bu bölümde detaylı olarak inceleyeceğiz.
* Bir hash map (hash map), belirli bir anahtar (key) ile bir değeri ilişkilendirmenize olanak tanır. Bu, map (harita) adı verilen daha genel veri yapısının belirli bir implementasyonudur.

Standart kütüphanede sunulan diğer koleksiyon türleri hakkında bilgi edinmek için belgeleri inceleyebilirsiniz.

Bu bölümde, vektörleri, string’leri ve hash map’leri nasıl oluşturup güncelleyeceğimizi ve her birini özel kılan şeyleri tartışacağız.
