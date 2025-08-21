## 🔤 UTF-8 Kodlanmış Metinleri String’lerle (String) Depolama

4. bölümde `string` (dizgi) konusunu ele almıştık, ancak şimdi bunu daha derinlemesine inceleyeceğiz. Yeni Rustacean’lar genellikle `string`’lerde üç nedenle takılır: Rust’ın olası hataları açığa çıkarma eğilimi, `string`’lerin birçok programcının düşündüğünden daha karmaşık bir veri yapısı olması ve UTF-8. Bu faktörler bir araya geldiğinde, başka programlama dillerinden gelenler için zorlayıcı görünebilir.

`string`’leri koleksiyonlar bağlamında ele alıyoruz çünkü `string`’ler baytların (byte) bir koleksiyonu olarak uygulanır; bu baytlar metin olarak yorumlandığında kullanışlı işlevsellik sağlayan bazı yöntemlerle desteklenir. Bu bölümde, her koleksiyon tipinde olduğu gibi `String` üzerinde yapılabilen işlemlerden, yani oluşturma, güncelleme ve okuma işlemlerinden bahsedeceğiz. Ayrıca `String`’in diğer koleksiyonlardan farklı olduğu yönleri de ele alacağız; özellikle de bir `String` içinde indeksleme işleminin, insanların ve bilgisayarların `String` verilerini farklı şekillerde yorumlaması nedeniyle karmaşık oluşunu tartışacağız.
## 🧾 String Nedir?

Öncelikle `string` teriminden ne kastettiğimizi tanımlayalım. Rust’ın çekirdek dilinde yalnızca bir `string` tipi vardır: bu da genellikle ödünç alınmış formu olan `&str` ile görülen `string slice (str)` tipidir. 4. bölümde `string slice`’ları ele almıştık; bunlar başka bir yerde depolanmış UTF-8 kodlu `string` verilerine referanslardır. Örneğin, `string literal`’ler programın ikili dosyasında (binary) saklanır ve bu nedenle birer `string slice`’tır.

Rust’ın standart kütüphanesi tarafından sağlanan `String` tipi ise çekirdek dile gömülü değildir; büyüyebilen (growable), değiştirilebilir (mutable), sahipli (owned), UTF-8 kodlu bir `string` tipidir. Rustacean’lar Rust’taki “string” kavramından bahsettiklerinde, yalnızca birinden değil, hem `String` tipinden hem de `string slice (&str)` tipinden bahsediyor olabilirler. Bu bölüm esas olarak `String` üzerine yoğunlaşsa da, Rust’ın standart kütüphanesinde her iki tip de yoğun olarak kullanılır ve hem `String` hem de `string slice` tipleri UTF-8 kodlamalıdır.

## ✨ Yeni Bir String (String) Oluşturma

`Vec<T>` ile kullanılabilen birçok işlem `String` için de geçerlidir çünkü `String` aslında bir bayt vektörünün (`vector of bytes`) etrafında bazı ek garantiler, kısıtlamalar ve yeteneklerle sarmalanmış (wrapper) bir yapıdır. `Vec<T>` ve `String` ile aynı şekilde çalışan bir fonksiyona örnek olarak yeni bir örnek (instance) oluşturan `new` fonksiyonu gösterilebilir (Liste 8-11).

```rust
let mut s = String::new();
```

Liste 8-11: Yeni, boş bir `String` oluşturma

Bu satır `s` adında yeni, boş bir `string` oluşturur ve sonrasında içine veri yükleyebiliriz. Çoğu zaman `string`’e başlamak istediğimiz bazı başlangıç verilerimiz olur. Bunun için `Display` trait’ini uygulayan (implement) herhangi bir tipte kullanılabilen `to_string` metodunu kullanırız; `string literal`’ler de bu özelliğe sahiptir. Liste 8-12 iki örnek göstermektedir.

```rust
let data = "initial contents";

let s = data.to_string();

// Bu metot doğrudan bir literal üzerinde de çalışır:
let s = "initial contents".to_string();
```

Liste 8-12: Bir `string literal`’dan `String` oluşturmak için `to_string` metodunu kullanma

Bu kod başlangıç içeriğine sahip bir `string` oluşturur.

Bir `string literal`’dan `String` oluşturmak için ayrıca `String::from` fonksiyonunu da kullanabiliriz. Liste 8-13’teki kod, Liste 8-12’de `to_string` kullanılan koda denktir.

```rust
let s = String::from("initial contents");
```

