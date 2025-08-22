## 📑 İndeks Formatı (index format)

Aşağıda, indeksin formatı tanımlanmaktadır. Ara sıra yeni özellikler eklenir ve bunlar yalnızca o özelliği tanıtan Cargo sürümünden itibaren anlaşılır. Daha eski Cargo sürümleri bu yeni özellikleri kullanan paketleri işleyemeyebilir. Ancak eski paketlerin formatı değişmediği için, eski Cargo sürümleri onları kullanabilmelidir.

---

## ⚙️ İndeks Yapılandırması (index configuration)

İndeksin kök dizininde `config.json` adlı bir dosya bulunur. Bu dosya, Cargo’nun kayda erişmesi için kullandığı JSON bilgilerini içerir. Örnek:

```json
{
    "dl": "https://crates.io/api/v1/crates",
    "api": "https://crates.io"
}
```

👉 `dl`: indeksde listelenen `crates`’leri indirmek için kullanılan URL. Aşağıdaki işaretleyiciler kullanılabilir:

* `{crate}`: `crate` adı
* `{version}`: `crate` sürümü
* `{prefix}`: `crate` adından türetilen dizin öneki (ör. `cargo` için `ca/rg`)
* `{lowerprefix}`: `{prefix}`’in küçük harfli hali
* `{sha256-checksum}`: `crate` dosyasının SHA256 özeti

👉 Eğer bu işaretleyicilerden hiçbiri kullanılmazsa, otomatik olarak `/{crate}/{version}/download` eklenir.

👉 `api`: web API’nin temel URL’si. İsteğe bağlıdır, ancak belirtilmezse `cargo publish` gibi komutlar çalışmaz.

👉 `auth-required`: `true` ise, bu özel kayıt tüm işlemler (API istekleri, `crate` indirmeleri, sparse indeks güncellemeleri) için kimlik doğrulaması ister.

---

## 📥 İndirme Uç Noktası (download endpoint)

İndirme uç noktası, istenen paketin `.crate` dosyasını göndermelidir. Cargo, `https`, `http` ve `file` URL’lerini, HTTP yönlendirmelerini, HTTP/1 ve HTTP/2’yi destekler. TLS desteği, Cargo’nun çalıştığı platforma, sürüme ve nasıl derlendiğine bağlıdır.

👉 Eğer `auth-required: true` ayarı yapılmışsa, Cargo HTTP(S) indirme isteklerine `Authorization` başlığını ekler.

---

## 📂 İndeks Dosyaları (index files)

İndeks deposunun geri kalanında, her paket için küçük harfli dosya adlarıyla bir dosya bulunur. Paketlerin her sürümü bu dosyada ayrı satır olarak yer alır. Dosyalar aşağıdaki gibi dizinlerde organize edilir:

* 1 karakterli paket adları `1` adlı dizine konur.
* 2 karakterli paket adları `2` adlı dizine konur.
* 3 karakterli paket adları `3/{ilk-harf}` dizinine konur.
* Daha uzun adlar `{ilk-iki}/{ikinci-iki}` dizinlerine konur. Örn. `cargo`, `ca/rg/cargo` içinde saklanır.

Not: Dosya adları küçük harfli olsa da, `Cargo.toml` ve indeks JSON’undaki paket adları büyük/küçük harf duyarlıdır.

👉 `{prefix}`: orijinal ada göre hesaplanan dizin öneki.
👉 `{lowerprefix}`: küçük harfe dönüştürülmüş önek.

Örn. `MyCrate`: `{prefix}` → `My/Cr`, `{lowerprefix}` → `my/cr`.

---

## ⛔ İsim Kısıtlamaları (name restrictions)

Kayıtlar, paket adları için sınırlamalar koymalıdır. Cargo şu karakterlere izin verir: `alphanumeric`, `-`, `_`.

`crates.io` kısıtlamaları:

* Yalnızca ASCII karakterleri
* Alfanümerik, `-`, `_`
* İlk karakter alfabetik olmalı
* Büyük/küçük harf çakışmalarını engelleme
* `-` ve `_` farkını önleme
* Maksimum 64 karakter
* Windows özel dosya adları (örn. `nul`) yasak

---

