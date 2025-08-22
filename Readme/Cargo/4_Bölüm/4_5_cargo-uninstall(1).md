## 📋 cargo-uninstall(1)

### 🏷️ İSİM (NAME)

cargo-uninstall — Bir Rust ikili dosyasını (binary) kaldır

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo uninstall [options] [spec…]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, `cargo-install(1)` ile kurulmuş bir paketi kaldırır. `spec` argümanı, kaldırılacak paketin kimliğini belirtir (bkz. `cargo-pkgid(1)`).

Varsayılan olarak bir `crate` için tüm ikili dosyalar kaldırılır. Ancak yalnızca belirli ikili dosyaları kaldırmak için `--bin` ve `--example` bayrakları kullanılabilir.

Kurulum kökü şu öncelik sırasına göre belirlenir:

* `--root` seçeneği
* `CARGO_INSTALL_ROOT` ortam değişkeni
* `install.root` Cargo yapılandırma değeri
* `CARGO_HOME` ortam değişkeni
* `$HOME/.cargo`

---

## ⚙️ SEÇENEKLER (OPTIONS)

### ❌ Kaldırma Seçenekleri (Uninstall Options)

* `-p`, `--package spec…`
  Kaldırılacak paketi belirtir.

* `--bin name…`
  Yalnızca belirtilen ikili dosyayı kaldırır.

* `--root dir`
  Paketlerin kaldırılacağı dizini belirtir.

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

* `+toolchain` → Eğer Cargo `rustup` ile kurulmuşsa, `+stable` veya `+nightly` gibi belirli bir araç zinciri seçilebilir.
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

Önceden kurulmuş bir paketi kaldır:

```
cargo uninstall ripgrep
```

👉 Bu komut, `ripgrep` paketini sistemden kaldırır.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-install(1)`
