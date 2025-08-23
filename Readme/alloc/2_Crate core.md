## 📦 Crate core (crate core)

Öğe yolunu kopyala (Copy item path) · Ayarlar (Settings) · Yardım (Help)

---

## 🧾 Özet (Summary)

**1.6.0** · Kaynak (Source)

Rust Çekirdek Kütüphanesi (Rust Core Library), Rust Standart Kütüphanesi’nin (Rust Standard Library) bağımlılıksız1 (dependency-free) temelidir. Dil ile kütüphaneleri arasında taşınabilir bağ (portable glue) işlevi görür; tüm Rust kodunun öz (intrinsic) ve ilkel (primitive) yapı taşlarını tanımlar. Herhangi bir üst (upstream) kütüphaneye, sistem kütüphanesine ya da `libc`’ye bağlanmaz.

Çekirdek kütüphane asgaridir (minimal): yığın ayırmanın (heap allocation) farkında değildir, eşzamanlılık (concurrency) veya G/Ç (I/O) sağlamaz. Bu konular platform entegrasyonu (platform integration) gerektirir ve bu kütüphane platformdan bağımsızdır (platform‑agnostic).

---

## 🛠️ Çekirdek kütüphanenin kullanımı (How to use the core library)

Lütfen şu ayrıntıların tümünün şu anda kararlı (stable) kabul edilmediğini unutmayın.

Bu kütüphane, bazı mevcut sembollerin var olduğu varsayımıyla inşa edilmiştir:

`memcpy`, `memmove`, `memset`, `memcmp`, `bcmp`, `strlen` - Bunlar, Rust kod üretim arka uçları (codegen backends) tarafından üretilen çekirdek bellek yordamlarıdır (core memory routines). Ayrıca, bu kütüphane `strlen`’e açık çağrılar yapabilir. İmzaları (signatures) C’de bulunanlarla aynıdır, ancak semantiklerine (semantics) ilişkin ek varsayımlar vardır: `memcpy`, `memmove`, `memset`, `memcmp` ve `bcmp` için `n` parametresi 0 ise, işaretçiler `NULL` veya sarkık (dangling) olsa bile işlevin UB (undefined behavior) üretmeyeceği varsayılır. (Derleyiciler arasında bu işlevler hakkında ek varsayımlar yapmak yaygındır: `clang` ve `GCC` de aynısını yapar.) Bu işlevler genellikle sistem `libc`’si tarafından sağlanır, ancak `compiler-builtins` crate’i tarafından da sağlanabilir. Kütüphanenin bu varsayımları her zaman yapacağını garanti etmediğine dikkat edin; bu nedenle C işlevlerini doğrudan çağıran Rust kullanıcı kodu C spesifikasyonuna (C specification) uymalıdır! Rust kullanıcı koduna tavsiye, bunun yerine bu kütüphanenin sağladığı işlevleri (ör. `ptr::copy`) çağırmaktır.

Panik işleyicisi (panic handler) - Bu işlev bir bağımsız değişken alır: `&panic::PanicInfo`. Bu çekirdek kütüphaneyi tüketenlerin bu panik işlevini tanımlaması gerekir; tek gereklilik hiçbir zaman dönmemesidir (never return). Gerçekleştiriminizi `#[panic_handler]` ile işaretlemelisiniz.

`rust_eh_personality` - Derleyicinin hata/başarısızlık mekanizmaları (failure mechanisms) tarafından kullanılır. Bu, sıklıkla GCC’nin “personality function”ına eşlenir; ancak panik tetiklemeyen crate’ler bu işlevin asla çağrılmayacağından emin olabilir. `lang` özniteliği (lang attribute) `eh_personality` olarak adlandırılır.

Katı konuşmak gerekirse, bazı semboller gereklidir, ancak her zaman gerekli olmazlar. ↩

---

## 🧱 İlkel Türler (Primitive Types)

array
Sabit boyutlu (fixed‑size) bir dizi; öğe türü `T` ve negatif olmayan derleme zamanı sabiti (compile‑time constant) `N` ile `[T; N]` olarak gösterilir.

bool
Boole türü (boolean type).

char
Karakter türü (character type).

