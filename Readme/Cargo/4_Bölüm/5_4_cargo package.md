## 📋 cargo-package(1)

### 🏷️ İSİM (NAME)

cargo-package — Yerel paketi dağıtılabilir bir `tarball` haline getir

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo package [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, mevcut dizindeki paketin kaynak kodlarını içeren dağıtılabilir, sıkıştırılmış bir `.crate` dosyası oluşturur. Ortaya çıkan dosya `target/package` dizinine kaydedilir. İşlem şu adımlardan oluşur:

* Mevcut çalışma alanı yüklenir ve bazı temel kontroller yapılır.
* Yol (`path`) bağımlılıkları yalnızca bir `version` anahtarı varsa kabul edilir. Yayınlanan paketlerde `path` anahtarı yok sayılır. `dev-dependencies` için bu kısıtlama yoktur.
* Sıkıştırılmış `.crate` dosyası oluşturulur.

  * `Cargo.toml` dosyası yeniden yazılır ve normalize edilir.
  * `[patch]`, `[replace]` ve `[workspace]` bölümleri kaldırılır.
  * `Cargo.lock` her zaman dahil edilir. Eksikse, `--exclude-lockfile` belirtilmedikçe yeni bir lock dosyası oluşturulur. `cargo-install(1)` `--locked` ile paketlenmiş lock dosyasını kullanır.
  * `.cargo_vcs_info.json` dosyası eklenir (VCS commit bilgileri ve çalışma alanının temiz/dirty durumu dahil).
  * Sembolik linkler hedef dosyalarına çözülür.
  * Dosya dahil/dışlama işlemleri `include` ve `exclude` alanlarına göre yapılır.
* `.crate` dosyası çıkarılır ve temiz bir ortamda derlenerek çalışabilirliği doğrulanır. (`--no-verify` ile atlanabilir).
* Derleme betiklerinin kaynak dosyaları değiştirmediği kontrol edilir.

Dosya listesi, manifestteki `include` ve `exclude` alanları ile kontrol edilebilir.

Daha fazla bilgi için yayınlama referansına bakınız.

---

## 📄 .cargo\_vcs\_info.json formatı

Paket oluşturulurken şu formatta bir `.cargo_vcs_info.json` dosyası eklenir:

```json
{
 "git": {
   "sha1": "aac20b6e7e543e6dd4118b246c77225e3a3a1302",
   "dirty": true
 },
 "path_in_vcs": ""
}
```

* `dirty`: Paket oluşturulduğunda çalışma alanının değiştirilmiş (dirty) olduğunu belirtir.
* `path_in_vcs`: Eğer paket VCS deposunun bir alt dizinindeyse, depo köküne göre göreli yol.

Bu dosyanın uyumluluğu `cargo-metadata(1)` çıktısıyla aynı politikaya tabidir. Ancak, bu bilgi yalnızca “anlık görüntü”dür; kaynak kodun gerçekten VCS bilgileriyle eşleştiğine dair bir garanti verilmez.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 📦 Paketleme Seçenekleri (Package Options)

* `-l`, `--list` → Dosyaları listele, paket oluşturma.
* `--no-verify` → Paket içeriğini derleyerek doğrulama yapma.
* `--no-metadata` → İnsan tarafından okunabilir meta veriler (ör. açıklama, lisans) uyarılarını yok say.
* `--allow-dirty` → Commit edilmemiş değişikliklere sahip çalışma dizinini paketle.
* `--exclude-lockfile` → Lock dosyasını dahil etme.
* `--index index` → Kullanılacak kayıt defteri index URL’si.
* `--registry registry` → Paketlenecek kayıt defterinin adı. (Yayınlama yapılmaz, sadece lock dosyaları bu varsayıma göre üretilir.)
* `--message-format fmt` → Çıktı formatı (`human`, `json`). Yalnızca `--list` ile çalışır.

---

### 📦 Paket Seçimi (Package Selection)

* `-p`, `--package spec…` → Belirtilen paket(ler)i paketle. Birden çok kez belirtilebilir.
* `--workspace` → Çalışma alanındaki tüm üyeleri paketle.
* `--exclude SPEC…` → Belirtilen paketleri hariç tut. Yalnızca `--workspace` ile kullanılabilir.

---

### 🏗️ Derleme Seçenekleri (Compilation Options)

* `--target triple` → Belirtilen mimari için paketle. Varsayılan ana makine mimarisidir.
* `--target-dir directory` → Üretilen dosyaların hedef dizini. Varsayılan `target`.

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
* `--lockfile-path PATH` → Varsayılan `Cargo.lock` yolunu değiştirir. (Sadece `nightly`).

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

Mevcut paketi sıkıştırılmış `.crate` dosyası olarak oluştur:

```
cargo package
```

👉 Bu komut, mevcut paketi `target/package` dizinine `.crate` dosyası halinde paketler.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-publish(1)`
