## 📦 Kayıtlar (registries)

Cargo, `crates` paketlerini yükler ve bağımlılıkları bir “kayıttan” (registry) getirir. Varsayılan kayıt `crates.io`’dur. Bir kayıt, mevcut `crates` listesini aramaya uygun şekilde içeren bir “indeks” barındırır. Ayrıca, Cargo’dan doğrudan yeni `crate` yayımlamayı destekleyen bir web API’si de sağlayabilir.

Not: Mevcut bir kaydı yansıtmak (mirroring) veya yerel almak (vendoring) istiyorsanız, bkz. **Source Replacement (kaynak değiştirme)**.

Bir kayıt sunucusu uyguluyorsanız, Cargo ile bir kayıt arasındaki protokol hakkında daha fazla ayrıntı için **Running a Registry (bir kayıt çalıştırma)** bölümüne bakın.

Kimlik doğrulaması gerektiren bir kayıt kullanıyorsanız bkz. **Registry Authentication (kayıt kimlik doğrulaması)**. Bir kimlik bilgisi sağlayıcısı uyguluyorsanız, ayrıntılar için **Credential Provider Protocol (kimlik bilgisi sağlayıcı protokolü)** bölümüne bakın.

---

## 🔄 Alternatif Kayıt Kullanma (using an alternate registry)

`crates.io` dışındaki bir kaydı kullanmak için, kaydın adı ve indeks URL’si `.cargo/config.toml` dosyasına eklenmelidir. `registries` tablosunda her kayıt için bir anahtar bulunur. Örneğin:

```toml
[registries]
my-registry = { index = "https://my-intranet:8080/git/index" }
```

👉 Bu ayar, `my-registry` adında özel bir kayıt tanımlar.

`index` anahtarı, kaydın indeksini içeren bir `git` deposunun URL’si ya da `sparse+` öneki ile başlayan bir Cargo sparse kayıt URL’si olmalıdır.

Bir `crate`, başka bir kayıttan gelen bir `crate`’e bağımlı olabilir. Bunun için `Cargo.toml` dosyasındaki bağımlılık girişinde `registry` anahtarı ve kaydın adı belirtilmelidir:

```toml
# Sample Cargo.toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2024"

[dependencies]
other-crate = { version = "1.0", registry = "my-registry" }
```

👉 Bu örnekte `other-crate`, `my-registry` kaydından alınır.

Çoğu yapılandırma değerinde olduğu gibi, `index` de bir yapılandırma dosyası yerine ortam değişkeni (environment variable) ile belirtilebilir. Örneğin:

```
CARGO_REGISTRIES_MY_REGISTRY_INDEX=https://my-intranet:8080/git/index
```

👉 Bu değişken, aynı ayarı yapılandırma dosyası olmadan sağlar.

Not: `crates.io`, başka kayıtlardan bağımlılıkları olan paketleri kabul etmez.

---

## 🚀 Alternatif Kayda Yayımlama (publishing to an alternate registry)

Kayıt, web API erişimini destekliyorsa, paketler doğrudan Cargo’dan kayda yayımlanabilir. Cargo’nun `cargo publish` gibi bazı komutları, hangi kaydın kullanılacağını belirtmek için `--registry` komut satırı bayrağını alır. Örneğin, mevcut dizindeki paketi yayımlamak için:

```
cargo login --registry=my-registry
```

👉 Bu komut yalnızca bir kez çalıştırılmalıdır. Sizden, kayıt sitesinden alınan gizli API jetonunu girmeniz istenir.

Alternatif olarak jeton, `--token` bayrağıyla doğrudan veya `CARGO_REGISTRIES_MY_REGISTRY_TOKEN` adlı ortam değişkeni aracılığıyla komuta verilebilir:

```
cargo publish --registry=my-registry
```

👉 Bu komut, paketi `my-registry` kaydına yayımlar.

Her zaman `--registry` bayrağı vermek yerine, varsayılan kayıt `.cargo/config.toml` dosyasında `registry.default` anahtarıyla ayarlanabilir:

```toml
[registry]
default = "my-registry"
```

👉 Bu, tüm yayımlamaların varsayılan olarak `my-registry` kaydına yapılmasını sağlar.

Bir paketin hangi kayıtlara yayımlanabileceğini sınırlamak için `Cargo.toml` dosyasındaki `package.publish` anahtarı kullanılabilir. Bu, kapalı kaynak bir paketi yanlışlıkla `crates.io`’ya yayımlamayı önlemek için kullanışlıdır. Değer, kayıt adlarının listesi olabilir. Örneğin:

```toml
[package]
# ...
publish = ["my-registry"]
```

👉 Bu durumda paket yalnızca `my-registry` kaydına yayımlanabilir.

`publish` değeri ayrıca `false` olarak da ayarlanabilir. Bu, yayımlamayı tamamen yasaklar (boş liste ile aynı etkiyi sağlar).

`cargo login` tarafından kaydedilen kimlik doğrulama bilgileri, Cargo ana dizinindeki (`$HOME/.cargo`) `credentials.toml` dosyasında saklanır. Her kayıt için ayrı bir tablo vardır. Örneğin:

```toml
[registries.my-registry]
token = "854DvwSlUwEHtIo3kWy6x7UCPKHfzCmy"
```

👉 Bu tabloda `my-registry` için kayıtlı erişim jetonu bulunur.

---

## 🌐 Kayıt Protokolleri (registry protocols)

Cargo, iki uzak kayıt protokolünü destekler: `git` ve `sparse`.

* Eğer kayıt indeks URL’si `sparse+` ile başlıyorsa, Cargo **sparse protokolünü** kullanır.
* Aksi halde Cargo **git protokolünü** kullanır.

`git` protokolü, indeks meta verilerini bir `git` deposunda saklar ve Cargo’nun tüm depoyu klonlamasını gerektirir.

`sparse` protokolü ise düz HTTP istekleri kullanarak yalnızca gerekli `crate` meta verilerini indirir. Bu sayede yalnızca ilgili meta veriler indirildiği için zamandan ve bant genişliğinden önemli ölçüde tasarruf sağlar.

`crates.io` kaydı her iki protokolü de destekler. `crates.io` için kullanılacak protokol, `registries.crates-io.protocol` yapılandırma anahtarıyla kontrol edilir.
