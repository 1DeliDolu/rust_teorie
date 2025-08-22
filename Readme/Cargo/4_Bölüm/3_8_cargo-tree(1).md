## 📋 cargo-tree(1)

### 🏷️ İSİM (NAME)

cargo-tree — Bir bağımlılık grafiğinin ağaç görselleştirmesini görüntüler

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo tree [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, bağımlılıkların bir ağacını terminalde gösterir.
Örneğin, “rand” paketine bağımlı basit bir proje:

```
myproject v0.1.0 (/myproject)
└── rand v0.7.3
    ├── getrandom v0.1.14
    │   ├── cfg-if v0.1.10
    │   └── libc v0.2.68
    ├── libc v0.2.68 (*)
    ├── rand_chacha v0.2.2
    │   ├── ppv-lite86 v0.2.6
    │   └── rand_core v0.5.1
    │       └── getrandom v0.1.14 (*)
    └── rand_core v0.5.1 (*)
[build-dependencies]
└── cc v1.0.50
```

`(*)` ile işaretli paketler **tekrarlanmamış (de-duplicated)** bağımlılıklardır. Bu paketin bağımlılıkları grafikte daha önce gösterildiği için tekrar edilmez. Tekrarların listelenmesi için `--no-dedupe` kullanılabilir.

`-e` bayrağı gösterilecek bağımlılık türlerini seçmek için kullanılabilir. Örneğin `cargo tree -e features` çıktısı özellikleri (features) gösterir:

```
myproject v0.1.0 (/myproject)
└── log feature "serde"
    └── log v0.4.8
        ├── serde v1.0.106
        └── cfg-if feature "default"
            └── cfg-if v0.1.10
```

Bu ağaçta `myproject`, `serde` özelliği etkinleştirilmiş `log` paketine bağımlıdır. `log` ise `cfg-if` paketine `default` özelliği ile bağımlıdır. Özellik akışını görmek için `-i` bayrağı kullanılabilir.

---

## 🔗 Özellik Birleştirme (Feature Unification)

Bu komut, `Cargo.toml`’da listelediğinizden çok daha fazla **özellik birleştirmesi (feature-unified graph)** gösterir.
Örneğin, aynı bağımlılığı `[dependencies]` ve `[dev-dependencies]` içinde farklı özelliklerle tanımlarsanız, bu komut tüm özellikleri birleştirir ve tekrar eden paketi `(*)` ile işaretler.

Genel bir bakış için:

* `cargo build`’e yakın bir çıktı: `cargo tree -e normal,build`
* `cargo test`’e yakın bir çıktı: `cargo tree`

Ancak derleme süreci çok faktöre bağlı olduğundan bu eşleşme kesin değildir.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🌲 Ağaç Seçenekleri (Tree Options)

* `-i spec`, `--invert spec`
  Belirtilen paket için ters bağımlılıkları gösterir.

* `--prune spec`
  Belirtilen paketi bağımlılık ağacından gizler.

* `--depth depth`
  Maksimum ağaç derinliği. Örn: `1` doğrudan bağımlılıkları gösterir.

* `--no-dedupe`
  Tekrarlanan bağımlılıkların tekrar edilmesini sağlar.

* `-d`, `--duplicates`
  Birden fazla sürümü bulunan bağımlılıkları gösterir (örneğin `1.0.0` ve `2.0.0`).

* `-e kinds`, `--edges kinds`
  Gösterilecek bağımlılık türleri:
  `all`, `normal`, `build`, `dev`, `features`, `no-normal`, `no-build`, `no-dev`, `no-proc-macro`

* `--target triple`
  Verilen hedef `triple` için bağımlılıkları filtreler. Varsayılan: ana sistem.

---

### 🎨 Ağaç Biçimlendirme (Tree Formatting Options)

* `--charset charset` → `utf8` veya `ascii`

* `-f format`, `--format format` → Paket gösterim formatı

  * `{p}` → Paket adı
  * `{l}` → Lisans
  * `{r}` → Depo URL
  * `{f}` → Etkinleştirilmiş özellikler
  * `{lib}` → Kütüphane adı

* `--prefix prefix`
  Satır başı biçimi: `indent`, `depth`, `none`

---

### 📦 Paket Seçimi (Package Selection)

* `-p spec…`, `--package spec…`
* `--workspace`
* `--exclude SPEC…`

---

### 📂 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path`
* `--locked`
* `--offline`
* `--frozen`
* `--lockfile-path PATH` (yalnızca nightly + `-Z unstable-options`)

---

### 🔧 Özellik Seçimi (Feature Selection)

* `-F`, `--features features`
* `--all-features`
* `--no-default-features`

---

### 🖥️ Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose`
* `-q`, `--quiet`
* `--color when` (`auto`, `always`, `never`)

---

### ⚙️ Ortak Seçenekler (Common Options)

* `+toolchain`
* `--config KEY=VALUE or PATH`
* `-C PATH` (yalnızca nightly + `-Z unstable-options`)
* `-h`, `--help`
* `-Z flag`

---

## 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🚦 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarıyla tamamlandı.
* `101`: Cargo hata ile tamamlandı.

---

## 📚 ÖRNEKLER (EXAMPLES)

Mevcut dizindeki paket için bağımlılık ağacını göster:

```
cargo tree
```

`syn` paketine bağımlı tüm paketleri göster:

```
cargo tree -i syn
```

Her pakette etkinleştirilmiş özellikleri göster:

```
cargo tree --format "{p} {f}"
```

Birden fazla kez derlenen (farklı sürümleri olan) paketleri göster:

```
cargo tree -d
```

`syn` paketi için özelliklerin neden etkinleştirildiğini açıkla:

```
cargo tree -e features -i syn
```

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-metadata(1)`
