## 🔄 Bağımlılıkları Geçersiz Kılma (overriding dependencies)

Bir bağımlılığı geçersiz kılma isteği birçok senaryoda ortaya çıkabilir. Çoğu durum, crates.io’da yayımlanmadan önce bir crate ile çalışabilme ihtiyacına dayanır. Örneğin:

* Üzerinde çalıştığınız bir crate, aynı zamanda daha büyük bir uygulamada da kullanılıyordur ve siz de bu büyük uygulama içinde kütüphanedeki hatayı düzeltmeyi test etmek istiyorsunuz.
* Çalışmadığınız bir üst seviye crate’in git deposundaki `master` dalında yeni bir özellik veya hata düzeltmesi vardır ve bunu test etmek istiyorsunuz.
* Crate’inizin yeni bir **major** sürümünü yayımlamak üzeresiniz, fakat tüm paket üzerinde entegrasyon testi yapmak istiyorsunuz.
* Bir üst seviye crate’e bulduğunuz bir hata için düzeltme gönderdiniz, ama hata düzeltmesi henüz birleştirilmeden uygulamanızın bu düzeltilmiş sürüme bağımlı olmasını istiyorsunuz.

Bu senaryolar, `Cargo.toml` içindeki `[patch]` bölümü ile çözülebilir.

Bu bölüm, farklı kullanım örneklerini ve bağımlılıkların geçersiz kılınmasının yollarını açıklar:

* Hata düzeltmesini test etme
* Yayınlanmamış minor sürüm ile çalışma
* Depo URL’sini geçersiz kılma
* Yayımlamadan önce kırıcı değişiklik test etme
* Birden fazla sürümle `[patch]` kullanma

---

## 🧪 Hata Düzeltmesini Test Etme (testing a bugfix)

Diyelim ki `uuid` crate’i ile çalışıyorsunuz ve bir hata buldunuz. Hatayı düzeltmeye karar verdiniz. Başlangıçta manifestiniz şöyleydi:

```toml
[package]
name = "my-library"
version = "0.1.0"

[dependencies]
uuid = "1.0"
```

Öncelikle `uuid` reposunu yerel makinenize klonlayın:

```bash
$ git clone https://github.com/uuid-rs/uuid.git
```

Daha sonra `my-library` manifestini şu şekilde güncelleyin:

```toml
[patch.crates-io]
uuid = { path = "../path/to/uuid" }
```

Burada `crates-io` kaynağını yerel bir sürümle yamaladığımızı belirtiyoruz. Böylece yerel `uuid`, crates.io’daki sürümün yerine geçer.

Artık `cargo build` çalıştırdığınızda yerel `uuid` derlenecektir:

```bash
$ cargo build
   Compiling uuid v1.0.0 (.../uuid)
   Compiling my-library v0.1.0 (.../my-library)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

👉 Derleme çıktısında `uuid` yolunu görüyorsanız, yerel sürüm kullanılıyordur. Görmüyorsanız:

```bash
cargo update uuid --precise $version
```

kullanabilirsiniz (`$version`, yerel `uuid`’nin sürüm numarasıdır).

Hatayı düzelttikten sonra, düzeltmeyi `uuid` projesine pull request olarak gönderebilirsiniz. Bundan sonra `[patch]` bölümünü şu şekilde güncelleyebilirsiniz:

```toml
[patch.crates-io]
uuid = { git = "https://github.com/uuid-rs/uuid.git" }
```

---

## ✨ Yayınlanmamış Minor Sürümle Çalışma (working with an unpublished minor version)

Diyelim ki `uuid` crate’inde yeni bir özellik eklediniz ve bu özellik git deposunda mevcut, fakat crates.io’da henüz yayımlanmadı.

Şu an crates.io’daki `uuid` sürümü `1.0.0`, ancak git reposunda `1.0.1` sürümü var. Bu durumda manifestiniz:

```toml
[package]
name = "my-library"
version = "0.1.0"

[dependencies]
uuid = "1.0.1"

[patch.crates-io]
uuid = { git = "https://github.com/uuid-rs/uuid.git" }
```

Burada:

* `uuid = "1.0.1"` → yayımlandığında ihtiyaç duyacağımız sürüm.
* `[patch]` → crates.io’da henüz yok, bu yüzden git deposundan alınacak.

Böylece, `cargo build` çalıştırıldığında `uuid` git deposundan alınacak ve `1.0.1` sürümü kullanılacak. Yayımlandığında `[patch]` bölümü silinebilir.

---

## 🌐 `[patch]`’in Transitif Etkisi (transitive effect)

`[patch]`, transitif olarak uygulanır.

Örneğin:

```toml
[package]
name = "my-binary"
version = "0.1.0"

[dependencies]
my-library = { git = "https://example.com/git/my-library" }
uuid = "1.0"

[patch.crates-io]
uuid = { git = "https://github.com/uuid-rs/uuid.git" }
```

Burada `my-library` → `uuid` bağımlılığı da yamalanır.
Sonuç:

* `uuid` her iki bağımlılık için de aynı sürüm (`1.0.1`) ile çözülür.
* Bu sürüm, crates.io yerine git deposundan alınır.

## 🌐 Depo URL’sini Geçersiz Kılma (overriding repository URL)

Geçersiz kılmak istediğiniz bağımlılık **crates.io’dan değilse**, `[patch]` kullanımında küçük bir fark gerekir.
Örneğin, bağımlılık bir git bağımlılığı ise, bunu yerel bir yol ile geçersiz kılabilirsiniz:

```toml
[patch."https://github.com/your/repository"]
my-library = { path = "../my-library/path" }
```

Ve bu kadar!

---

## 🚀 Yayın Öncesi Kırıcı Değişiklik (prepublishing a breaking change)

Yeni bir **major** sürüm üzerinde çalışırken, genellikle kırıcı değişiklikler olur.
Örneğin, `uuid` crate’inin `2.0.0` sürümünü geliştiriyorsunuz:

```toml
[dependencies]
uuid = "2.0"