Liste 8-13: Bir `string literal`’dan `String` oluşturmak için `String::from` fonksiyonunu kullanma

`string`’ler pek çok amaç için kullanıldığından, onlar için birçok farklı jenerik API kullanabiliriz ve bu da bize birçok seçenek sunar. Bunların bazıları gereksiz gibi görünebilir, ancak hepsinin bir yeri vardır! Bu durumda, `String::from` ve `to_string` aynı işi yapar; hangisini seçeceğiniz tamamen tarz ve okunabilirlik meselesidir.

Unutmayın ki `string`’ler UTF-8 ile kodlanır; bu nedenle içine uygun şekilde kodlanmış herhangi bir veriyi koyabiliriz (Liste 8-14’te gösterildiği gibi).

```rust
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שלום");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

Liste 8-14: Farklı dillerde selamlamaları `string` içinde depolama

Bunların hepsi geçerli `String` değerleridir.
## 🔄 Bir String (String) Güncelleme

Bir `String`, tıpkı bir `Vec<T>` gibi boyut olarak büyüyebilir ve içeriği değişebilir; bunun için içine daha fazla veri eklememiz yeterlidir. Ayrıca `+` operatörünü veya `format!` makrosunu kullanarak `String` değerlerini birleştirebiliriz.

### ➕ push\_str ve push ile String’e Ekleme

Bir `String`’i, `push_str` metodunu kullanarak bir `string slice` ekleyerek büyütebiliriz (Liste 8-15).

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

Liste 8-15: `push_str` metodunu kullanarak bir `String`’e `string slice` ekleme

Bu iki satırdan sonra `s` değeri `foobar` içerecektir. `push_str` metodu bir `string slice` alır çünkü parametrenin sahipliğini (ownership) almak zorunda değiliz. Örneğin, Liste 8-16’daki kodda, `s2`’nin içeriğini `s1`’e ekledikten sonra `s2`’yi hâlâ kullanabilmek isteriz.

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");
```

Liste 8-16: İçeriği bir `String`’e eklendikten sonra bir `string slice` kullanma

Eğer `push_str` metodu `s2`’nin sahipliğini alsaydı, son satırda değerini yazdıramazdık. Ancak bu kod beklediğimiz gibi çalışır!

`push` metodu bir `karakter` parametresi alır ve onu `String`’e ekler. Liste 8-17, `push` metodunu kullanarak bir `String`’e `l` harfi eklemektedir.

```rust
let mut s = String::from("lo");
s.push('l');
```

Liste 8-17: `push` kullanarak bir `String` değerine tek bir karakter ekleme

Sonuç olarak, `s` değeri `lol` olacaktır.

### 🔗 + Operatörü veya format! Makrosu ile Birleştirme

