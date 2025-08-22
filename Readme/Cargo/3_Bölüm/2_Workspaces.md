## 🗂️ Çalışma Alanları (workspaces)

Bir çalışma alanı (workspace), birlikte yönetilen bir veya daha fazla paketten oluşur. Bu paketlere çalışma alanı üyeleri (workspace members) denir.

Çalışma alanlarının temel noktaları şunlardır:

* `cargo check --workspace` gibi ortak komutlar tüm çalışma alanı üyelerinde çalıştırılabilir.
* Tüm paketler, çalışma alanı kökünde bulunan ortak bir `Cargo.lock` dosyasını paylaşır.
* Tüm paketler, varsayılan olarak çalışma alanı kökünde `target` adlı dizin olan ortak bir çıktı dizinini paylaşır.
* `workspace.package` ile paket metadatası paylaşımı yapılabilir.
* `Cargo.toml` dosyasındaki `[patch]`, `[replace]` ve `[profile.*]` bölümleri yalnızca kök manifestte tanınır, üye crate manifestlerinde yok sayılır.

Bir çalışma alanının kök `Cargo.toml` dosyası aşağıdaki bölümleri destekler:

* `[workspace]` — Bir çalışma alanını tanımlar.
* `resolver` — Kullanılacak bağımlılık çözümleyicisini ayarlar.
* `members` — Çalışma alanına dahil edilecek paketler.
* `exclude` — Çalışma alanından hariç tutulacak paketler.
* `default-members` — Belirli bir paket seçilmediğinde çalışılacak paketler.
* `package` — Paketlerde devralınacak anahtarlar.
* `dependencies` — Paket bağımlılıklarında devralınacak anahtarlar.
* `lints` — Paket lint ayarlarında devralınacak anahtarlar.
* `metadata` — Harici araçlar için ek ayarlar.
* `[patch]` — Bağımlılıkları geçersiz kılar.
* `[replace]` — Bağımlılıkları geçersiz kılar (kullanımdan kaldırıldı).
* `[profile]` — Derleyici ayarları ve optimizasyonlar.

---

## 🏗️ \[workspace] Bölümü

Bir çalışma alanı oluşturmak için `Cargo.toml` dosyasına `[workspace]` tablosu eklenir:

```toml
[workspace]
# ...
```

En az bir üyeye sahip olmak zorundadır; bu, kök paket ile ya da sanal manifest (virtual manifest) ile sağlanabilir.

### 📦 Kök paket (root package)

Eğer `[workspace]` bölümü zaten bir `[package]` tanımlayan `Cargo.toml` dosyasına eklenirse, bu paket çalışma alanının kök paketi olur. Çalışma alanı kökü, `Cargo.toml` dosyasının bulunduğu dizindir.

```toml
[workspace]

[package]
name = "hello_world" # paketin adı
version = "0.1.0"    # geçerli sürüm, semver kurallarına uyar
```

### 📂 Sanal çalışma alanı (virtual workspace)

Alternatif olarak, `[package]` bölümü olmadan yalnızca `[workspace]` bölümü içeren bir `Cargo.toml` dosyası oluşturulabilir. Buna sanal manifest denir. Bu yöntem, “birincil” paket olmadığında ya da tüm paketleri ayrı dizinlerde düzenlemek istediğinizde kullanışlıdır.

```toml
# [PROJECT_DIR]/Cargo.toml
[workspace]
members = ["hello_world"]
resolver = "3"

# [PROJECT_DIR]/hello_world/Cargo.toml
[package]
name = "hello_world" # paketin adı
version = "0.1.0"    # geçerli sürüm, semver kurallarına uyar
edition = "2024"     # edition, çalışma alanındaki resolver üzerinde etkili olmayacaktır
```

Bir kök paketi olmayan çalışma alanlarında:

* `resolver`, `package.edition` değerinden türetilemeyeceği için açıkça belirtilmelidir.
* Çalışma alanı kökünde çalıştırılan komutlar varsayılan olarak tüm üyelerde çalışır, bkz. `default-members`.

---

## 📋 members ve exclude Alanları

`members` ve `exclude` alanları, çalışma alanının üyelerini tanımlar:

```toml
[workspace]
members = ["member1", "path/to/member2", "crates/*"]
exclude = ["crates/foo", "path/to/other"]
```

* Çalışma alanı dizininde bulunan tüm yol bağımlılıkları otomatik olarak üye olur.
* `members` anahtarı ile ek üyeler belirtilebilir; bu, `Cargo.toml` dosyası içeren dizinlerin yollarını içeren bir dizi olmalıdır.
* `members` listesi, `*` ve `?` gibi klasik glob desenlerini destekler.
* `exclude` anahtarı, belirli yolların çalışma alanına dahil edilmemesini sağlar.

Bir alt dizinde çalışırken Cargo, üst dizinlerde `[workspace]` tanımı olan bir `Cargo.toml` arar. `package.workspace` manifest anahtarı, üye crate’lerde manuel olarak kök çalışma alanını işaret etmek için kullanılabilir. Bu ayar, üyenin çalışma alanı kökünün alt dizininde olmaması durumunda faydalıdır.

---

## 🎯 Paket Seçimi (package selection)