[patch.crates-io]
uuid = { git = "https://github.com/uuid-rs/uuid.git", branch = "2.0.0" }
```

Burada `2.0.0` crates.io’da mevcut değildir, ama `[patch]` ile git üzerinden kullanılabilir.

Daha büyük bir senaryoda, örneğin `my-binary` adlı bir crate:

```toml
[package]
name = "my-binary"
version = "0.1.0"

[dependencies]
my-library = { git = "https://example.com/git/my-library" }
uuid = "1.0"

[patch.crates-io]
uuid = { git = "https://github.com/uuid-rs/uuid.git", branch = "2.0.0" }
```

Bu durumda:

* `my-binary` → `uuid` için `1.x.y` serisini kullanmaya devam eder.
* `my-library` → `uuid` için `2.0.0` sürümünü kullanır.

👉 Bu sayede, tüm bağımlılık grafiğini aynı anda güncellemek zorunda kalmadan kırıcı değişiklikler aşamalı olarak dağıtılabilir.

---

## 🔀 Birden Fazla Sürümle \[patch] Kullanma (using \[patch] with multiple versions)

Aynı crate’in birden fazla sürümünü `[patch]` ile kullanabilirsiniz. Bunun için `package` anahtarı ile yeniden adlandırma yapılır.

Örneğin, `serde` için:

```toml
[patch.crates-io]
serde = { git = "https://github.com/serde-rs/serde.git" }
serde2 = { git = "https://github.com/example/serde.git", package = "serde", branch = "v2" }
```

* İlk satır → `serde 1.*` sürümü, git reposundan alınır.
* İkinci satır → `serde` paketinin `2.0.0` sürümü, farklı bir repodaki `v2` branch’ten alınır.

⚠️ Not: `serde2` burada yalnızca eşsiz bir isimdir, Cargo tarafından aslında göz ardı edilir.

---

## 📑 \[patch] Bölümü

`Cargo.toml` içindeki `[patch]` bölümü, bağımlılıkların farklı kopyalarla geçersiz kılınmasını sağlar. Sözdizimi `[dependencies]` bölümüne benzer:

```toml
[patch.crates-io]
foo = { git = "https://github.com/example/foo.git" }
bar = { path = "my/local/bar" }

[dependencies.baz]
git = "https://github.com/example/baz.git"

[patch."https://github.com/example/baz"]
baz = { git = "https://github.com/example/patched-baz.git", branch = "my-branch" }
```

Not: `[patch]` tablosu `.cargo/config.toml` dosyasında veya CLI ile (`--config`) da belirtilebilir. Bu, yalnızca yerel değişikliklerde veya geçici testlerde faydalıdır.

* `[patch]` tabloları, bağımlılık benzeri alt tablolar içerir.
* `[patch]` sonrasındaki anahtar → yamalanacak kaynağın URL’si veya kayıt defteri adı.
* `crates-io` → varsayılan kayıt defterini (crates.io) geçersiz kılmak için kullanılabilir.

Örnekte:

* `foo` ve `bar`, crates.io kaynağını yamalar.
* `baz`, farklı bir git kaynağını (`https://github.com/example/baz`) yamalar.

Cargo yalnızca **çalışma alanı kökündeki** `Cargo.toml` dosyasında belirtilen `[patch]` ayarlarını dikkate alır.

---

## ⚠️ \[replace] Bölümü (deprecated)

`[replace]` bölümü artık önerilmez, `[patch]` kullanılmalıdır.

```toml
[replace]
"foo:0.1.0" = { git = "https://github.com/example/foo.git" }
"bar:1.0.2" = { path = "my/local/bar" }
```

* `[replace]` → bağımlılık grafiğindeki belirli bir düğümü sürüm bazında geçersiz kılar.
* Anahtar → paket kimliği (isim + 3 parçalı sürüm).
* Değer → `[dependencies]` sözdizimi ile aynı.

⚠️ Kısıtlar:

* Özellikler (`features`) belirtilemez.
* Crate adı ve sürümü aynı kalmalı, yalnızca kaynak değişebilir.

---

## 📂 Yol Geçersiz Kılmaları (path overrides)

Bazen bir crate üzerinde yalnızca geçici olarak çalışırsınız ve `Cargo.toml` dosyasını değiştirmek istemezsiniz. Bu durumda Cargo, sınırlı bir yöntem olan **path overrides** özelliğini sunar.

Bu, `.cargo/config.toml` içinde belirtilir:

```toml
paths = ["/path/to/uuid"]
```

* Bu dizi, `Cargo.toml` dosyası içeren dizinlerin yollarını içerir.
* Yol, mutlak veya `.cargo` dizinine göreceli olabilir.

⚠️ Kısıtlar:

* `[patch]`’in aksine, bağımlılık grafiğinin yapısını değiştiremez.
* Yeni bağımlılık eklemek için kullanılamaz (sadece hata düzeltmeleri için uygundur).
* Yalnızca **crates.io’da yayımlanmış crate’ler** için geçerlidir.

👉 Daha kapsamlı değişiklikler için `[patch]` kullanılmalıdır.
