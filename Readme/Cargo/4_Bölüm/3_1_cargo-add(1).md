
## 📦 cargo-add(1)

### 🏷️ İSİM (NAME)

cargo-add — `Cargo.toml` bildirim (manifest) dosyasına bağımlılık (dependency) ekleme

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo add [options] crate…
cargo add [options] --path path
cargo add [options] --git url [crate…]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut bağımlılıkları ekleyebilir veya değiştirebilir.

Bağımlılık kaynağı şu şekilde belirtilebilir:

* `crate@version`: Belirtilen sürüm kısıtlamasıyla kayıt defterinden (registry) getir.
* `--path path`: Belirtilen dosya yolundan getir.
* `--git url`: Belirtilen git deposundan çek.

Eğer kaynak belirtilmezse, en uygun seçenek otomatik olarak seçilir. Örneğin:

* Diğer tablolarındaki mevcut bağımlılıklar (`dev-dependencies` gibi)
* Çalışma alanı (workspace) üyeleri
* Kayıttaki (registry) en son sürüm

Zaten mevcut olan bir paket eklendiğinde, var olan giriş belirtilen bayraklarla güncellenir.

Başarılı çalıştırma sonrasında, belirtilen bağımlılığın etkin (+) ve devre dışı (-) özellikleri (features) komut çıktısında listelenir.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🔗 Kaynak Seçenekleri (Source options)

* `--git url` → Belirtilen crate’i eklemek için Git URL.
* `--branch branch` → Git’ten eklerken kullanılacak dal (branch).
* `--tag tag` → Git’ten eklerken kullanılacak etiket (tag).
* `--rev sha` → Git’ten eklerken kullanılacak belirli commit.
* `--path path` → Yerel crate için dosya yolu.
* `--base base` → Yerel crate eklerken kullanılacak yol tabanı. *(yalnızca nightly)*
* `--registry registry` → Kullanılacak kayıt defteri. Belirtilmezse `crates-io` varsayılandır.

### 📑 Bölüm Seçenekleri (Section options)

* `--dev` → Geliştirme bağımlılığı olarak ekle.
* `--build` → Derleme bağımlılığı olarak ekle.
* `--target target` → Belirtilen hedef platforma bağımlılık ekle.
  (Kabuk genişlemelerinden kaçınmak için tırnak işareti kullanın, örn: `--target 'cfg(unix)'`).

### 📌 Bağımlılık Seçenekleri (Dependency options)

* `--dry-run` → Bildirim dosyasını gerçekten yazma.
* `--rename name` → Bağımlılığı yeniden adlandır.
* `--optional` → Bağımlılığı isteğe bağlı (optional) yap.
* `--no-optional` → Bağımlılığı zorunlu (required) yap.
* `--public` → Bağımlılığı herkese açık (public) işaretle. *(yalnızca nightly)*
* `--no-public` → Bağımlılığı özel (private) işaretle. *(yalnızca nightly)*
* `--no-default-features` → Varsayılan özellikleri devre dışı bırak.
* `--default-features` → Varsayılan özellikleri yeniden etkinleştir.
* `-F features` / `--features features` → Etkinleştirilecek özellikleri belirt.
  (Virgülle veya boşlukla ayrılabilir. Birden çok kez kullanılabilir.)

### 🖥️ Görüntüleme Seçenekleri (Display options)

* `-v, --verbose` → Ayrıntılı çıktı. İki kez belirtilirse çok ayrıntılı.
* `-q, --quiet` → Çıktıyı sessize al.
* `--color when` → Renkli çıktı ayarı (`auto`, `always`, `never`).

### 📂 Bildirim Seçenekleri (Manifest options)

* `--manifest-path path` → Kullanılacak `Cargo.toml` dosya yolu.
* `-p spec, --package spec` → Yalnızca belirtilen pakete ekle.
* `--ignore-rust-version` → Paketlerdeki `rust-version` bilgisini yok say.
* `--locked` → `Cargo.lock` dosyasıyla aynı bağımlılıkların kullanılmasını zorunlu kılar.
* `--offline` → Ağ erişimini engeller.
* `--frozen` → `--locked` ve `--offline` ile eşdeğer.
* `--lockfile-path PATH` → Kilit dosyası yolunu değiştir. *(yalnızca nightly)*

### 🌍 Ortak Seçenekler (Common options)

* `+toolchain` → Belirtilen Rustup araç zinciriyle çalıştır.
* `--config KEY=VALUE or PATH` → Cargo yapılandırmasını geçersiz kıl.
* `-C PATH` → Komut çalıştırılmadan önce çalışma dizinini değiştir. *(yalnızca nightly)*
* `-h, --help` → Yardım bilgisini yazdır.
* `-Z flag` → Deneysel (nightly) bayraklar.

---

## 🌱 ORTAM DEĞİŞKENLERİ (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🚦 ÇIKIŞ DURUMU (EXIT STATUS)

* `0` → Cargo başarıyla tamamlandı.
* `101` → Cargo hata ile tamamlandı.

---

## 📚 ÖRNEKLER (EXAMPLES)

Regex bağımlılığını ekle:

```
cargo add regex
```

👉 Bu komut `regex` kütüphanesini bağımlılık olarak ekler.

Trybuild’i geliştirme bağımlılığı olarak ekle:

```
cargo add --dev trybuild
```

👉 Bu komut `trybuild` bağımlılığını yalnızca geliştirme için ekler.

Nom’un eski bir sürümünü ekle:

```
cargo add nom@5
```

👉 Bu komut `nom` kütüphanesinin 5.x sürümünü ekler.

Serileştirme (serialization) için `serde` ve `serde_json` ekle:

```
cargo add serde serde_json -F serde/derive
```

👉 Bu komut `serde` ve `serde_json` kütüphanelerini ekler ve `serde/derive` özelliğini etkinleştirir.

Windows için platforma özel bağımlılık ekle:

```
cargo add windows --target 'cfg(windows)'
```

👉 Bu komut yalnızca Windows platformunda geçerli olan `windows` bağımlılığını ekler.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-remove(1)`
