## 📚 Ek B: Operatörler (operators) ve Semboller (symbols)

Bu ek, Rust’ın sözdiziminin (syntax) sözlüğünü içermektedir. Operatörler ve yollar (paths), generics, trait sınırları (trait bounds), makrolar (macros), öznitelikler (attributes), yorumlar (comments), demetler (tuples) ve köşeli parantezler (brackets) bağlamında görülen diğer semboller açıklanmıştır.

---

## 🔢 Operatörler (operators)

Tablo B-1 Rust’taki operatörleri, bağlamda (context) nasıl göründüklerine dair örnekleri, kısa bir açıklamayı ve bu operatörün aşırı yüklenebilir (overloadable) olup olmadığını içermektedir. Eğer bir operatör aşırı yüklenebilir ise, o operatörü aşırı yüklemek için kullanılacak ilgili trait belirtilmiştir.

### Tablo B-1: Operatörler

| Operatör | Örnek                                            | Açıklama                                                              | Aşırı Yüklenebilir mi? |                                 |               |                               |   |
| -------- | ------------------------------------------------ | --------------------------------------------------------------------- | ---------------------- | ------------------------------- | ------------- | ----------------------------- | - |
| `!`      | `ident!(...)`, `ident!{...}`, `ident![...]`      | Makro genişletme (macro expansion)                                    |                        |                                 |               |                               |   |
| `!`      | `!expr`                                          | Bit düzeyinde (bitwise) veya mantıksal (logical) tümleme (complement) | `Not`                  |                                 |               |                               |   |
| `!=`     | `expr != expr`                                   | Eşitsizlik karşılaştırması                                            | `PartialEq`            |                                 |               |                               |   |
| `%`      | `expr % expr`                                    | Aritmetik kalan (remainder)                                           | `Rem`                  |                                 |               |                               |   |
| `%=`     | `var %= expr`                                    | Aritmetik kalan ve atama                                              | `RemAssign`            |                                 |               |                               |   |
| `&`      | `&expr`, `&mut expr`                             | Ödünç alma (borrow)                                                   |                        |                                 |               |                               |   |
| `&`      | `&type`, `&mut type`, `&'a type`, `&'a mut type` | Ödünç alınmış işaretçi türü (borrowed pointer type)                   |                        |                                 |               |                               |   |
| `&`      | `expr & expr`                                    | Bit düzeyinde VE (AND)                                                | `BitAnd`               |                                 |               |                               |   |
| `&=`     | `var &= expr`                                    | Bit düzeyinde VE ve atama                                             | `BitAndAssign`         |                                 |               |                               |   |
| `&&`     | `expr && expr`                                   | Kısa devre yapan (short-circuiting) mantıksal VE                      |                        |                                 |               |                               |   |
| `*`      | `expr * expr`                                    | Aritmetik çarpma                                                      | `Mul`                  |                                 |               |                               |   |
| `*=`     | `var *= expr`                                    | Aritmetik çarpma ve atama                                             | `MulAssign`            |                                 |               |                               |   |
| `*`      | `*expr`                                          | Göstericiyi çözme (dereference)                                       | `Deref`                |                                 |               |                               |   |
| `*`      | `*const type`, `*mut type`                       | Ham işaretçi (raw pointer)                                            |                        |                                 |               |                               |   |
| `+`      | `trait + trait`, `'a + trait`                    | Bileşik tür kısıtı (compound type constraint)                         |                        |                                 |               |                               |   |
| `+`      | `expr + expr`                                    | Aritmetik toplama                                                     | `Add`                  |                                 |               |                               |   |
| `+=`     | `var += expr`                                    | Aritmetik toplama ve atama                                            | `AddAssign`            |                                 |               |                               |   |
| `,`      | `expr, expr`                                     | Argüman ve eleman ayırıcı                                             |                        |                                 |               |                               |   |
| `-`      | `-expr`                                          | Aritmetik negatifleme                                                 | `Neg`                  |                                 |               |                               |   |
| `-`      | `expr - expr`                                    | Aritmetik çıkarma                                                     | `Sub`                  |                                 |               |                               |   |
| `-=`     | `var -= expr`                                    | Aritmetik çıkarma ve atama                                            | `SubAssign`            |                                 |               |                               |   |
| `->`     | `fn(...) -> type`, \`                            | …                                                                     | -> type\`              | Fonksiyon ve closure dönüş türü |               |                               |   |
| `.`      | `expr.ident`                                     | Alan erişimi (field access)                                           |                        |                                 |               |                               |   |
| `.`      | `expr.ident(expr, ...)`                          | Metot çağrısı                                                         |                        |                                 |               |                               |   |
| `.`      | `expr.0`, `expr.1`                               | Demet (tuple) indeksleme                                              |                        |                                 |               |                               |   |
| `..`     | `..`, `expr..`, `..expr`, `expr..expr`           | Sağdan hariç aralık (right-exclusive range)                           | `PartialOrd`           |                                 |               |                               |   |
| `..=`    | `..=expr`, `expr..=expr`                         | Sağdan dahil aralık (right-inclusive range)                           | `PartialOrd`           |                                 |               |                               |   |
| `..`     | `..expr`                                         | Struct literal güncelleme sözdizimi                                   |                        |                                 |               |                               |   |
| `..`     | `variant(x, ..)`, `struct_type { x, .. }`        | “Ve geri kalanı” desen bağlaması                                      |                        |                                 |               |                               |   |
| `...`    | `expr...expr`                                    | (Eskimiş – `..=` kullanın) Kapsayıcı aralık deseni                    |                        |                                 |               |                               |   |
| `/`      | `expr / expr`                                    | Aritmetik bölme                                                       | `Div`                  |                                 |               |                               |   |
| `/=`     | `var /= expr`                                    | Aritmetik bölme ve atama                                              | `DivAssign`            |                                 |               |                               |   |
| `:`      | `pat: type`, `ident: type`                       | Kısıtlar (constraints)                                                |                        |                                 |               |                               |   |
| `:`      | `ident: expr`                                    | Struct alan başlatıcı                                                 |                        |                                 |               |                               |   |
| `:`      | `'a: loop {...}`                                 | Döngü etiketi                                                         |                        |                                 |               |                               |   |
| `;`      | `expr;`                                          | İfade ve öğe sonlandırıcı                                             |                        |                                 |               |                               |   |
| `;`      | `[...,; len]`                                    | Sabit boyutlu dizi sözdiziminin bir parçası                           |                        |                                 |               |                               |   |
| `<<`     | `expr << expr`                                   | Bit kaydırma sola (left shift)                                        | `Shl`                  |                                 |               |                               |   |
| `<<=`    | `var <<= expr`                                   | Sola kaydırma ve atama                                                | `ShlAssign`            |                                 |               |                               |   |
| `<`      | `expr < expr`                                    | Küçüktür karşılaştırması                                              | `PartialOrd`           |                                 |               |                               |   |
| `<=`     | `expr <= expr`                                   | Küçük veya eşittir karşılaştırması                                    | `PartialOrd`           |                                 |               |                               |   |
| `=`      | `var = expr`, `ident = type`                     | Atama / eşitleme                                                      |                        |                                 |               |                               |   |
| `==`     | `expr == expr`                                   | Eşitlik karşılaştırması                                               | `PartialEq`            |                                 |               |                               |   |
| `=>`     | `pat => expr`                                    | `match` kolu sözdiziminin parçası                                     |                        |                                 |               |                               |   |
| `>`      | `expr > expr`                                    | Büyüktür karşılaştırması                                              | `PartialOrd`           |                                 |               |                               |   |
| `>=`     | `expr >= expr`                                   | Büyük veya eşittir karşılaştırması                                    | `PartialOrd`           |                                 |               |                               |   |
| `>>`     | `expr >> expr`                                   | Sağa kaydırma (right shift)                                           | `Shr`                  |                                 |               |                               |   |
| `>>=`    | `var >>= expr`                                   | Sağa kaydırma ve atama                                                | `ShrAssign`            |                                 |               |                               |   |
| `@`      | `ident @ pat`                                    | Desen bağlama                                                         |                        |                                 |               |                               |   |
| `^`      | `expr ^ expr`                                    | Bit düzeyinde özel VEYA (XOR)                                         | `BitXor`               |                                 |               |                               |   |
| `^=`     | `var ^= expr`                                    | XOR ve atama                                                          | `BitXorAssign`         |                                 |               |                               |   |
| \`       | \`                                               | \`pat                                                                 | pat\`                  | Desen alternatifleri            |               |                               |   |
| \`       | \`                                               | \`expr                                                                | expr\`                 | Bit düzeyinde VEYA (OR)         | `BitOr`       |                               |   |
| \`       | =\`                                              | \`var                                                                 | = expr\`               | OR ve atama                     | `BitOrAssign` |                               |   |
| \`       |                                                  | \`                                                                    | \`expr                 |                                 | expr\`        | Kısa devre yapan mantıksal OR |   |
| `?`      | `expr?`                                          | Hata yayılımı (error propagation)                                     |                        |                                 |               |                               |   |

---

## 🌀 Operatör Olmayan Semboller (non-operator symbols)

Aşağıdaki liste, fonksiyon veya metot çağrısı gibi davranmayan tüm sembolleri içermektedir.

### Tablo B-2: Bağımsız Sözdizimi (stand-alone syntax)

| Sembol                                  | Açıklama                                                                           |        |         |
| --------------------------------------- | ---------------------------------------------------------------------------------- | ------ | ------- |
| `'ident`                                | Adlandırılmış yaşam süresi (named lifetime) veya döngü etiketi                     |        |         |
| `...u8`, `...i32`, `...f64`, `...usize` | Belirli türde sayısal literal                                                      |        |         |
| `"..."`                                 | String literal                                                                     |        |         |
| `r"..."`, `r#"..."#`, `r##"..."##`      | Ham string literal (escape işaretleri işlenmez)                                    |        |         |
| `b"..."`                                | Byte string literal; string yerine byte dizisi oluşturur                           |        |         |
| `br"..."`, `br#"..."#`, `br##"..."##`   | Ham byte string literal                                                            |        |         |
| `'...'`                                 | Karakter literal                                                                   |        |         |
| `b'...'`                                | ASCII byte literal                                                                 |        |         |
| \`                                      | …                                                                                  | expr\` | Closure |
| `!`                                     | Çıkışsız (diverging) fonksiyonlar için her zaman boş dip türü (bottom type)        |        |         |
| `_`                                     | “Yok sayılan” desen bağlaması; ayrıca tamsayı literallerini okunabilir yapmak için |        |         |

---

### Tablo B-3: Yol (path) ile İlgili Sözdizimi

| Sembol                                  | Açıklama                                                           |
| --------------------------------------- | ------------------------------------------------------------------ |
| `ident::ident`                          | Ad alanı yolu (namespace path)                                     |
| `::path`                                | `extern prelude`’den göreli yol (crate adıyla başlayan mutlak yol) |
| `self::path`                            | Mevcut modüle göre yol                                             |
| `super::path`                           | Mevcut modülün ebeveynine göre yol                                 |
| `type::ident`, `<type as trait>::ident` | İlişkili sabitler, fonksiyonlar ve türler                          |
| `<type>::...`                           | Doğrudan adlandırılamayan türün ilişkili öğesi                     |
| `trait::method(...)`                    | Bir metot çağrısını trait adıyla ayırt etme                        |
| `type::method(...)`                     | Bir metot çağrısını tür adıyla ayırt etme                          |
| `<type as trait>::method(...)`          | Trait ve tür adıyla metot çağrısını ayırt etme                     |

---

### Tablo B-4: Generics

| Sembol                         | Açıklama                                                             |
| ------------------------------ | -------------------------------------------------------------------- |
| `path<...>`                    | Generic tür parametreleri belirtir (`Vec<u8>`)                       |
| `path::<...>`, `method::<...>` | Generic tür, fonksiyon veya metot parametreleri belirtir (turbofish) |
| `fn ident<...> ...`            | Generic fonksiyon tanımlar                                           |
| `struct ident<...> ...`        | Generic yapı tanımlar                                                |
| `enum ident<...> ...`          | Generic numaralandırma tanımlar                                      |
| `impl<...> ...`                | Generic implementasyon tanımlar                                      |
| `for<...> type`                | Üst dereceli yaşam süresi sınırları                                  |
| `type<ident=type>`             | İlişkili türleri belirli atamalara sahip generic tür                 |

---

### Tablo B-5: Trait Sınırları (trait bound constraints)

| Sembol                        | Açıklama                                                       |
| ----------------------------- | -------------------------------------------------------------- |
| `T: U`                        | `T` generic parametresi `U`’yu uygulayan türlerle sınırlıdır   |
| `T: 'a`                       | `T` türü `'a` yaşam süresinden uzun olmalıdır                  |
| `T: 'static`                  | `T` türü `'static` dışında ödünç alınmış referans içermez      |
| `'b: 'a`                      | `'b` yaşam süresi `'a` yaşam süresinden uzun olmalıdır         |
| `T: ?Sized`                   | Generic tür parametresinin dinamik boyutlu olmasına izin verir |
| `'a + trait`, `trait + trait` | Bileşik tür kısıtı                                             |

