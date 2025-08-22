## 📋 cargo-login(1)

### 🏷️ İSİM (NAME)

cargo-login — Bir kayıt defterine (registry) giriş yap

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo login [options] [-- args]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, bir kimlik sağlayıcı (credential provider) çalıştırarak bir erişim anahtarını (`token`) kaydeder. Böylece kimlik doğrulama gerektiren komutlar (ör. `cargo-publish(1)`) otomatik olarak kimlik doğrulaması yapabilir.

`--` işaretinden sonraki tüm argümanlar kimlik sağlayıcıya iletilir.

Varsayılan `cargo:token` kimlik sağlayıcısı için `token`, `$CARGO_HOME/credentials.toml` dosyasına kaydedilir. `CARGO_HOME` varsayılan olarak ev dizininizde `.cargo` dizinidir.

Eğer bir kayıt defteri için özel bir `credential-provider` tanımlanmışsa o kullanılır. Aksi takdirde, yapılandırmadaki `registry.global-credential-providers` listesindeki sağlayıcılar sondan başlayarak denenir.

Anahtar (`token`) standart girdiden (`stdin`) okunur.

`crates.io` için API anahtarı şu adresten alınabilir:
[https://crates.io/me](https://crates.io/me)

Anahtarın gizli tutulması çok önemlidir; başkalarıyla paylaşılmamalıdır.

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🔑 Giriş Seçenekleri (Login Options)

* `--registry registry`
  Kullanılacak kayıt defterinin adını belirtir. Kayıt defteri adları Cargo yapılandırma dosyalarında tanımlıdır. Belirtilmezse, varsayılan kayıt defteri (`registry.default`, yani `crates-io`) kullanılır.

---

### 👀 Görüntüleme Seçenekleri (Display Options)

* `-v`, `--verbose` → Ayrıntılı çıktı verir. İki kez belirtilirse çok ayrıntılı olur.
* `-q`, `--quiet` → Cargo günlük mesajlarını bastırır.
* `--color when` → Renkli çıktının ne zaman kullanılacağını kontrol eder:

  * `auto` (varsayılan)
  * `always`
  * `never`

---

### 🔨 Ortak Seçenekler (Common Options)

* `+toolchain` → Eğer Cargo `rustup` ile kurulmuşsa, `+stable` veya `+nightly` gibi bir araç zinciri seçilebilir.
* `--config KEY=VALUE or PATH` → Bir Cargo yapılandırma değerini geçersiz kılar.
* `-C PATH` → Çalışma dizinini değiştirir (sadece `nightly` sürümünde, `-Z unstable-options` ile).
* `-h`, `--help` → Yardım bilgisini gösterir.
* `-Z flag` → `nightly` için kararsız bayraklar.

---

## 🌍 ORTAM DEĞİŞKENLERİ (ENVIRONMENT)

Cargo’nun okuduğu ortam değişkenleri için referansa bakınız.

---

## 🔚 ÇIKIŞ DURUMU (EXIT STATUS)

* `0`: Cargo başarıyla tamamlandı.
* `101`: Cargo tamamlanamadı.

---

## 📚 ÖRNEKLER (EXAMPLES)

Varsayılan kayıt defteri için erişim anahtarını kaydet:

```
cargo login
```

👉 Bu komut, `crates.io` için erişim anahtarını kaydeder.

Belirli bir kayıt defteri için erişim anahtarını kaydet:

```
cargo login --registry my-registry
```

👉 Bu komut, `my-registry` adlı kayıt defteri için erişim anahtarını kaydeder.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-logout(1)`, `cargo-publish(1)`
