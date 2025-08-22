## 📊 Özellik Örnekleri (Features Examples)

Aşağıda özelliklerin (features) uygulamadaki bazı gerçek örnekleri gösterilmektedir.

## ⚡ Derleme Sürelerini ve Dosya Boyutlarını Minimize Etme (Minimizing build times and file sizes)

Bazı paketler, özellikler etkinleştirilmediğinde `crate` boyutunu ve derleme süresini azaltmak için özellikler kullanır. Bazı örnekler:

* `syn` Rust kodunu ayrıştırmak için popüler bir `crate`tir. Çok popüler olduğundan, birçok projeyi etkilediği için derleme sürelerini azaltmak faydalıdır. İçerdiği kod miktarını en aza indirmek için kullanılabilecek açıkça belgelenmiş bir özellik listesine sahiptir.
* `regex` iyi belgelenmiş birçok özelliğe sahiptir. Unicode desteğini kaldırmak, büyük tabloları ortadan kaldırabileceği için çıkan dosya boyutunu azaltabilir.
* `winapi`, desteklediği Windows API bağlamalarını sınırlayan çok sayıda özelliğe sahiptir.
* `web-sys`, `winapi`ye benzer bir başka örnektir ve çok geniş bir API bağlama yüzeyi sağlar, bu bağlamalar özellikler kullanılarak sınırlandırılabilir.

## 🔧 Davranışı Genişletme (Extending behavior)

`serde_json` paketi, JSON haritalarının anahtarların eklenme sırasını koruyacak şekilde davranışını değiştiren `preserve_order` özelliğine sahiptir. Bu, yeni davranışı uygulamak için isteğe bağlı bir bağımlılık olan `indexmap`i etkinleştirir.

Bu tür davranış değişikliklerinde dikkatli olunmalıdır. Değişikliklerin **SemVer** uyumlu olduğundan emin olun. Yani, özelliğin etkinleştirilmesi, özelliğin kapalı olduğu durumlarda normalde derlenen kodu bozmamalıdır.

## 📱 no\_std Desteği (no\_std support)

Bazı paketler hem `no_std` hem de `std` ortamlarını desteklemek ister. Bu, gömülü ve kaynak kısıtlı platformları desteklemek için faydalıdır, ancak tam standart kütüphaneyi destekleyen platformlar için genişletilmiş yeteneklere de izin verir.

`wasm-bindgen` paketi varsayılan olarak etkinleştirilen bir `std` özelliği tanımlar. Kütüphanenin en üstünde koşulsuz olarak `no_std` özniteliğini etkinleştirir. Bu, `std` ve `std prelude`’ün otomatik olarak kapsamda olmamasını sağlar. Daha sonra, kodun çeşitli yerlerinde (`example1`, `example2`), `#[cfg(feature = "std")]` öznitelikleri kullanılarak `std` gerektiren ek işlevler koşullu olarak etkinleştirilir.

## 🔄 Bağımlılık Özelliklerini Yeniden İhraç Etme (Re-exporting dependency features)

Bağımlılıklardan gelen özellikleri yeniden ihraç etmek faydalı olabilir. Bu, `crate`e bağlı kullanıcıların bu bağımlılıkları doğrudan belirtmeden özellikleri kontrol etmesine izin verir.

Örneğin, `regex` paketi `regex_syntax` paketinden gelen özellikleri yeniden ihraç eder. `regex` kullanıcılarının `regex_syntax` paketini bilmesine gerek yoktur, ancak yine de içerdiği özelliklere erişebilirler.

## 🏗️ C Kütüphanelerini Dahil Etme (Vendoring of C libraries)

Bazı paketler, yaygın C kütüphanelerine bağlamalar sağlar (bazen “sys” `crate`leri olarak adlandırılır). Bazen bu paketler, sistemde yüklü olan C kütüphanesini kullanma veya onu kaynaktan derleme seçeneği sunar.

