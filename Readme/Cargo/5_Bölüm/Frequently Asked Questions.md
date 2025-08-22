## ❓ Sıkça Sorulan Sorular (Frequently Asked Questions)

### 📦 Plan GitHub’ı paket deposu (package repository) olarak kullanmak mı?

Hayır. Cargo’nun planı, `npmjs.com` ve `rubygems.org` gibi `crates.io` kullanmaktır.

Git depolarını (git repositories) paket kaynağı olarak her zaman desteklemeyi planlıyoruz çünkü bunlar erken geliştirme ve geçici yamalar için kullanılabilir, insanlar kayıt defterini (registry) birincil kaynak olarak kullansalar bile.

---

### 🏗️ Neden GitHub yerine crates.io oluşturuldu?

Paketleri indirmek için GitHub’dan indirme ve paketleri doğrudan kendi paketinize kopyalama dahil olmak üzere birden fazla yolu desteklemenin çok önemli olduğunu düşünüyoruz.

Bununla birlikte, `crates.io` birçok önemli avantaj sunar ve muhtemelen Cargo’daki paketlerin indirilmesinin birincil yolu haline gelecektir.

Örneğin, Node.js’in `npm`’i ve Ruby’nin `bundler`’ı hem merkezi bir kayıt defteri (registry) modelini hem de Git tabanlı bir modeli destekler. Ancak bu ekosistemlerde paketlerin çoğu kayıt defteri üzerinden indirilir, küçük ama önemli bir kısmı ise Git tabanlı paketlerden gelir.

Merkezi bir kayıt defterini popüler kılan bazı avantajlar şunlardır:

* **Keşfedilebilirlik (Discoverability):** Merkezi bir kayıt defteri mevcut paketleri bulmak için kolay bir alan sağlar. Etiketleme (tagging) ile birleştiğinde, en popüler ya da en çok bağımlı olunan paketlerin listesi gibi ekosistem genelinde bilgiler sunabilir.
* **Hız (Speed):** Merkezi bir kayıt defteri, paketlerin sadece meta verilerini hızlı ve verimli şekilde getirmeyi ve yalnızca yayımlanmış paketi indirmeyi mümkün kılar. Git depolarındaki gereksiz fazlalık indirilmeyerek bağımlılık çözümleme ve indirme hızı ciddi şekilde artar. Ayrıca herkesin yüksek hızlı, düşük gecikmeli internet bağlantısı olmadığını da unutmamak gerekir.

---

### 🛠️ Cargo C kodu (veya diğer diller) ile çalışır mı?

Evet!

Cargo Rust kodunu derler, ancak birçok Rust paketinin C koduna bağlandığını biliyoruz. Ayrıca, Rust dışındaki dilleri derlemek için onlarca yıllık araçlar bulunduğunu da biliyoruz.

Çözümümüz: Cargo, `rustc` çalıştırılmadan önce çalıştırılacak bir betiğin (script) Rust ile yazılmasına izin verir. Rust burada platforma özgü yapılandırmaları uygulamak ve paketler arasında ortak derleme işlevlerini soyutlamak için kullanılır.

---

### 🧩 Cargo `make` (veya `ninja`, ya da diğerleri) içinde kullanılabilir mi?

Kesinlikle. Cargo’nun bağımsız olarak Rust paketlerini derlemek için yararlı olmasını istiyoruz, ancak bazı kişilerin Cargo’yu başka yapı araçlarından çağırmak isteyeceğini de biliyoruz.

Cargo’yu bu bağlamlarda iyi çalışacak şekilde tasarladık; hata kodları ve makinece okunabilir çıktı modları gibi konulara dikkat ettik. Bu alanlarda hâlâ çalışmalar yapıyoruz, ancak geleneksel betikler içinde Cargo kullanımı baştan beri tasarım önceliklerimizden biri oldu.

---

### 🌍 Cargo çoklu platform paketleri veya çapraz derlemeyi (cross-compilation) destekler mi?

Rust, kod bölümlerini platforma göre yapılandırma imkânı sağlar. Cargo ayrıca platforma özgü bağımlılıkları destekler ve gelecekte `Cargo.toml` dosyasında daha fazla platform bazlı yapılandırma desteği eklemeyi planlıyoruz.

Uzun vadede, Cargo ile paketleri kolayca çapraz derlemek için yolları da araştırıyoruz.

---

### 🧪 Cargo, üretim (production) veya test gibi ortamları (environments) destekler mi?

Ortamlar, profiller (profiles) aracılığıyla desteklenir:

* ortama özgü bayraklar (`-g --opt-level=0` geliştirme için, `--opt-level=3` üretim için)
* ortama özgü bağımlılıklar (örneğin test doğrulamaları için `hamcrest`)
* ortama özgü `#[cfg]`
* `cargo test` komutu

---

### 🪟 Cargo Windows’ta çalışır mı?

Evet!

Cargo’ya yapılan tüm commit’lerin Windows’ta yerel test süitinden geçmesi gerekir. Eğer Windows’ta bir sorunla karşılaşırsanız, bunu bir hata olarak kabul ediyoruz, lütfen bir sorun bildirimi (issue) açın.

---

### 🔒 Neden `Cargo.lock` sürüm kontrolünde tutulmalı?

