## 🔍 Bağımlılık Çözümü (dependency resolution)

Cargo’nun temel görevlerinden biri, her pakette belirtilen sürüm gereksinimlerine göre kullanılacak bağımlılık sürümlerini belirlemektir. Bu işleme **bağımlılık çözümü (dependency resolution)** denir ve **çözücü (resolver)** tarafından gerçekleştirilir.

Çözümün sonucu `Cargo.lock` dosyasında saklanır. Bu dosya, bağımlılıkları belirli sürümlere “kilitler” ve zamanla sabit kalmalarını sağlar. Çözüm ağacını görselleştirmek için `cargo tree` komutu kullanılabilir.

---

## ⚖️ Kısıtlamalar ve Sezgisel Kurallar (constraints and heuristics)

Çoğu durumda tek bir “en iyi” çözüm yoktur. Çözücü, çeşitli kısıtlamalar ve sezgisel kurallar altında genel olarak uygulanabilir bir çözüm arar.

Cargo’nun çözücüsünün işleyişi aşağıdaki sahte kod (pseudo-code) ile özetlenebilir:

```rust
pub fn resolve(workspace: &[Package], policy: Policy) -> Option<ResolveGraph> {
    let dep_queue = Queue::new(workspace);
    let resolved = ResolveGraph::new();
    resolve_next(dep_queue, resolved, policy)
}

fn resolve_next(dep_queue: Queue, resolved: ResolveGraph, policy: Policy) -> Option<ResolveGraph> {
    let Some(dep_spec) = policy.pick_next_dep(dep_queue) else {
        // Bitti
        return Some(resolved);
    };

    if let Some(resolved) = policy.try_unify_version(dep_spec, resolved.clone()) {
        return Some(resolved);
    }

    let dep_versions = dep_spec.lookup_versions()?;
    let mut dep_versions = policy.filter_versions(dep_spec, dep_versions);
    while let Some(dep_version) = policy.pick_next_version(&mut dep_versions) {
        if policy.needs_version_unification(dep_version, &resolved) {
            continue;
        }

        let mut dep_queue = dep_queue.clone();
        dep_queue.enqueue(dep_version.dependencies);
        let mut resolved = resolved.clone();
        resolved.register(dep_version);
        if let Some(resolved) = resolve_next(dep_queue, resolved) {
            return Some(resolved);
        }
    }

    // Geçerli çözüm bulunamadı, geri dön ve `pick_next_version`
    None
}
```

### 🔑 Temel adımlar

* **Bağımlılıkları dolaşma (`pick_next_dep`)**
  Bağımlılıkların çözülme sırası, aynı bağımlılık için sürüm gereksinimlerinin nasıl çözüleceğini ve çözücünün ne kadar geri dönüş (backtracking) yapacağını etkiler.

* **Sürümleri birleştirme (`try_unify_version`, `needs_version_unification`)**
  Cargo mümkün olduğunda sürümleri yeniden kullanır. Bu, derleme sürelerini azaltır ve ortak bağımlılıklardan gelen türlerin API’ler arasında paylaşılmasına izin verir.
  Eğer sürüm gereksinimleri çakışıyorsa, Cargo birden fazla sürüm seçmek yerine geri döner ve gerekirse hata verir.

* **Sürümleri tercih etme (`pick_next_version`)**
  Cargo genellikle en yüksek sürümü tercih eder, ancak gerekirse geri dönüş yaparak sonraki sürümü seçebilir.

---

## 🔢 Sürüm Numaraları (version numbers)

Genellikle Cargo, mevcut en yüksek sürümü tercih eder.

```toml
[dependencies]
bitflags = "*"
```

Eğer `Cargo.lock` oluşturulduğunda `bitflags`’in en yüksek sürümü `1.2.1` ise, bu sürüm seçilecektir.

---

## 📌 Sürüm Gereksinimleri (version requirements)

Paketler, hangi sürümleri desteklediklerini sürüm gereksinimleriyle belirtir.

