## 📦 Crate std (crate std)

Öğe yolunu kopyala (Copy item path) · Ayarlar (Settings) · Yardım (Help)

---

## 🧾 Özet (Summary)

**1.0.0** · Kaynak (Source)

Rust Standart Kütüphanesi (Rust Standard Library), taşınabilir (portable) Rust yazılımının temelidir; Rust ekosisteminin geneli için en aza indirgenmiş (minimal) ve savaşta test edilmiş (battle-tested) paylaşılan soyutlamalar (shared abstractions) kümesidir.

`Vec<T>` ve `Option<T>` gibi çekirdek türler, dil ilkel türleri (language primitives) üzerinde tanımlanmış kütüphane işlemleri, standart makrolar, G/Ç (I/O), çok iş parçacığı (multithreading) ve daha birçok şeyi sunar.

`std`, varsayılan olarak tüm Rust crate’lerinde kullanılabilir. Dolayısıyla standart kütüphaneye `use std::env` gibi `use` ifadeleri üzerinden `std` yolu ile erişilebilir.

---

## 📖 Bu belgeler nasıl okunmalı? (How to read this documentation)

* Aradığınız şeyin adını zaten biliyorsanız, en hızlı yol sayfanın üst kısmındaki arama çubuğunu kullanmaktır.
* Aksi halde aşağıdaki bölümlerden birine atlayabilirsiniz:

  * `std::*` modülleri
  * İlkel türler (primitive types)
  * Standart makrolar (standard macros)
  * Rust Önyükleme (Prelude)

Belgeler, rahatça göz atılacak şekilde yazılmıştır. İlginç bağlantılara tıklamak sizi genellikle ilginç yerlere götürür. Yine de kaçırmamanız gereken önemli kısımlar vardır; bu nedenle standart kütüphaneye dair kısa bir tur önerilir.

Standart kütüphaneye alıştıkça metnin uzunluğu dikkatinizi dağıtabilir. Bu noktada sayfanın üst kısmındaki **“Summary”** düğmesine basarak özet görünüme geçebilirsiniz.

Ayrıca **“Source”** bağlantısına da dikkat edin. Rust’ın API belgeleri kaynak koduyla birlikte gelir ve okunması teşvik edilir. Kaynak kod genellikle yüksek kalitelidir ve perde arkasına bakmak çoğu zaman aydınlatıcıdır.

---

## 📂 Standart kütüphane belgelerinde neler var? (What is in the standard library documentation?)

1. Rust Standart Kütüphanesi birçok odaklı modüle bölünmüştür. Bunlar, Rust’ın temeli olup `std::slice` ve `std::cmp` gibi güçlü adlara sahiptir. Modül belgeleri genellikle genel bir bakış ve örnekler içerir.

2. İlkel türler (primitives) üzerindeki örtük yöntemler burada belgelenir.

   * İlkel türler derleyici tarafından uygulanır. Ancak standart kütüphane, bu türlerin üzerine doğrudan yöntemler tanımlar ve bunlar yalnızca burada belgelenir.
   * Ayrıca standart kütüphane, ilkel türlerle aynı adı taşıyan modüller dışa aktarır. Bunlar türle ilgili ek öğeleri tanımlar, fakat yöntemleri içermez.
     Örneğin:
   * `i32` için sayfada, 32 bit tamsayılarda çağrılabilecek yöntemler belgelenmiştir.
   * `std::i32` modülünde ise `MIN` ve `MAX` sabitleri belgelenmiştir.

3. `str` ve `[T]` (slice) belgelerine dikkat edin. `String` ve `Vec<T>` üzerindeki birçok yöntem çağrısı aslında `str` ve `[T]` yöntemleridir (deref coercion yoluyla).

4. Standart kütüphane, Rust Prelude adında küçük bir koleksiyon tanımlar. Bu, her crate’in her modülüne otomatik olarak içe aktarılan (imported) öğelerden oluşur. Prelude çoğunlukla trait’lerden oluşur ve öğrenmeye başlamak için iyi bir noktadır.

5. Standart makrolar (standard macros) da burada listelenir. Bunların bazıları derleyici tarafından tanımlanır, ancak yine de belgeler bu sayfada bulunur. Prelude gibi, tüm crate’lere varsayılan olarak dahil edilirler.

---

## 🤝 Belgeler için katkı (Contributing changes to the documentation)

Belgeler için katkı kurallarına Rust katkı rehberinden ulaşabilirsiniz. Kaynak kodu GitHub’da `library/std/` dizininde bulunur. Katkı yapmadan önce kuralları okuyun, ardından pull request gönderin.

Her türlü katkı memnuniyetle karşılanır!

---

## 🚀 Rust Standart Kütüphanesine kısa tur (A Tour of The Rust Standard Library)

### 📦 Kaplar ve koleksiyonlar (Containers and collections)

