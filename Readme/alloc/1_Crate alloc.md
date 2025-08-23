## 📦 Crate alloc (crate alloc)

Öğe yolunu kopyala (Copy item path) · Ayarlar (Settings) · Yardım (Help)

---

## 🧾 Özet (Summary)

**1.36.0** · Kaynak (Source)

Rust çekirdek bellek ayırma ve koleksiyon kütüphanesi (core allocation and collections library).
Bu kütüphane, yığında ayrılmış (heap-allocated) değerleri yönetmek için akıllı işaretçiler (smart pointers) ve koleksiyonlar sağlar.

Bu kütüphane, **core** gibi, normalde doğrudan kullanılmaz; çünkü içerikleri **std** crate’i (std crate) içinde yeniden dışa aktarılır (re-exported). Ancak `#![no_std]` özniteliğini (attribute) kullanan crate’ler genellikle **std**’a bağımlı olmaz; bu nedenle bunun yerine bu crate’i kullanırlar.

---

## 🗃️ Kutulanmış değerler (Boxed values)

`Box` türü bir akıllı işaretçi türüdür (smart pointer type). Bir `Box`’ın yalnızca bir sahibi (owner) olabilir ve sahibi, yığında (heap) yaşayan içeriği değiştirmeye karar verebilir.

Bu tür, bir `Box` değerinin boyutu bir işaretçiyle aynı olduğundan iş parçacıkları (threads) arasında verimli biçimde gönderilebilir (sendable). Ağaç benzeri veri yapıları (tree-like data structures), genellikle her düğümün yalnızca bir sahibi —ebeveyn— olduğundan kutularla (boxes) inşa edilir.

---

## 🔗 Referans sayımlı işaretçiler (reference counted pointers)

`Rc` türü, bir iş parçacığı için güvenli olmayan (non-threadsafe) referans sayımlı işaretçi (reference-counted pointer) türüdür ve bir iş parçacığı içinde belleği paylaşmaya yöneliktir. Bir `Rc` işaretçisi bir `T` türünü sarmalar ve yalnızca `&T`, yani paylaşılan başvuruya (shared reference) erişime izin verir.

Bu tür, kalıtsal değiştirilebilirliğin (inherited mutability) —örneğin `Box` kullanımı— bir uygulama için fazla kısıtlayıcı olduğu durumlarda kullanışlıdır ve sıklıkla değişikliğe izin vermek için `Cell` veya `RefCell` türleriyle birlikte kullanılır.

---

## 🧵 Atomik referans sayımlı işaretçiler (atomically reference counted pointers)

`Arc` türü, `Rc` türünün iş parçacığı için güvenli (threadsafe) karşılığıdır. `Rc` ile aynı tüm işlevselliği sağlar; ancak içerilen tür `T`’nin paylaşılabilir (shareable) olmasını gerektirir. Ek olarak, `Arc<T>`’nin kendisi gönderilebilir (sendable) iken `Rc<T>` değildir.

Bu tür, içerilen veriye paylaşımlı erişime izin verir ve paylaşılan kaynakların değiştirilebilmesi için genellikle kilitler gibi eşzamanlılık ilkel yapılarıyla (synchronization primitives) —örneğin `mutex`ler (mutexes)— birlikte kullanılır.

---

## 🗂️ Koleksiyonlar (Collections)

En yaygın genel amaçlı veri yapılarının (general purpose data structures) gerçekleştirimleri bu kütüphanede tanımlanır. Bunlar standart koleksiyonlar kütüphanesi (standard collections library) aracılığıyla yeniden dışa aktarılır.

---

## 🧰 Yığın arayüzleri (heap interfaces)

`alloc` modülü, öntanımlı küresel ayırıcıya (default global allocator) yönelik düşük seviyeli arayüzü tanımlar. `libc` ayırıcı API’siyle (libc allocator API) uyumlu değildir.

---

## 🧩 Modüller (Modules)

alloc
Bellek ayırma API’leri (Memory allocation APIs).

borrow
Ödünç alınan verilerle (borrowed data) çalışmaya yönelik bir modül.

boxed
Yığın ayırma (heap allocation) için `Box<T>` türü (the `Box<T>` type for heap allocation).

collections
Koleksiyon türleri (Collection types).

ffi
FFI bağlamalarıyla (FFI bindings) ilgili yardımcılar.

fmt
`String`lerin biçimlendirilmesi ve yazdırılması (formatting and printing Strings) için yardımcılar.

rc
Tek iş parçacıklı (single-threaded) referans sayımlı işaretçiler (reference-counting pointers). ‘Rc’, ‘Reference Counted’ın (Reference Counted) kısaltmasıdır.

slice
`slice` ilkel türü (slice primitive type) için yardımcılar.

str
`str` ilkel türü (str primitive type) için yardımcılar.

string
UTF-8 ile kodlanmış (UTF-8–encoded), büyüyebilir (growable) bir `String`.

sync
İş parçacığı için güvenli (thread-safe) referans sayımlı işaretçiler.

task
Eşzamansız görevlerle (asynchronous tasks) çalışmaya yönelik türler ve özellikler (Types and Traits).

vec
`Vec<T>` olarak yazılan, yığın üzerinde ayrılmış içeriklere sahip bitişik (contiguous) büyüyebilir (growable) bir dizi türü.

bstr Deneysel (Experimental)
`ByteStr` ve `ByteString` türleri ile trait gerçekleştirimleri (trait implementations).

---

## 🔧 Makrolar (Macros)

format
Çalışma zamanı ifadelerinin ara yerleştirilmesiyle (interpolation of runtime expressions) bir `String` oluşturur.

vec
Bağımsız değişkenleri (arguments) içeren bir `Vec` oluşturur.
