## ⚖️ SemVer Uyumluluğu (SemVer Compatibility)

Bu bölüm, bir paketin yeni sürümlerinde geleneksel olarak hangi değişikliklerin uyumlu (compatibility) ya da kırıcı (breaking) SemVer (SemVer) değişiklik sayıldığına dair ayrıntılar sunar. SemVer’in ne olduğu ve Cargo’nun (Cargo) kütüphanelerin uyumluluğunu sağlamak için bunu nasıl kullandığı hakkında ayrıntılar için SemVer uyumluluğu (SemVer compatibility) bölümüne bakınız.

Bunlar yalnızca yönergelerdir; tüm projelerin uyması gereken katı kurallar olmak zorunda değildir. Değişiklik kategorileri (Change categories) bölümü, bu kılavuzun bir değişikliğin düzeyini ve ciddiyetini nasıl sınıflandırdığını açıklar. Bu kılavuzun çoğu, cargo ve rustc’nin (rustc) daha önce çalışan bir şeyi derlemeyi başaramasına neden olacak değişikliklere odaklanır. Neredeyse her değişiklik, çalışma zamanındaki davranışı olumsuz etkileyebilecek bir risk taşır ve bu gibi durumlarda bunun SemVer’le uyumsuz bir değişiklik olup olmadığına yönelik karar genellikle proje bakımcılarının kanaatine bağlıdır.

## 🗂️ Değişiklik Kategorileri (Change categories)

Aşağıda listelenen tüm politikalar, değişiklik düzeyine göre kategorize edilmiştir:

Büyük değişiklik (Major change): büyük bir SemVer artışı gerektiren değişiklik.
Küçük değişiklik (Minor change): yalnızca küçük bir SemVer artışı gerektiren değişiklik.
Olası kırıcı değişiklik (Possibly-breaking change): bazı projelerin büyük, bazılarının ise küçük sayabileceği değişiklik.

“**Olası kırıcı**” kategorisi, bir güncelleme sırasında kırılma potansiyeli taşıyan ancak mutlaka kırılmaya yol açmayan değişiklikleri kapsar. Bu değişikliklerin etkisi dikkatlice değerlendirilmelidir. Tam niteliği, değişikliğe ve proje bakımcılarının ilkelerine bağlıdır.

Bazı projeler, küçük bir değişiklikte yalnızca yama numarasını artırmayı seçebilir. SemVer belirtimini (SemVer spec) takip etmek ve yama sürümlerinde yalnızca hata düzeltmeleri uygulamak teşvik edilir. Ancak bir hata düzeltmesi, “küçük değişiklik” olarak işaretlenen ve uyumluluğu etkilememesi gereken bir API değişikliği gerektirebilir. Bu kılavuz, tek tek “küçük değişikliklerin” nasıl ele alınması gerektiği konusunda bir tutum almaz; çünkü küçük ve yama arasındaki fark, değişikliğin niteliğine bağlı teamüllerdir.

Bazı değişiklikler “küçük” olarak işaretlenmiştir; ancak bir yapıyı kırma potansiyeli taşırlar. Bu, potansiyelin son derece düşük olduğu ve muhtemelen kırılacak kodun idiomatik Rust’ta yazılmayacağı ya da özellikle kullanımının kaçınılmasının önerildiği durumlar içindir.

Bu kılavuz, “büyük” ve “küçük” terimlerini “1.0.0” veya sonraki sürümler için varsayarak kullanır. “0.y.z” ile başlayan başlangıç geliştirme sürümleri için “y”deki değişiklikler büyük, “z”deki değişiklikler ise küçük sürüm olarak değerlendirilebilir. “0.0.z” sürümleri her zaman büyük değişikliktir. Bunun nedeni, Cargo’nun yalnızca en soldaki sıfır olmayan bileşendeki değişiklikleri uyumsuz olarak değerlendirme teamülünü kullanmasıdır.

## 🔗 API Uyumluluğu (API compatibility)

Items
Büyük: Herhangi bir genel (public) öğeyi yeniden adlandırma/taşıma/kaldırma
Küçük: Yeni genel öğeler ekleme

Types
Büyük: Tanımlı (well-defined) bir türün hizalamasını, yerleşimini (layout) veya boyutunu değiştirme

Structs
Büyük: Mevcut tüm alanlar genel iken özel (private) bir yapı alanı ekleme
Büyük: Özel alan yokken genel bir alan ekleme
Küçük: En az bir özel alan zaten mevcutken özel alanlar ekleme veya kaldırma
Küçük: Tüm alanları özel olan (en az bir alanlı) bir tuple struct’tan normal struct’a veya tersine geçiş

Enums
Büyük: Yeni enum varyantları ekleme (non\_exhaustive olmadan)
Büyük: Bir enum varyantına yeni alanlar ekleme

Traits
Büyük: Varsayılanı olmayan (non-defaulted) trait öğesi ekleme
Büyük: Trait öğesi imzalarında herhangi bir değişiklik
Olası kırıcı: Varsayılanlı (defaulted) bir trait öğesi ekleme
Büyük: Trait’i nesne güvenli (object safe) olmaktan çıkaracak bir trait öğesi ekleme
Büyük: Varsayılanı olmayan bir tür parametresi (type parameter) ekleme
Küçük: Varsayılanlı bir trait tür parametresi ekleme

Implementations
Olası kırıcı: Herhangi bir gömülü (inherent) öğe ekleme

Generics
Büyük: Genel (generic) sınırları (bounds) sıkılaştırma
Küçük: Genel sınırları gevşetme
Küçük: Varsayılanlı tür parametreleri ekleme
Küçük: Bir türü, türleri özdeş kalacak şekilde genelleştirme (generics kullanma)
Büyük: Bir türü, türler olası farklı olabilecek şekilde genelleştirme
Küçük: Genel bir türü daha genel bir türe değiştirme
Büyük: RPIT’te (Return Position Impl Trait — RPIT) daha fazla genel parametreyi yakalama

Functions
Büyük: İşlev parametreleri ekleme/kaldırma
Olası kırıcı: Yeni bir işlev tür parametresi tanıtma
Küçük: Bir işlevi, özgün türü destekleyecek şekilde generics ile genelleştirme
Büyük: Bir işlevi, tür uyumsuzluğu ile generics’e genelleştirme
Küçük: `unsafe` bir işlevi güvenli (safe) yapma

Attributes
Büyük: `no_std` desteğinden `std` gerektirmeye geçiş
Büyük: Özel alanı olmayan mevcut bir enum, varyant veya struct’a `non_exhaustive` ekleme

Tooling and environment compatibility (araçlar ve ortam uyumluluğu)
Olası kırıcı: Gerekli asgari Rust sürümünü değiştirme
Olası kırıcı: Platform ve ortam gereksinimlerini değiştirme
Küçük: Yeni denetimler (lints) ekleme

Cargo
Küçük: Yeni bir Cargo özelliği (feature) ekleme
Büyük: Bir Cargo özelliğini kaldırma
Büyük: İşlevselliği veya genel öğeleri değiştiriyorsa bir özelliği özellik listeden kaldırma
Olası kırıcı: İsteğe bağlı (optional) bir bağımlılığı kaldırma
Küçük: Bağımlılık özelliklerini değiştirme
Küçük: Bağımlılıklar ekleme

