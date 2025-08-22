## 🧩 İlkel Türler (primitives)

Rust, çok çeşitli ilkel türlere erişim sağlar. Örnek olarak:

### 🔢 Skaler Türler (scalar types)

* İşaretli tamsayılar (signed integers): `i8`, `i16`, `i32`, `i64`, `i128` ve `isize` (işaretçi boyutu)
* İşaretsiz tamsayılar (unsigned integers): `u8`, `u16`, `u32`, `u64`, `u128` ve `usize` (işaretçi boyutu)
* Kayan nokta (floating point): `f32`, `f64`
* `char` Unicode skaler değerler (ör. `'a'`, `'α'`, `'∞'`) — her biri 4 bayt
* `bool` ya `true` ya da `false`
* Birim türü (unit type) `()`, tek olası değeri boş bir tuple’dır: `()`

Birim türü bir tuple olmasına rağmen, birden fazla değer içermediği için bileşik tür (compound type) olarak kabul edilmez.

### 🧱 Bileşik Türler (compound types)

* Diziler (arrays): `[1, 2, 3]`
* Tuple’lar (tuples): `(1, true)`

Değişkenler her zaman tür açıklamalı (type annotated) olabilir. Sayılar ayrıca bir son ek (suffix) ile veya varsayılan (default) olarak açıklanabilir. Tamsayılar varsayılan olarak `i32`, kayan noktalar varsayılan olarak `f64` türündedir. Rust aynı zamanda türleri bağlamdan da çıkarabilir (infer types).

```rust
fn main() {
    // Değişkenler tür açıklamalı olabilir.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Normal açıklama
    let an_integer   = 5i32; // Sonek açıklaması

    // Ya da varsayılan tür kullanılabilir.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // Tür bağlamdan da çıkarılabilir.
    let mut inferred_type = 12; // Başka bir satırdan `i64` olarak çıkarılır.
    inferred_type = 4294967296i64;

    // Değiştirilebilir bir değişkenin değeri değiştirilebilir.
    let mut mutable = 12; // Değiştirilebilir `i32`
    mutable = 21;

    // Hata! Bir değişkenin türü değiştirilemez.
    mutable = true;

    // Gölgeleme (shadowing) ile değişkenin üzerine yazılabilir.
    let mutable = true;

    /* Bileşik türler - Dizi (Array) ve Tuple */

    // Dizi imzası Tür T ve uzunluk [T; uzunluk] şeklindedir.
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple farklı türde değerlerin bir koleksiyonudur
    // ve parantez () kullanılarak oluşturulur.
    let my_tuple = (5u32, 1u8, true, -5.04f32);
}
```

👉 Bu örnek, Rust’ta skaler ve bileşik türlerin nasıl tanımlandığını ve kullanıldığını göstermektedir.

## 📚 Ayrıca bakınız

* `std` kütüphanesi
* `mut`
* Tür çıkarımı (inference)
* Gölgeleme (shadowing)