```toml
[dependencies]
bitflags = "1.0"  # >=1.0.0, <2.0.0
```

Eğer `bitflags` için en yüksek sürüm `1.2.1` ise, bu seçilir.
Eğer `2.0.0` yayımlansa bile, uyumsuz olduğu için çözüm yine `1.2.1` olacaktır.

---

## 🔗 SemVer Uyumluluğu (semver compatibility)

Cargo, paketlerin SemVer’e uyduğunu varsayar ve `^` (caret) sürüm gereksinimlerine göre sürümleri birleştirmeye çalışır.

* Uyumsuz gereksinimler varsa Cargo hata verir.
* Ayrıntılar için *SemVer Compatibility* bölümüne bakınız.

### ✅ Birleştirilen sürüm örneği

```toml
# Paket A
[dependencies]
bitflags = "1.0"  # >=1.0.0,<2.0.0

# Paket B
[dependencies]
bitflags = "1.1"  # >=1.1.0,<2.0.0
```

Her iki paket için de `bitflags` aynı sürüme birleştirilir (ör. `1.2.1`).

---

### ❌ Çakışan sürüm örneği

```toml
# Paket A
[dependencies]
log = "=0.4.11"

# Paket B
[dependencies]
log = "=0.4.8"
```

Burada sürüm gereksinimleri çakışır → hata oluşur.

---

### ⚠️ Uyumsuz sürümler örneği

```toml
# Paket A
[dependencies]
rand = "0.7"  # >=0.7.0,<0.8.0

# Paket B
[dependencies]
rand = "0.6"  # >=0.6.0,<0.7.0
```

* Burada `rand` bağımlılığı iki ayrı sürüm (`0.6.5` ve `0.7.3`) olarak çözülür.
* Bu, API uyumsuzluklarına yol açabilir (bkz. *Version-incompatibility hazards*).

---

### 🔄 Kısmi uyum örneği

```toml
# Paket A
[dependencies]
rand = ">=0.6,<0.8.0"

# Paket B
[dependencies]
rand = "0.6"  # >=0.6.0,<0.7.0
```

* Genel olarak, bu bağımlılıklar birleşmez.
* Ancak bazı kısıtlamalar veya sezgiler devreye girerse tek bir sürüm seçilebilir (ör. `0.6.5`).

## ⚠️ Sürüm Uyumsuzluğu Tehlikeleri (version-incompatibility hazards)

Bir crate’in birden fazla sürümü çözüm grafiğinde (resolve graph) göründüğünde, bu kütüphanelerin türlerini (types) dışarıya aktardıkları durumlarda sorunlar oluşabilir. Bunun nedeni, aynı ada sahip olsalar bile Rust derleyicisinin bu türleri farklı kabul etmesidir.

Bu yüzden kütüphaneler, özellikle yaygın kullanılanlar, SemVer ile uyumsuz bir sürüm yayımlarken (ör. `1.0.0`’dan sonra `2.0.0` yayımlamak) dikkatli olmalıdır.

Bu problemi aşmak için **“semver trick”** adı verilen bir yöntem vardır.
Özetle:

* Yeni sürüm yayımlanır (ör. `2.0.0`).
* Aynı zamanda önceki sürüm için ek bir sürüm yayımlanır (ör. `1.0.1`), bu sürüm yeni sürümdeki türleri yeniden dışa aktarır (reexport).

Böylece, eski sürüm kullanan kütüphaneler yeni sürümle uyumlu hale getirilebilir.

---

### 💥 Hata Örneği

Diyelim ki `foo` adında ortak bir kütüphane hem `1.0.0` hem de `2.0.0` sürümleriyle çözüm grafiğinde yer aldı.

* Bir kütüphane `foo 1.0.0`’dan bir tür oluşturur.
* Başka bir kütüphane `downcast_ref` kullanarak `foo 2.0.0` türüne dönüştürmeye çalışır.
* Çalışma zamanında `downcast_ref` başarısız olur.