## 🧩 Uygulama Uyumluluğu (Application compatibility)

### 🔗 API Uyumluluğu (API compatibility)

Aşağıdaki örneklerin tümü üç bölüm içerir: özgün kod, değiştirildikten sonraki kod ve başka bir projede yer alabilecek bir kullanım örneği. Küçük bir değişiklikte, örnek kullanım hem önceki hem de sonraki sürümle başarılı şekilde derlenebilmelidir.

**Büyük: Herhangi bir genel öğeyi yeniden adlandırma/taşıma/kaldırma**
Genel olarak sunulan bir öğenin yokluğu, o öğenin tüm kullanımlarının derlenememesine neden olur.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub fn foo() {}

///////////////////////////////////////////////////////////
// After
// ... item has been removed

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    updated_crate::foo(); // Error: cannot find function `foo`
}
```

Bu, koşullu derlemeye (conditional compilation) bağlı olarak hangi öğelerin veya davranışın mevcut olabileceğini değiştirebilen herhangi bir `cfg` özniteliği (attribute) eklemeyi de kapsar.

Azaltma stratejileri (Mitigating strategies):

* Kaldırılacak öğeleri kullanım dışı (deprecated) olarak işaretleyin ve daha sonraki bir SemVer kırıcı sürümde kaldırın.
* Yeniden adlandırılan öğeleri kullanım dışı olarak işaretleyin ve eski ada `pub use` öğesiyle yeniden dışa aktarım (re-export) yapın.

**Küçük: Yeni genel öğeler ekleme**
Yeni, genel öğeler eklemek küçük bir değişikliktir.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
// ... absence of item

///////////////////////////////////////////////////////////
// After
pub fn foo() {}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
// `foo` is not used since it didn't previously exist.
```

Nadir durumlarda bu, `glob` içe aktarımları (glob imports) nedeniyle kırıcı olabilir. Örneğin, yeni bir trait eklerseniz ve bir proje bu trait’i kapsama alanına getiren bir `glob` içe aktarımla dışa aktarmışsa ve yeni trait, üzerinde uygulandığı türlerle çakışan ilişkili bir öğe tanıtıyorsa, belirsizlik nedeniyle derleme zamanı hatası oluşabilir. Örnek:

```rust
// Breaking change example

///////////////////////////////////////////////////////////
// Before
// ... absence of trait

///////////////////////////////////////////////////////////
// After
pub trait NewTrait {
    fn foo(&self) {}
}

impl NewTrait for i32 {}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::*;

pub trait LocalTrait {
    fn foo(&self) {}
}

impl LocalTrait for i32 {}

fn main() {
    123i32.foo(); // Error:  multiple applicable items in scope
}
```

Bu, teamül gereği `glob` içe aktarımlarının ileriye dönük uyumluluk riski taşıdığı bilindiğinden büyük değişiklik olarak değerlendirilmez. Harici crate’lerden öğeleri `glob` ile içe aktarmaktan kaçınılmalıdır.

**Büyük: Tanımlı bir türün hizalamasını, yerleşimini veya boyutunu değiştirme**
Daha önce tanımlı (well-defined) olan bir türün hizalamasını, yerleşimini (layout) veya boyutunu değiştirmek kırıcı bir değişikliktir.

Genel olarak, varsayılan gösterimi kullanan türlerin tanımlı bir hizalama, yerleşim veya boyutu yoktur. Derleyici, hizalamayı, yerleşimi veya boyutu değiştirmekte serbesttir; bu nedenle kod bu konuda varsayımda bulunmamalıdır.

Not: Dış crate’ler, varsayılan gösterimli türler için bile hizalama, yerleşim veya boyut hakkında varsayımda bulunursa kırılabilir. Bu, bu varsayımlar yapılmaması gerektiğinden SemVer kırıcı değişiklik sayılmaz.

Kırıcı olmayan bazı değişiklik örnekleri şunlardır (bu kılavuzdaki diğer kuralların ihlal edilmediği varsayımıyla):

* Varsayılan gösterimli bir struct, union veya enum’un alanlarını, bu kılavuzdaki diğer kurallara uyacak şekilde eklemek, kaldırmak, yeniden sıralamak veya değiştirmek (örneğin, bu değişikliklere izin vermek için `non_exhaustive` kullanmak veya zaten özel alanlara özel değişiklikler yapmak). Bkz. `struct-add-private-field-when-public`, `struct-add-public-field-when-no-private`, `struct-private-fields-with-private`, `enum-fields-new`.
* Varsayılan gösterimli bir enum’a, enum `non_exhaustive` ise varyant eklemek. Bu, enum’un hizalamasını veya boyutunu değiştirebilir; ancak bunlar tanımlı değildir. Bkz. `enum-variant-new`.
* `repr(C)` bir struct, union veya enum’un özel alanlarını, bu kılavuzdaki diğer kurallara uyarak eklemek, kaldırmak, yeniden sıralamak veya değiştirmek (örneğin `non_exhaustive` kullanmak ya da zaten özel alanlar varken özel alan eklemek). Bkz. `repr-c-private-change`.
* `repr(C)` bir enum’a, enum `non_exhaustive` ise varyant eklemek. Bkz. `repr-c-enum-variant-new`.
* Varsayılan gösterimli bir struct, union veya enum’a `repr(C)` eklemek. Bkz. `repr-c-add`.
* Bir enum’a ilkel gösterim `repr(<int>)` eklemek. Bkz. `repr-int-enum-add`.
* Varsayılan gösterimli bir struct veya enum’a `repr(transparent)` eklemek. Bkz. `repr-transparent-add`.

`repr` özniteliğini kullanan türler, kodun bazı varsayımlarda bulunabileceği ve bu tür değiştirildiğinde bozulabileceği bir hizalama ve yerleşime sahiptir.

Bazı durumlarda, `repr` özniteliğine sahip türlerin tanımlı bir hizalama, yerleşim veya boyutu olmayabilir. Bu durumlarda türlerde değişiklik yapmak güvenli olabilir; ancak dikkatli olunmalıdır. Örneğin, hizalama, yerleşim veya boyut garantilerini belgelemediği sürece özel alanlara sahip türler, türün genel API’si hizalamayı, yerleşimi veya boyutu tam olarak tanımlamadığından dış crate’ler tarafından güvenilemez.

Tek bir özel alana ve genel bir türe sahip, `repr(transparent)` kullanan ve dokümantasyonunda genel türe şeffaf olduğu anlatılan türler gibi, özel alanı olan ve yine de iyi tanımlı olan örnekler vardır. Örneğin, `UnsafeCell`’e bakınız.

Kırıcı değişiklik örnekleri:

* Bir struct veya union’a `repr(packed)` eklemek. Bkz. `repr-packed-add`.
* Bir struct, union veya enum’a `repr(align)` eklemek. Bkz. `repr-align-add`.
* Bir struct veya union’dan `repr(packed)` kaldırmak. Bkz. `repr-packed-remove`.
* `repr(packed(N))` değerindeki `N`’i hizalamayı veya yerleşimi değiştirecek şekilde değiştirmek. Bkz. `repr-packed-n-change`.
* `repr(align(N))` değerindeki `N`’i hizalamayı değiştirecek şekilde değiştirmek. Bkz. `repr-align-n-change`.
* Bir struct, union veya enum’dan `repr(align)` kaldırmak. Bkz. `repr-align-remove`.
* `repr(C)` türünün genel alanlarının sırasını değiştirmek. Bkz. `repr-c-shuffle`.
* Bir struct, union veya enum’dan `repr(C)` kaldırmak. Bkz. `repr-c-remove`.
* Bir enum’dan `repr(<int>)` kaldırmak. Bkz. `repr-int-enum-remove`.
* Bir `repr(<int>)` enum’un ilkel gösterimini değiştirmek. Bkz. `repr-int-enum-change`.
* Bir struct veya enum’dan `repr(transparent)` kaldırmak. Bkz. `repr-transparent-remove`.

**Küçük: `repr(C)` için özel alan ekleme, kaldırma veya değiştirme**
`repr(C)` bir struct, union veya enum’un özel bir alanını eklemek, kaldırmak veya değiştirmek genellikle güvenlidir; bu kılavuzdaki diğer ilkelere uymak kaydıyla (bkz. `struct-add-private-field-when-public`, `struct-add-public-field-when-no-private`, `struct-private-fields-with-private`, `enum-fields-new`).

Örneğin, özel alanlar ancak zaten başka özel alanlar varsa veya tür `non_exhaustive` ise eklenebilir. Genel alanlar, özel alanlar varsa veya tür `non_exhaustive` ise ve ekleme diğer alanların yerleşimini değiştirmiyorsa eklenebilir.

Bununla birlikte, bu durum türün boyutunu ve hizalamasını değiştirebilir. Boyut veya hizalama değişirse dikkat edilmelidir. Özel alanlara veya `non_exhaustive`’e sahip türlerin boyutu veya hizalaması belgelenmedikçe bu konularda varsayım yapılmamalıdır.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[derive(Default)]
#[repr(C)]
pub struct Example {
    pub f1: i32,
    f2: i32, // a private field
}

///////////////////////////////////////////////////////////
// After
#[derive(Default)]
#[repr(C)]
pub struct Example {
    pub f1: i32,
    f2: i32,
    f3: i32, // a new field
}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
fn main() {
    // NOTE: Users should not make assumptions about the size or alignment
    // since they are not documented.
    let f = updated_crate::Example::default();
}
```

**Küçük: `repr(C)` enum varyantı ekleme**
Enum `non_exhaustive` ise `repr(C)` bir enum’a varyant eklemek genellikle güvenlidir. Daha fazla tartışma için bkz. `enum-variant-new`.

Bu, türün boyutunu ve hizalamasını değiştirdiğinden kırıcı bir değişiklik olabilir. Benzer endişeler için bkz. `repr-c-private-change`.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(C)]
#[non_exhaustive]
pub enum Example {
    Variant1 { f1: i16 },
    Variant2 { f1: i32 },
}

///////////////////////////////////////////////////////////
// After
#[repr(C)]
#[non_exhaustive]
pub enum Example {
    Variant1 { f1: i16 },
    Variant2 { f1: i32 },
    Variant3 { f1: i64 }, // added
}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
fn main() {
    // NOTE: Users should not make assumptions about the size or alignment
    // since they are not specified. For example, this raised the size from 8
    // to 16 bytes.
    let f = updated_crate::Example::Variant2 { f1: 123 };
}
```

**Küçük: Varsayılan gösterime `repr(C)` ekleme**
Varsayılan gösterime sahip bir struct, union veya enum’a `repr(C)` eklemek güvenlidir. Bu güvenlidir; çünkü varsayılan gösterime sahip türlerin hizalaması, yerleşimi veya boyutu hakkında varsayım yapılmamalıdır.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Example {
    pub f1: i32,
    pub f2: i16,
}

///////////////////////////////////////////////////////////
// After
#[repr(C)] // added
pub struct Example {
    pub f1: i32,
    pub f2: i16,
}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
fn main() {
    let f = updated_crate::Example { f1: 123, f2: 456 };
}
```

**Küçük: Bir enum’a `repr(<int>)` ekleme**
Varsayılan gösterime sahip bir enum’a ilkel gösterim `repr(<int>)` eklemek güvenlidir. Bu güvenlidir; çünkü varsayılan gösterime sahip bir enum’un hizalaması, yerleşimi veya boyutu hakkında varsayım yapılmamalıdır.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub enum E {
    Variant1,
    Variant2(i32),
    Variant3 { f1: f64 },
}

///////////////////////////////////////////////////////////
// After
#[repr(i32)] // added
pub enum E {
    Variant1,
    Variant2(i32),
    Variant3 { f1: f64 },
}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
fn main() {
    let x = updated_crate::E::Variant3 { f1: 1.23 };
}
```

**Küçük: Varsayılan gösterimli bir struct veya enum’a `repr(transparent)` ekleme**
Varsayılan gösterime sahip bir struct veya enum’a `repr(transparent)` eklemek güvenlidir. Bu güvenlidir; çünkü varsayılan gösterime sahip bir struct veya enum’un hizalaması, yerleşimi veya boyutu hakkında varsayım yapılmamalıdır.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[derive(Default)]
pub struct Example<T>(T);

///////////////////////////////////////////////////////////
// After
#[derive(Default)]
#[repr(transparent)] // added
pub struct Example<T>(T);

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
fn main() {
    let x = updated_crate::Example::<i32>::default();
}
```
## ⚠️ Büyük Değişiklikler: `repr(packed)` ve `repr(align)` Kullanımı (Major Changes: Adding/Removing `repr(packed)` and `repr(align)`)

### 📦 Büyük: Bir `struct` veya `union`’a `repr(packed)` ekleme (Adding `repr(packed)` to a struct or union)

Bir `struct` veya `union`’a `repr(packed)` eklemek kırıcı bir değişikliktir. Bir türü `repr(packed)` yapmak, bir alana referans almanın geçersiz olması veya ayrı (disjoint) closure yakalamalarında kesilme (truncation) gibi kodu bozabilecek değişikliklere yol açar.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Example {
    pub f1: u8,
    pub f2: u16,
}

///////////////////////////////////////////////////////////
// After
#[repr(packed)] // added
pub struct Example {
    pub f1: u8,
    pub f2: u16,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    let f = updated_crate::Example { f1: 1, f2: 2 };
    let x = &f.f2; // Error: reference to packed field is unaligned
}
```

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Example(pub i32, pub i32);

