## 📋 cargo-install(1)

### 🏷️ İSİM (NAME)

cargo-install — Bir Rust ikili dosyasını (binary) derle ve kur

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo install [options] crate[@version]…
cargo install [options] --path path
cargo install [options] --git url [crate…]
cargo install [options] --list
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, Cargo’nun yerel kurulu ikili `crate` (binary crate) kümesini yönetir. Yalnızca yürütülebilir `[[bin]]` veya `[[example]]` hedefleri olan paketler kurulabilir ve tüm yürütülebilir dosyalar kurulum kökünün `bin` klasörüne yerleştirilir. Varsayılan olarak yalnızca ikili dosyalar kurulur, `example` dosyaları değil.

Kurulum kökü şu öncelik sırasına göre belirlenir:

* `--root` seçeneği
* `CARGO_INSTALL_ROOT` ortam değişkeni
* `install.root` Cargo yapılandırma değeri
* `CARGO_HOME` ortam değişkeni
* `$HOME/.cargo`

Bir `crate`, farklı kaynaklardan kurulabilir. Varsayılan kaynak `crates.io`’dur ancak `--git`, `--path` ve `--registry` bayrakları bu kaynağı değiştirebilir. Eğer kaynak birden fazla paket içeriyorsa (`crates.io` ya da birden fazla crate barındıran bir git deposu gibi) hangi `crate`’in kurulacağı belirtilmelidir.

* `crates.io`’dan gelen `crate`ler için `--version` bayrağı ile istenilen sürüm belirtilebilir.
* Git depolarından gelen paketlerde dal (`branch`), etiket (`tag`) veya belirli bir commit (`revision`) seçilebilir.
* Bir `crate` birden fazla ikili dosya içeriyorsa, yalnızca birini kurmak için `--bin` kullanılabilir. Örnekleri kurmak için `--example` kullanılabilir.

Eğer paket zaten kuruluysa ve mevcut sürüm güncel görünmüyorsa Cargo paketi yeniden kurar. Aşağıdaki değerlerden biri değişmişse yeniden kurulum yapılır:

* Paket sürümü ve kaynağı
* Kurulu ikili dosyaların kümesi
* Seçilen `feature`ler
* Profil (`--profile`)
* Hedef (`--target`)

`--path` ile kurulum yapıldığında, başka bir paketin çakışan ikili dosyaları yoksa her zaman yeniden derlenip kurulacaktır. `--force` bayrağı, Cargo’nun her zaman yeniden kurulum yapmasını sağlar.

Eğer kaynak `crates.io` veya `--git` ise, crate varsayılan olarak geçici bir hedef dizininde derlenir. Bunu önlemek için `CARGO_TARGET_DIR` ortam değişkeni ayarlanabilir. Bu, özellikle CI sistemlerinde yapı artefaktlarını önbelleğe almak için faydalıdır.

#### 🔒 Lockfile ile Çalışma

Varsayılan olarak paketle birlikte gelen `Cargo.lock` dosyası göz ardı edilir. Bu, Cargo’nun bağımlılık sürümlerini yeniden hesaplaması ve paket yayınlandığından beri çıkmış olabilecek daha yeni sürümleri kullanması anlamına gelir.

`--locked` bayrağı, eğer mevcutsa paketle gelen `Cargo.lock` dosyasını kullanmaya zorlar. Bu, tekrarlanabilir (reproducible) derlemeler için faydalıdır. Ancak `--locked` kullanıldığında bağımlılık güncellemeleri alınmaz.

Not: Cargo 1.37’den önce yayınlanan paketlerde `Cargo.lock` dosyası mevcut değildir.

#### ⚙️ Yapılandırma Keşfi

Bu komut sistem veya kullanıcı seviyesinde çalışır, proje seviyesinde değil. Yani yerel yapılandırma keşfi göz ardı edilir. Keşif `$CARGO_HOME/config.toml` dosyasından başlar. Eğer `--path $PATH` ile kurulum yapılırsa, yerel yapılandırma kullanılacaktır.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 📦 Kurulum Seçenekleri (Install Options)

