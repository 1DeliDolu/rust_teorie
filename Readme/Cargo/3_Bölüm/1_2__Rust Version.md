## 🦀 Rust Version / Rust Sürümü

`rust-version` alanı, paketinizi hangi **Rust toolchain sürümlerinde** desteklediğinizi Cargo’ya bildirir.

```toml
[package]
rust-version = "1.56"
```

* Sadece **sayı** yazılır (`1.56` gibi).
* **SemVer operatörleri** (`>=`, `<`) veya **pre-release etiketleri** (`-nightly`) kullanılamaz.
* Nightly gibi tanımlayıcılar Cargo tarafından yok sayılır.
* **MSRV (Minimum Supported Rust Version)**: 1.56’dan beri desteklenir.

---

### 📋 Kullanım Alanları

#### 🔎 Diagnostik (Teşhis)

* Kullanıcı uyumsuz bir derleyiciyle derlemeye çalışırsa → Cargo **hata verir**.
* Böylece “syntax error” gibi dolaylı hatalar yerine doğrudan uyumsuz sürüm bilgisi gösterilir.
* Tüm hedefleri kapsar: binaries, examples, tests, benchmarks.
* Kullanıcı isterse `--ignore-rust-version` ile bu kontrolü atlayabilir.

#### 🛠️ Geliştirme Yardımı

* `cargo add`, bağımlılık sürümlerini **rust-version ile uyumlu en son sürüm** olarak seçer.
* Eğer uyumsuzsa → kullanıcıya bilgi verir.
* **Resolver** bağımlılık seçiminde Rust sürümünü dikkate alabilir.
* `cargo clippy` → `incompatible_msrv` linti ile uyarı verebilir.

---

### 🤝 Destek Beklentileri

* **Complete (Tam)**: API ve binary’ler desteklenen sürümlerde çalışır.
* **Verified (Doğrulanmış)**: CI/testlerde bu sürümlerde denenmiştir.
* **Patchable (Yamalanabilir)**: Kullanıcı forku aynı Rust sürümlerinde çalışabilmeli.
* **Dependency Support**:

  * Bağımlılıklarınızın sürüm aralıkları, `rust-version` ile uyumlu en az 1 sürümü desteklemeli.
  * Ama uyumsuz sürümleri tamamen hariç tutmanıza gerek yoktur.

---

### ⚖️ Rust Sürümü Seçiminde Denge

Seçim yaparken iki maliyet vardır:

1. **Bakımcı için maliyet** → Yeni Rust özelliklerini kullanamamak.
2. **Kullanıcı için maliyet** → Daha yeni Rust özelliklerinin avantajlarını (daha hızlı derleme, std desteği) kaçırmak.

**Tavsiye edilen politika:**

* En basit → her zaman en yeni Rust sürümünü desteklemek.
* Alternatif → eski sürümleri destekleyen eski paket versiyonlarını ayakta tutmak.

---

### 📅 Yaygın Politika Örnekleri

* **N-2** → Son sürüm + 2 sürüm geriye destek.
* **Çift sürümler** → Her çift numaralı sürüm + 2 sürüm tolerans.
* **Yıllık destek** → O yıl çıkan tüm sürümler + 1 yıl tolerans.

👉 Minimum sürümünüzü bulmak için:

```
cargo-msrv
```

kullanılabilir.

---

### 🔄 Güncelleme Zamanı

* Politikanız eski bir Rust sürümünü artık desteklemiyorsa → `rust-version` hemen veya ihtiyaç olduğunda güncellenir.
* Çok bekletmek → kullanıcıların yanlış politika beklentisi oluşturmasına yol açar.

---

### 🧩 Workspace İçinde Çoklu Politikalar

* Bir workspace içinde farklı paketler farklı `rust-version` kullanabilir.
* Araçlar (`cargo-hack`) spesifik sürümlerde doğrulamada yardımcı olur.
* Ortak bağımlılıklar → en düşük sürüme göre seçilir.
* `incompatible-rust-versions = "fallback"` → bir paketin Rust sürümü, diğer paketlerin bağımlılık seçimini etkileyebilir.

---

### 📌 Örnek Politika

* Geliştirme dalı → **daima en son stable Rust**
* `rust-version` güncellemesi → minor versiyon artışıyla yapılır
* O yıl çıkan tüm Rust sürümleri + 1 yıl tolerans desteklenir
* Son desteklenen minor sürüm → topluluk tarafından backport bugfix alır

---

İstersen sana **farklı rust-version politikaları** için örnek `Cargo.toml` senaryoları hazırlayayım mı? (örneğin: latest-only, N-2 support, yearly support)
