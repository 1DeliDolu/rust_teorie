## 📋 cargo-publish(1)

### 🏷️ İSİM (NAME)

cargo-publish — Bir paketi kayıt defterine (registry) yükle

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo publish [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, mevcut dizindeki paketin kaynak kodlarını içeren dağıtılabilir, sıkıştırılmış bir `.crate` dosyası oluşturur ve bir kayıt defterine yükler. Varsayılan kayıt defteri `https://crates.io`’dur. İşlem şu adımlardan oluşur:

* Bazı kontroller yapılır (ör. `Cargo.toml` içindeki `package.publish` anahtarı).
* `cargo-package(1)` adımlarını izleyerek `.crate` dosyası oluşturulur.
* Paket kayıt defterine yüklenir. Sunucu ek kontroller yapar.
* İstemci, paketin index’e eklenmesini bekler. Zaman aşımı olursa tamamlandığını manuel olarak kontrol etmek gerekir. (Bu zaman aşımı yüklemeyi etkilemez.)

Bu komutun çalışması için kimlik doğrulaması gereklidir: `--token` seçeneği veya `cargo-login(1)` ile giriş yapılmış olmalıdır.

Daha fazla bilgi için paketleme ve yayınlama referansına bakınız.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🚀 Yayınlama Seçenekleri (Publish Options)

* `--dry-run` → Yükleme yapmadan tüm kontrolleri çalıştır.
* `--token token` → Kimlik doğrulamada kullanılacak API anahtarı. `cargo-login(1)` ile kaydedilmiş kimlik bilgilerini geçersiz kılar.

  * `crates.io` için: `CARGO_REGISTRY_TOKEN`
  * Diğer kayıt defterleri için: `CARGO_REGISTRIES_NAME_TOKEN` (NAME büyük harflerle).
* `--no-verify` → Paket içeriğini derleyerek doğrulama yapma.
* `--allow-dirty` → Commit edilmemiş değişikliklere sahip dizini paketlemeye izin ver.
* `--index index` → Kullanılacak kayıt defteri index URL’si.
* `--registry registry` → Yükleme yapılacak kayıt defterini belirtir. Belirtilmezse varsayılan kayıt defteri (`crates-io`) kullanılır.

---

### 📦 Paket Seçimi (Package Selection)

* `-p`, `--package spec…` → Belirtilen paketleri yayınla. Birden fazla kez belirtilebilir.
* `--workspace` → Çalışma alanındaki tüm üyeleri yayınla. (Yalnızca `nightly` + `-Z package-workspace`).
* `--exclude SPEC…` → Belirtilen paketleri hariç tut. Yalnızca `--workspace` ile kullanılabilir (`nightly`).

---

### 🏗️ Derleme Seçenekleri (Compilation Options)

* `--target triple` → Belirtilen mimari için yayınla. Varsayılan ana makine mimarisidir.
* `--target-dir directory` → Üretilen dosyaların hedef dizini (`target`).

---

### 🔧 Özellik Seçimi (Feature Selection)

* `-F`, `--features features` → Etkinleştirilecek özellikleri belirtir.
* `--all-features` → Tüm özellikleri etkinleştirir.
* `--no-default-features` → Varsayılan özelliği devre dışı bırakır.

---

### 📑 Manifest Seçenekleri (Manifest Options)

* `--manifest-path path` → Kullanılacak `Cargo.toml` dosyasının yolu.
* `--locked` → Var olan `Cargo.lock` ile aynı bağımlılıkları kullanmaya zorlar.
* `--offline` → Ağ erişimini devre dışı bırakır.
* `--frozen` → Hem `--locked` hem de `--offline` etkisini uygular.
* `--lockfile-path PATH` → Varsayılan `Cargo.lock` yolunu değiştirir (sadece `nightly`).

---

### ⚡ Çeşitli Seçenekler (Miscellaneous Options)

* `-j N`, `--jobs N` → Paralel iş sayısını ayarlar (varsayılan CPU çekirdek sayısı).
* `--keep-going` → İlk hata sonrası durmak yerine mümkün olduğunca çok paketi derlemeye devam et.

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

Mevcut paketi yayınla:

```
cargo publish
```

👉 Bu komut, mevcut paketi varsayılan kayıt defterine (`crates.io`) yükler.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-package(1)`, `cargo-login(1)`