f32
32 bit kayan noktalı tür (floating‑point type) (özellikle IEEE 754‑2008’de tanımlanan “binary32”).

f64
64 bit kayan noktalı tür (floating‑point type) (özellikle IEEE 754‑2008’de tanımlanan “binary64”).

fn
`fn(usize) -> bool` gibi işlev işaretçileri (function pointers).

i8
8 bit işaretli tamsayı türü.

i16
16 bit işaretli tamsayı türü.

i32
32 bit işaretli tamsayı türü.

i64
64 bit işaretli tamsayı türü.

i128
128 bit işaretli tamsayı türü.

isize
İşaretçi boyutunda (pointer‑sized) işaretli tamsayı türü.

pointer
Ham, güvensiz işaretçiler (raw, unsafe pointers): `*const T` ve `*mut T`.

reference
Başvurular (references): `&T` ve `&mut T`.

slice
Bitişik bir dizilim üzerinde dinamik boyutlu (dynamically‑sized) bir görünüm: `[T]`.

str
Dize dilimleri (string slices).

tuple
Sınırlı, heterojen (heterogeneous) bir dizilim: `(T, U, ..)`.

u8
8 bit işaretsiz tamsayı türü.

u16
16 bit işaretsiz tamsayı türü.

u32
32 bit işaretsiz tamsayı türü.

u64
64 bit işaretsiz tamsayı türü.

u128
128 bit işaretsiz tamsayı türü.

unit
`()` türü, “birim” (unit) olarak da adlandırılır.

usize
İşaretçi boyutunda (pointer‑sized) işaretsiz tamsayı türü.

f16 Deneysel (Experimental)
16 bit kayan noktalı tür (özellikle IEEE 754‑2008’de tanımlanan “binary16”).

f128 Deneysel (Experimental)
128 bit kayan noktalı tür (özellikle IEEE 754‑2008’de tanımlanan “binary128”).

never Deneysel (Experimental)
`!` türü, “asla” (never) olarak da adlandırılır.

---

## 🧩 Modüller (Modules)

alloc
Bellek ayırma API’leri (Memory allocation APIs).

any
Dinamik türleme (dynamic typing) veya tür yansıtma (type reflection) için yardımcılar.

arch
SIMD ve üreticiye özgü içyordamlar (vendor intrinsics) modülü.

array
`array` ilkel türü için yardımcılar.

ascii
ASCII dizeleri ve karakterleri üzerinde işlemler.

borrow
Ödünç alınan verilerle (borrowed data) çalışma yardımcıları.

cell
Paylaşılabilir değiştirilebilir kapsayıcılar (shareable mutable containers).

char
`char` ilkel türü için yardımcılar.

clone
Örtük olarak kopyalanamayan (implicitly copied) türler için `Clone` özelliği (Clone trait).

cmp
Değerleri karşılaştırma ve sıralama (ordering) yardımcıları.

convert
Türler arası dönüşümler (conversions) için özellikler (traits).

default
Öntanımlı değeri olan türler için `Default` özelliği (Default trait).

error
Hatalarla (Errors) çalışmaya yönelik arayüzler.

f32
`f32` tek duyarlıklı kayan nokta türü için sabitler (constants).

f64
`f64` çift duyarlıklı kayan nokta türü için sabitler (constants).

ffi
C tarafından tanımlanan platforma özgü türler (platform‑specific types).

fmt
Dizeleri biçimlendirme ve yazdırma (formatting and printing) yardımcıları.

future
Eşzamansız (asynchronous) temel işlevsellik.

hash
Genel amaçlı karma (hashing) desteği.

hint
Derleyiciye, kodun nasıl yayılacağı/iyileştirileceğine dair ipuçları (hints).

i8 Kullanımdan kaldırma planlanıyor (Deprecation planned)
`i8` ilkel türü için artık gereksiz (redundant) sabit modülü.

i16 Kullanımdan kaldırma planlanıyor (Deprecation planned)
`i16` ilkel türü için artık gereksiz sabit modülü.

i32 Kullanımdan kaldırma planlanıyor (Deprecation planned)
`i32` ilkel türü için artık gereksiz sabit modülü.