///////////////////////////////////////////////////////////
// After
#[repr(packed)]
pub struct Example(pub i32, pub i32);

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    let mut f = updated_crate::Example(123, 456);
    let c = || {
        // Without repr(packed), the closure precisely captures `&f.0`.
        // With repr(packed), the closure captures `&f` to avoid undefined behavior.
        let a = f.0;
    };
    f.1 = 789; // Error: cannot assign to `f.1` because it is borrowed
    c();
}
```

---

### 📏 Büyük: Bir `struct`, `union` veya `enum`’a `repr(align)` ekleme (Adding `repr(align)` to a struct, union, or enum)

Bir `struct`, `union` veya `enum`’a `repr(align)` eklemek kırıcı bir değişikliktir. Bir türü `repr(align)` yapmak, bu türün `repr(packed)` içinde kullanılmasını bozacaktır; çünkü bu kombinasyona izin verilmez.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Aligned {
    pub a: i32,
}

///////////////////////////////////////////////////////////
// After
#[repr(align(8))] // added
pub struct Aligned {
    pub a: i32,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Aligned;

#[repr(packed)]
pub struct Packed { // Error: packed type cannot transitively contain a `#[repr(align)]` type
    f1: Aligned,
}

fn main() {
    let p = Packed {
        f1: Aligned { a: 123 },
    };
}
```

---

### ❌ Büyük: Bir `struct` veya `union`’dan `repr(packed)` kaldırma (Removing `repr(packed)` from a struct or union)

Bir `struct` veya `union`’dan `repr(packed)` kaldırmak kırıcı bir değişikliktir. Bu, dış crate’lerin (external crates) güvenebileceği hizalamayı veya yerleşimi değiştirebilir.

Alanlardan herhangi biri genel ise, `repr(packed)`’in kaldırılması ayrık (disjoint) closure yakalamalarının çalışma şeklini değiştirebilir. Bu, sürüm kılavuzunda (edition guide) açıklananlara benzer şekilde kodu bozabilir.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(C, packed)]
pub struct Packed {
    pub a: u8,
    pub b: u16,
}

///////////////////////////////////////////////////////////
// After
#[repr(C)] // removed packed
pub struct Packed {
    pub a: u8,
    pub b: u16,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Packed;

fn main() {
    let p = Packed { a: 1, b: 2 };
    // Some assumption about the size of the type.
    // Without `packed`, this fails since the size is 4.
    const _: () = assert!(std::mem::size_of::<Packed>() == 3); // Error: evaluation of constant value failed
}
```

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(C, packed)]
pub struct Packed {
    pub a: *mut i32,
    pub b: i32,
}
unsafe impl Send for Packed {}

///////////////////////////////////////////////////////////
// After
#[repr(C)] // removed packed
pub struct Packed {
    pub a: *mut i32,
    pub b: i32,
}
unsafe impl Send for Packed {}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Packed;

fn main() {
    let mut x = 123;

    let p = Packed {
        a: &mut x as *mut i32,
        b: 456,
    };

    // When the structure was packed, the closure captures `p` which is Send.
    // When `packed` is removed, this ends up capturing `p.a` which is not Send.
    std::thread::spawn(move || unsafe {
        *(p.a) += 1; // Error: cannot be sent between threads safely
    });
}
```

---

### 🔄 Büyük: `repr(packed(N))` değerini değiştirme (Changing the value N of `repr(packed(N)`)

`repr(packed(N))` değerini hizalamayı veya yerleşimi değiştirecek şekilde değiştirmek kırıcı bir değişikliktir. Bu, dış crate’lerin güvenebileceği hizalamayı veya yerleşimi değiştirebilir.

Eğer `N` değeri, genel bir alanın hizalamasının altına düşürülürse, o alana referans almaya çalışan herhangi bir kod kırılır.

Not: Bazı `N` değişiklikleri hizalamayı veya yerleşimi değiştirmeyebilir. Örneğin, mevcut değer zaten türün doğal hizalamasına eşitse, `N`’in artırılması değişiklik yapmaz.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(packed(4))]
pub struct Packed {
    pub a: u8,
    pub b: u32,
}

///////////////////////////////////////////////////////////
// After
#[repr(packed(2))] // changed to 2
pub struct Packed {
    pub a: u8,
    pub b: u32,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Packed;

fn main() {
    let p = Packed { a: 1, b: 2 };
    let x = &p.b; // Error: reference to packed field is unaligned
}
```

---

### 🔧 Büyük: `repr(align(N))` değerini değiştirme (Changing the value N of `repr(align(N))`)

`repr(align(N))` değerini hizalamayı değiştirecek şekilde değiştirmek kırıcı bir değişikliktir. Bu, dış crate’lerin güvenebileceği hizalamayı değiştirebilir.

Eğer tür, tür yerleşimi (type layout) bölümünde açıklandığı gibi iyi tanımlı değilse (örneğin, herhangi bir özel alanı varsa ve hizalaması/yerleşimi belgelenmemişse), bu değişiklik güvenli olabilir.

Not: Bazı `N` değişiklikleri hizalamayı veya yerleşimi değiştirmeyebilir. Örneğin, mevcut değer zaten türün doğal hizalamasına eşitse veya daha düşükse `N`’in azaltılması değişiklik yapmaz.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(align(8))]
pub struct Packed {
    pub a: u8,
    pub b: u32,
}

///////////////////////////////////////////////////////////
// After
#[repr(align(4))] // changed to 4
pub struct Packed {
    pub a: u8,
    pub b: u32,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Packed;

fn main() {
    let p = Packed { a: 1, b: 2 };
    // Some assumption about the size of the type.
    // The alignment has changed from 8 to 4.
    const _: () = assert!(std::mem::align_of::<Packed>() == 8); // Error: evaluation of constant value failed
}
```
## ⚠️ Büyük ve Küçük Değişiklikler: `repr`, Struct, Enum ve Trait (Major and Minor Changes: `repr`, Structs, Enums, and Traits)

### 🔧 Büyük: Bir `struct`, `union` veya `enum`’dan `repr(align)` kaldırma (Removing `repr(align)`)

Eğer düzeni (layout) iyi tanımlanmışsa (`well-defined`), bir `struct`, `union` veya `enum`’dan `repr(align)` kaldırmak kırıcı bir değişikliktir. Bu, dış crate’lerin güvenmekte olduğu hizalamayı veya düzeni değiştirebilir.

Eğer tür, tür düzeni (`type layout`) bölümünde anlatıldığı gibi iyi tanımlı değilse (örneğin özel alanlara sahipse ve hizalama belgelenmemişse), bu değişiklik güvenli olabilir.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(C, align(8))]
pub struct Packed {
    pub a: u8,
    pub b: u32,
}

///////////////////////////////////////////////////////////
// After
#[repr(C)] // removed align
pub struct Packed {
    pub a: u8,
    pub b: u32,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Packed;

fn main() {
    let p = Packed { a: 1, b: 2 };
    // Some assumption about the size of the type.
    // The alignment has changed from 8 to 4.
    const _: () = assert!(std::mem::align_of::<Packed>() == 8); // Error
}
```

---

### 🔄 Büyük: `repr(C)` türünde genel alanların sırasını değiştirme (Changing the order of public fields of a `repr(C)` type)

Bir `repr(C)` türündeki genel alanların sırasını değiştirmek kırıcı bir değişikliktir. Dış crate’ler alanların belirli bir sırasına güveniyor olabilir.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(C)]
pub struct SpecificLayout {
    pub a: u8,
    pub b: u32,
}

