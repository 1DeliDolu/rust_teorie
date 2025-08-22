## 🔐 Kayıt Kimlik Doğrulaması (registry authentication)

Cargo, kayıtlara **kimlik bilgisi sağlayıcıları** (credential providers) ile kimlik doğrulaması yapar. Bu sağlayıcılar, Cargo’nun kimlik bilgilerini saklamak ve almak için kullandığı harici çalıştırılabilir dosyalar veya yerleşik sağlayıcılardır.

Kimlik doğrulama gerektiren alternatif kayıtların kullanılması, şifrelenmemiş kimlik bilgilerinin diske yanlışlıkla kaydedilmesini önlemek için bir kimlik bilgisi sağlayıcısının yapılandırılmasını gerektirir. Tarihsel nedenlerden ötürü, herkese açık (kimlik doğrulaması gerektirmeyen) kayıtlar için kimlik bilgisi sağlayıcısı yapılandırması gerekmez; bu durumda herhangi bir sağlayıcı ayarlanmamışsa `cargo:token` sağlayıcısı kullanılır.

Cargo ayrıca, jetonları güvenli şekilde saklamak için işletim sistemine özgü sağlayıcılar içerir. Bunun yanında, kimlik bilgilerini şifrelenmemiş düz metin olarak `credentials` dosyasında saklayan `cargo:token` sağlayıcısı da bulunur.

---

## ⚙️ Önerilen Yapılandırma (recommended configuration)

Global bir kimlik bilgisi sağlayıcı listesi `$CARGO_HOME/config.toml` dosyasında yapılandırılmalıdır. Varsayılan konum:

* Windows: `%USERPROFILE%\.cargo\config.toml`
* Unix: `~/.cargo/config.toml`

Önerilen yapılandırma, işletim sistemi sağlayıcısını kullanır ve geriye dönük olarak `cargo:token` ile Cargo’nun `credentials` dosyasına veya ortam değişkenlerine bakar:

```toml
# ~/.cargo/config.toml
[registry]
global-credential-providers = ["cargo:token", "cargo:libsecret", "cargo:macos-keychain", "cargo:wincred"]
```

👉 Buradaki listede, daha sonra gelen girişler daha yüksek önceliğe sahiptir. Ayrıntılar için `registry.global-credential-providers` belgesine bakın.

Bazı özel kayıtlar ayrıca kayıt-özel bir kimlik bilgisi sağlayıcısı önerebilir. Bunun geçerli olup olmadığını görmek için kullandığınız kaydın belgelerini kontrol edin.

---

## 🛠️ Yerleşik Sağlayıcılar (built-in providers)

Cargo, birkaç yerleşik kimlik bilgisi sağlayıcısı içerir. Bu sağlayıcılar ilerideki Cargo sürümlerinde değişebilir (şu an için değiştirme planı yoktur).

* **cargo\:token**
  Jetonları şifrelenmemiş düz metin olarak Cargo’nun kimlik bilgisi dosyasında saklar.
  Jetonları alırken `CARGO_REGISTRIES_<NAME>_TOKEN` ortam değişkenini kontrol eder.
  Eğer bu sağlayıcı listelenmezse, `*_TOKEN` ortam değişkenleri çalışmaz.

* **cargo\:wincred**
  Jetonları Windows Credential Manager’da saklar.
  Kimlik bilgileri, “Windows Credentials” altında `cargo-registry:<index-url>` olarak depolanır.

* **cargo\:macos-keychain**
  Jetonları macOS Keychain’de saklar.
  Saklanan jetonları görüntülemek için “Keychain Access” uygulaması kullanılabilir.

* **cargo\:libsecret**
  Jetonları `libsecret` kullanarak saklar.
  `libsecret` desteği olan herhangi bir parola yöneticisi ile jetonlar görüntülenebilir. Örnekler:

  * GNOME Keyring
  * KDE Wallet Manager (KDE Frameworks 5.97.0’dan itibaren)
  * KeePassXC (2.5.0’dan itibaren)

* **cargo\:token-from-stdout <command> <args>**
  Standart çıktıda (stdout) bir jeton döndüren bir alt işlem başlatır.

  * Yeni satırlar kırpılır.
  * İşlem, başarı için `0` koduyla çıkmalı, hata için sıfırdan farklı bir kod döndürmelidir.
  * `cargo login` ve `cargo logout` bu sağlayıcıyla desteklenmez ve hata döndürür.
  * Çalıştırılan komuta şu ortam değişkenleri aktarılır:

    * `CARGO` — Komutu çalıştıran Cargo ikili dosyasının yolu.
    * `CARGO_REGISTRY_INDEX_URL` — Kayıt indeksinin URL’si.
    * `CARGO_REGISTRY_NAME_OPT` — Kayıt için isteğe bağlı ad (anahtar olarak kullanılmamalıdır).
  * Argümanlar, alt komuta iletilir.

---

## 🔌 Kimlik Bilgisi Eklentileri (credential plugins)

Cargo’nun kimlik bilgisi sağlayıcı protokolünü izleyen sağlayıcı eklentileri için yapılandırma değeri, çalıştırılabilir dosyanın yolu (veya `PATH`’teyse yalnızca adı) olmalıdır.

Örneğin, `cargo-credential-1password` sağlayıcısını `crates.io`’dan yüklemek için:

1. Sağlayıcıyı yükleyin:

```
cargo install cargo-credential-1password
```

👉 Bu komut, `cargo-credential-1password` eklentisini sisteme kurar.

2. Yapılandırmaya ekleyin:

```toml
[registry]
global-credential-providers = ["cargo:token", "cargo-credential-1password --account my.1password.com"]
```

👉 Bu ayar, Cargo’ya `1Password` sağlayıcısını global kimlik bilgisi sağlayıcıları arasına eklemesini söyler.

`global-credential-providers` içindeki değerler, boşluklara göre bölünerek yol ve komut satırı argümanları olarak ayrıştırılır.

Eğer yol veya argümanlarda boşluk bulunuyorsa, bunun için `[credential-alias]` tablosu kullanılmalıdır.
