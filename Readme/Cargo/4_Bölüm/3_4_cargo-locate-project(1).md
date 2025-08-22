## 📋 cargo-locate-project(1)

### 🏷️ İSİM (NAME)

cargo-locate-project — Bir `Cargo.toml` dosyasının konumunun JSON gösterimini yazdırır

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo locate-project [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, manifest (manifesto) dosyasının tam yolunu içeren bir JSON nesnesini `stdout` çıktısına yazdırır. Manifest, geçerli çalışma dizininden başlayarak yukarıya doğru `Cargo.toml` adlı dosya aranarak bulunur.

Eğer proje bir `workspace`’in (çalışma alanı) parçasıysa, kök `workspace` yerine projenin manifest dosyası çıktı olarak verilir. Bu davranış `--workspace` bayrağı ile geçersiz kılınabilir. Kök `workspace`, manifest dosyası bulunduktan sonra daha yukarıya çıkarak veya `package.workspace` alanı kullanılarak bulunur.

---

### ⚙️ SEÇENEKLER (OPTIONS)

#### Workspace Seçenekleri

* `--workspace`
  Geçerli üye yerine `workspace` kökünde bulunan `Cargo.toml` dosyasını bulur.

#### Görüntüleme Seçenekleri (Display Options)

* `--message-format fmt`
  Proje konumunun yazdırılacağı formatı belirler. Geçerli değerler:

  * `json` (varsayılan): Yol, `root` anahtarının altında JSON nesnesi olarak yazdırılır.
  * `plain`: Sadece yol yazdırılır.

* `-v`, `--verbose`
  Ayrıntılı çıktı kullan. İki kez belirtilirse çok ayrıntılı (very verbose) çıktı sağlar; bağımlılık uyarıları ve `build script` çıktıları gibi ek bilgileri de içerir. Ayrıca `term.verbose` yapılandırma değeri ile belirtilebilir.

* `-q`, `--quiet`
  Cargo günlük mesajlarını yazdırmaz. `term.quiet` yapılandırma değeriyle de belirtilebilir.

* `--color when`
  Renkli çıktının ne zaman kullanılacağını denetler. Geçerli değerler:

  * `auto` (varsayılan): Terminalde renk desteği varsa otomatik olarak algılar.
  * `always`: Her zaman renkli çıktıyı gösterir.
  * `never`: Hiçbir zaman renkli çıktı göstermez.
    Ayrıca `term.color` yapılandırma değeri ile belirtilebilir.

#### Manifest Seçenekleri

* `--manifest-path path`
  `Cargo.toml` dosyasının yolu. Varsayılan olarak, Cargo geçerli dizinde veya üst dizinlerde bu dosyayı arar.

#### Ortak Seçenekler (Common Options)

* `+toolchain`
  Cargo `rustup` ile kurulmuşsa ve `cargo`ya verilen ilk argüman `+` ile başlıyorsa, bu bir `rustup` toolchain adı olarak yorumlanır (ör. `+stable`, `+nightly`). Toolchain geçersiz kılmaları hakkında daha fazla bilgi için `rustup` belgelerine bakın.

* `--config KEY=VALUE` veya `PATH`
  Bir Cargo yapılandırma değerini geçersiz kılar. Argüman, TOML söz diziminde `KEY=VALUE` olmalı ya da ek bir yapılandırma dosyasının yolu verilmelidir. Birden fazla kez belirtilebilir. Daha fazla bilgi için komut satırı geçersiz kılmaları bölümüne bakın.

* `-C PATH`
  Belirtilen işlemleri yürütmeden önce geçerli çalışma dizinini değiştirir. Bu, Cargo’nun varsayılan olarak proje manifestini (`Cargo.toml`) nerede aradığını ve `.cargo/config.toml` gibi yapılandırma dosyalarını keşfetmek için aranan dizinleri etkiler.
  Bu seçenek komut adından önce belirtilmelidir. Örnek:

  ```
  cargo -C path/to/my-project build
  ```

  Bu seçenek yalnızca `nightly` kanalında mevcuttur ve etkinleştirmek için `-Z unstable-options` bayrağı gerektirir (#10098’e bakınız).

* `-h`, `--help`
  Yardım bilgisini yazdırır.

* `-Z flag`
  Cargo için kararsız (`nightly-only`) bayraklar. Ayrıntılar için `cargo -Z help` komutunu çalıştırın.

---

### 🌍 ORTAM (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri hakkında ayrıntılar için referansa bakınız.

---

### 🔚 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarılı oldu.
* `101`: Cargo işlemi tamamlayamadı.

---

### 📚 ÖRNEKLER (EXAMPLES)

Geçerli dizine göre manifest yolunu görüntüle:

```
cargo locate-project
```

👉 Bu komut, `Cargo.toml` dosyasının yolunu JSON formatında yazdırır.

---

### 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-metadata(1)`
