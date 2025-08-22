## 📋 cargo-yank(1)

### 🏷️ İSİM (NAME)

cargo-yank — Yüklenmiş bir crate’i kayıt defteri indeksinden kaldır

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo yank [options] crate@version
cargo yank [options] --version version [crate]
```

### 📝 AÇIKLAMA (DESCRIPTION)

`yank` komutu, daha önce yayınlanmış bir `crate` sürümünü sunucunun indeksinden kaldırır.
Bu komut herhangi bir veriyi silmez; `crate` hâlâ kayıt defterinin indirme bağlantısı üzerinden indirilebilir durumda kalır.

Cargo, `yank` edilmiş bir sürümü yeni projelerde veya mevcut bir `lockfile` olmayan durumlarda kullanmaz. Eğer uyumlu başka bir sürüm yoksa hata üretir.

Bu komutun çalışması için kimlik doğrulaması gereklidir: `--token` seçeneği veya `cargo-login(1)` ile giriş yapılmış olmalıdır.

Eğer crate adı belirtilmezse, mevcut dizindeki paket adı kullanılır.

---

## ⚙️ Yank İşleyişi (How yank works)

Örnek:
`foo` crate’i `1.5.0` sürümünü yayınladı ve `bar` crate’i `foo = "1.5"` sürümüne bağımlı. Daha sonra `foo`, `2.0.0` sürümünü yayınladı (SemVer uyumlu değil) ve `1.5.0` sürümünde kritik bir hata buldu. Eğer `1.5.0` sürümü `yank` edilirse, yeni projeler veya `lockfile` olmayan ortamlar `bar` crate’ini kullanamayacaktır.

Bu durumda, `foo` geliştiricileri önce `1.5.1` gibi uyumlu bir sürüm yayınlamalı, ardından `1.5.0` sürümünü `yank` etmelidir. Böylece `bar` ve ona bağımlı tüm projeler çalışmaya devam eder.

---

## 📊 Yank Senaryoları Tablosu

| Yanked Version | bar = "1.5.0"                  | bar = "=1.5.0" | bar = "2.0.0" |
| -------------- | ------------------------------ | -------------- | ------------- |
| 1.5.0          | 1.5.1 veya 1.5.2 kullan        | Hata           | 2.0.0 kullan  |
| 1.5.1          | 1.5.0 veya 1.5.2 kullan        | 1.5.0 kullan   | 2.0.0 kullan  |
| 2.0.0          | 1.5.0, 1.5.1 veya 1.5.2 kullan | 1.5.0 kullan   | Hata          |

---

## ⏳ Ne Zaman Yank Yapılır? (When to yank)

Crate’ler yalnızca istisnai durumlarda `yank` edilmelidir:

* Yanlışlıkla yayınlama
* İstenmeyen SemVer uyumsuzluğu
* Ciddi şekilde bozuk veya kullanılamaz crate

Güvenlik açıklarında, genellikle `RustSec` aracılığıyla kullanıcıları bilgilendirmek daha doğru bir yaklaşımdır. Bu yöntem, bağımlı paketlerin derlenmesini engellemeden kullanıcıları yükseltmeye teşvik eder.

Sıklıkla kullanılan yöntem: Önce uyumlu bir sürüm yayınlamak, sonra hatalı sürümü `yank` etmek.

⚠️ Telif, lisans veya kişisel veri sorunlarında yalnızca `yank` yeterli olmayabilir. Bu durumda, kullandığınız kayıt defteri yöneticileriyle iletişime geçmeniz gerekir (`crates.io` için [help@crates.io](mailto:help@crates.io)).

⚠️ Kimlik bilgileriniz sızdırılmışsa, yapılması gereken hemen iptal etmektir. `yank`, bu bilgilerin daha önce indirilmiş kopyalarını engelleyemez.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### ❌ Yank Seçenekleri (Yank Options)

* `--vers version`, `--version version`
  Yank veya `undo` işlemi yapılacak sürüm.

* `--undo`
  Daha önce yank edilmiş sürümü geri ekler (index’e tekrar koyar).

* `--token token`
  Kimlik doğrulamada kullanılacak API anahtarı. `cargo-login(1)` ile kaydedilmiş kimlik bilgilerini geçersiz kılar.

  * `crates.io` için: `CARGO_REGISTRY_TOKEN`
  * Diğer kayıt defterleri için: `CARGO_REGISTRIES_NAME_TOKEN` (NAME büyük harflerle).

* `--index index`
  Kullanılacak kayıt defteri `index` URL’si.

* `--registry registry`
  Kullanılacak kayıt defterinin adı. Belirtilmezse varsayılan (`crates-io`) kullanılır.

---

### 👀 Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose` → Ayrıntılı çıktı.
* `-q`, `--quiet` → Sessiz çıktı.
* `--color when` → Renkli çıktıyı kontrol et (`auto`, `always`, `never`).

---

### 🔨 Ortak Seçenekler (Common Options)

* `+toolchain` → Belirli bir Rust `toolchain` ile çalıştır.
* `--config KEY=VALUE or PATH` → Cargo yapılandırma değerini geçersiz kıl.
* `-C PATH` → Çalışma dizinini değiştir (sadece `nightly`, `-Z unstable-options` ile).
* `-h`, `--help` → Yardım bilgisini göster.
* `-Z flag` → Deneysel (`nightly`) bayraklar.

---

## 🌍 ORTAM DEĞİŞKENLERİ (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🔚 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarıyla tamamlandı.
* `101`: Cargo tamamlanamadı.

---

## 📚 ÖRNEKLER (EXAMPLES)

Bir crate sürümünü yank et:

```
cargo yank foo@1.0.7
```

👉 Bu komut, `foo` crate’inin `1.0.7` sürümünü indeks dışına alır.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-login(1)`, `cargo-publish(1)`