Bu nedenle, aynı türler farklı sürümlerde kullanılıyorsa dikkat edilmelidir.

👉 Çözüm: `cargo tree -d` komutuyla kopya (duplicate) sürümler ve kaynaklarını görebilirsiniz.

Ayrıca, ekosistemi etkileyebilecek popüler bir kütüphanede SemVer ile uyumsuz sürüm yayımlamadan önce sonuçlarını düşünmek önemlidir.

---

## 🦀 Rust Sürümü (rust version)

Cargo’nun çözücüsü, minimum desteklenen Rust sürümünü dikkate alabilir. Bu, `resolver.incompatible-rust-versions` ayarıyla kontrol edilir.

* **Fallback ayarı**: Çözücü, kendi Rust sürümünüzden küçük veya eşit bir `rust-version` değerine sahip paketleri tercih eder.

### Örnek 1

Siz Rust 1.85 kullanıyorsunuz. Paketin `Cargo.toml` dosyası:

```toml
[package]
name = "my-cli"
rust-version = "1.62"

[dependencies]
clap = "4.0"  # resolves to 4.0.32
```

* `clap 4.0.32` seçilir çünkü `rust-version = 1.60.0`.
* `4.0.0` seçilmez çünkü daha düşük bir sürüm numarası.
* `4.5.20` seçilmez çünkü `rust-version = 1.74.0` ve bu, `my-cli`’nin `1.62` sürümüyle uyumsuz.

---

### Örnek 2 – Uyumlu sürüm yok

```toml
[package]
name = "my-cli"
rust-version = "1.62"

[dependencies]
clap = "4.2"  # resolves to 4.5.20
```

* `4.2` aralığında `1.62` ile uyumlu bir sürüm yok.
* Çözücü hata vermez, bunun yerine `4.5.20` gibi uyumsuz bir sürümü seçer.

---

## 🏢 Çalışma Alanında Farklı Rust Sürümleri (workspace with different rust versions)

Bir çalışma alanında farklı paketler farklı Rust sürümleri belirttiğinde, çözücü “yeterince iyi” bir çözüm bulmaya çalışır.

### Örnek 3 – Daha düşük sürüm seçimi

```toml
# Paket A
[package]
name = "a"
rust-version = "1.62"

[dependencies]
clap = "4.2"  # resolves to 4.0.32

# Paket B
[package]
name = "b"

[dependencies]
clap = "4.2"  # Paket B’nin rust-version yok
```

* Paket B aslında daha yüksek sürüm (`4.5.20`) kullanabilirdi.
* Ancak çözücü, Paket A’yı dikkate alır ve `4.0.32` seçilir.

---

### Örnek 4 – Daha yüksek sürüm seçimi

```toml
# Paket A
[package]
name = "a"
rust-version = "1.62"

[dependencies]
clap = "4.2"  # resolves to 4.5.20

# Paket B
[package]
name = "b"

[dependencies]
clap = "4.5"  # resolves to 4.5.20
```

* Paket A kendi başına daha düşük sürüm kullanabilirdi.
* Ancak **sürüm birleştirme (version unification)** nedeniyle çözücü tek bir sürüm seçer.
* Sonuç: Her iki paket için de `4.5.20`.

## 🔧 Özellikler (Features)

`Cargo.lock` dosyasını üretmek için çözücü (resolver), **tüm çalışma alanı (workspace) üyelerindeki tüm özellikler (features) etkinleştirilmiş gibi** bağımlılık grafiğini kurar.
Bunun amacı: `--features` bayrağı ile özellikler eklenip çıkarıldığında gerekli olan isteğe bağlı bağımlılıkların (optional dependencies) çözüm grafiğine doğru şekilde dahil edilmesini sağlamaktır.

