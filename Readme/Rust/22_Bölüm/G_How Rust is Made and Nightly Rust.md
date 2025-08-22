## 🚂 Ek G: Rust Nasıl Geliştirilir ve “Nightly Rust”

Bu ekte, Rust’ın nasıl geliştirildiğini ve bunun bir Rust geliştiricisi olarak sizi nasıl etkilediğini ele alıyoruz.

---

## 🛡️ Durgunluk Olmadan Kararlılık (stability without stagnation)

Rust, dil olarak kodunuzun **kararlılığına** büyük önem verir. Rust’ın sağlam bir temel olmasını isteriz; eğer her şey sürekli değişseydi bu imkânsız olurdu. Aynı zamanda, yeni özelliklerle deney yapmazsak, önemli hataları ancak yayınladıktan sonra fark edebiliriz ve bu noktada değişiklik yapmak artık mümkün olmaz.

Buna çözümümüz “**durgunluk olmadan kararlılık**” ilkesidir. Rehber kuralımız şudur: **yeni bir kararlı (stable) Rust sürümüne yükselmekten asla korkmamalısınız.** Her yükseltme sorunsuz olmalı, size yeni özellikler, daha az hata ve daha hızlı derleme süreleri getirmelidir.

---

## 🚆 Yayın Kanalları ve Tren Modeli (release channels and train model)

Rust geliştirme süreci bir **tren takvimi** ile işler. Tüm geliştirmeler Rust deposunun `master` dalında yapılır. Yayınlar, Cisco IOS gibi diğer projelerde de kullanılan yazılım tren modeli ile takip edilir. Rust için üç yayın kanalı vardır:

* **Nightly**
* **Beta**
* **Stable**

Çoğu Rust geliştiricisi **stable** kanalı kullanır. Ancak deneysel özellikleri denemek isteyenler **nightly** veya **beta** kullanabilir.

### Yayın Sürecine Örnek:

1. Yeni bir özellik `master` dalına eklenir.
2. **Her gece** yeni bir **nightly** sürüm üretilir.
3. **Altı haftada bir**, yeni bir **beta** dalı `master`’dan ayrılır.
4. Hatalar `nightly`’da düzeltilir, gerekirse `beta`ya geri taşınır.
5. Altı hafta sonra, **stable** dalı `beta`dan ayrılır ve yeni kararlı sürüm yayınlanır.
6. Aynı anda, bir sonraki sürüm için yeni bir `beta`, `nightly`’den tekrar ayrılır.

Bu yüzden her altı haftada bir tren istasyondan kalkar ve kararlı sürüm olmadan önce `beta` kanalından geçer.

Rust **altı haftada bir düzenli olarak** yeni sürüm yayınlar. Bir özelliğiniz bu sürüme yetişmezse endişelenmeyin: bir sonraki sadece altı hafta uzakta! Bu, aceleyle olgunlaşmamış özelliklerin yayınlanmasını engeller.

---

## ⏳ Bakım Süresi (maintenance time)

Rust projesi yalnızca **en güncel kararlı sürümü** destekler. Yeni bir stable sürüm çıktığında, önceki sürümün yaşam döngüsü (EOL) sona erer. Yani her sürüm **6 hafta boyunca** desteklenir.

---

## ⚡ Kararsız Özellikler (unstable features)

Bu yayın modelinde bir nokta daha vardır: **kararsız özellikler (unstable features).**

* Rust, hangi özelliklerin etkin olacağını belirlemek için **feature flag** tekniğini kullanır.
* Yeni bir özellik geliştirilirken `master` dalına eklenir → yani nightly’de bulunur → ama bir feature flag arkasındadır.
* Denemek isteyen kullanıcılar nightly sürüm kullanmalı ve kaynak kodlarına ilgili flag’i eklemelidir.
* Beta veya stable sürümlerde feature flag kullanılamaz.

Bu yöntem sayesinde:

* **Son kullanıcılar** sağlam, kırılmayan stable deneyimini alır.
* **Deneysel kullanıcılar** ise bleeding-edge özellikleri nightly’de test edebilir.

Bu kitap yalnızca **stable özellikleri** kapsar. Nightly’ye özel özellikler değişmeye devam ettiğinden, belgelerini çevrimiçi bulabilirsiniz.

---

## 🔄 rustup ve Nightly’nin Rolü

`rustup`, Rust’ın farklı yayın kanalları arasında geçiş yapmayı kolaylaştırır. Varsayılan olarak stable Rust kurulu olur. Nightly kurmak için:

```bash
$ rustup toolchain install nightly
```

Kurulu toolchain’leri görmek için:

```bash
$ rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

Gördüğünüz gibi stable varsayılandır. Ama bir proje için nightly kullanmak isterseniz:

```bash
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

Artık bu dizinde `cargo` veya `rustc` çalıştırdığınızda, stable yerine nightly kullanılacaktır.

---

## 📜 RFC Süreci ve Takımlar (RFC process and teams)

Yeni özellikler nasıl duyurulur? Rust geliştirme modeli **RFC (Request For Comments)** sürecini takip eder.

* Rust’ta geliştirme görmek istiyorsanız, bir **RFC** yazabilirsiniz.
* Herkes RFC yazabilir.
* RFC’ler, Rust ekibi ve alt takımlar (dil tasarımı, derleyici, altyapı, belgeler vb.) tarafından incelenir ve tartışılır.
* Uygun bulunursa kabul edilir, bir issue açılır ve özellik uygulanır.
* Özelliği uygulayan kişi, fikri öneren kişi olmak zorunda değildir.

Özellik uygulandıktan sonra:

* Önce `master` dalına bir **feature gate** arkasında eklenir.
* Nightly kullanıcıları özelliği deneyimledikten sonra ekip tartışır.
* Uygun görülürse, feature gate kaldırılır → özellik **stable** olur → tren modelinde kararlı sürüm olarak yoluna devam eder.
