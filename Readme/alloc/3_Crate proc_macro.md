## 📦 Crate proc\_macro (crate proc\_macro)

Öğe yolunu kopyala (Copy item path) · Ayarlar (Settings) · Yardım (Help)

---

## 🧾 Özet (Summary)

**1.15.0** · Kaynak (Source)

Yeni makrolar tanımlarken (defining new macros) makro yazarları için bir destek kütüphanesi.

Standart dağıtım (standard distribution) tarafından sağlanan bu kütüphane, işlev benzeri makrolar (`#[proc_macro]`), makro öznitelikleri (`#[proc_macro_attribute]`) ve özel türetme öznitelikleri (`#[proc_macro_derive]`) gibi işlemsel olarak tanımlanmış makro tanımlarının arabirimlerinde kullanılan türleri sağlar.

Daha fazlası için kitaba bakın.

---

## 🧩 Modüller (Modules)

* `token_stream`
  `TokenStream` türü için yineleyiciler (iterators) gibi genel gerçekleştirim ayrıntıları.

* `tracked_env` Deneysel (Experimental)
  Ortam değişkenlerine (environment variables) izlenmiş erişim.

* `tracked_path` Deneysel (Experimental)
  Ek dosyalara izlenmiş erişim.

---

## 🔧 Makrolar (Macros)

* `quote` Deneysel (Experimental)
  `quote!(..)` herhangi bir belirteci (token) kabul eder ve girdiyi tanımlayan bir `TokenStream`e genişler.
  Örneğin, `quote!(a + b)` değerlendirildiğinde `TokenStream` oluşturur: `[Ident("a"), Punct('+', Alone), Ident("b")]`.

---

## 🏗️ Yapılar (Structs)

* `Group`
  Ayrılmış (delimited) bir belirteç akışı (token stream).

* `Ident`
  Bir tanımlayıcı (identifier, ident).

* `LexError`
  `TokenStream::from_str` tarafından döndürülen hata.

* `Literal`
  Bir sabit dize (`"hello"`), bayt dizesi (`b"hello"`), C dizesi (`c"hello"`), karakter (`'a'`), bayt karakter (`b'a'`), sonekli veya soneksiz bir tamsayı ya da kayan nokta sayısı (`1`, `1u8`, `2.3`, `2.3f32`).
  Boole sabitleri (`true`, `false`) buraya dahil değildir; bunlar `Ident` olarak kabul edilir.

* `Punct`
  `+`, `-`, `#` gibi tek bir noktalama karakteri.

* `Span`
  Kaynak kodunda bir bölge, makro genişletme (macro expansion) bilgisiyle birlikte.

* `TokenStream`
  Bu crate tarafından sağlanan ana tür; soyut bir belirteç akışını (abstract stream of tokens) ya da daha özel olarak, belirteç ağaçlarının (token trees) bir dizisini temsil eder. Bu tür, belirteç ağaçları üzerinde yineleme (iteration) arabirimleri sağlar ve tersine birçok belirteç ağacını tek bir akışta toplama (collecting) olanağı sunar.

* `Diagnostic` Deneysel (Experimental)
  Bir tanılama (diagnostic) mesajını ve ilişkili alt iletileri temsil eden yapı.

* `ExpandError` Deneysel (Experimental)
  `TokenStream::expand_expr` tarafından döndürülen hata.

---

## 🔄 Numaralandırmalar (Enums)

* `Delimiter`
  Belirteç ağaçlarının nasıl ayrıldığını (delimited) açıklar.

* `Spacing`
  Bir `Punct` belirtecinin, sonraki belirteç ile birleşerek çok karakterli bir işleç (multi-character operator) oluşturup oluşturamayacağını belirtir.

* `TokenTree`
  Tek bir belirteç veya ayrılmış bir belirteç ağacı dizisi (örn. `[1, (), ..]`).

* `ConversionErrorKind` Deneysel (Experimental)
  Kaçışsız (unescaped) değer elde etmeye çalışırken döndürülen hatalar.

* `EscapeError`
  Dize çözümlemesi (string unescaping) sırasında oluşabilecek hatalar ve uyarılar. Genellikle hatalı kaçış dizileriyle ilgilidir, ancak başka sorunlarla da ilgili olabilir.

* `Level` Deneysel (Experimental)
  Bir tanılama (diagnostic) düzeyini temsil eden numaralandırma.

---

## 🧩 Özellikler (Traits)

* `MultiSpan` Deneysel (Experimental)
  `Span` kümelerine dönüştürülebilen türler tarafından uygulanır.

* `ToTokens` Deneysel (Experimental)
  `quote!` çağrısı içinde ara yerleştirilebilecek (interpolated) türler.

---

## ⚙️ Fonksiyonlar (Functions)

* `is_available`
  `proc_macro`nun, çalışan programa erişilebilir hale getirilip getirilmediğini belirler.

* `quote` Deneysel (Experimental)
  Bir `TokenStream`i başka bir `TokenStream` içine alıntılar (quote). Bu, `quote!()` işlemsel makrosunun (proc macro) gerçek gerçekleştirimidir.

* `quote_span` Deneysel (Experimental)
  Bir `Span`i `TokenStream` içine alıntılar. Özel bir `quoter` uygulamak için gereklidir.
