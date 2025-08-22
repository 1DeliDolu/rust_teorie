## 📋 cargo-logout(1)

### 🏷️ İSİM (NAME)

cargo-logout — Bir kayıt defterinden (registry) yerel olarak API anahtarını kaldır

### 📌 SÖZDİZİMİ (SYNOPSIS)

```
cargo logout [options]
```

### 📝 AÇIKLAMA (DESCRIPTION)

Bu komut, kayıtlı bir erişim anahtarını (`token`) kaldırmak için bir kimlik sağlayıcı (credential provider) çalıştırır.

Varsayılan `cargo:token` kimlik sağlayıcısı için bilgiler `$CARGO_HOME/credentials.toml` dosyasında saklanır. `CARGO_HOME` varsayılan olarak ev dizininizde `.cargo` olur.

Eğer bir kayıt defteri için özel bir `credential-provider` tanımlanmışsa o kullanılır. Aksi halde, yapılandırmadaki `registry.global-credential-providers` listesindeki sağlayıcılar sondan başlanarak denenir.

`--registry` belirtilmezse, varsayılan kayıt defteri (`registry.default`, varsayılan olarak `https://crates.io/`) için kayıtlı kimlik bilgileri kaldırılır.

Bu işlem, sunucudaki erişim anahtarını iptal etmez. Anahtarı tamamen iptal etmek için ilgili kayıt defterinin web sitesinden işlem yapmanız gerekir (ör. `https://crates.io/me` adresinden `crates.io` için anahtar iptal edilebilir).

---

## ⚙️ SEÇENEKLER (OPTIONS)

### 🚪 Çıkış Seçenekleri (Logout Options)

* `--registry registry`
  Kullanılacak kayıt defterinin adını belirtir. Kayıt defteri adları Cargo yapılandırma dosyalarında tanımlıdır. Belirtilmezse, varsayılan kayıt defteri (`crates-io`) kullanılır.

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
* `--config KEY=VALUE or PATH` → Bir Cargo yapılandırma değerini geçersiz kılar. Birden fazla kez belirtilebilir.
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

Varsayılan kayıt defterinden erişim anahtarını kaldır:

```
cargo logout
```

👉 Bu komut, `crates.io` için erişim anahtarını kaldırır.

Belirli bir kayıt defterinden erişim anahtarını kaldır:

```
cargo logout --registry my-registry
```

👉 Bu komut, `my-registry` adlı kayıt defteri için erişim anahtarını kaldırır.

---

## 🔗 BAKINIZ (SEE ALSO)

`cargo(1)`, `cargo-login(1)`
