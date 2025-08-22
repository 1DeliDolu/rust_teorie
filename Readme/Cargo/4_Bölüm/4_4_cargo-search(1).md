## 📋 cargo-search(1)

### 🏷️ İSİM (NAME)

cargo-search — Kayıt defterinde paket arama. Varsayılan kayıt defteri `crates.io`’dur.

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo search [options] [query…]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, [https://crates.io](https://crates.io) üzerinde `crate`ler için metinsel arama yapar. Eşleşen `crate`ler açıklamalarıyla birlikte, `Cargo.toml` manifestine kopyalamaya uygun TOML formatında görüntülenir.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🔍 Arama Seçenekleri (Search Options)

* `--limit limit`
  Sonuç sayısını sınırla (varsayılan: `10`, maksimum: `100`).

* `--index index`
  Kullanılacak kayıt defteri `index`inin URL’si.

* `--registry registry`
  Kullanılacak kayıt defterinin adı. Kayıt defteri adları Cargo yapılandırma dosyalarında tanımlıdır. Belirtilmezse, `registry.default` anahtarıyla tanımlanan varsayılan kayıt defteri (`crates-io`) kullanılır.

---

### 👀 Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose` → Ayrıntılı çıktı verir. İki kez belirtilirse çok ayrıntılı olur.
* `-q`, `--quiet` → Cargo günlük mesajlarını bastırır.
* `--color when` → Renkli çıktının ne zaman kullanılacağını kontrol eder.

  * `auto` (varsayılan): Terminal renk desteğini otomatik algılar.
  * `always`: Her zaman renkli çıktı.
  * `never`: Hiç renkli çıktı kullanma.

---

### 🔨 Ortak Seçenekler (Common Options)

* `+toolchain` → Eğer Cargo `rustup` ile kurulmuşsa, `+stable` veya `+nightly` gibi belirli bir araç zinciri (toolchain) seçilebilir.
* `--config KEY=VALUE or PATH` → Bir Cargo yapılandırma değerini geçersiz kılar. Birden fazla kez belirtilebilir.
* `-C PATH` → Çalışma dizinini değiştirme. Yalnızca `nightly` sürümünde kullanılabilir ve `-Z unstable-options` gerektirir.
* `-h`, `--help` → Yardım bilgisini gösterir.
* `-Z flag` → Sadece `nightly` sürümünde kullanılabilen kararsız (unstable) bayraklar.

---

## 🌍 ORTAM DEĞİŞKENLERİ (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🔚 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarıyla tamamlandı.
* `101`: Cargo tamamlanamadı.

---

## 📚 ÖRNEKLER (EXAMPLES)

`crates.io` üzerinde bir paket ara:

```
cargo search serde
```

👉 Bu komut, `serde` adlı `crate` için kayıt defterinde arama yapar ve sonuçları listeler.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-install(1)`, `cargo-publish(1)`