i64 Kullanımdan kaldırma planlanıyor (Deprecation planned)
`i64` ilkel türü için artık gereksiz sabit modülü.

i128 Kullanımdan kaldırma planlanıyor (Deprecation planned)
`i128` ilkel türü için artık gereksiz sabit modülü.

isize Kullanımdan kaldırma planlanıyor (Deprecation planned)
`isize` ilkel türü için artık gereksiz sabit modülü.

iter
Bileşebilir (composable) harici yineleme (external iteration).

marker
Türlerin temel özelliklerini temsil eden ilkel özellikler ve türler (primitive traits and types).

mem
Bellekle çalışmaya yönelik temel işlevler.

net
IP iletişimi için ağ ilkeleleri (networking primitives).

num
Yerleşik sayısal türler (built‑in numeric types) için sayısal özellikler ve işlevler.

ops
Aşırı yüklenebilir (overloadable) işleçler (operators).

option
İsteğe bağlı (optional) değerler.

panic
Standart kütüphanede panik (panic) desteği.

pin
Veriyi bellekte bir konuma sabitleyen (pin) türler.

prelude
Çekirdek önyükleme modülü (the core prelude).

primitive
Gölgeleme (shadowing) olasılığı olmadan kullanım için ilkel türleri yeniden dışa aktarır (reexports).

ptr
Ham işaretçilerle (raw pointers) el ile bellek yönetimi.

result
`Result` türüyle hata yönetimi (error handling).

slice
Dilimin (slice) yönetimi ve işlenmesi.

str
Dize işleme (string manipulation).

sync
Eşzamanlama ilkeleleri (synchronization primitives).

task
Eşzamanlı olmayan görevlerle (asynchronous tasks) çalışmaya yönelik türler ve özellikler.

time
Zamansal niceliklendirme (temporal quantification).

u8 Kullanımdan kaldırma planlanıyor (Deprecation planned)
`u8` ilkel türü için artık gereksiz sabit modülü.

u16 Kullanımdan kaldırma planlanıyor (Deprecation planned)
`u16` ilkel türü için artık gereksiz sabit modülü.

u32 Kullanımdan kaldırma planlanıyor (Deprecation planned)
`u32` ilkel türü için artık gereksiz sabit modülü.

u64 Kullanımdan kaldırma planlanıyor (Deprecation planned)
`u64` ilkel türü için artık gereksiz sabit modülü.

u128 Kullanımdan kaldırma planlanıyor (Deprecation planned)
`u128` ilkel türü için artık gereksiz sabit modülü.

usize Kullanımdan kaldırma planlanıyor (Deprecation planned)
`usize` ilkel türü için artık gereksiz sabit modülü.

assert\_matches Deneysel (Experimental)
Kararsız `assert_matches` makrosunu içeren kararsız modül.

async\_iter Deneysel (Experimental)
Bileşebilir eşzamansız yineleme.

autodiff Deneysel (Experimental)
Kararsız `autodiff` makrosunu içeren kararsız modül.

bstr Deneysel (Experimental)
`ByteStr` türü ve trait gerçekleştirimleri.

contracts Deneysel (Experimental)
Kararsız contracts dil ögeleri (lang items) ve öznitelik makrolarını içeren modül.

f16 Deneysel (Experimental)
Yarım duyarlıklı `f16` kayan nokta türü için sabitler.

f128 Deneysel (Experimental)
Dörtlü duyarlıklı `f128` kayan nokta türü için sabitler.

intrinsics Deneysel (Experimental)
Derleyici içyordamları (compiler intrinsics).

io Deneysel (Experimental)
Çekirdek G/Ç işlevselliği için özellikler, yardımcılar ve tür tanımları.

panicking Deneysel (Experimental)
Çekirdek için panik desteği.

pat Deneysel (Experimental)
`pattern_type` makrosunu dışa aktarmaya yardımcı modül.

random Deneysel (Experimental)
Rastgele değer üretimi.

range Deneysel (Experimental)
Deneysel yedek aralık (range) türleri.

simd Deneysel (Experimental)
Taşınabilir SIMD modülü.

ub\_checks Deneysel (Experimental)
`assert_unsafe_precondition` makrosunu ve yaygın önkoşulları kapsayan yardımcı işlevleri sağlar.