* Sonrasında çözücü, gerçekten hangi özelliklerin kullanılacağını belirlemek için ikinci kez çalışır.
* Bir bağımlılık, **üzerinde etkinleştirilen tüm özelliklerin birleşimi** (union) ile çözülür.

👉 Örnek:

* Paket A → `im` paketini `serde` özelliği ile ister.
* Paket B → `im` paketini `rayon` özelliği ile ister.
* Sonuç: `im` hem `serde` hem de `rayon` özellikleri etkinleştirilerek inşa edilir.

Eğer hiçbir paket bu özellikleri istemezse, ilgili bağımlılıklar çözüm grafiğine dahil edilmez.

> Aynı workspace içinde birden fazla paketi birlikte derlerseniz (`--workspace` veya `-p` bayrağı ile), tüm bağımlılıkların özellikleri **birleştirilir**. Eğer bu birleşimi istemiyorsanız, her paketi ayrı `cargo` çağrıları ile derlemelisiniz.

---

## ⏳ Özellikler ve Sürüm Uyumları

Çözücü, **gerekli özelliği içermeyen sürümleri** atlar.

* Örn: Bir paket `regex ^1` sürümünü `perf` özelliği ile isterse, seçilebilecek en düşük sürüm `1.3.0` olur çünkü `perf` özelliği daha eski sürümlerde yoktur.
* Eğer bir özellik yeni bir sürümde kaldırılırsa, o özelliğe ihtiyaç duyan paketler eski sürümde takılı kalır.

⚠️ Not: İsteğe bağlı bağımlılıklar (`optional dependencies`) da **örtük (implicit) bir özellik** tanımlar.
Dolayısıyla bu tür bağımlılıkları kaldırmak veya zorunlu hale getirmek sorun çıkarabilir.

---

## 🔄 Özellik Çözücü Sürüm 2 (Feature resolver v2)

Eğer `Cargo.toml` içinde `resolver = "2"` ayarlanırsa, daha gelişmiş bir algoritma kullanılır.

Sürüm `"1"` her durumda özellikleri birleştirir.
Sürüm `"2"` ise bazı durumlarda özellikleri birleştirmez:

1. **Hedefe özgü bağımlılıklar (target-specific dependencies)**

   ```toml
   [dependencies.common]
   version = "1.0"
   features = ["f1"]

   [target.'cfg(windows)'.dependencies.common]
   version = "1.0"
   features = ["f2"]
   ```

   Windows dışındaki platformlarda `f2` özelliği etkinleşmez.

2. **build-dependencies / proc-macros ile normal bağımlılıklar**

   ```toml
   [dependencies]
   log = "0.4"

   [build-dependencies]
   log = { version = "0.4", features = ["std"] }
   ```

   * Build script → `log` kütüphanesi `std` özelliği ile inşa edilir.
   * Ana kütüphane → `std` özelliği etkinleştirilmez.

3. **dev-dependencies ile normal bağımlılıklar**

   ```toml
   [dependencies]
   serde = { version = "1.0", default-features = false }

   [dev-dependencies]
   serde = { version = "1.0", features = ["std"] }
   ```

   * Normalde kütüphane `serde`’yi `std` özelliği olmadan bağlar.
   * Ancak testler veya örnekler (`cargo test`, `cargo build --all-targets`) derlenirken `std` özelliği eklenir.
   * ⚠️ dev-dependencies içindeki bağımlılıklar, **alt bağımlılıklarda (dependencies)** hiçbir zaman dikkate alınmaz; yalnızca en üst düzey paketler için geçerlidir.

---

## 🔗 `links` Alanı

`links` alanı, aynı yerel kütüphanenin (`native library`) yalnızca tek kopyasının bağlanmasını garanti eder.

* Eğer bir paket `libgit2-sys 0.11`, başka bir paket `libgit2-sys 0.12` isterse hata olur.
* Çünkü ikisi de aynı `git2` kütüphanesine bağlanır ve Cargo bunları birleştiremez.