///////////////////////////////////////////////////////////
// After
#[repr(C)]
pub struct SpecificLayout {
    pub b: u32, // changed order
    pub a: u8,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::SpecificLayout;

unsafe extern "C" {
    fn c_fn_get_b(x: &SpecificLayout) -> u32;
}

fn main() {
    let p = SpecificLayout { a: 1, b: 2 };
    unsafe { assert_eq!(c_fn_get_b(&p), 2) } // Error
}
```

---

### ❌ Büyük: Bir `struct`, `union` veya `enum`’dan `repr(C)` kaldırma (Removing `repr(C)`)

Bir `struct`, `union` veya `enum`’dan `repr(C)` kaldırmak kırıcı bir değişikliktir. Dış crate’ler bu türün belirli düzenine güveniyor olabilir.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(C)]
pub struct SpecificLayout {
    pub a: u8,
    pub b: u32,
}

///////////////////////////////////////////////////////////
// After
// removed repr(C)
pub struct SpecificLayout {
    pub a: u8,
    pub b: u32,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::SpecificLayout;

unsafe extern "C" {
    fn c_fn_get_b(x: &SpecificLayout) -> u32; // Error: is not FFI-safe
}

fn main() {
    let p = SpecificLayout { a: 1, b: 2 };
    unsafe { assert_eq!(c_fn_get_b(&p), 2) }
}
```

---

### 🔢 Büyük: Bir enum’dan `repr(<int>)` kaldırma (Removing `repr(<int>)` from an enum)

Bir enum’dan `repr(<int>)` kaldırmak kırıcı bir değişikliktir. Dış crate’ler, ayrımcının (discriminant) belirli bir boyutta olmasına güveniyor olabilir.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(u16)]
pub enum Example {
    Variant1,
    Variant2,
    Variant3,
}

///////////////////////////////////////////////////////////
// After
// removed repr(u16)
pub enum Example {
    Variant1,
    Variant2,
    Variant3,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    let e = updated_crate::Example::Variant2;
    let i: u16 = unsafe { std::mem::transmute(e) }; // Error
}
```

---

### 🔄 Büyük: Bir `repr(<int>)` enum’un ilkel gösterimini değiştirme (Changing the primitive representation of a `repr(<int>)` enum)

Bir `repr(<int>)` enum’un ilkel gösterimini değiştirmek kırıcı bir değişikliktir. Dış crate’ler, ayrımcının (discriminant) belirli bir boyutta olmasına güveniyor olabilir.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(u16)]
pub enum Example {
    Variant1,
    Variant2,
    Variant3,
}

///////////////////////////////////////////////////////////
// After
#[repr(u8)] // changed repr size
pub enum Example {
    Variant1,
    Variant2,
    Variant3,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    let e = updated_crate::Example::Variant2;
    let i: u16 = unsafe { std::mem::transmute(e) }; // Error
}
```

---

### 🪟 Büyük: Bir `struct` veya `enum`’dan `repr(transparent)` kaldırma (Removing `repr(transparent)`)

Bir `struct` veya `enum`’dan `repr(transparent)` kaldırmak kırıcı bir değişikliktir. Dış crate’ler türün şeffaf alanın hizalamasına, düzenine veya boyutuna sahip olmasına güveniyor olabilir.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[repr(transparent)]
pub struct Transparent<T>(T);

///////////////////////////////////////////////////////////
// After
// removed repr
pub struct Transparent<T>(T);

///////////////////////////////////////////////////////////
// Example usage that will break.
#![deny(improper_ctypes)]
use updated_crate::Transparent;

unsafe extern "C" {
    fn c_fn() -> Transparent<f64>; // Error: is not FFI-safe
}

fn main() {}
```

---

### 🏗️ Büyük: Tüm alanlar genel iken özel alan ekleme (Adding a private struct field when all current fields are public)

Tüm alanları genel (`public`) olan bir yapıya özel (`private`) alan eklemek kırıcı bir değişikliktir. Bu, struct literal ile oluşturmayı bozacaktır.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Foo {
    pub f1: i32,
}

///////////////////////////////////////////////////////////
// After
pub struct Foo {
    pub f1: i32,
    f2: i32,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    let x = updated_crate::Foo { f1: 123 }; // Error
}
```

Azaltma stratejileri:

* Tüm alanları genel olan yapılara yeni alan eklemeyin.
* Yapılar ilk tanıtıldığında `#[non_exhaustive]` ile işaretleyin ve kullanıcıları yapıcı (constructor) yöntemlere veya `Default` uygulamasına yönlendirin.

---

### 🏗️ Büyük: Özel alan yokken genel alan ekleme (Adding a public field when no private field exists)

Tüm alanları genel (`public`) olan bir yapıya yeni bir genel alan eklemek kırıcı bir değişikliktir. Bu, struct literal ile oluşturmayı bozacaktır.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Foo {
    pub f1: i32,
}

///////////////////////////////////////////////////////////
// After
pub struct Foo {
    pub f1: i32,
    pub f2: i32,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    let x = updated_crate::Foo { f1: 123 }; // Error
}
```

Azaltma stratejileri:

* Tüm alanları genel olan yapılara yeni alan eklemeyin.
* Yapılar ilk tanıtıldığında `#[non_exhaustive]` ile işaretleyin ve kullanıcıları yapıcı (constructor) yöntemlere veya `Default` uygulamasına yönlendirin.

---

### 🔑 Küçük: Zaten en az bir özel alan varsa özel alan ekleme veya kaldırma (Adding or removing private fields when at least one already exists)

Bir yapıda zaten en az bir özel alan varsa, özel alan eklemek veya kaldırmak güvenlidir.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[derive(Default)]
pub struct Foo {
    f1: i32,
}

///////////////////////////////////////////////////////////
// After
#[derive(Default)]
pub struct Foo {
    f2: f64,
}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
fn main() {
    let x = updated_crate::Foo::default();
}
```

Not: Tuple struct’lar için, tuple’da genel alanlar varsa ve özel alan ekleme veya kaldırma genel alanın indeksini değiştiriyorsa bu büyük değişikliktir.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[derive(Default)]
pub struct Foo(pub i32, i32);

///////////////////////////////////////////////////////////
// After
#[derive(Default)]
pub struct Foo(f64, pub i32, i32);

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    let x = updated_crate::Foo::default();
    let y = x.0; // Error
}
```

---

### 🔄 Küçük: Tüm özel alanlı tuple struct’tan normal struct’a (veya tam tersi) geçiş (Going from a tuple struct with all private fields to a normal struct, or vice versa)

Tüm alanları özel olan bir tuple struct’ı normal struct’a (veya tam tersi) dönüştürmek güvenlidir.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[derive(Default)]
pub struct Foo(i32);

///////////////////////////////////////////////////////////
// After
#[derive(Default)]
pub struct Foo {
    f1: i32,
}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
fn main() {
    let x = updated_crate::Foo::default();
}
```

---

### 🧩 Büyük: Yeni enum varyantları ekleme (non\_exhaustive olmadan) (Adding new enum variants without `non_exhaustive`)

Bir enum’a yeni varyant eklemek kırıcı bir değişikliktir, eğer enum `#[non_exhaustive]` ile işaretlenmemişse.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub enum E {
    Variant1,
}

