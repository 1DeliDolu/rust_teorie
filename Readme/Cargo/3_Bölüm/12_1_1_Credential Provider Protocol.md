## 🔑 Kimlik Bilgisi Sağlayıcı Protokolü (credential provider protocol)

Bu belge, bir Cargo **kimlik bilgisi sağlayıcısı** (credential provider) oluşturmak için gerekli bilgileri açıklar. Bir kimlik bilgisi sağlayıcısının kurulumu veya kullanımı için bkz. **Registry Authentication (kayıt kimlik doğrulaması)**.

Harici bir kimlik bilgisi sağlayıcısı kullanıldığında, Cargo ile sağlayıcı arasındaki iletişim `stdin/stdout` üzerinden, tek satırlık JSON mesajları ile yapılır.

Cargo, kimlik bilgisi sağlayıcısını her zaman `--cargo-plugin` argümanı ile çalıştırır. Bu, sağlayıcı çalıştırılabilir dosyasının Cargo’nun ihtiyaç duyduğundan fazlasını da desteklemesine olanak tanır. Ek argümanlar JSON içindeki `args` alanında iletilir.

---

## 📄 JSON Mesajları (JSON messages)

Bu belgedeki JSON mesajlarına okunabilirlik için yeni satırlar eklenmiştir. Gerçek mesajlarda yeni satırlar bulunmamalıdır.

---

### 👋 Kimlik Bilgisi Selamlaması (credential hello)

* **Gönderen:** kimlik bilgisi sağlayıcı
* **Amaç:** işlem başlangıcında desteklenen protokolleri bildirmek

```json
{
    "v":[1]
}
```

👉 Cargo tarafından gönderilen isteklerde `v` alanı burada listelenen sürümlerden biriyle eşleşir. Eğer Cargo, sağlayıcının sunduğu sürümlerden hiçbirini desteklemiyorsa hata verir ve süreci kapatır.

---

### 🗂️ Kayıt Bilgisi (registry information)

* **Gönderen:** Cargo
* **Amaç:** tüm Cargo mesajlarına `registry` alanı olarak eklenir

```json
{
    "index-url":"https://github.com/rust-lang/crates.io-index",
    "name": "crates-io",
    "headers": ["WWW-Authenticate: cargo"]
}
```

👉 `index-url`: kayıt indeksinin URL’si.
👉 `name`: yapılandırmadaki kayıt adı (isteğe bağlı).
👉 `headers`: kimlik doğrulamalı kayıt erişiminde alınan HTTP başlıkları (isteğe bağlı).

---

### 🔑 Giriş İsteği (login request)

* **Gönderen:** Cargo
* **Amaç:** kimlik bilgilerini toplamak ve saklamak

```json
{
    "v":1,
    "kind":"login",
    "registry":{"index-url":"sparse+https://registry-url/index/", "name": "my-registry"},
    "token": "<the token value>",
    "login-url": "http://registry-url/login",
    "args":[]
}
```

👉 `token` varsa, sağlayıcı bu değeri kullanmalıdır. Eğer yoksa, sağlayıcı kullanıcıdan jeton istemelidir.

`cargo login -- <ek argümanlar>` ile sağlanan argümanlar da `args` alanında yer alır.

---

### 📖 Okuma İsteği (read request)

* **Gönderen:** Cargo
* **Amaç:** crate bilgilerini okumak için kimlik bilgisi almak

```json
{
    "v":1,
    "kind":"get",
    "operation":"read",
    "registry":{"index-url":"sparse+https://registry-url/index/", "name": "my-registry"},
    "args":[]
}
```

---

### 📤 Yayımlama İsteği (publish request)

* **Gönderen:** Cargo
* **Amaç:** crate yayımlamak için kimlik bilgisi almak

```json
{
    "v":1,
    "kind":"get",
    "operation":"publish",
    "name":"sample",
    "vers":"0.1.0",
    "cksum":"...",
    "registry":{"index-url":"sparse+https://registry-url/index/", "name": "my-registry"},
    "args":[]
}
```

---

### ✅ Başarılı Get Yanıtı (get success response)

* **Gönderen:** kimlik bilgisi sağlayıcı
* **Amaç:** Cargo’ya kimlik bilgisi vermek

```json
{"Ok":{
    "kind":"get",
    "token":"...",
    "cache":"expires",
    "expiration":1693942857,
    "operation_independent":true
}}
```

👉 `token`, kayıt için `Authorization` HTTP başlığı olarak gönderilir.
👉 `cache`: `never`, `session` veya `expires` olabilir.
👉 `operation_independent`: jetonun farklı işlemler arasında (publish, fetch) geçerli olup olmadığını belirtir.

---

### 🔓 Başarılı Login Yanıtı (login success response)

```json
{"Ok":{
    "kind":"login"
}}
```

---

### 🚪 Başarılı Logout Yanıtı (logout success response)

```json
{"Ok":{
    "kind":"logout"
}}
```

---

### ❌ Başarısız Yanıt (URL desteklenmiyor)

```json
{"Err":{
    "kind":"url-not-supported"
}}
```

👉 Sağlayıcı yalnızca belirli kayıt URL’lerini destekliyorsa ve verilen URL bu listede değilse döner. Cargo varsa başka bir sağlayıcıyı dener.

---

### ❌ Başarısız Yanıt (bulunamadı)

```json
{"Err":{
    "kind":"not-found"
}}
```

👉 Kimlik bilgisi bulunamadığında döner. Bu durum `get` isteklerinde (bilgi mevcut değilse) veya `logout` isteklerinde (silinecek bilgi yoksa) beklenir.

---

### ❌ Başarısız Yanıt (işlem desteklenmiyor)

```json
{"Err":{
    "kind":"operation-not-supported"
}}
```

👉 Sağlayıcı, istenen işlemi desteklemiyorsa döner. Örneğin, yalnızca `get` destekliyorsa ve `login` istenirse bu hata döndürülür.

---

### ❌ Başarısız Yanıt (diğer)

```json
{"Err":{
    "kind":"other",
    "message": "free form string error message",
    "caused-by": ["cause 1", "cause 2"]
}}
```

👉 Genel hata yanıtıdır. `message` serbest metinli hata mesajıdır, `caused-by` ise isteğe bağlı ayrıntılı hata zinciridir.

---

## 🔄 Örnek İletişim (example communication)

Bir `read` işlemi için jeton alma süreci:

1. Cargo, kimlik bilgisi sürecini başlatır, `stdin/stdout`’u yakalar.
2. Kimlik bilgisi süreci, Cargo’ya **Hello** mesajı gönderir:

```json
{ "v": [1] }
```

3. Cargo, kimlik bilgisi isteğini gönderir:

```json
{
    "v": 1,
    "kind": "get",
    "operation": "read",
    "registry":{"index-url":"sparse+https://registry-url/index/"}
}
```

4. Kimlik bilgisi süreci yanıt verir:

```json
{
    "token": "...",
    "cache": "session",
    "operation_independent": true
}
```

5. Cargo, `stdin` kanalını kapatır ve sağlayıcı çıkar.
6. Cargo, oturum boyunca (Cargo kapanana kadar) bu jetonu kullanır.
