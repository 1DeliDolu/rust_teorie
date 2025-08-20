## 🛠️ Fonksiyonlar (functions)

Fonksiyonlar (functions), Rust kodunda çok yaygındır. Dilin en önemli fonksiyonlarından biri olan `main` fonksiyonunu, yani birçok programın giriş noktasını, zaten gördünüz. Ayrıca yeni fonksiyonlar tanımlamanıza olanak sağlayan `fn` anahtar sözcüğünü de gördünüz.

Rust kodu, fonksiyon ve değişken adları için geleneksel stil olarak `snake case` kullanır; yani tüm harfler küçük yazılır ve kelimeler alt çizgi ile ayrılır. İşte bir fonksiyon tanımına örnek içeren bir program:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Rust’ta bir fonksiyon tanımlamak için `fn` yazar, ardından fonksiyon adını ve bir parantez çifti eklersiniz. Süslü parantezler, derleyiciye fonksiyon gövdesinin nerede başlayıp nerede bittiğini belirtir.

Tanımladığımız herhangi bir fonksiyonu, adını yazıp parantez açıp kapatarak çağırabiliriz. `another_function` fonksiyonu programda tanımlandığı için, `main` fonksiyonu içerisinden çağrılabilir. Dikkat ederseniz `another_function`, kaynak kodda `main` fonksiyonundan sonra tanımlanmıştır; ancak Rust fonksiyonların nerede tanımlandığına değil, çağıranın görebileceği bir kapsamda tanımlanıp tanımlanmadığına önem verir.

Yeni bir ikili (binary) proje oluşturalım ve adını `functions` koyalım. `src/main.rs` dosyasına `another_function` örneğini yerleştirin ve çalıştırın. Şu çıktıyı görmelisiniz:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/functions`
Hello, world!
Another function.
```

Kod satırları, `main` fonksiyonunda göründükleri sırayla yürütülür. Önce “Hello, world!” mesajı yazdırılır, ardından `another_function` çağrılır ve mesajı yazdırılır.

---

## 🔑 Parametreler (parameters)

Fonksiyonlara parametreler (parameters) ekleyebiliriz. Parametreler, bir fonksiyonun imzasının (signature) parçası olan özel değişkenlerdir. Bir fonksiyon parametre aldığında, çağrıldığında ona somut değerler verebilirsiniz. Teknik olarak bu somut değerlere argüman (arguments) denir, ancak gündelik kullanımda insanlar genellikle parametre ve argüman sözcüklerini birbirinin yerine kullanır.

Aşağıdaki sürümde `another_function` fonksiyonuna bir parametre eklenmiştir:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

Bu programı çalıştırmayı deneyin; şu çıktıyı almalısınız:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.21s
     Running `target/debug/functions`
The value of x is: 5
```

`another_function` fonksiyonunun bildirimi, `x` adında bir parametreye sahiptir. `x` parametresinin türü `i32` olarak belirtilmiştir. `another_function` fonksiyonuna `5` değerini gönderdiğimizde, `println!` makrosu biçimlendirme dizesinde `x` için ayrılan süslü parantezli yere `5` değerini yerleştirir.

Fonksiyon imzalarında, her parametrenin türünü açıkça belirtmelisiniz. Bu, Rust’ın tasarımında kasıtlı bir tercihtir: fonksiyon tanımlarında tür açıklamaları zorunlu olduğundan, derleyicinin başka yerlerde türü çıkarmak için size nadiren ihtiyaç duyması gerekir. Ayrıca derleyici, fonksiyonun hangi türleri beklediğini bildiğinde daha faydalı hata mesajları verebilir.

Birden fazla parametre tanımlarken, parametre bildirimlerini virgül ile ayırmalısınız, örneğin:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

Bu örnek, `print_labeled_measurement` adında iki parametre alan bir fonksiyon oluşturur. İlk parametre `value` adında olup `i32` türündedir. İkinci parametre ise `unit_label` adında olup `char` türündedir. Fonksiyon, `value` ve `unit_label` değerlerini içeren bir metin yazdırır.

Şimdi bu kodu çalıştırmayı deneyelim. `functions` projenizdeki `src/main.rs` dosyasını yukarıdaki örnekle değiştirin ve `cargo run` komutuyla çalıştırın:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/functions`
The measurement is: 5h
```

`print_labeled_measurement` fonksiyonunu `value` için `5` ve `unit_label` için `'h'` argümanlarıyla çağırdığımız için, program çıktısında bu değerler yer alır.
## 📜 İfadeler ve Deyimler (statements and expressions)

Fonksiyon gövdeleri (function bodies), bir dizi deyimden (statements) oluşur ve isteğe bağlı olarak bir ifadeyle (expression) sona erebilir. Şimdiye kadar ele aldığımız fonksiyonlarda bitişte bir ifade yoktu, ancak bir deyimin parçası olarak bir ifade gördünüz. Rust ifade-temelli (expression-based) bir dil olduğundan, bu ayrımı anlamak önemlidir. Diğer dillerde bu ayrım aynı şekilde bulunmaz; bu yüzden deyimlerin ve ifadelerin ne olduğuna ve aralarındaki farkların fonksiyon gövdelerini nasıl etkilediğine bakalım.

* **Deyimler (statements):** Bir eylem gerçekleştiren, fakat değer döndürmeyen talimatlardır.
* **İfadeler (expressions):** Bir değere dönüşen yapılardır.

---

## 📝 Deyim Örnekleri

Aslında biz deyim ve ifadeleri çoktan kullandık. `let` anahtar sözcüğüyle bir değişken oluşturmak ve ona değer atamak bir deyimdir. Liste 3-1’deki `let y = 6;` bir deyimdir.

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let y = 6;
}
```

