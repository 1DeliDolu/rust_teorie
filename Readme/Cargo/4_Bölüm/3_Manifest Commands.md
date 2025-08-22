## 📦 Manifest Komutları (Manifest Commands)

### ➕ cargo-add (cargo add)

Bir paketi (crate) mevcut `Cargo.toml` dosyasına bağımlılık (dependency) olarak ekler.

### ℹ️ cargo-info (cargo info)

Bir paket hakkında bilgi gösterir. Paket sürümleri, yazarlar, lisans ve diğer meta verileri listeler.

### 🔒 cargo-generate-lockfile (cargo generate-lockfile)

Bir `Cargo.lock` dosyası oluşturur. Bu dosya, bağımlılıkların kesin sürümlerini kilitlemek için kullanılır.

### 📂 cargo-locate-project (cargo locate-project)

Belirtilen dizin veya proje için `Cargo.toml` dosyasının yolunu bulur ve gösterir.

### 📝 cargo-metadata (cargo metadata)

Bir projenin paket yapısını ve bağımlılıklarını JSON formatında çıktı verir. Genellikle araçlar ve IDE entegrasyonları için kullanılır.

### 🆔 cargo-pkgid (cargo pkgid)

Bir paketin kimliğini (package ID) döndürür. Paketleri benzersiz şekilde tanımlamak için kullanılır.

### ➖ cargo-remove (cargo remove)

Mevcut `Cargo.toml` dosyasından bir bağımlılığı kaldırır.

### 🌳 cargo-tree (cargo tree)

Bir projenin bağımlılık ağacını (dependency tree) gösterir. Paketlerin hangi bağımlılıklar üzerinden geldiğini anlamak için kullanılır.

### 🔄 cargo-update (cargo update)

Bağımlılıkları günceller ve `Cargo.lock` dosyasını yeniden yazar. Varsayılan olarak en son uyumlu sürümlere günceller.

### 📦 cargo-vendor (cargo vendor)

Tüm bağımlılıkları yerel bir `vendor` dizinine kopyalar. Çevrimdışı derleme veya bağımlılıkların sabitlenmesi gerektiğinde kullanılır.