* `option` ve `result` modülleri: `Option<T>` ve `Result<T, E>` türlerini tanımlar.
* `iter` modülü: `Iterator` trait’ini tanımlar ve `for` döngüsüyle çalışır.

Üç yaygın bellek bölgesi yönetim şekli:

* `Vec<T>` → Yığın üzerinde ayrılmış (heap-allocated), çalışma zamanında yeniden boyutlandırılabilir vektör.
* `[T; N]` → Derleme zamanında sabit boyutlu satır içi dizi.
* `[T]` → Dinamik boyutlu dilim (slice).

Dilim çeşitleri:

* `&[T]` → Paylaşımlı dilim.
* `&mut [T]` → Değiştirilebilir dilim.
* `Box<[T]>` → Sahip olunan (owned) dilim.

`str` → UTF-8 dize dilimi, genellikle `&str` olarak erişilir. Sahip olunan `String` türü değiştirme ve inşa etme için kullanılır.

Dönüşümler:

* Dizeye dönüştürmek için `format!`.
* Dizelerden dönüştürmek için `FromStr` trait’i.

Veri paylaşımı:

* `Rc` → Tek iş parçacıklı referans sayımı.
* `Arc + Mutex` → Çok iş parçacıklı ortamda paylaşımlı mutasyon.

`collections` modülü → `HashMap<K, V>`, küme, bağlı liste ve diğer tipik koleksiyonlar.

---

### 🖥️ Platform soyutlamaları ve G/Ç (Platform abstractions and I/O)

* `io`, `fs`, `net` → Dosya, TCP ve UDP işlemleri.
* `thread` → İş parçacığı soyutlamaları.
* `sync` → Atomik türler ve kanal tipleri (mpmc, mpsc).

---

### 🏁 main() öncesi ve sonrası kullanım (Use before and after main())

* Çoğu özellik `main()` öncesinde/sonrasında çalışabilir, ancak garanti edilmez.
* OS veya global durum ile etkileşen özellikler taşınabilirlik garantisi dışındadır.
* Hatalar raporlanmalıdır.

Bilinen sınırlamalar:

* `thread::current()` gibi iş parçacığı-yereli (thread-local) özellikler.
* UNIX altında `main` öncesi dosya tanımlayıcıları (0,1,2) açık olmayabilir.

---

## 🧱 İlkel Türler (Primitive Types)

* `array` → `[T; N]`
* `bool`
* `char`
* `f32`, `f64`
* `fn` işlev işaretçileri
* `i8`, `i16`, `i32`, `i64`, `i128`
* `isize`
* `pointer` → `*const T`, `*mut T`
* `reference` → `&T`, `&mut T`
* `slice` → `[T]`
* `str`
* `tuple`
* `u8`, `u16`, `u32`, `u64`, `u128`
* `unit` → `()`
* `usize`
* `f16` (deneysel)
* `f128` (deneysel)
* `never` → `!` (deneysel)

---

## 🧩 Modüller (Modules)

Başlıca modüller:

* `alloc`, `any`, `arch`, `array`, `ascii`, `backtrace`, `borrow`, `boxed`, `cell`, `char`, `clone`, `cmp`, `collections`, `convert`, `default`, `env`, `error`, `f32`, `f64`, `ffi`, `fmt`, `fs`, `future`, `hash`, `hint`, `io`, `iter`, `marker`, `mem`, `net`, `num`, `ops`, `option`, `os`, `panic`, `path`, `pin`, `prelude`, `primitive`, `process`, `ptr`, `rc`, `result`, `slice`, `str`, `string`, `sync`, `task`, `thread`, `time`, `vec`.

Deneysel modüller:

* `assert_matches`, `async_iter`, `autodiff`, `bstr`, `f16`, `f128`, `intrinsics`, `pat`, `random`, `range`, `simd`, `unsafe_binder`.

---

## 🔧 Makrolar (Macros)

* `assert`, `assert_eq`, `assert_ne`
* `cfg`, `column`, `compile_error`, `concat`
* `dbg`, `debug_assert`, `debug_assert_eq`, `debug_assert_ne`
* `env`, `eprint`, `eprintln`
* `file`, `format`, `format_args`
* `include`, `include_bytes`, `include_str`
* `is_x86_feature_detected`
* `line`, `matches`, `module_path`, `option_env`
* `panic`, `print`, `println`
* `stringify`, `thread_local`, `todo`
* `try` (kullanımdan kaldırıldı)
* `unimplemented`, `unreachable`, `vec`
* `write`, `writeln`
* Deneysel: `cfg_select`, `concat_bytes`, `concat_idents`, `const_format_args`, `format_args_nl`, `log_syntax`, `trace_macros`.

---

## 🔑 Anahtar Kelimeler (Keywords)

* `SelfTy`, `as`, `async`, `await`, `break`, `const`, `continue`, `crate`, `dyn`, `else`, `enum`, `extern`, `false`, `fn`, `for`, `if`, `impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`, `self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `union`, `unsafe`, `use`, `where`, `while`.
