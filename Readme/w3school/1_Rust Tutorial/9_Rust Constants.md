## 🔒 Rust Sabitler (Rust Constants)

## 📌 Sabitler (Constants)

Sabit değişkenler (constant variables), hiç değişmeyen değerleri saklamak için kullanılır.

Normal değişkenlerden farklı olarak sabitlerin mutlaka bir türü (type) olmalıdır (örneğin `i32` veya `char`).

---

## 🛠️ Sabit Oluşturma (Creating a Constant)

Bir sabit oluşturmak için `const` anahtar kelimesi (keyword) kullanılır, ardından ad, tür ve değer yazılır:

### Örnek (Example)

```rust
const BIRTHYEAR: i32 = 1980;
const MINUTES_PER_HOUR: i32 = 60;
```

---

## 📝 Sabitlerin Türü Olmalıdır (Constants Must Have a Type)

Sabit oluştururken türü mutlaka yazmalısınız. Normal değişkenlerde olduğu gibi Rust’ın türü tahmin etmesine izin veremezsiniz:

### Örnek (Example)

```rust
const BIRTHYEAR: i32 = 1980; // Ok
const BIRTHYEAR = 1980;      // Error: missing type
```

---

## 🔠 İsimlendirme Kuralları (Naming Rules)

Sabitler hakkında başka bir önemli nokta: Sabitleri büyük harflerle yazmak iyi bir uygulamadır.

Zorunlu değildir ama kodun okunabilirliği için faydalıdır ve Rust programcıları arasında yaygındır.

### Örnekler (Examples):

```
MAX_SPEED
PI
MINUTES_PER_HOUR
```

---

## ⚖️ Sabitler ve Değişkenler (Constants vs Variables)

| Özellik (Feature) | Sabit (const) | Değişken (let)       |
| ----------------- | ------------- | -------------------- |
| Değişebilir mi?   | Hayır         | Evet, `mut` ile      |
| Tür gerekli mi?   | Evet          | Hayır (isteğe bağlı) |