**Liste 3-1:** Tek bir deyim içeren bir `main` fonksiyonu bildirimi

Fonksiyon tanımlamaları da birer deyimdir; yukarıdaki örneğin tamamı aslında başlı başına bir deyimdir. (Aşağıda göreceğimiz gibi, bir fonksiyonu çağırmak ise deyim değildir.)

Deyimler değer döndürmez. Bu nedenle, bir `let` deyimini başka bir değişkene atayamazsınız. Aşağıdaki kodun yaptığı gibi denerseniz hata alırsınız:

**Dosya adı:** `src/main.rs`

```rust
// Bu kod derlenmez!
fn main() {
    let x = (let y = 6);
}
```

Bu programı çalıştırmaya çalıştığınızda şu hatayı alırsınız:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: only supported directly in conditions of `if` and `while` expressions
```

`let y = 6` deyimi bir değer döndürmez; dolayısıyla `x`’in bağlanacağı bir şey yoktur. Bu, C veya Ruby gibi dillerden farklıdır. Bu dillerde atama işlemi, atanan değeri döndürür. Örneğin `x = y = 6` yazabilir ve hem `x` hem de `y` değerini `6` olarak alır. Rust’ta bu geçerli değildir.

---

## 🔢 İfade Örnekleri

İfadeler bir değere dönüşür ve Rust’ta yazacağınız kodların çoğunu oluşturur. Örneğin, `5 + 6` ifadesi `11` değerine dönüşür.

İfadeler deyimlerin parçası olabilir: Liste 3-1’deki `let y = 6;` deyiminde `6`, `6` değerine dönüşen bir ifadedir.

* Bir fonksiyon çağırmak ifadedir.
* Bir makro çağırmak ifadedir.
* Süslü parantezlerle oluşturulan yeni bir kapsam (scope) bloğu da ifadedir.

Örneğin:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

Bu ifade:

```rust
{
    let x = 3;
    x + 1
}
```

bir bloktur ve bu durumda `4` değerine dönüşür. Bu değer, `let` deyiminin parçası olarak `y` değişkenine bağlanır.

Dikkat edin, `x + 1` satırının sonunda noktalı virgül (`;`) yoktur. Bu, şimdiye kadar gördüğünüz çoğu satırdan farklıdır. İfadelerin sonunda noktalı virgül bulunmaz. Bir ifadenin sonuna noktalı virgül eklerseniz, onu bir deyime dönüştürmüş olursunuz ve artık bir değer döndürmez.

Bunu, fonksiyon dönüş değerlerini ve ifadeleri incelerken aklınızda tutmalısınız.
## 🔄 Dönüş Değerine Sahip Fonksiyonlar (functions with return values)

Fonksiyonlar (functions), kendilerini çağıran koda değer döndürebilir. Dönüş değerlerine bir isim vermeyiz, ancak türünü `->` işaretinden sonra belirtmemiz gerekir. Rust’ta bir fonksiyonun dönüş değeri, fonksiyon gövdesindeki bloğun son ifadesinin (expression) değeri ile aynıdır.

Bir fonksiyondan erken dönmek için `return` anahtar sözcüğünü ve bir değer belirtebilirsiniz, ancak çoğu fonksiyon, son ifadeyi dolaylı olarak (implicitly) döndürür. İşte bir değer döndüren fonksiyona örnek:

**Dosya adı:** `src/main.rs`

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

`five` fonksiyonunda fonksiyon çağrısı, makro ya da `let` deyimi yok—sadece tek başına `5` var. Bu, Rust’ta tamamen geçerli bir fonksiyondur. Dikkat edin, fonksiyonun dönüş türü de `-> i32` şeklinde belirtilmiştir.

Bu kodu çalıştırdığınızda şu çıktıyı almalısınız:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/functions`
The value of x is: 5
```

Buradaki `5`, `five` fonksiyonunun dönüş değeridir; bu yüzden dönüş türü `i32` olarak belirtilmiştir.

Bunu biraz daha detaylı inceleyelim:

1. `let x = five();` satırı, bir değişkeni başlatmak için bir fonksiyonun dönüş değerini kullandığımızı gösteriyor. `five` fonksiyonu `5` döndürdüğü için, bu satır aslında şununla aynıdır:

   ```rust
   let x = 5;
   ```

2. `five` fonksiyonu parametre almaz ve dönüş türünü belirtir; fakat gövdesinde sadece `5` vardır ve sonunda noktalı virgül (`;`) yoktur. Çünkü bu, döndürmek istediğimiz bir ifadedir.

---

## ➕ Başka Bir Örnek

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Bu kodu çalıştırdığınızda çıktı şu olur:

```
The value of x is: 6
```

Ama eğer `x + 1` satırının sonuna bir noktalı virgül (`;`) koyarsak, yani ifadeyi bir deyime çevirirsek, hata alırız:

**Dosya adı:** `src/main.rs`

```rust
// Bu kod derlenmez!
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

Bu kodu derlediğinizde şu hatayı görürsünüz:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value
```

Ana hata mesajı `mismatched types` (eşleşmeyen türler) sorunun kaynağını gösteriyor. `plus_one` fonksiyonunun tanımı, bir `i32` döneceğini söylüyor. Ancak deyimler bir değer üretmez; bunun yerine `()` (birim tür, unit type) ile ifade edilir. Bu nedenle fonksiyon aslında hiçbir şey döndürmemektedir, bu da fonksiyon tanımıyla çelişir ve hataya yol açar.

Çıktıda Rust ayrıca sorunu düzeltmeye yönelik bir öneri de sunar: noktalı virgülü kaldırmanızı önerir. Bu değişiklik hatayı ortadan kaldıracaktır.