Örneğin, `openssl` paketi `vendored` adında bir özelliğe sahiptir ve bu, `openssl-sys` paketinin karşılık gelen `vendored` özelliğini etkinleştirir. `openssl-sys` yapı betiği, OpenSSL kaynak kodunun yerel bir kopyasından derlemesine neden olan koşullu mantığa sahiptir.

`curl-sys` paketi başka bir örnektir. `static-curl` özelliği, `libcurl`’ü kaynaktan derlemesine neden olur. Ayrıca `force-system-lib-on-osx` özelliği vardır; bu özellik, `static-curl` ayarını geçersiz kılarak sistem `libcurl`ünü kullanmaya zorlar.

## ⚖️ Özellik Önceliği (Feature precedence)

Bazı paketlerde karşılıklı olarak dışlayıcı özellikler olabilir. Bunu ele almanın bir yolu, bir özelliğe diğerine göre öncelik vermektir.

`log` paketi buna örnektir. Derleme zamanında maksimum günlükleme seviyesini seçmek için çeşitli özelliklere sahiptir. `cfg-if` kullanarak bir öncelik sırası seçer. Birden fazla özellik etkinleştirilirse, daha yüksek “max” seviyeleri, daha düşük seviyelere göre tercih edilir.

## 🧩 Proc-macro Eşlik Paketi (Proc-macro companion package)

Bazı paketlerin, kendisiyle yakından ilişkili bir `proc-macro`su vardır. Ancak tüm kullanıcıların `proc-macro`yu kullanması gerekmez. `proc-macro`yu isteğe bağlı bağımlılık (`optional-dependency`) yapmak, bunun dahil edilip edilmeyeceğini seçmeyi kolaylaştırır.

Bu faydalıdır çünkü bazen `proc-macro` sürümünün ana paketle senkronize kalması gerekir ve kullanıcıların her iki bağımlılığı da belirtip senkronize tutmasını zorlamak istemezsiniz.

Bir örnek `serde` paketidir; `derive` özelliği, `serde_derive` `proc-macro`’sunu etkinleştirir. `serde_derive` `crate`i `serde` ile çok yakından bağlantılıdır, bu yüzden eşit sürüm gereksinimi kullanarak senkronize kalmalarını sağlar.

## 🌙 Sadece Nightly Özellikleri (Nightly-only features)

Bazı paketler, yalnızca Rust’ın `nightly` kanalında mevcut olan API’ler veya dil özellikleriyle deneme yapmak isteyebilir. Ancak, kullanıcılarının da `nightly` kanalını kullanmasını istemeyebilirler.

Bir örnek `wasm-bindgen` paketidir; `nightly` özelliği, yalnızca `nightly` kanalında mevcut olan `Unsize` marker `trait`ini kullanan genişletilmiş bir API’yi etkinleştirir.

Dikkat edin: `crate` kökünde `cfg_attr` kullanarak `nightly` özelliğini etkinleştirir. Burada kullanılan `feature` özniteliğinin Cargo özellikleriyle ilgisi yoktur; deneysel dil özelliklerini etkinleştirmek için kullanılır.

`rand` paketindeki `simd_support` özelliği başka bir örnektir; yalnızca `nightly` kanalında derlenen bir bağımlılığa dayanır.

## 🧪 Deneysel Özellikler (Experimental features)

Bazı paketler, API’lerin kararlılığına bağlı kalmadan yeni işlevler denemek isteyebilir. Özellikler genellikle belgelenir ve deneysel oldukları belirtilir. Bu nedenle, gelecekte, hatta küçük bir sürüm sırasında bile değişebilir veya bozulabilirler.

Bir örnek `async-std` paketidir; `unstable` özelliği, yeni API’leri kontrol eder. Kullanıcılar bu API’leri kullanmayı seçebilir, ancak bunlara tamamen güvenmek için henüz hazır olmayabilirler.
