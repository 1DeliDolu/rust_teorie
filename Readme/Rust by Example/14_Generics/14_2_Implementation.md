## 🏗️ Uygulamalar (Implementation)

Fonksiyonlarda olduğu gibi, **uygulamalar (implementations)** da jenerik (generic) kalması için dikkatli tanımlanmalıdır.

```rust
struct S; // Somut tür `S`
struct GenericVal<T>(T); // Jenerik tür `GenericVal`

// `GenericVal` için tür parametresi açıkça belirtilmiş uygulamalar:
impl GenericVal<f32> {} // `f32` türü için
impl GenericVal<S> {}   // Yukarıda tanımlanan `S` türü için

// Jenerik kalması için `<T>` tipi önceden belirtilmeli
impl<T> GenericVal<T> {}
```

### 📝 Örnek – Jenerik ve Somut Uygulama

```rust
struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// `Val` için impl
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// Jenerik tür `T` için `GenVal` impl
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}
```

Bu örnekte:

* `Val`, somut bir türdür ve `value()` metodu `f64` döndürür.
* `GenVal<T>`, jenerik bir türdür ve `value()` metodu hangi tür verilirse onun referansını döndürür.