---

### Tablo B-6: Makrolar (macros) ve Öznitelikler (attributes)

| Sembol                                      | Açıklama                        |
| ------------------------------------------- | ------------------------------- |
| `#[meta]`                                   | Dış öznitelik (outer attribute) |
| `#![meta]`                                  | İç öznitelik (inner attribute)  |
| `$ident`                                    | Makro ikamesi (substitution)    |
| `$ident:kind`                               | Makro yakalaması (capture)      |
| `$(…)…`                                     | Makro tekrarları                |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Makro çağrısı                   |

---

### Tablo B-7: Yorumlar (comments)

| Sembol     | Açıklama                       |
| ---------- | ------------------------------ |
| `//`       | Satır yorumu                   |
| `//!`      | İç satır dokümantasyon yorumu  |
| `///`      | Dış satır dokümantasyon yorumu |
| `/*...*/`  | Blok yorumu                    |
| `/*!...*/` | İç blok dokümantasyon yorumu   |
| `/**...*/` | Dış blok dokümantasyon yorumu  |

---

### Tablo B-8: Parantezler (parentheses)

| Sembol            | Açıklama                                                                            |
| ----------------- | ----------------------------------------------------------------------------------- |
| `()`              | Boş demet (unit) – hem literal hem tür                                              |
| `(expr)`          | Parantez içine alınmış ifade                                                        |
| `(expr,)`         | Tek elemanlı demet ifadesi                                                          |
| `(type,)`         | Tek elemanlı demet türü                                                             |
| `(expr, ...)`     | Demet ifadesi                                                                       |
| `(type, ...)`     | Demet türü                                                                          |
| `expr(expr, ...)` | Fonksiyon çağrısı ifadesi; ayrıca demet struct ve enum varyantlarını başlatmak için |

---

### Tablo B-9: Süslü Parantezler (curly brackets)

| Bağlam       | Açıklama       |
| ------------ | -------------- |
| `{...}`      | Blok ifadesi   |
| `Type {...}` | Struct literal |

---

### Tablo B-10: Köşeli Parantezler (square brackets)

| Bağlam                                             | Açıklama                                                                               |
| -------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `[...]`                                            | Dizi literal                                                                           |
| `[expr; len]`                                      | `expr` ifadesinin `len` kopyasından oluşan dizi literal                                |
| `[type; len]`                                      | `len` adet tür içeren dizi türü                                                        |
| `expr[expr]`                                       | Koleksiyon indeksleme (overloadable: `Index`, `IndexMut`)                              |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Koleksiyon dilimleme (slicing) – `Range`, `RangeFrom`, `RangeTo`, `RangeFull` kullanır |