* `--vers version`, `--version version` → Kurulacak sürümü belirtir.
* `--git url` → Belirtilen `crate`’i git URL’sinden kurar.
* `--branch branch`, `--tag tag`, `--rev sha` → Git deposundan dal, etiket veya commit seçer.
* `--path path` → Yerel dosya sisteminden kurulum yapar.
* `--list` → Tüm kurulu paketleri listeler.
* `-n`, `--dry-run` → Kurulum yapmadan tüm kontrolleri gerçekleştirir.
* `-f`, `--force` → Mevcut crate’leri veya ikili dosyaları üzerine yazar.
* `--no-track` → Kurulum meta verilerini takip etmeyi devre dışı bırakır.
* `--bin name…`, `--bins` → Yalnızca belirli ikili dosyaları kurar.
* `--example name…`, `--examples` → Yalnızca örnekleri kurar.
* `--root dir` → Paketlerin kurulacağı dizini belirtir.
* `--registry registry`, `--index index` → Kullanılacak kayıt defterini veya index URL’sini belirtir.

### 🔧 Özellik Seçimi (Feature Selection)

* `-F`, `--features features` → Belirli özellikleri etkinleştirir.
* `--all-features` → Tüm mevcut özellikleri etkinleştirir.
* `--no-default-features` → Varsayılan özelliği devre dışı bırakır.

### 🏗️ Derleme Seçenekleri (Compilation Options)

* `--target triple` → Belirli bir hedef mimari için kurulum yapar.
* `--target-dir directory` → Üretilen dosyaların hedef dizini.
* `--debug` → `release` yerine `dev` profiliyle derleme yapar.
* `--profile name` → Belirli bir profil ile kurulum yapar.
* `--timings=fmts` → Derleme sürelerini raporlar.

### 📑 Manifest Seçenekleri (Manifest Options)

* `--ignore-rust-version` → `rust-version` kontrolünü yoksayar.
* `--locked` → `Cargo.lock` dosyasındaki bağımlılıkların aynısını kullanır.
* `--offline` → Ağ erişimini devre dışı bırakır.
* `--frozen` → Hem `--locked` hem de `--offline` etkisini uygular.

### ⚡ Çeşitli Seçenekler (Miscellaneous Options)

* `-j N`, `--jobs N` → Paralel iş sayısını belirler.
* `--keep-going` → Hata olsa bile mümkün olduğunca çok crate’i derler.

### 👀 Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose` → Ayrıntılı çıktı.
* `-q`, `--quiet` → Sessiz çıktı.
* `--color when` → Renkli çıktı kontrolü.
* `--message-format fmt` → Hata mesajı formatını belirler.

### 🔨 Ortak Seçenekler (Common Options)

* `+toolchain` → Belirli bir Rust `toolchain` ile çalıştırır.
* `--config KEY=VALUE or PATH` → Yapılandırma değerini geçersiz kılar.
* `-C PATH` → Çalışma dizinini değiştirir.
* `-h`, `--help` → Yardım bilgisini gösterir.
* `-Z flag` → Deneysel bayraklar.

---

## 🌍 ORTAM DEĞİŞKENLERİ (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🔚 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarıyla tamamlandı.
* `101`: Cargo tamamlanamadı.

---

## 📚 ÖRNEKLER (EXAMPLES)

`crates.io`’dan bir paket kurma veya yükseltme:

```
cargo install ripgrep
```

👉 Bu komut, `ripgrep` paketini `crates.io`’dan kurar.

Bulunduğunuz dizindeki paketi kurma veya yeniden kurma:

```
cargo install --path .
```

👉 Bu komut, mevcut dizindeki `crate`’i kurar veya yeniden kurar.

Kurulu paketleri listeleme:

```
cargo install --list
```

👉 Bu komut, kurulu paketlerin listesini ve sürümlerini gösterir.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-uninstall(1)`, `cargo-search(1)`, `cargo-publish(1)`