///////////////////////////////////////////////////////////
// After
pub enum E {
    Variant1,
    Variant2,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    use updated_crate::E;
    let x = E::Variant1;
    match x { // Error
        E::Variant1 => {}
    }
}
```

Azaltma stratejisi: Enum ilk tanıtıldığında `#[non_exhaustive]` ile işaretleyin.

---

### 🧩 Büyük: Enum varyantına yeni alan ekleme (Adding new fields to an enum variant)

Bir enum varyantına yeni alan eklemek kırıcı bir değişikliktir. Tüm alanlar genel olduğundan, oluşturma (constructor) ve eşleme (match) başarısız olur.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub enum E {
    Variant1 { f1: i32 },
}

///////////////////////////////////////////////////////////
// After
pub enum E {
    Variant1 { f1: i32, f2: i32 },
}

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    use updated_crate::E;
    let x = E::Variant1 { f1: 1 }; // Error
    match x {
        E::Variant1 { f1 } => {} // Error
    }
}
```

Azaltma stratejileri:

* Enum varyantını `#[non_exhaustive]` ile işaretleyin.
* Alan görünürlüğünü kontrol edebileceğiniz ayrı bir struct kullanın.

---

### 🔗 Büyük: Varsayılanı olmayan (non-defaulted) bir trait öğesi ekleme (Adding a non-defaulted trait item)

Bir trait’e varsayılanı olmayan bir öğe eklemek kırıcı bir değişikliktir. Bu, trait’i uygulayan tüm türleri bozacaktır.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub trait Trait {}

