## 🔄 Kaynak Değiştirme (source replacement)

Bu bölüm, **crate index**’in değiştirilmesi hakkındadır. Bağımlılıkların geçersiz kılınması için bkz. *Overriding Dependencies* bölümü.

**Kaynak (source)**, bir paketin bağımlılık olarak eklenebilecek crate’leri barındıran sağlayıcıdır. Cargo, bir kaynağı başka bir kaynakla değiştirmeyi destekler. Bu özellikle şu stratejilerde kullanılır:

* **Vendoring** — Yerel dosya sisteminde crate’leri temsil eden özel kaynaklar tanımlanabilir. Bunlar, değiştirdikleri kaynağın alt kümeleridir ve gerekirse paketlere dahil edilebilir.
* **Mirroring** — Kaynaklar, crates.io’nun önbelleği gibi çalışan eşdeğer sürümlerle değiştirilebilir.

👉 Cargo’nun temel varsayımı: her iki kaynaktaki kaynak kod **tamamen aynı** olmalıdır.

* Bu, değiştirme kaynağının orijinal kaynakta olmayan crate’lere sahip olamayacağı anlamına gelir.
* Dolayısıyla kaynak değiştirme, bir bağımlılığı yamalamak veya özel bir registry için uygun değildir.
* Bağımlılık yamalama için `[patch]` kullanılmalıdır; özel registry için ise *Registries* bölümüne bakın.

Kaynak değiştirme kullanıldığında, registry ile doğrudan iletişim kurması gereken komutlar `--registry` seçeneğiyle çalıştırılmalıdır. Bu, hangi registry’nin kullanılacağını netleştirir ve ilgili kimlik doğrulama token’ını kullanır.

---

## ⚙️ Yapılandırma (configuration)

Kaynak değiştirme ayarları `.cargo/config.toml` dosyasında yapılır. Kullanılabilen anahtarlar:

```toml
# Kaynak değiştirme ile ilgili tüm ayarlar `source` tablosunda saklanır.
[source]

# Yeni bir kaynak tanımı: `my-vendor-source`
[source.my-vendor-source]
directory = "vendor"

# crates.io için varsayılan kaynak "crates-io" ismiyle erişilir.
# Burada `replace-with` ile yukarıdaki vendor kaynağı kullanılır.
[source.crates-io]
replace-with = "my-vendor-source"

# Başka bir kaynağın da değiştirilebileceğini gösteren örnek
[source.the-source-name]
replace-with = "another-source"

# Kaynak türleri (detayları aşağıda):
registry = "https://example.com/path/to/index"
local-registry = "path/to/registry"
directory = "path/to/vendor"

# Git kaynağı (opsiyonel branch/tag/rev ile)
git = "https://example.com/path/to/repo"
# branch = "master"
# tag = "v1.0.1"
# rev = "313f44e8"
```

---

## 🗄️ Registry Kaynakları (registry sources)

Bir **registry kaynağı**, crates.io’nun kendisiyle aynı formattadır.

* Git reposu içinde bir index barındırır.
* Bu repo, crate’lerin nereden indirileceğini belirtir.

Şu anda crates.io’nun resmi bir mirror projesi yoktur, ancak gelecekte olabilir.

---

## 💾 Yerel Registry Kaynakları (local registry sources)

Bir **yerel registry kaynağı**, başka bir registry kaynağının alt kümesi olup yerel dosya sisteminde bulunur (**vendoring**).

* Genellikle `Cargo.lock` ile senkronize edilir.
* `.crate` dosyaları ve bir index içerir.

👉 Yönetim için `cargo-local-registry` alt komutu kullanılır (crates.io’dan `cargo install cargo-local-registry` ile yüklenebilir).

Yerel registry:

* Tek bir dizin içinde bulunur.
* İçinde crates.io’dan indirilen `.crate` dosyaları vardır.
* `crates.io-index` ile aynı formatta bir `index` dizini bulunur (sadece mevcut crate girişleriyle doldurulmuş).

---

## 📂 Dizin Kaynakları (directory sources)

Bir **dizin kaynağı**, yerel dosya sisteminde crate’lerin bulunduğu bir dizindir.

* `cargo vendor` komutu ile yönetilir.
* `.crate` dosyalarının açılmış (unpacked) sürümlerini içerir.

Yerel registry’den farkı:

* `.crate` arşivleri yerine, açılmış kaynak kod klasörleri bulunur.
* Bu nedenle sürüm kontrolüne eklemek için daha uygundur.
* Dizin adı için bir kısıtlama yoktur.

Her crate, dosyaların bütünlüğünü sağlamak için dosya checksum bilgisi içeren bir metadata dosyası bulundurur.

---

👉 Özetle:

* **registry** = crates.io veya benzeri kayıt defteri
* **local-registry** = önceden indirilen `.crate` arşivleri
* **directory** = açılmış crate kaynak kodları (vendor)
* **git** = git deposundaki kaynak
