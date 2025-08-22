## 📋 cargo-update(1)

### 🏷️ İSİM (NAME)

cargo-update — Yerel `Cargo.lock` dosyasında kayıtlı bağımlılıkları günceller

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo update [options] spec
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, `Cargo.lock` dosyasındaki bağımlılıkları en son sürüme günceller. Eğer `Cargo.lock` dosyası yoksa, mevcut en güncel sürümlerle oluşturulur.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🔄 Güncelleme Seçenekleri (Update Options)

* `spec…`
  Yalnızca belirtilen paketleri günceller. Birden fazla kez kullanılabilir. (Bakınız: `cargo-pkgid(1)`)

  Belirtilmişse, yalnızca ilgili bağımlılık güncellenir. Gerekmedikçe transitif bağımlılıklar değiştirilmez. Diğer bağımlılıklar kilitli sürümlerinde kalır.

  Eğer `spec` verilmezse, tüm bağımlılıklar güncellenir.

* `--recursive`
  `spec` ile birlikte kullanıldığında, onun bağımlılıklarını da güncellemeye zorlar. `--precise` ile birlikte kullanılamaz.

* `--precise precise`
  Belirli bir sürümü doğrudan ayarlamaya izin verir. Git tabanlı paketlerde bu bir commit SHA veya etiket olabilir.

* `--breaking directory`
  `spec` paketini en son **SemVer-uyumsuz (breaking)** sürüme günceller.
  Bu seçenek yalnızca nightly kanalında kullanılabilir (`-Z unstable-options` ile).

* `-w`, `--workspace`
  Yalnızca çalışma alanında tanımlanan paketleri güncellemeyi dener.

* `--dry-run`
  Güncellenecek bağımlılıkları gösterir ama `Cargo.lock` dosyasına yazmaz.

---

### 🖥️ Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose`
* `-q`, `--quiet`
* `--color when` (`auto`, `always`, `never`)

---

### 📂 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path`
* `--ignore-rust-version`
* `--locked`
* `--offline`
* `--frozen`
* `--lockfile-path PATH` (yalnızca nightly + `-Z unstable-options`)

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

Tüm bağımlılıkları güncelle:

```
cargo update
```

👉 Bu komut, `Cargo.lock` içindeki tüm bağımlılıkları günceller.

Yalnızca belirli bağımlılıkları güncelle:

```
cargo update foo bar
```

👉 Bu komut, sadece `foo` ve `bar` bağımlılıklarını günceller.

Belirli bir bağımlılığı belirli bir sürüme sabitle:

```
cargo update foo --precise 1.2.3
```

👉 Bu komut, `foo` paketini `1.2.3` sürümüne günceller.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-generate-lockfile(1)`
