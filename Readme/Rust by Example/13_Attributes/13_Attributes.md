## 🏷️ Öznitelikler (Attributes)

**Öznitelik (attribute)**, bir modüle, crate’e veya öğeye uygulanan metaveridir (metadata). Bu metaveri şunlar için kullanılabilir:

* Kodun koşullu derlenmesi (conditional compilation)
* Crate adı, sürümü ve türünün (ikili veya kütüphane) ayarlanması
* Uyarıların (lints/warnings) devre dışı bırakılması
* Derleyici özelliklerinin (macros, glob imports vb.) etkinleştirilmesi
* Yabancı bir kütüphaneye bağlanma (link)
* Fonksiyonların birim testi (unit test) olarak işaretlenmesi
* Fonksiyonların benchmark parçası olarak işaretlenmesi
* Makro benzeri öznitelikler (attribute-like macros)

Öznitelikler şu şekilde görünür:

* `#[outer_attribute]`
* `#![inner_attribute]`

Aralarındaki fark, uygulandıkları yerle ilgilidir.

* `#[outer_attribute]`, hemen ardından gelen öğeye uygulanır. Örnek öğeler: fonksiyon, modül bildirimi, sabit, yapı (struct), enum.
  Örnek:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

Burada `#[derive(Debug)]` özniteliği `Rectangle` yapısına uygulanır.

* `#![inner_attribute]`, kapsayan öğeye (genellikle bir modül veya crate) uygulanır. Yani, yerleştirildiği kapsamın tamamına uygulanır.
  Örnek:

```rust
#![allow(unused_variables)]

fn main() {
    let x = 3; // Normalde kullanılmayan değişken uyarısı verirdi.
}
```

### 🎛️ Özniteliklerin Argümanları

Öznitelikler farklı sözdizimleriyle argüman alabilir:

```rust
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
```

Öznitelikler birden fazla değer alabilir ve birden fazla satıra bölünebilir:

```rust
#[attribute(value, value2)]

#[attribute(value, value2, value3,
            value4, value5)]
```