///////////////////////////////////////////////////////////
// After
pub trait Trait {
    fn foo(&self);
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Trait;
struct Foo;

impl Trait for Foo {}  // Error: not all trait items implemented
```

Azaltma stratejileri:

* Yeni ilişkili trait öğeleri için daima varsayılan bir uygulama sağlayın.
* Trait ilk tanıtıldığında, harici kullanıcıların trait’i uygulamasını engellemek için sealed trait tekniğini kullanın.

## ⚠️ Büyük ve Küçük Değişiklikler: Trait, Generics ve Fonksiyonlar (Major and Minor Changes: Traits, Generics, and Functions)

### 🔗 Büyük: Trait öğesi imzasında herhangi bir değişiklik (Any change to trait item signatures)

Bir trait öğesi imzasında herhangi bir değişiklik yapmak kırıcıdır. Bu, trait’in harici implementasyonlarını bozabilir.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub trait Trait {
    fn f(&self, x: i32) {}
}

///////////////////////////////////////////////////////////
// After
pub trait Trait {
    fn f<V>(&self, x: V) {}
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Trait;
struct Foo;

impl Trait for Foo {
    fn f(&self, x: i32) {}  // Error
}
```

Azaltma stratejileri:

* Mevcut öğeleri değiştirmek yerine, yeni işlevselliği kapsayan varsayılan implementasyonlu yeni öğeler tanıtın.
* Trait’i tanıtırken `sealed trait` tekniğini kullanarak harici kullanıcıların implementasyon yapmasını engelleyin.

---

### ⚖️ Olası kırıcı: Varsayılanlı bir trait öğesi ekleme (Adding a defaulted trait item)

Varsayılanlı bir trait öğesi eklemek genellikle güvenlidir. Ancak bazı durumlarda derleme hatası oluşturabilir, örneğin başka bir trait ile ad çakışması olduğunda.

```rust
// Breaking change example

///////////////////////////////////////////////////////////
// Before
pub trait Trait {}

///////////////////////////////////////////////////////////
// After
pub trait Trait {
    fn foo(&self) {}
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Trait;
struct Foo;

trait LocalTrait {
    fn foo(&self) {}
}

impl Trait for Foo {}
impl LocalTrait for Foo {}

fn main() {
    let x = Foo;
    x.foo(); // Error
}
```

Not: Bu belirsizlik, gömülü (inherent) implementasyonlardaki ad çakışmalarında oluşmaz.

Azaltma stratejileri:

* Çakışma ihtimalini en aza indirmek için yeni öğeler için dikkatli isimler seçin.
* Kullanıcılardan güncelleme sırasında ayrıştırma (disambiguation) sözdizimini kullanmaları gerekebilir.

---

### 🔗 Büyük: Trait’i nesne güvenli olmaktan çıkarmak (Adding a trait item that makes the trait non-object safe)

Bir trait’i nesne güvenli (`object safe`) olmaktan çıkaran öğeler eklemek kırıcıdır.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub trait Trait {}

///////////////////////////////////////////////////////////
// After
pub trait Trait {
    const CONST: i32 = 123;
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Trait;
struct Foo;

impl Trait for Foo {}

fn main() {
    let obj: Box<dyn Trait> = Box::new(Foo); // Error
}
```

Tersi durum (nesne güvenli olmayan bir trait’i güvenli yapmak) güvenlidir.

---

### 🔗 Büyük: Varsayılanı olmayan bir tür parametresi ekleme (Adding a type parameter without a default)

Bir trait’e varsayılanı olmayan bir tür parametresi eklemek kırıcıdır.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub trait Trait {}

///////////////////////////////////////////////////////////
// After
pub trait Trait<T> {}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Trait;
struct Foo;

impl Trait for Foo {}  // Error
```

Azaltma stratejisi: Varsayılanlı tür parametresi eklemeyi tercih edin.

---

### 🟢 Küçük: Varsayılanlı bir tür parametresi ekleme (Adding a defaulted trait type parameter)

Varsayılanı olan bir tür parametresi eklemek güvenlidir.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub trait Trait {}

///////////////////////////////////////////////////////////
// After
pub trait Trait<T = i32> {}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
use updated_crate::Trait;
struct Foo;

impl Trait for Foo {}
```

---

### ⚖️ Olası kırıcı: Gömülü öğeler ekleme (Adding any inherent items)

Gömülü (`inherent`) öğeler eklemek genellikle güvenlidir çünkü gömülü öğeler trait öğelerinden önceliklidir. Ancak aynı isimde farklı imza olduğunda çakışma olabilir.

```rust
// Breaking change example

///////////////////////////////////////////////////////////
// Before
pub struct Foo;

///////////////////////////////////////////////////////////
// After
pub struct Foo;

impl Foo {
    pub fn foo(&self) {}
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Foo;

trait Trait {
    fn foo(&self, x: i32) {}
}

impl Trait for Foo {}

fn main() {
    let x = Foo;
    x.foo(1); // Error
}
```

Azaltma stratejileri:

* Yeni öğe adlarını çakışma olasılığı düşük şekilde seçin.
* Kullanıcılardan gerekli olduğunda ayrıştırma (disambiguation) sözdizimini kullanmalarını isteyin.

---

### 🔒 Büyük: Generics sınırlarını sıkılaştırma (Tightening generic bounds)

Bir türün generic sınırlarını sıkılaştırmak kırıcıdır.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Foo<A> {
    pub f1: A,
}

///////////////////////////////////////////////////////////
// After
pub struct Foo<A: Eq> {
    pub f1: A,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Foo;

fn main() {
    let s = Foo { f1: 1.23 }; // Error
}
```

---

### 🟢 Küçük: Generics sınırlarını gevşetme (Loosening generic bounds)

Generic sınırlarını gevşetmek güvenlidir.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Foo<A: Clone> {
    pub f1: A,
}

///////////////////////////////////////////////////////////
// After
pub struct Foo<A> {
    pub f1: A,
}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
use updated_crate::Foo;

fn main() {
    let s = Foo { f1: 123 };
}
```

---

### 🟢 Küçük: Varsayılanlı tür parametreleri ekleme (Adding defaulted type parameters)

Varsayılanlı parametreler eklemek güvenlidir.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
#[derive(Default)]
pub struct Foo {}

///////////////////////////////////////////////////////////
// After
#[derive(Default)]
pub struct Foo<A = i32> {
    f1: A,
}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
use updated_crate::Foo;

fn main() {
    let s: Foo = Default::default();
}
```

---

### 🟢 Küçük: Bir türü generics ile genelleştirme (özdeş tiplerle) (Generalizing a type to use generics with identical types)

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Foo(pub u8);

///////////////////////////////////////////////////////////
// After
pub struct Foo<T = u8>(pub T);

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
use updated_crate::Foo;

fn main() {
    let s: Foo = Foo(123);
}
```

---

### 🔴 Büyük: Bir türü generics ile genelleştirme (farklı tiplerle) (Generalizing a type with possibly different types)

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Foo<T = u8>(pub T, pub u8);

///////////////////////////////////////////////////////////
// After
pub struct Foo<T = u8>(pub T, pub T);

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::Foo;

fn main() {
    let s: Foo<f32> = Foo(3.14, 123); // Error
}
```

---

### 🟢 Küçük: Daha genel bir türe dönüştürme (Changing a generic type to a more generic type)

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Foo<T>(pub T, pub T);

///////////////////////////////////////////////////////////
// After
pub struct Foo<T, U = T>(pub T, pub U);

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
use updated_crate::Foo;

fn main() {
    let s: Foo<f32> = Foo(1.0, 2.0);
}
```

---

### 🔴 Büyük: RPIT’te (Return-Position Impl Trait) daha fazla generic parametre yakalama (Capturing more generic parameters in RPIT)

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub fn f<'a, 'b>(x: &'a str, y: &'b str) -> impl Iterator<Item = char> + use<'a> {
    x.chars()
}

///////////////////////////////////////////////////////////
// After
pub fn f<'a, 'b>(x: &'a str, y: &'b str) -> impl Iterator<Item = char> + use<'a, 'b> {
    x.chars().chain(y.chars())
}

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    let a = String::new();
    let b = String::new();
    let iter = updated_crate::f(&a, &b);
    drop(b); // Error
}
```

Not: Daha az generic parametre yakalamak küçük değişikliktir.

---

### 🔴 Büyük: Fonksiyon parametreleri ekleme/kaldırma (Adding/removing function parameters)

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub fn foo() {}

///////////////////////////////////////////////////////////
// After
pub fn foo(x: i32) {}

///////////////////////////////////////////////////////////
// Example usage that will break.
fn main() {
    updated_crate::foo(); // Error
}
```

Azaltma stratejileri:

* Yeni imzayla yeni bir fonksiyon tanıtın, eskisini `deprecated` yapın.
* Parametreleri struct olarak kabul eden fonksiyonlar tanıtın; böylece ileride yeni alanlar eklenebilir.

---

### ⚖️ Olası kırıcı: Yeni bir fonksiyon tür parametresi ekleme (Introducing a new function type parameter)

```rust
// Breaking change example

///////////////////////////////////////////////////////////
// Before
pub fn foo<T>() {}

///////////////////////////////////////////////////////////
// After
pub fn foo<T, U>() {}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::foo;

fn main() {
    foo::<u8>(); // Error
}
```

---

### 🟢 Küçük: Fonksiyonu generics ile genelleştirme (özgün türü destekleyerek) (Generalizing a function to use generics, supporting original type)

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub fn foo(x: u8) -> u8 {
    x
}
pub fn bar<T: Iterator<Item = u8>>(t: T) {}

///////////////////////////////////////////////////////////
// After
use std::ops::Add;
pub fn foo<T: Add>(x: T) -> T {
    x
}
pub fn bar<T: IntoIterator<Item = u8>>(t: T) {}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
use updated_crate::{bar, foo};

fn main() {
    foo(1);
    bar(vec![1, 2, 3].into_iter());
}
```

---

### 🔴 Büyük: Fonksiyonu generics ile genelleştirme (tip uyumsuzluğu ile) (Generalizing a function with type mismatch)

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub fn foo(x: Vec<u8>) {}

///////////////////////////////////////////////////////////
// After
pub fn foo<T: Copy + IntoIterator<Item = u8>>(x: T) {}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::foo;

fn main() {
    foo(vec![1, 2, 3]); // Error
}
```
## ⚖️ Küçük ve Büyük Değişiklikler: `unsafe`, `no_std`, `non_exhaustive`, Araçlar ve Cargo (Minor and Major Changes: `unsafe`, `no_std`, `non_exhaustive`, Tooling and Cargo)

### 🟢 Küçük: `unsafe` bir fonksiyonu güvenli yapmak (Making an unsafe function safe)

Daha önce `unsafe` olan bir fonksiyonun güvenli hale getirilmesi kırıcı değildir.
Ancak bu, `unused_unsafe` lint’ini tetikleyebilir ve `#![deny(warnings)]` kullanan projelerin derlenmemesine yol açabilir.

Tersi (güvenli fonksiyonu `unsafe` yapmak) kırıcı değişikliktir.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub unsafe fn foo() {}

///////////////////////////////////////////////////////////
// After
pub fn foo() {}

///////////////////////////////////////////////////////////
// Example use of the library that will trigger a lint.
use updated_crate::foo;

unsafe fn bar(f: unsafe fn()) {
    f()
}

fn main() {
    unsafe { foo() }; // Warning: unused_unsafe
    unsafe { bar(foo) };
}
```

Not: Bu, trait üzerindeki ilişkili fonksiyonlar için geçerli değildir (bkz. trait item signatures).

---

### 🔴 Büyük: `no_std` desteğinden `std` gerektirmeye geçmek (Switching from `no_std` to requiring `std`)

`no_std` ortamını destekleyen bir kütüphaneyi `std` gerektiren hale getirmek kırıcı bir değişikliktir.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
#![no_std]
pub fn foo() {}

///////////////////////////////////////////////////////////
// After
pub fn foo() {
    std::time::SystemTime::now();
}

///////////////////////////////////////////////////////////
// Example usage that will break.
#![no_std]
use updated_crate::foo;

fn example() {
    foo(); // Error: `std` yok
}
```

Azaltma stratejisi:

* `std` desteğini isteğe bağlı bir Cargo özelliği (`feature`) yapın.

---

### 🔴 Büyük: Var olan öğelere `#[non_exhaustive]` ekleme (Adding `non_exhaustive`)

Daha önce özel alanı olmayan yapılara (`struct`, `enum`, `variant`) `#[non_exhaustive]` eklemek kırıcıdır.

Bu değişiklik:

* Struct literal kullanımıyla oluşturmayı engeller.
* Enum eşleşmelerinde `..` zorunlu kılar.
* `as` ile discriminant dönüşümünü engeller.

```rust
// MAJOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub struct Foo {
    pub bar: usize,
}

pub enum Bar {
    X,
    Y(usize),
    Z { a: usize },
}

pub enum Quux {
    Var,
}

///////////////////////////////////////////////////////////
// After
#[non_exhaustive]
pub struct Foo {
    pub bar: usize,
}

pub enum Bar {
    #[non_exhaustive]
    X,
    #[non_exhaustive]
    Y(usize),
    #[non_exhaustive]
    Z { a: usize },
}

#[non_exhaustive]
pub enum Quux {
    Var,
}

///////////////////////////////////////////////////////////
// Example usage that will break.
use updated_crate::{Bar, Foo, Quux};

fn main() {
    let foo = Foo { bar: 0 }; // Error
    let bar_x = Bar::X;       // Error
    let bar_y = Bar::Y(0);    // Error
    let bar_z = Bar::Z { a: 0 }; // Error

    let q = Quux::Var;
    match q {
        Quux::Var => 0,
        // Error: non-exhaustive patterns
    };
}
```

Azaltma stratejisi:

* Yapılar, enumlar ve varyantlar ilk tanıtıldığında `#[non_exhaustive]` ile işaretleyin.

---

### ⚖️ Olası kırıcı: Gerekli minimum Rust sürümünü değiştirme (Changing the minimum Rust version required)

Yeni bir Rust özelliği kullanmak eski sürümlerde projeleri bozabilir.

Önerilen: Bunu büyük değil, küçük değişiklik olarak ele alın. Rust hızlı sürüm döngüsüne sahiptir (6 hafta).

Azaltma stratejileri:

* Yeni özellikleri `Cargo feature` ile opsiyonel yapın.
* Eski sürümlere geniş destek penceresi sunun.
* Mümkünse yeni std öğelerinin kopyasını kullanın.
* Önemli hata düzeltmeleri için eski sürümler için ayrı dal (branch) tutun.

---

### ⚖️ Olası kırıcı: Platform ve ortam gereksinimlerini değiştirme (Changing the platform and environment requirements)

Yeni sürüm, önceden desteklenen bir ortamı artık desteklemiyorsa kırıcı olabilir (örneğin daha yeni bir işletim sistemi gerektirmek).

Azaltma stratejileri:

* Desteklediğiniz platformları belgelerinizde net belirtin.
* CI’da çok çeşitli ortamlarda test yapın.

---

### 🟢 Küçük: Yeni lint’ler eklemek (Introducing new lints)

Yeni uyarılar (`lints`) uyumluluk açısından güvenli kabul edilir. Ancak `#![deny(warnings)]` kullanan projelerde derleme hatası olabilir.

```rust
// MINOR CHANGE

///////////////////////////////////////////////////////////
// Before
pub fn foo() {}

///////////////////////////////////////////////////////////
// After
#[deprecated]
pub fn foo() {}

///////////////////////////////////////////////////////////
// Example use of the library that will safely work.
fn main() {
    updated_crate::foo(); // Warning: deprecated
}
```

Örnek yeni lint’ler:

* `deprecated`
* `unused_must_use`
* `unused_unsafe`

---

### 🟢 Küçük: Yeni Cargo özelliği ekleme (Adding a new Cargo feature)

Genellikle güvenlidir. Ancak varsayılan (`default`) listesine eklenirse kırıcı olabilir.

```toml
# MINOR CHANGE

########################################################### 
# Before
[features]
# ..empty

########################################################### 
# After
[features]
std = []
```

---

### 🔴 Büyük: Cargo özelliğini kaldırma (Removing a Cargo feature)

Bir Cargo özelliğini kaldırmak kırıcıdır. Özelliği etkinleştiren projeler hata alacaktır.

```toml
# MAJOR CHANGE

########################################################### 
# Before
[features]
logging = []

########################################################### 
# After
[dependencies]
# logging removed
```

Azaltma stratejileri:

* Özellikleri belgelerken net olun.
* Özelliği `deprecated` yapın, gelecekteki major sürümde kaldırın.

---

### 🔴 Büyük: Bir özelliği başka bir özellik listesinden kaldırma (Removing a feature from a feature list)

Bu, beklenen işlevselliği bozabilir.

```toml
# Breaking change example

########################################################### 
# Before
[features]
default = ["std"]
std = []

########################################################### 
# After
[features]
default = [] # Error: bazı paketler `std` varsayıyordu
std = []
```

---

### ⚖️ Olası kırıcı: İsteğe bağlı bağımlılığı kaldırma (Removing an optional dependency)

İsteğe bağlı bağımlılık kaldırıldığında, başka projeler `Cargo feature` aracılığıyla bu bağımlılığı etkinleştirmiş olabilir.

Azaltma stratejileri:

* `dep:` sözdizimini kullanın.
* Yüksek seviyeli özellik adları (örn. `networking`) kullanın.

```toml
# Breaking change example

########################################################### 
# Before
[dependencies]
curl = { version = "0.4.31", optional = true }

########################################################### 
# After
# ..curl removed
```

Güvenli alternatif:

```toml
# MINOR CHANGE

########################################################### 
# Before
[dependencies]
curl = { version = "0.4.31", optional = true }

[features]
networking = ["dep:curl"]

########################################################### 
# After
[dependencies]
hyper = { version = "0.14.27", optional = true }

[features]
networking = ["dep:hyper"]
```

---

### 🟢 Küçük: Bağımlılık özelliklerini değiştirme (Changing dependency features)

Bağımlılıktaki özellikleri değiştirmek genellikle güvenlidir.

```toml
# MINOR CHANGE

########################################################### 
# Before
[dependencies]
rand = { version = "0.7.3", features = ["small_rng"] }

########################################################### 
# After
[dependencies]
rand = "0.7.3"
```

---

### 🟢 Küçük: Bağımlılık ekleme (Adding dependencies)

Genellikle güvenlidir. Ancak yeni bağımlılık nightly gerektiriyorsa, bu kırıcı olur.

```toml
# MINOR CHANGE

########################################################### 
# Before
[dependencies]
# ..empty

########################################################### 
# After
[dependencies]
log = "0.4.11"
```

---

### 📦 Uygulama uyumluluğu (Application compatibility)

Cargo projeleri çalıştırılabilir ikili dosyalar (CLI, OS etkileşimi vb.) da içerebilir. Bunlar paket sürümüyle aynı sürümü paylaşır.

Burada SemVer kontratını nasıl uygulayacağınıza siz karar verirsiniz.

👉 Uygulama değişiklikleri için SemVer ruhunu takip edin ve kullanıcılarınıza taahhütlerinizi belgelendirin.
