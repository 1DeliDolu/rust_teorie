## 🏠 Cargo Home (Cargo Home)

**Cargo home**, bir **indirme ve kaynak önbelleği (download and source cache)** işlevi görür. Bir crate derlenirken Cargo, indirilen bağımlılıkları Cargo home içinde saklar.

* Cargo home’un konumunu `CARGO_HOME` ortam değişkenini ayarlayarak değiştirebilirsiniz.
* `home` crate’i, bu konuma Rust kodunuz içinden erişmeniz için bir API sağlar.
* Varsayılan olarak Cargo home şu konumdadır:

```
$HOME/.cargo/
```

⚠️ Cargo home’un **iç yapısı kararlı değildir**; herhangi bir zamanda değiştirilebilir.

---

### 📂 Cargo Home Bileşenleri

#### 📄 Dosyalar

* `config.toml` → Cargo’nun **küresel yapılandırma dosyası**
* `credentials.toml` → `cargo login` ile kaydedilen özel giriş bilgileri
* `.crates.toml`, `.crates2.json` → `cargo install` ile yüklenmiş crate’lerin paket bilgileri (**elle düzenlenmemelidir!**)

#### 📂 Dizinler

* `bin/` → `cargo install` veya `rustup` ile yüklenen çalıştırılabilir dosyalar burada bulunur.
  👉 Bu dizindeki programlara erişebilmek için yolunu `$PATH` değişkenine ekleyin.

* `git/` → Git kaynakları burada saklanır:

  * `git/db/` → Bağımlılık bir Git deposuna bağlıysa, repo bare repo olarak buraya klonlanır.
  * `git/checkouts/` → Gerekli commit, `git/db` içinden çıkarılır ve buraya kopyalanır. Aynı repodan birden fazla commit çıkartılması mümkündür.

* `registry/` → crates.io gibi kayıt defterlerindeki paketler ve metadata bilgileri burada tutulur:

  * `registry/index/` → Tüm paketlerin metadata’sını içeren bare git deposu
  * `registry/cache/` → İndirilen bağımlılıkların `.crate` uzantılı gzip arşivleri
  * `registry/src/` → `.crate` arşivleri açılır ve `.rs` dosyaları buraya çıkarılır

---

### 🔄 CI’da Cargo Home Önbellekleme (Caching the Cargo home in CI)

CI ortamında her seferinde tüm bağımlılıkların yeniden indirilmesini önlemek için `$CARGO_HOME` önbelleğe alınabilir.

⚠️ Ancak tüm dizini önbelleğe almak **verimsizdir**, çünkü aynı kaynaklar iki kez saklanır:

* `serde-1.0.92.crate` → `registry/cache` içinde
* Açılmış `.rs` dosyaları → `registry/src` içinde

Bu durum gereksiz yere derlemeyi yavaşlatır.

👉 Eğer `cargo install` ile yüklenen binary’leri önbelleğe almak istiyorsanız, şu bileşenleri saklamanız gerekir:

* `.crates.toml`
* `.crates2.json`
* `bin/`
* `registry/index/`
* `registry/cache/`
* `git/db/`

---

### 📦 Tüm Bağımlılıkları Vendor Etme (Vendoring all dependencies of a project)

Bunun için `cargo vendor` alt komutunu kullanabilirsiniz.

---

### 🧹 Önbelleği Temizleme (Clearing the cache)

İstediğiniz zaman önbelleğin herhangi bir kısmını silebilirsiniz. Cargo, ihtiyaç duyulduğunda kaynakları yeniden yükler, yeniden çıkarır veya internetten indirir.

Alternatif olarak, `cargo-cache` crate’i ile:

* Belirli kısımları temizleyebilir
* Bileşenlerin boyutlarını komut satırında görüntüleyebilirsiniz.