## 🔢 Sürüm Benzersizliği (version uniqueness)

İndeks, her sürümün yalnızca bir kez görünmesini sağlamalıdır. Örn. `1.0.7` ve `1.0.7+extra` birlikte bulunamaz.

---

## 📋 JSON Şeması (JSON schema)

Her satır, yayımlanmış bir paket sürümünü tanımlayan JSON nesnesidir. Örnek:

```json
{
    "name": "foo",
    "vers": "0.1.0",
    "deps": [
        {
            "name": "rand",
            "req": "^0.6",
            "features": ["i128_support"],
            "optional": false,
            "default_features": true,
            "target": null,
            "kind": "normal",
            "registry": null,
            "package": null
        }
    ],
    "cksum": "d867001db0e2b6e0496f9fac96930e2d42233ecd3ca0413e0753d4c7695d289c",
    "features": {
        "extras": ["rand/simd_support"]
    },
    "yanked": false,
    "links": null,
    "v": 2,
    "features2": {
        "serde": ["dep:serde", "chrono?/serde"]
    },
    "rust_version": "1.60"
}
```

👉 `yanked` dışında JSON nesneleri eklendikten sonra değişmemelidir.

---

## 📊 Yayınlama API’si ile Farklar (publish API differences)

* `deps.name`: publish API → orijinal isim, index → takma isim
* `req`: publish API → `version_req`
* `cksum`: publish API hesaplamaz, kayıt ekler
* `features`: bazıları `features2`’ye ayrılabilir
* publish API, `description`, `readme` gibi ek alanlar içerir

---

## 📊 Cargo Metadata ile Farklar (cargo metadata differences)

* `vers`: metadata → `version`
* `deps.name`: metadata → orijinal isim, index → takma isim
* `default_features`: metadata → `uses_default_features`
* `registry`: metadata → `null` crates.io için, index → aynı kayıt için `null`
* metadata → ek alanlar (`source`, `path`)
* index → ek alanlar (`yanked`, `cksum`, `v`)

---

## 🔗 İndeks Protokolleri (index protocols)

Cargo, iki protokolü destekler:

* **Git protokolü**: indeks dosyaları `git` deposunda saklanır. Örn. crates.io: `https://github.com/rust-lang/crates.io-index`.
* **Sparse protokolü**: `sparse+` öneki kullanılır. Örn. crates.io: `sparse+https://index.crates.io/`.

👉 Sparse protokolünde her dosya HTTP isteği ile ayrı indirilir. HTTP/2 ve pipelining ile performans artar.

---

## 🔐 Sparse Kimlik Doğrulama (sparse authentication)

Cargo önce `config.json`’ı ister. Sunucu `401` dönerse, Cargo kimlik doğrulaması gerektiğini varsayar ve yeniden deneme yapar.

Başarısız olursa sunucu, kullanıcıya jeton almak için `www-authenticate` başlığı ile `Cargo login_url="<URL>"` dönebilir.

---

## 🗄️ Önbellekleme (caching)

Cargo, crate meta verilerini önbelleğe alır ve `ETag` veya `Last-Modified` başlıklarını saklar. Yenilemede `If-None-Match` veya `If-Modified-Since` gönderir. Sunucu `304 Not Modified` yanıtı verebilir.

---

## 🔄 Önbellek Geçersiz Kılma (cache invalidation)

Eğer bir kayıt CDN/proxy kullanıyorsa, dosyalar güncellendiğinde önbelleğin temizlenmesi gerekir. Aksi halde yeni paketlere erişim engellenir.

---

## 🚫 Var Olmayan Crates (nonexistent crates)

Var olmayan paketlerde sunucu `404 Not Found`, `410 Gone` veya `451 Unavailable For Legal Reasons` döndürmelidir.

---

## ⚠️ Sparse Kısıtlamaları (sparse limitations)

Lockfile’da kayıt URL’si saklandığı için, her iki protokolü de aynı anda sunmak önerilmez.
`crates.io` istisnadır: Cargo, sparse kullanıldığında otomatik olarak eşdeğer `git` URL’sine dönüştürür.

👉 Eğer iki protokol de sunuluyorsa, birini kanonik olarak seçmek ve diğerini **source replacement** ile yönlendirmek önerilir.