`cargo new` varsayılan olarak `Cargo.lock` dosyasını sürüm kontrolüne ekler, ancak bunu yapıp yapmamak paketinizin ihtiyaçlarına bağlıdır.

`Cargo.lock` kilit dosyasının amacı, başarılı bir derlemenin gerçekleştiği andaki bağımlılıkların durumunu tanımlamaktır. Böylece farklı zamanlarda ve farklı sistemlerde aynı bağımlılık sürümleriyle deterministik (öngörülebilir) derlemeler yapılır.

Deterministik derlemeler şu avantajları sağlar:

* `git bisect` ile bir hatanın kök nedenini bulmak
* CI’nın sadece yeni commit’lerden dolayı başarısız olmasını sağlamak
* Katkıcıların farklı sistemlerde tutarsız davranışlarla karşılaşmasının önüne geçmek

Ancak `Cargo.lock`, paketinizin tüketicilerini etkilemez, sadece `Cargo.toml` bunu yapar. Örneğin:

* `cargo install` en son bağımlılıkları seçecektir, `--locked` geçilmediği sürece.
* `cargo add` ile eklenen yeni bağımlılıklar en güncel sürüme kilitlenir.

Bu dosya ayrıca birleşme çatışmalarının (merge conflict) kaynağı da olabilir.

---

### 📌 Kütüphaneler bağımlılıkları için `*` sürümünü kullanabilir mi?

22 Ocak 2016’dan beri `crates.io`, yalnızca kütüphaneler değil tüm paketlerde joker (`*`) bağımlılık kısıtlamalarını reddediyor.

Kütüphaneler teorik olarak kullanabilir, ama kullanmamalıdır. Çünkü `*` ifadesi “her sürümle çalışır” demektir, ki bu asla doğru değildir. Kütüphaneler, çalıştıkları sürüm aralığını her zaman belirtmelidir.

---

### 📄 Neden `Cargo.toml`?

Cargo ile en sık etkileşilen dosyalardan biri olduğu için dosya adının neden `Cargo.toml` olduğu sıkça sorulur.

* Büyük `C`, dosyanın `Makefile` gibi diğer yapılandırma dosyalarıyla dizin listelerinde gruplanmasını sağlar.
* `.toml`, dosyanın TOML formatında olduğunu vurgular.

`cargo.toml` veya `Cargofile` gibi diğer adlar Cargo tarafından kabul edilmez; bu da bir Cargo deposunun kolayca tanımlanabilmesini sağlar.

---

### 🔌 Cargo çevrimdışı (offline) çalışabilir mi?

`--offline` veya `--frozen` bayrakları Cargo’ya ağı (network) kullanmamasını söyler. Bu durumda ağ erişimi gerekirse hata döner.

Bağımlılıkları çevrimdışına çıkmadan önce `cargo fetch` ile indirebilir ve sonra başka projelerde aynı bağımlılıkları kullanabilirsiniz.

Ayrıca `vendoring` ile ilgili daha fazla bilgi için kaynak değiştirme (source replacement) belgelerine bakabilirsiniz.

---

### 🔄 Neden Cargo kodumu yeniden derliyor?

Cargo, projedeki crate’leri artımlı (incremental) olarak derlemekten sorumludur. Ancak bazen beklenmedik şekilde yeniden derleme olabilir.

Sorunu çözmek için şu komutu çalıştırabilirsiniz:

```
$ CARGO_LOG=cargo::core::compiler::fingerprint=info cargo build
```

👉 Bu, Cargo’nun yeniden derleme ile ilgili teşhis bilgilerini ayrıntılı olarak yazdırmasını sağlar.

Bazı yaygın nedenler:

* `cargo::rerun-if-changed=foo` yazdıran ama `foo` dosyasını üretmeyen betikler.
* Ardışık derlemelerde farklı özellik (features) kümelerinin etkinleştirilmesi.
* Dosya sistemi zaman damgalarıyla ilgili sorunlar.
* Arka planda çalışan başka bir derleme sürecinin dosyaları değiştirmesi.

---

### ⚠️ “Sürüm çakışması (version conflict)” ne anlama gelir ve nasıl çözülür?

Hata mesajı şu şekilde olabilir:

```
failed to select a version for x which could resolve this conflict
```

Olası nedenler:

* Aynı yerel kütüphaneyi (native library) bağlayan farklı paketler.
* Aynı bağımlılığa farklı sürüm kısıtlamalarıyla ihtiyaç duyan crate’ler.
* `direct-minimal-versions` ile en düşük sürüm gereksinimlerinin karşılanamaması.
* Seçilen bağımlı sürümün gerekli özellikleri (features) desteklememesi.
* Dal (branch) veya PR birleştirmelerinden (merge) kaynaklanan çatışmalar.

Çözüm yolları: sürüm gereksinimlerini uyumlu hale getirmek, `cargo update` komutlarını yeniden çalıştırmak veya `Cargo.lock` dosyasını yeniden oluşturmak.

Topluluk ayrıca `Cargo.lock` ve `Cargo.toml` dosyalarındaki birleşme çatışmalarını çözmek için özel bir birleştirme aracı (merge tool) üzerinde çalışmaktadır.