Bir çalışma alanında `cargo build` gibi paketlerle ilgili Cargo komutları, hangi paketlerde çalışılacağını belirlemek için `-p` / `--package` veya `--workspace` bayraklarını kullanabilir.

* Bu bayraklar belirtilmezse Cargo, geçerli çalışma dizinindeki paketi kullanır.
* Eğer geçerli dizin çalışma alanı kökü ise, `default-members` devreye girer.

## ⚙️ default-members Alanı

`default-members` alanı, çalışma alanı kökünde bulunulduğunda ve paket seçim bayrakları kullanılmadığında hangi üyeler üzerinde işlem yapılacağını belirtir:

```toml
[workspace]
members = ["path/to/member1", "path/to/member2", "path/to/member3/*"]
default-members = ["path/to/member2", "path/to/member3/foo"]
```

Not: Bir kök paket mevcutsa, yalnızca `--package` ve `--workspace` bayrakları ile üzerinde işlem yapılabilir.

Belirtilmediğinde:

* Kök paket kullanılır.
* Sanal bir çalışma alanı durumunda, tüm üyeler kullanılır (komut satırında `--workspace` belirtilmiş gibi).

---

## 📦 package Tablosu

`workspace.package` tablosu, çalışma alanı üyelerinin devralabileceği (inherit) anahtarların tanımlandığı yerdir.
Üye paketlerde `{key}.workspace = true` şeklinde tanımlanarak bu değerler devralınabilir.

Desteklenen anahtarlar:

* `authors`
* `categories`
* `description`
* `documentation`
* `edition`
* `exclude`
* `homepage`
* `include`
* `keywords`
* `license`
* `license-file`
* `publish`
* `readme`
* `repository`
* `rust-version`
* `version`

Notlar:

* `license-file` ve `readme`, çalışma alanı köküne göre görecelidir.
* `include` ve `exclude`, paket köküne göre görecelidir.

Örnek:

```toml
# [PROJECT_DIR]/Cargo.toml
[workspace]
members = ["bar"]

[workspace.package]
version = "1.2.3"
authors = ["Nice Folks"]
description = "A short description of my package"
documentation = "https://example.com/bar"

# [PROJECT_DIR]/bar/Cargo.toml
[package]
name = "bar"
version.workspace = true
authors.workspace = true
description.workspace = true
documentation.workspace = true
```

MSRV: 1.64+ gerektirir.

---

## 📚 dependencies Tablosu

`workspace.dependencies` tablosu, çalışma alanı üyelerinin devralabileceği bağımlılıkların tanımlandığı yerdir.

Çalışma alanı bağımlılığı tanımlamak, paket bağımlılıklarına benzer, fakat:

* Bu tablodaki bağımlılıklar opsiyonel olarak tanımlanamaz.
* Bu tabloda belirtilen özellikler (`features`), `[dependencies]` içindekilerle birleştirilir.

Daha sonra, paket bağımlılığı olarak `workspace = true` ile devralınabilir.

Örnek:

```toml
# [PROJECT_DIR]/Cargo.toml
[workspace]
members = ["bar"]

[workspace.dependencies]
cc = "1.0.73"
rand = "0.8.5"
regex = { version = "1.6.0", default-features = false, features = ["std"] }

# [PROJECT_DIR]/bar/Cargo.toml
[package]
name = "bar"
version = "0.2.0"

[dependencies]
regex = { workspace = true, features = ["unicode"] }

[build-dependencies]
cc.workspace = true

[dev-dependencies]
rand.workspace = true
```

MSRV: 1.64+ gerektirir.

---

## 🧹 lints Tablosu

`workspace.lints` tablosu, çalışma alanı üyelerinin devralabileceği lint yapılandırmalarının tanımlandığı yerdir.

Bir çalışma alanı lint yapılandırması tanımlamak, paket lint’lerine benzer şekilde yapılır.

Örnek:

```toml
# [PROJECT_DIR]/Cargo.toml
[workspace]
members = ["crates/*"]

[workspace.lints.rust]
unsafe_code = "forbid"

# [PROJECT_DIR]/crates/bar/Cargo.toml
[package]
name = "bar"
version = "0.1.0"

[lints]
workspace = true
```

MSRV: 1.74 itibarıyla desteklenir.

---

## 📑 metadata Tablosu

`workspace.metadata` tablosu Cargo tarafından yok sayılır ve uyarı verilmez. Bu bölüm, `Cargo.toml` içinde çalışma alanı yapılandırmasını saklamak isteyen araçlar için kullanılabilir.

Örneğin:

```toml
[workspace]
members = ["member1", "member2"]

[workspace.metadata.webcontents]
root = "path/to/webproject"
tool = ["npm", "run", "build"]
# ...
```

Aynı şekilde, paket düzeyinde `package.metadata` tablosu vardır. Cargo, bu tabloların içeriği için bir format belirlemez. Ancak harici araçların, veriler tutarlı olduğunda `workspace.metadata` ve `package.metadata` arasında uyumlu bir şekilde çalışması önerilir.

Örneğin, bir araç `package.metadata` içinde eksik olan bilgileri `workspace.metadata` içinden alabilir.