👉 Bu yüzden `links` kullanan kütüphanelerde SemVer ile uyumsuz sürümler yayımlarken ekstra dikkatli olunmalıdır.

---

## 🚫 Yanked Sürümler

* Yanked edilen sürümler (geri çekilmiş sürümler) çözücü tarafından **görmezden gelinir**.
* İstisna: Eğer sürüm zaten `Cargo.lock` dosyasında kayıtlıysa veya özel olarak istenirse (`cargo update --precise`, sadece nightly’de).

---

İstersen sana `cargo tree -e features` ile bir paketin özelliklerinin nasıl birleştiğini görme örneklerini gösterebilirim. İstiyor musun?

## 🔄 Bağımlılık Güncellemeleri (Dependency updates)

Bağımlılık çözümü, bağımlılık grafiğini bilmesi gereken tüm Cargo komutları tarafından otomatik olarak yapılır.
Örneğin `cargo build`, derlenecek tüm bağımlılıkları bulmak için çözücüyü çalıştırır. İlk çalıştırmadan sonra sonuç `Cargo.lock` dosyasında saklanır. Daha sonraki komutlarda çözücü tekrar çalıştırılır ancak mümkünse `Cargo.lock` içindeki sürümlere bağlı kalınır.

* Eğer `Cargo.toml` içindeki bağımlılık listesi değişirse (örn. `1.0` sürümünden `2.0` sürümüne), çözücü yeni sürümü seçer ve `Cargo.lock` dosyası güncellenir.
* Yeni sürüm yeni gereksinimler getirirse, ek güncellemeler tetiklenebilir.
* `--locked` veya `--frozen` bayrakları, bu otomatik güncellemeleri engelleyip hata döndürmek için kullanılabilir.

`cargo update`, `Cargo.lock` içindeki bağımlılıkları yeni sürümler yayımlandığında güncellemek için kullanılır:

* Varsayılan olarak tüm paketler güncellenir.
* `-p` ile belirli bir paket güncellenebilir.
* `--recursive` ve `--precise` gibi seçenekler sürüm seçimini daha ayrıntılı kontrol eder.

---

## 📝 Overrides (Geçersiz Kılmalar)

Cargo, bağımlılıkları geçersiz kılmak için çeşitli mekanizmalara sahiptir.

* Ayrıntılar **Overriding Dependencies** bölümünde açıklanır.
* Bu mekanizmalar bir registry üzerinde bir **katman (overlay)** oluşturur ve yamalanmış sürümün yerine yenisini koyar.
* Çözümleme işlemi ise normal şekilde yapılır.

---

## 🧩 Bağımlılık Türleri (Dependency kinds)

Bir pakette üç tür bağımlılık vardır:

* normal
* build
* dev

Çözücü açısından çoğunlukla aynı şekilde işlenirler.
Farklılık: **dev-dependencies**, eğer paket çalışma alanı üyesi değilse, yok sayılır ve çözümlemeyi etkilemez.

* `[target]` tablosuyla belirtilen platforma özgü bağımlılıklar, **tüm platformlar etkinmiş gibi** çözülür.

---

## 🔁 dev-dependency Döngüleri

Çözücü genellikle döngülere izin vermez, ancak `dev-dependencies` için istisna yapılır.

👉 Örnek:

* Proje **foo** → `bar`’ı dev-dependency olarak ister.
* `bar` → `foo`’ya normal bağımlıdır (genellikle path dependency olarak).

Bu durum izinlidir çünkü derleme çıktısı açısından gerçek bir döngü yoktur:

1. `foo` kütüphanesi inşa edilir (`bar` gerekmez çünkü sadece testlerde kullanılır).
2. `bar`, `foo`’ya bağlı olarak inşa edilir.
3. Son olarak `foo` testleri, `bar` ile bağlanarak derlenir.

⚠️ Dikkat: Bu durum kafa karıştırıcı hatalara yol açabilir. Testler sırasında `foo` kütüphanesinin iki kopyası linklenebilir:

* `bar` ile birlikte kullanılan kopya,
* testleri içeren yerel kopya.

Bu iki kopyadaki tipler uyumsuz olacağı için, `bar` üzerinden `foo` tiplerini dışa aktarmak sorun yaratabilir.
👉 Mümkünse paketi parçalara bölüp döngüden tamamen kaçının.

---

## 🏷️ Çözücü Sürümleri (Resolver versions)

`Cargo.toml` içinde çözücü sürümü belirtilerek farklı davranışlar seçilebilir:

```toml
[package]
name = "my-package"
version = "1.0.0"
resolver = "2"
```

* `"1"` → Varsayılan (MSRV 1.51+)
* `"2"` → (Rust 2021 sürümü için varsayılan). Özellik birleştirme davranışını değiştirir.
* `"3"` → (Rust 2024 sürümü için varsayılan, Rust 1.84+ gerekli). `resolver.incompatible-rust-versions` için varsayılan değeri `allow` yerine `fallback` yapar.

👉 Çözücü sürümü çalışma alanı (workspace) için **küresel** bir seçenektir.

* Alt bağımlılıkların ayarı yok sayılır.
* Sanal çalışma alanı (virtual workspace) kullanılıyorsa, `[workspace]` tablosunda belirtilmelidir.

---

## ✅ Öneriler (Recommendations)

* **SemVer kurallarına uyun.**
* Çoğu durumda bağımlılıklar için `caret` (`^`) gereksinimleri kullanın → `"1.2.3"`.
* Her zaman üç bileşenli sürüm belirtin (örn. `1.2.3`).
* `*` gereksiniminden kaçının (crates.io’da yasak, tehlikeli).
* Çok geniş gereksinimlerden kaçının (`>=2.0.0` gibi).
* Çok dar gereksinimlerden de kaçının (`~1.3` ile `1.4` uyuşmaz).
* Minimum gereken sürümleri doğru belirtin.
* Sıkı ilişkili paketler için `=` gereksinimi kullanılabilir.
* `0.0.x` sürümleri kalıcı olarak kararsız paketler için uygundur.

⚖️ Genel kural:

* Gereksinimler ne kadar katıysa, çözücünün hata verme olasılığı o kadar yüksek.
* Gereksinimler çok gevşekse, ileride yayımlanan sürümler derlemeyi bozabilir.

---

## 🛠️ Sorun Giderme (Troubleshooting)

### ❓ Neden bir bağımlılık dahil edildi?

```bash
$ cargo tree --workspace --target all --all-features --invert rand
```

### ❓ Neden bu bağımlılıkta bir özellik etkinleşti?

```bash
$ cargo tree --workspace --target all --all-features --edges features --invert rand
```

### ❓ Beklenmedik bağımlılık çoğaltması

```bash
$ cargo tree --workspace --target all --all-features --duplicates
```

👉 Eğer `>=0.6` gibi geniş sürüm aralıkları kullanılırsa, Cargo iki sürümü de inşa edebilir (`0.7.3` ve `0.8.5`).
Bunu düzeltmek için `cargo update --precise` kullanılabilir.

### ❓ Neden daha yeni bir sürüm seçilmedi?

```bash
$ env CARGO_LOG=cargo::core::resolver=trace cargo update
```

### ❗ SemVer-breaking patch release buildi bozdu

* En iyi çözüm: Hatalı sürüm **yank** edilmeli.
* Beklerken:

  * Binary proje ise `--precise` ile güncellemeyi engelleyin.
  * crates.io’ya yayımlıyorsanız, geçici olarak `=` kullanarak sabitleyin.
  * Kullanıcılara `--locked` ile kurulum önerin.
  * Kütüphaneler, geçici olarak daha sıkı gereksinimler yayımlayabilir.

---

İstiyorsan sana `cargo update --precise` ile sürüm sabitlemenin birkaç pratik örneğini gösterebilirim. İster misin?
