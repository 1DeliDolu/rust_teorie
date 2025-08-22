## 🌐 Web API (web API)

Bir kayıt, `config.json` dosyasında tanımlanan konumda bir **web API** barındırabilir ve aşağıda listelenen işlemleri destekleyebilir.

Cargo, kimlik doğrulama gerektiren istekler için **Authorization** başlığını ekler. Başlık değeri API jetonudur. Jeton geçerli değilse sunucu `403` döndürmelidir. Kullanıcılar jetonu almak için kaydın web sitesini ziyaret eder. Cargo bu jetonu `cargo login` komutu ile veya komut satırından alabilir.

Yanıtlar:

* Başarı: `2xx`
* Hata: uygun durum kodu (örn. `404`)
* Hata yanıtı, aşağıdaki yapıya sahip bir JSON nesnesi olmalıdır:

```json
{
    "errors": [
        {
            "detail": "error message text"
        }
    ]
}
```

👉 Bu yapı kullanıldığında Cargo, ayrıntılı hata mesajını kullanıcıya gösterir.

Sunucular, geriye dönük uyumluluk için beklenmeyen parametreleri veya JSON alanlarını yok saymalıdır. Eksik alanlar `null` kabul edilmelidir.

Tüm isteklerde Cargo şu başlıkları ayarlar:

* `Content-Type: application/json` (gövde içeren isteklerde)
* `Accept: application/json`
* `User-Agent: cargo/{sürüm}`

---

## 📤 Yayınlama (publish)

* **Endpoint:** `/api/v1/crates/new`
* **Method:** `PUT`
* **Authorization:** zorunlu

Yeni bir crate sürümü yayımlamak için kullanılır. Sunucu, paketi doğrulamalı, indirilmeye açmalı ve indekse eklemelidir.

Cargo tarafından gönderilen veri gövdesi:

1. JSON verisinin uzunluğunu belirten 32-bit küçük endian tamsayı
2. Paket meta verisi (JSON nesnesi)
3. `.crate` dosyasının uzunluğu (32-bit küçük endian tamsayı)
4. `.crate` dosyasının kendisi

JSON örneği:

```json
{
    "name": "foo",
    "vers": "0.1.0",
    "deps": [
        {
            "name": "rand",
            "version_req": "^0.6",
            "features": ["i128_support"],
            "optional": false,
            "default_features": true,
            "target": null,
            "kind": "normal",
            "registry": null,
            "explicit_name_in_toml": null
        }
    ],
    "features": { "extras": ["rand/simd_support"] },
    "authors": ["Alice <a@example.com>"],
    "description": null,
    "documentation": null,
    "homepage": null,
    "readme": null,
    "readme_file": null,
    "keywords": [],
    "categories": [],
    "license": null,
    "license_file": null,
    "repository": null,
    "badges": {
        "travis-ci": {
            "branch": "master",
            "repository": "rust-lang/cargo"
        }
    },
    "links": null,
    "rust_version": null
}
```

Başarılı yanıt:

```json
{
    "warnings": {
        "invalid_categories": [],
        "invalid_badges": [],
        "other": []
    }
}
```

---

## 🪓 Yank (yank)

* **Endpoint:** `/api/v1/crates/{crate_name}/{version}/yank`
* **Method:** `DELETE`
* **Authorization:** zorunlu

Belirtilen crate sürümünün indeksindeki `yank` alanını `true` yapar.

Başarılı yanıt:

```json
{ "ok": true }
```

---

## 🔄 Unyank (unyank)

* **Endpoint:** `/api/v1/crates/{crate_name}/{version}/unyank`
* **Method:** `PUT`
* **Authorization:** zorunlu

Belirtilen crate sürümünün `yank` alanını `false` yapar.

Başarılı yanıt:

```json
{ "ok": true }
```

---

## 👥 Sahipler (owners)

Cargo’nun dahili kullanıcı/sahip kavramı yoktur, ancak `cargo owner` komutu crate sahiplerini yönetmeye yardımcı olur.

### 📋 Sahipleri Listeleme (list owners)

* **Endpoint:** `/api/v1/crates/{crate_name}/owners`
* **Method:** `GET`
* **Authorization:** zorunlu

Başarılı yanıt:

```json
{
    "users": [
        {
            "id": 70,
            "login": "github:rust-lang:core",
            "name": "Core"
        }
    ]
}
```

### ➕ Sahip Ekleme (add owner)

* **Endpoint:** `/api/v1/crates/{crate_name}/owners`
* **Method:** `PUT`
* **Authorization:** zorunlu

İstek:

```json
{ "users": ["login_name"] }
```

Başarılı yanıt:

```json
{
    "ok": true,
    "msg": "user ehuss has been invited to be an owner of crate cargo"
}
```

### ➖ Sahip Kaldırma (remove owner)

* **Endpoint:** `/api/v1/crates/{crate_name}/owners`
* **Method:** `DELETE`
* **Authorization:** zorunlu

İstek:

```json
{ "users": ["login_name"] }
```

Başarılı yanıt:

```json
{
    "ok": true,
    "msg": "owners successfully removed"
}
```

---

## 🔍 Arama (search)

* **Endpoint:** `/api/v1/crates`
* **Method:** `GET`
* **Query params:**

  * `q`: arama sorgusu
  * `per_page`: sonuç sayısı (varsayılan 10, maksimum 100)

Başarılı yanıt:

```json
{
    "crates": [
        {
            "name": "rand",
            "max_version": "0.6.1",
            "description": "Random number generators and other randomness functionality.\n"
        }
    ],
    "meta": { "total": 119 }
}
```

---

## 🔑 Giriş (login)

* **Endpoint:** `/me`

Bu gerçek bir API isteği değildir. Sadece `cargo login` komutu tarafından kullanıcıya tarayıcıdan giriş yapıp API jetonu alması için bir URL göstermek amacıyla vardır.
