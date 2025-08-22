## 📚 Ek A: Anahtar Kelimeler (keywords)

Aşağıdaki liste, Rust dili tarafından şu anda veya gelecekte kullanılmak üzere ayrılmış anahtar kelimeleri (keywords) içermektedir. Bu nedenle, bunlar tanımlayıcı (identifier) olarak kullanılamaz (yalnızca “Ham Tanımlayıcılar (Raw Identifiers)” bölümünde açıklanacağı gibi ham tanımlayıcı olarak kullanılabilirler). Tanımlayıcılar (identifiers), fonksiyonların, değişkenlerin, parametrelerin, struct alanlarının, modüllerin, crate’lerin, sabitlerin (constants), makroların, statik değerlerin, özniteliklerin (attributes), türlerin (types), trait’lerin veya yaşam sürelerinin (lifetimes) isimleridir.

---

## 🔑 Hâlihazırda Kullanılan Anahtar Kelimeler (keywords currently in use)

Aşağıda hâlihazırda kullanılan anahtar kelimeler ve işlevlerinin açıklaması bulunmaktadır.

* `as` - ilkel tür dönüşümü (primitive casting) yapmak, bir öğeyi içeren belirli bir trait’i ayırt etmek veya `use` ifadelerinde öğeleri yeniden adlandırmak
* `async` - mevcut iş parçacığını (thread) engellemek yerine bir `Future` döndürmek
* `await` - bir `Future` sonucu hazır olana kadar yürütmeyi askıya almak
* `break` - bir döngüden (loop) hemen çıkmak
* `const` - sabit (constant) öğeler veya sabit ham işaretçiler (raw pointers) tanımlamak
* `continue` - döngünün bir sonraki yinelemesine (iteration) geçmek
* `crate` - bir modül yolunda (module path), crate köküne (crate root) başvurur
* `dyn` - bir trait nesnesine (trait object) dinamik gönderim (dynamic dispatch) yapmak
* `else` - `if` ve `if let` kontrol akışı yapıları için alternatif (fallback) sağlamak
* `enum` - bir numaralandırma (enumeration) tanımlamak
* `extern` - harici bir fonksiyona veya değişkene bağlanmak
* `false` - Boolean `false` sabiti
* `fn` - bir fonksiyon veya fonksiyon işaretçi türü (function pointer type) tanımlamak
* `for` - bir yineleyici (iterator) üzerinden döngü kurmak, bir trait uygulamak veya üst dereceli yaşam süresi (higher-ranked lifetime) belirtmek
* `if` - koşullu bir ifadenin (conditional expression) sonucuna göre dallanmak
* `impl` - doğal (inherent) veya trait işlevselliğini uygulamak
* `in` - `for` döngüsü sözdiziminin bir parçası
* `let` - bir değişken bağlamak
* `loop` - koşulsuz döngü (unconditional loop) oluşturmak
* `match` - bir değeri desenlerle (patterns) eşleştirmek
* `mod` - bir modül (module) tanımlamak
* `move` - bir kapanışın (closure) yakaladıklarının tamamının sahipliğini (ownership) almasını sağlamak
* `mut` - referanslarda, ham işaretçilerde veya desen bağlamalarında değiştirilebilirlik (mutability) belirtmek
* `pub` - struct alanlarında, `impl` bloklarında veya modüllerde kamusal görünürlük (public visibility) belirtmek
* `ref` - referans ile bağlamak
* `return` - bir fonksiyondan geri dönmek
* `Self` - tanımlamakta veya uygulamakta olduğumuz tür için bir tür takma adı (type alias)
* `self` - metot öznesi (method subject) veya mevcut modül (current module)
* `static` - tüm program yürütmesi boyunca geçerli olan küresel değişken veya yaşam süresi (lifetime)
* `struct` - bir yapı (structure) tanımlamak
* `super` - mevcut modülün üst modülü (parent module)
* `trait` - bir trait tanımlamak
* `true` - Boolean `true` sabiti
* `type` - bir tür takma adı (type alias) veya ilişkili tür (associated type) tanımlamak
* `union` - bir birlik (union) tanımlamak; yalnızca bir `union` bildirimi içinde anahtar kelimedir
* `unsafe` - güvensiz (unsafe) kodu, fonksiyonları, trait’leri veya uygulamaları belirtmek
* `use` - sembolleri kapsam içine almak; generic ve yaşam süresi sınırları için kesin yakalamaları (precise captures) belirtmek
* `where` - bir türü kısıtlayan hükümler (clauses) belirtmek
* `while` - bir ifadenin sonucuna bağlı olarak koşullu döngü oluşturmak

---

## ⏳ Gelecek İçin Ayrılmış Anahtar Kelimeler (keywords reserved for future use)

Aşağıdaki anahtar kelimeler henüz bir işlevselliğe sahip değildir ancak gelecekte kullanım için Rust tarafından ayrılmıştır.

* `abstract`
* `become`
* `box`
* `do`
* `final`
* `gen`
* `macro`
* `override`
* `priv`
* `try`
* `typeof`
* `unsized`
* `virtual`
* `yield`

---

## 🆔 Ham Tanımlayıcılar (raw identifiers)

Ham tanımlayıcılar (raw identifiers), normalde izin verilmeyen yerlerde anahtar kelimeleri kullanmanıza olanak tanıyan sözdizimidir. Bir ham tanımlayıcı kullanmak için bir anahtar kelimeyi `r#` ile öneklersiniz.

Örneğin, `match` bir anahtar kelimedir. Eğer `match` ismini kullanan şu fonksiyonu derlemeyi denerseniz:

**Dosya Adı: src/main.rs**

Bu kod derlenmez!

```rust
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

Aşağıdaki hatayı alırsınız:

```
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

Bu hata, `match` anahtar kelimesini bir fonksiyon tanımlayıcısı olarak kullanamayacağınızı gösterir. `match` kelimesini fonksiyon ismi olarak kullanmak için ham tanımlayıcı sözdizimini şu şekilde kullanmanız gerekir:

**Dosya Adı: src/main.rs**

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Bu kod herhangi bir hata olmadan derlenecektir. Fonksiyon adının tanımında ve `main` fonksiyonunda çağrılırken `r#` önekinin kullanıldığına dikkat edin.

Ham tanımlayıcılar (raw identifiers), ayrılmış bir anahtar kelime olsa bile istediğiniz herhangi bir kelimeyi tanımlayıcı olarak kullanmanıza olanak sağlar. Bu bize daha fazla adlandırma özgürlüğü verir ve bu kelimelerin anahtar kelime olmadığı bir dilde yazılmış programlarla bütünleşmemizi mümkün kılar. Ayrıca, ham tanımlayıcılar crate’inizin kullandığı Rust sürümünden (edition) farklı bir sürümde yazılmış kütüphaneleri kullanmanıza olanak tanır. Örneğin, `try` 2015 sürümünde bir anahtar kelime değildir ancak 2018, 2021 ve 2024 sürümlerinde anahtar kelimedir. Eğer 2015 sürümünü kullanan bir kütüphaneye bağımlıysanız ve bu kütüphane bir `try` fonksiyonu içeriyorsa, daha sonraki sürümlerde bu fonksiyonu çağırmak için `r#try` sözdizimini kullanmanız gerekir. Daha fazla bilgi için Ek E (Appendix E) bölümüne bakınız.
