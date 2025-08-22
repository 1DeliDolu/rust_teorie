## 📦 Yayınlama Komutları (Publishing Commands)

### 🔑 cargo-login(1)

Cargo’ya bir kayıt defteri (registry) için kimlik doğrulama anahtarı ekler. Bu, paket yayınlamak için gereklidir.

```
cargo login <token>
```

👉 Bu komut, verilen erişim anahtarını (`token`) Cargo yapılandırmasına kaydeder.

---

### 🚪 cargo-logout(1)

Cargo’daki mevcut kimlik doğrulama anahtarını kaldırır.

```
cargo logout
```

👉 Bu komut, kayıtlı olan erişim anahtarını siler ve oturumu kapatır.

---

### 👤 cargo-owner(1)

Bir paketin sahiplerini (owners) yönetir. Paket sahipliği ekleme, kaldırma veya listeleme işlemleri yapılabilir.

```
cargo owner --add username crate
cargo owner --remove username crate
cargo owner --list crate
```

👉 Bu komut, `crate` için sahiplik haklarını yönetir.

---

### 📦 cargo-package(1)

Bir Cargo paketini yayınlamaya hazırlamak için paketler. Bu işlem, paket içeriğini `.crate` dosyası olarak oluşturur.

```
cargo package
```

👉 Bu komut, paket dosyasını oluşturur ancak yayınlamaz.

---

### 🚀 cargo-publish(1)

Bir Cargo paketini belirtilen kayıt defterine (`crates.io` varsayılan) yayınlar.

```
cargo publish
```

👉 Bu komut, paketi hazırlayıp kayıt defterine gönderir.

---

### 🪝 cargo-yank(1)

Yayınlanmış bir paket sürümünü kullanılmaz hale getirir (yank). Paket kaldırılmaz, sadece yeni bağımlılık çözümlerinde kullanılamaz hale gelir. `--undo` ile işlem geri alınabilir.

```
cargo yank --vers 1.2.3
cargo yank --vers 1.2.3 --undo
```

👉 Bu komut, belirtilen sürümü bağımlılık çözümlemelerinden çıkarır veya tekrar kullanılabilir hale getirir.
