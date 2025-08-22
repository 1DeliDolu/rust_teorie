## ⚖️ Döngüler ile Yineleyicilerin Karşılaştırmalı Performansı (Comparing Performance: Loops vs. Iterators)

Döngüler (loops) mi yoksa yineleyiciler (iterators) mı kullanılacağına karar vermek için hangi uygulamanın daha hızlı olduğunu bilmeniz gerekir: `search` işlevinin (search function) açık bir `for` döngüsü (for loop) kullanan sürümü mü yoksa yineleyicileri kullanan sürümü mü.

Sir Arthur Conan Doyle’un *The Adventures of Sherlock Holmes* eserinin tamamını bir `String` (String) içine yükleyip içerikte `the` kelimesini arayarak bir kıyaslama (benchmark) yaptık. İşte `search`’ün döngü kullanan sürümü ile yineleyici kullanan sürümünün kıyaslama sonuçları:

test bench\_search\_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench\_search\_iter ... bench:  19,234,900 ns/iter (+/- 657,200)

İki uygulamanın benzer performansa sahip olduğu görülüyor! Burada kıyaslama kodunu açıklamayacağız; amaç iki sürümün eşdeğer olduğunu kanıtlamak değil, performans açısından genel bir fikir edinmektir.

Daha kapsamlı bir kıyaslama için içeriği farklı boyutlarda ve türlerde metinlerle, sorguyu farklı sözcükler ve farklı uzunluklarda sözcüklerle ve her türden başka varyasyonlarla denemelisiniz. Buradaki esas nokta şu: yineleyiciler (iterators), yüksek seviyeli bir soyutlama olmalarına rağmen, derlemede aşağı yukarı daha düşük seviyeli kodu kendiniz yazmışsınız gibi bir koda indirgenir. Yineleyiciler, Rust’ın sıfır maliyetli soyutlamalarından (zero-cost abstractions) biridir; yani bu soyutlamayı kullanmak ek bir çalışma zamanı maliyeti (runtime overhead) getirmez. Bu, C++’ın özgün tasarımcısı ve uygulayıcısı Bjarne Stroustrup’un “Foundations of C++” (2012) eserinde sıfır maliyet ilkesini tanımlamasına benzerdir:

Genelde C++ uygulamaları sıfır maliyet ilkesine uyar: Kullanmadığınız şey için ödeme yapmazsınız. Ve dahası: Kullandığınız şey için, onu el ile daha iyi kodlayamazdınız.

Başka bir örnek olarak, aşağıdaki kod bir ses (audio) kod çözücüsünden alınmıştır. Kod çözme algoritması, önceki örneklerden doğrusal bir fonksiyona dayanarak gelecekteki değerleri tahmin etmek için doğrusal kestirim (linear prediction) matematiksel işlemini kullanır. Bu kod, kapsamda bulunan üç değişken üzerinde bazı hesaplamalar yapmak için bir yineleyici zinciri (iterator chain) kullanır: bir arabellek dilimi (buffer slice) veri, 12 katsayıdan oluşan bir dizi ve veriyi kaydırmak için kullanılan `qlp_shift` miktarı. Değişkenleri bu örnek içinde bildirdik ancak herhangi bir değer vermedik; bu kod bağlamı dışında çok anlamlı olmasa da, Rust’ın yüksek seviyeli fikirleri düşük seviyeli koda nasıl çevirdiğine dair özlü, gerçek dünya kaynaklı bir örnek sunar.

```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

`prediction` değerini hesaplamak için bu kod, `coefficients` içindeki 12 değerin her biri üzerinde yineler ve `zip` yöntemiyle (zip method) katsayı değerlerini `buffer` içindeki önceki 12 değerle eşler. Daha sonra her bir çift için değerleri çarpar, tüm sonuçları toplar ve toplamın bitlerini `qlp_shift` kadar sağa kaydırır.

Ses kod çözücüleri gibi uygulamalardaki hesaplamalar genellikle performansı en üst düzeye koyar. Burada bir yineleyici oluşturuyor, iki uyarlayıcı (adapters) kullanıyor ve ardından değeri tüketiyoruz (consume). Bu Rust kodu hangi makine diline derlenir? Şu anda, el ile yazacağınız kodla aynı makine koduna derlenmektedir. `coefficients` üzerindeki yinelemeye karşılık gelen bir döngü bile yoktur: Rust 12 yineleme olduğunu bildiği için döngüyü açar (unrolling). Döngü açma (unrolling), döngü kontrol kodunun maliyetini ortadan kaldıran ve bunun yerine her yineleme için tekrarlayan kod üreten bir iyileştirmedir.

Tüm katsayılar kayıtçılarda (registers) saklanır; bu da değerlere erişimi çok hızlı kılar. Çalışma zamanında dizi erişiminde sınır denetimi (bounds checks) yoktur. Rust’ın uygulayabildiği tüm bu iyileştirmeler, ortaya çıkan kodu son derece verimli hale getirir. Artık bunu bildiğinize göre, yineleyicileri ve kapanışları (closures) korkmadan kullanabilirsiniz! Kodu daha yüksek seviyeli gösterirler ancak bunun için çalışma zamanı performans cezası (runtime performance penalty) oluşturmazlar.

## 🧾 Özet (Summary)

Kapanışlar (closures) ve yineleyiciler (iterators), işlevsel programlama dillerinden ilham alan Rust özellikleridir. Yüksek seviyeli fikirleri, düşük seviyeli performansla açıkça ifade etmeye katkıda bulunurlar. Kapanışların ve yineleyicilerin uygulamaları, çalışma zamanı performansının etkilenmemesini sağlar. Bu, Rust’ın sıfır maliyetli soyutlamalar (zero-cost abstractions) sağlama hedefinin bir parçasıdır.

Artık G/Ç (I/O) projemizin ifade gücünü artırdığımıza göre, projeyi dünyayla paylaşmamıza yardımcı olacak cargo’nun daha fazla özelliğine bakalım.
