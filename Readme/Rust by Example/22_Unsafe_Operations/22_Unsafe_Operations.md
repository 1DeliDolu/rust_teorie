## ⚠️ Güvensiz İşlemler (unsafe operations)

Bu bölüme giriş olarak, resmi belgelerden alıntıyla başlarsak: *"bir kod tabanında güvensiz (unsafe) kod miktarını en aza indirmek gerekir."* Bunu akılda tutarak başlayalım! Rust’ta `unsafe` ek açıklamaları (annotations), derleyicinin koyduğu korumaları atlatmak için kullanılır; özellikle `unsafe` dört temel amaç için kullanılır:

* Ham işaretçilerin (raw pointers) ayrıştırılması (dereferencing)
* Güvensiz (unsafe) olarak tanımlanmış fonksiyonların veya metotların çağrılması (FFI üzerinden yapılan çağrılar dahil, bkz. önceki bölüm)
* Statik değiştirilebilir (static mutable) değişkenlere erişmek veya onları değiştirmek
* Güvensiz trait’leri (unsafe traits) uygulamak

---

## 📌 Ham İşaretçiler (raw pointers)

Ham işaretçiler (`*`) ve referanslar (`&T`) benzer şekilde çalışır, ancak referanslar her zaman güvenlidir çünkü ödünç alma denetleyicisi (borrow checker) sayesinde geçerli verilere işaret edecekleri garanti edilir. Ham bir işaretçinin ayrıştırılması yalnızca `unsafe` bir blok içinde yapılabilir.

```rust
fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}
```

---

## 📞 Güvensiz Fonksiyonları Çağırma (calling unsafe functions)

Bazı fonksiyonlar `unsafe` olarak tanımlanabilir; bu da doğruluğu sağlama sorumluluğunu derleyiciden alıp programcıya yükler. Bunun bir örneği, ilk elemana işaretçi ve uzunluk verilerek bir slice oluşturan `std::slice::from_raw_parts` fonksiyonudur.

```rust
use std::slice;

fn main() {
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
```

`slice::from_raw_parts` için korunması gereken varsayımlardan biri, verilen işaretçinin geçerli belleğe işaret etmesi ve bu belleğin doğru türde olmasıdır. Bu kurallar korunmazsa programın davranışı tanımsız (undefined behaviour) olur ve ne olacağı önceden bilinemeyecektir.