Çoğu zaman iki mevcut `string`’i birleştirmek isteriz. Bunun bir yolu `+` operatörünü kullanmaktır (Liste 8-18).

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // dikkat: s1 move edildi ve artık kullanılamaz
```

Liste 8-18: İki `String` değerini `+` operatörü ile yeni bir `String`’de birleştirme

Bu koddan sonra `s3`, `Hello, world!` değerini içerir. `s1`’in artık geçerli olmamasının ve `s2`’ye referans kullanmamızın nedeni, `+` operatörünü kullandığımızda çağrılan metodun imzasıyla ilgilidir. `+` operatörü `add` metodunu kullanır, ve bu metodun imzası aşağıdaki gibidir:

```rust
fn add(self, s: &str) -> String {
```

* İlk olarak, `s2`’nin önünde `&` vardır; yani ikinci `string`’in referansını birinciye ekliyoruz. Çünkü `add` fonksiyonundaki `s` parametresi yalnızca bir `&str` olabilir; iki `String` doğrudan toplanamaz. Burada derleyici, `&String` türünü otomatik olarak `&str`’e dönüştürür (deref coercion). Bu yüzden `&s2` çalışır ve `s2` hâlâ geçerli kalır.

* İkinci olarak, `self` parametresinde `&` yoktur; yani `s1` fonksiyona geçirilirken sahipliği devredilir (move edilir) ve artık geçerli değildir. Yani `let s3 = s1 + &s2;` ifadesi her iki `string`’i kopyalıyormuş gibi görünse de aslında öyle değildir; `s1`’in sahipliğini alır, `s2`’nin içeriğinin bir kopyasını ekler ve sonucu döndürür. Bu nedenle kopyalama gibi görünse de aslında daha verimlidir.

Birden fazla `string` birleştirmek istediğimizde `+` operatörünün kullanımı karmaşık hale gelir:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

Bu noktada `s`, `tic-tac-toe` değerini içerir. Ancak tüm `+` ve `" "` karakterleriyle kodun okunması zorlaşır. Daha karmaşık birleştirmeler için bunun yerine `format!` makrosunu kullanabiliriz:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

Bu kod da `s`’yi `tic-tac-toe` olarak ayarlar. `format!` makrosu `println!` gibi çalışır, ancak çıktıyı ekrana yazdırmak yerine bir `String` döndürür. `format!` kullanılan versiyon çok daha okunabilirdir ve parametrelerin sahipliğini almadığı için daha güvenlidir.
## 🔢 String’lerde (String) İndeksleme

Birçok programlama dilinde, bir `string` içindeki tek tek karakterlere indeks ile erişmek geçerli ve yaygın bir işlemdir. Ancak Rust’ta bir `String` üzerinde indeksleme söz dizimini kullanmaya çalışırsanız hata alırsınız. Aşağıdaki geçersiz kodu inceleyelim (Liste 8-19).

```rust
// Bu kod derlenmez!
let s1 = String::from("hi");
let h = s1[0];
```

Liste 8-19: Bir `String` üzerinde indeksleme söz dizimini kullanma girişimi

Bu kod şu hatayı verecektir:

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:16
  |
3 |     let h = s1[0];
  |                ^ string indices are ranges of `usize`
  |
  = note: you can use `.chars().nth()` or `.bytes().nth()`
          for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
          but trait `SliceIndex<[_]>` is implemented for `usize`
  = help: for that trait implementation, expected `[_]`, found `str`
  = note: required for `String` to implement `Index<{integer}>`

error: could not compile `collections` (bin "collections") due to 1 previous error
```

Hata ve not bize şunu anlatıyor: Rust `string`’ler indekslemeyi desteklemez. Peki neden? Bu soruyu cevaplamak için Rust’ın `string`’leri bellekte nasıl sakladığını tartışmamız gerekir.

---

### 🧩 İçsel Temsil (Internal Representation)

Bir `String`, bir `Vec<u8>` etrafında sarmalayıcıdır. Liste 8-14’teki UTF-8 ile düzgün kodlanmış örneklerden bazılarına bakalım.

```rust
let hello = String::from("Hola");
```

Bu durumda `len` değeri `4` olacaktır, yani `"Hola"` `string`’ini saklayan vektör 4 bayt uzunluğundadır. Her harf UTF-8 ile kodlandığında bir bayt alır. Ancak şu örnek sizi şaşırtabilir:

```rust
let hello = String::from("Здравствуйте");
```

Bu `string`’in uzunluğunu 12 sanabilirsiniz. Oysa Rust’ın cevabı `24` olacaktır; çünkü `"Здравствуйте"` UTF-8’de 24 bayt gerektirir. Bunun nedeni, bu `string`’deki her Unicode skaler değerinin (`Unicode scalar value`) 2 bayt yer kaplamasıdır. Bu yüzden baytlara indeksleme, her zaman geçerli bir Unicode skaler değerine karşılık gelmez.

Örneğin şu geçersiz Rust kodunu düşünelim:

```rust
// Bu kod derlenmez!
let hello = "Здравствуйте";
let answer = &hello[0];
```

`answer` kesinlikle ilk harf olan `З` olmayacaktır. UTF-8’de `З` harfi şu baytlarla kodlanır: 208 ve 151. Yani indeks `0`’dan elde edilen değer aslında 208’dir, ama tek başına 208 geçerli bir karakter değildir. Eğer bu kod geçerli olsaydı, kullanıcı beklenmeyen bir değer alırdı. Rust’ın bu kodu hiç derlememesi, böyle hataların erkenden önlenmesi için daha güvenlidir.

Hatta Latin harfleriyle bile aynı durum geçerlidir. Örneğin `&"hi"[0]` geçerli olsaydı, kullanıcı `"h"` yerine `104` (ASCII kodu) değerini alırdı.

---

### 🔠 Baytlar (Bytes), Skaler Değerler (Scalar Values) ve Harf Grupları (Grapheme Clusters)

UTF-8 ile ilgili bir diğer nokta, `string`’lere üç farklı açıdan bakabilmemizdir:

1. **Baytlar (bytes)**
2. **Skaler değerler (scalar values / `char` türü)**
3. **Harf grupları (grapheme clusters / insanların “harf” olarak algıladığı birimler)**

Örneğin Hintçe’deki **“नमस्ते”** kelimesine bakalım:

* **Baytlar (u8 vektörü olarak):**

```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 
 224, 165, 141, 224, 164, 164, 224, 165, 135]
```

Bu veri 18 bayttır ve bilgisayarın ham olarak sakladığı şeklidir.

* **Skaler değerler (Unicode `char`):**

```
['न', 'म', 'स', '्', 'त', 'े']
```

Burada 6 `char` vardır, ancak 4. ve 6. değer bağımsız harf değil, diakritik işaretlerdir.

* **Harf grupları (grapheme clusters):**

```
["न", "म", "स्", "ते"]
```

İşte insanların “नमस्ते” kelimesini 4 harf olarak algılamasının sebebi budur.

Rust, ham `string` verilerini farklı şekillerde yorumlamamıza izin verir. Böylece her program ihtiyacı olan yorumu seçebilir.

---

### ⚡ Neden String İndeksleme Yok?

Bir başka neden ise indeksleme işlemlerinden beklenen performansla ilgilidir. İndeksleme işlemleri genellikle sabit zamanda (O(1)) çalışmalıdır. Ancak `String` için bunu garanti etmek mümkün değildir çünkü indeksin hangi karaktere karşılık geldiğini anlamak için Rust’ın baştan itibaren `string`’i taraması gerekir.

Bu nedenle Rust, `String` üzerinde doğrudan indekslemeyi yasaklar ve bunun yerine `chars().nth()` veya `bytes().nth()` gibi güvenli yöntemler sunar.
## ✂️ String’leri (String) Dilimleme

Bir `string` üzerinde indeksleme genellikle kötü bir fikirdir çünkü indeksleme işleminin dönüş tipinin ne olması gerektiği belirsizdir: bir bayt değeri mi, bir karakter mi, bir harf grubu (grapheme cluster) mu, yoksa bir `string slice` mı? Bu nedenle, gerçekten indeks kullanarak `string slice` oluşturmanız gerekiyorsa, Rust sizden daha açık olmanızı ister.

Bir sayı ile `[]` kullanmak yerine, belirli baytları içeren bir `string slice` oluşturmak için bir aralık (range) kullanabilirsiniz:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Burada `s`, `string`’in ilk dört baytını içeren bir `&str` olacaktır. Daha önce her karakterin iki bayt olduğunu belirtmiştik; bu nedenle `s` değeri `Зд` olacaktır.

Eğer yalnızca bir karakterin baytlarının bir kısmını dilimlemeye çalışırsak, örneğin `&hello[0..1]` ile, Rust çalışma zamanında panik yapacaktır. Bu, bir vektörde geçersiz bir indekse erişmeye benzer bir durumdur:

```
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/collections`

thread 'main' panicked at src/main.rs:4:19:
byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Bu nedenle aralıklarla `string slice` oluştururken dikkatli olunmalıdır; aksi halde programınız çökebilir.

---

## 🔄 String’ler Üzerinde Yineleme (Iterating) Yöntemleri

`string` parçaları üzerinde işlem yapmanın en iyi yolu, karakter mi yoksa bayt mı istediğinizi açıkça belirtmektir.

* Tek tek Unicode skaler değerleri (`char`) için `chars` metodunu kullanın. Örneğin:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Bu kod şunu yazdırır:

```
З
д
```

* Alternatif olarak, `bytes` metodu her ham baytı döndürür. Bu bazı durumlarda daha uygun olabilir:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

Bu kod şu dört baytı yazdırır:

```
208
151
208
180
```

Ancak unutmayın: geçerli bir Unicode skaler değeri birden fazla bayttan oluşabilir.

Harf gruplarını (grapheme clusters), örneğin **Devanagari** alfabesinde olduğu gibi, `string`’lerden elde etmek karmaşıktır. Bu işlevsellik standart kütüphanede bulunmaz, ancak ihtiyacınız varsa **crates.io** üzerinde bu işlevi sağlayan kütüphaneler vardır.
Güzel çevirmişsin 👍 Metin hem teknik olarak doğru hem de akıcı bir Türkçe ile aktarılmış. Başlıklar, kod blokları ve açıklamalar da doğru şekilde korunmuş.

Dilersen bir sonraki bölüm olan **Hash Maps** kısmına geçebiliriz — ister ben sana İngilizce metni aktarabilirim, ister sen paylaşırsın; ben yine aynı formatta Türkçeye çevirebilirim. İstiyor musun ki ben sonraki kısmı getireyim?