unicode Deneysel (Experimental)

unsafe\_binder Deneysel (Experimental)
Türleri “unsafe binder”lara dönüştürmek ve geri almak için kullanılan işleçler.

---

## 🧷 Makrolar (Macros)

assert
Çalışma zamanında bir boole ifadesinin doğru olduğunu doğrular (asserts).

assert\_eq
İki ifadenin birbirine eşit olduğunu doğrular (`PartialEq` kullanarak).

assert\_ne
İki ifadenin birbirine eşit olmadığını doğrular (`PartialEq` kullanarak).

cfg
Derleme zamanında (compile‑time) yapılandırma bayraklarının (configuration flags) boole birleşimlerini değerlendirir.

column
Çağrıldığı sütun numarasına genişler.

compile\_error
Karşılaşıldığında derlemenin, verilen hata iletisiyle (error message) başarısız olmasına neden olur.

concat
Sabitleri (literals) statik bir dize dilimine (static string slice) birleştirir.

debug\_assert
Çalışma zamanında bir boole ifadesinin doğru olduğunu doğrular.

debug\_assert\_eq
İki ifadenin eşit olduğunu doğrular.

debug\_assert\_ne
İki ifadenin eşit olmadığını doğrular.

env
Derleme zamanında bir ortam değişkenini (environment variable) inceler.

file
Çağrıldığı dosya adına genişler.

format\_args
Diğer dize biçimlendirme makroları için parametreler oluşturur.

include
Bağlama (context) göre bir dosyayı ifade (expression) ya da öğe (item) olarak ayrıştırır.

include\_bytes
Bir dosyayı bayt dizisine (byte array) başvuru olarak dahil eder.

include\_str
Bir dosyayı UTF‑8 kodlu dize olarak dahil eder.

line
Çağrıldığı satır numarasına genişler.

matches
Verilen ifadenin, sağlanan kalıpla (pattern) eşleşip eşleşmediğini döndürür.

module\_path
Geçerli modül yolunu temsil eden bir dizeye genişler.

option\_env
Derleme zamanında bir ortam değişkenini isteğe bağlı (optionally) inceler.

panic
Geçerli iş parçacığını (current thread) panikletir.

stringify
Bağımsız değişkenlerini dizeleştirir (stringifies).

todo
Tamamlanmamış kodu belirtir.

try Kullanımdan kaldırıldı (Deprecated)
Bir sonucu açar (unwrap) ya da hatasını yayar (propagate).

unimplemented
“not implemented” iletisiyle panikleyerek uygulanmamış kodu belirtir.

unreachable
Ulaşılamaz kodu belirtir.

write
Biçimlendirilmiş veriyi bir arabelleğe (buffer) yazar.

writeln
Biçimlendirilmiş veriyi bir arabelleğe yazar ve yeni satır ekler.

assert\_unsafe\_precondition Deneysel (Experimental)
Güvensiz (unsafe) bir işlevin önkoşullarının (preconditions) izlendiğini denetler.

cfg\_select Deneysel (Experimental)
`#[cfg]` eşleşme benzeri (match‑like) deyimler tanımlamaya yönelik bir makro.

concat\_bytes Deneysel (Experimental)
Sabitleri bir bayt dilimine (byte slice) birleştirir.

concat\_idents Kullanımdan kaldırıldı · Deneysel (Deprecated · Experimental)
Tanımlayıcıları (identifiers) tek bir tanımlayıcıda birleştirir.

const\_format\_args Deneysel (Experimental)
`format_args` ile aynıdır, ancak bazı `const` bağlamlarında kullanılabilir.

format\_args\_nl Deneysel (Experimental)
`format_args` ile aynıdır, ancak sona yeni satır ekler.

log\_syntax Deneysel (Experimental)
Aktarılan belirteçleri (tokens) standart çıktıya yazdırır.

pattern\_type Deneysel (Experimental)
Bir kalıp türü (pattern type) oluşturur.

trace\_macros Deneysel (Experimental)
Diğer makroları hata ayıklamada (debugging) kullanılan izlemeyi (tracing) etkinleştirir veya devre dışı bırakır.
