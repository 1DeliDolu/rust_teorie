## 📚 Ortak Programlama Kavramları (common programming concepts)

Bu bölüm, neredeyse her programlama dilinde bulunan kavramları ve bunların Rust içindeki işleyişini kapsar. Çoğu programlama dili özünde pek çok ortak özelliğe sahiptir. Bu bölümde ele alınan kavramların hiçbiri Rust’a özgü değildir, ancak biz bunları Rust bağlamında tartışacak ve bu kavramları kullanmaya dair gelenekleri açıklayacağız.

Özellikle değişkenler (variables), temel türler (basic types), fonksiyonlar (functions), yorumlar (comments) ve kontrol akışı (control flow) hakkında bilgi edineceksiniz. Bu temeller her Rust programında bulunur ve bunları erken öğrenmek, güçlü bir başlangıç yapmanızı sağlayacaktır.

## 🔑 Anahtar Sözcükler (keywords)

Rust dili, yalnızca dilin kendisi için ayrılmış bir anahtar sözcükler (keywords) kümesine sahiptir; bu durum diğer dillerde olduğu gibidir. Bu sözcükleri değişken (variable) veya fonksiyon (function) adı olarak kullanamayacağınızı unutmayın. Anahtar sözcüklerin çoğu özel anlamlara sahiptir ve Rust programlarınızda çeşitli görevleri yerine getirmek için bunları kullanacaksınız. Bazılarının ise henüz bir işlevi yoktur, ancak gelecekte Rust’a eklenebilecek özellikler için ayrılmıştır. Anahtar sözcüklerin listesini Ek A’da (Appendix A) bulabilirsiniz.


## 🔄 Değişkenler ve Değiştirilebilirlik (variables and mutability)

“Değerleri Değişkenlerde Saklamak” (storing values with variables) bölümünde bahsedildiği gibi, varsayılan olarak değişkenler (variables) değiştirilemezdir (immutable). Bu, Rust’ın sunduğu güvenlikten ve kolay eşzamanlılıktan (concurrency) yararlanarak kod yazmanız için size verdiği birçok küçük yönlendirmeden biridir. Ancak, değişkenlerinizi değiştirilebilir (mutable) hale getirme seçeneğiniz de vardır. Şimdi Rust’ın neden değiştirilemezliği tercih etmenizi teşvik ettiğini ve neden bazen bundan vazgeçmek isteyebileceğinizi inceleyelim.

Bir değişken değiştirilemez (immutable) olduğunda, bir değerin bir isme bağlanmasından sonra o değeri değiştiremezsiniz. Bunu göstermek için `projects` dizininizde `cargo new variables` komutunu kullanarak `variables` adında yeni bir proje oluşturun.

Ardından yeni `variables` dizininizde `src/main.rs` dosyasını açın ve içeriğini aşağıdaki kodla değiştirin. Bu kod henüz derlenmeyecektir:

**Dosya adı: src/main.rs**

```rust
// Bu kod derlenmez!
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

Programı `cargo run` komutuyla kaydedip çalıştırın. Aşağıdaki çıktıda gösterildiği gibi bir değiştirilemezlik (immutability) hatası alacaksınız:

```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++
```

```
For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to 1 previous error
```

Bu örnek, derleyicinin (compiler) programlarınızdaki hataları bulmanıza nasıl yardımcı olduğunu gösteriyor. Derleyici hataları can sıkıcı olabilir, ancak aslında bu, programınızın güvenli bir şekilde yapmak istediğiniz şeyi henüz yapmadığı anlamına gelir; iyi bir programcı olmadığınız anlamına gelmez! Deneyimli Rust geliştiricileri (Rustaceans) bile derleyici hataları alır.

`cannot assign twice to immutable variable x` hatasını aldınız çünkü değiştirilemez (immutable) `x` değişkenine ikinci bir değer atamaya çalıştınız.

Değiştirilemez olarak belirlenmiş bir değeri değiştirmeye çalıştığımızda derleme zamanı (compile-time) hatası almamız önemlidir çünkü bu durum hatalara yol açabilir. Kodumuzun bir kısmı bir değerin asla değişmeyeceğini varsayarken, başka bir kısmı bu değeri değiştirirse, ilk kısmın tasarlandığı gibi çalışmama ihtimali vardır. Bu tür bir hatanın nedenini sonradan bulmak zor olabilir, özellikle de ikinci kod parçası değeri yalnızca bazen değiştirdiğinde. Rust derleyicisi, bir değerin değişmeyeceğini belirttiğinizde gerçekten değişmeyeceğini garanti eder, bu yüzden bunu kendiniz takip etmek zorunda kalmazsınız. Kodunuzu anlamak böylece daha kolay hale gelir.

Ancak değiştirilebilirlik (mutability) çok faydalı olabilir ve kod yazmayı daha elverişli hale getirebilir. Varsayılan olarak değişkenler değiştirilemez olsa da, değişken adının önüne `mut` ekleyerek onları değiştirilebilir hale getirebilirsiniz. `mut` eklemek, aynı zamanda kodun gelecekteki okuyucularına bu değişkenin değerinin diğer bölümler tarafından değiştirileceğini de gösterir.

Örneğin, `src/main.rs` dosyasını şu şekilde değiştirelim:

**Dosya adı: src/main.rs**

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

Şimdi programı çalıştırdığımızda şu çıktıyı alırız:

```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

`mut` kullanıldığında `x` değişkenine bağlı değeri 5’ten 6’ya değiştirmemize izin verilir. Sonuç olarak, değiştirilebilirlik (mutability) kullanıp kullanmamaya karar vermek size bağlıdır ve bu karar, belirli bir durumda hangisinin daha anlaşılır olduğuna göre değişir.


## 🏷️ Sabitler (constants)

Değiştirilemez (immutable) değişkenler gibi, sabitler (constants) de bir isme bağlanan ve değiştirilemeyen değerlerdir, ancak sabitler ile değişkenler arasında birkaç fark vardır.

İlk olarak, sabitlerle `mut` kullanmanıza izin verilmez. Sabitler sadece varsayılan olarak değiştirilemez değildir — her zaman değiştirilemezdir. Sabitleri tanımlarken `let` anahtar sözcüğü yerine `const` anahtar sözcüğünü kullanırsınız ve değerin türü (type) mutlaka belirtilmelidir (annotated). Türler ve tür açıklamalarını (type annotations) bir sonraki bölüm olan “Veri Türleri”nde (data types) ele alacağız, bu yüzden şimdilik ayrıntılar hakkında endişelenmeyin. Şunu bilmeniz yeterlidir: her zaman türü belirtmeniz gerekir.

Sabitler herhangi bir kapsamda (scope), küresel kapsam (global scope) dahil olmak üzere, tanımlanabilir; bu da onları kodun birçok bölümünün bilmesi gereken değerler için faydalı hale getirir.

Son fark ise sabitlerin yalnızca sabit bir ifadeye (constant expression) atanabilmesidir; yani yalnızca çalışma zamanında (runtime) hesaplanabilecek bir değere atanamazlar.

İşte bir sabit tanımlamasına örnek:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Bu sabitin adı `THREE_HOURS_IN_SECONDS` ve değeri, 60 (bir dakikadaki saniye sayısı) ile 60 (bir saatteki dakika sayısı) ve 3 (bu programda saymak istediğimiz saat sayısı) çarpılarak elde edilen sonuca ayarlanmıştır. Rust’taki sabitler için adlandırma kuralı, tüm harflerin büyük olması ve kelimeler arasında alt çizgi kullanılmasıdır. Derleyici (compiler), sınırlı bir işlem kümesini derleme zamanında (compile time) değerlendirebilir; bu da, bu sabiti 10.800 değerine ayarlamak yerine, değeri daha anlaşılır ve doğrulanabilir bir şekilde yazmamıza olanak tanır. Sabitlerin bildirilmesinde hangi işlemlerin kullanılabileceği hakkında daha fazla bilgi için Rust Referansı’ndaki (Rust Reference) sabit değerlendirme (constant evaluation) bölümüne bakabilirsiniz.

Sabitler, bildirildikleri kapsam içinde, bir program çalıştığı sürece geçerlidir. Bu özellik, sabitleri, bir oyunda herhangi bir oyuncunun kazanmasına izin verilen maksimum puan sayısı veya ışık hızı gibi, programın birden fazla bölümünün bilmesi gereken uygulama alanı değerleri için faydalı hale getirir.

Programınız boyunca kullanılan sabit değerlerin (hardcoded values) sabitler olarak adlandırılması, bu değerin anlamını gelecekte kodu sürdürecek kişilere aktarmada faydalıdır. Ayrıca, gelecekte bu sabit değerin güncellenmesi gerekirse, kodunuzda yalnızca tek bir yerde değişiklik yapmanız yeterli olur.


## 🌑 Gölgeleme (shadowing)

2. bölümdeki tahmin oyunu (guessing game) öğreticisinde gördüğünüz gibi, önceki bir değişkenle aynı ada sahip yeni bir değişken tanımlayabilirsiniz. Rust geliştiricileri (Rustaceans), ilk değişkenin ikinci tarafından gölgelendiğini (shadowed) söyler. Bu, derleyicinin (compiler) değişkenin adını kullandığınızda ikinci değişkeni göreceği anlamına gelir. Etkili bir şekilde ikinci değişken ilki üzerine “örtülür” ve kendisi gölgelenene ya da kapsam (scope) bitene kadar tüm değişken adı kullanımlarını üzerine alır. Bir değişkeni gölgelemenin yolu, aynı değişken adını kullanmak ve `let` anahtar sözcüğünü tekrarlamaktır:

**Dosya adı: src/main.rs**

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

Bu program önce `x` değerini 5’e bağlar. Sonra `let x =` ifadesini tekrarlayarak yeni bir `x` oluşturur, orijinal değere 1 ekler ve böylece `x` artık 6 olur. Ardından süslü parantezlerle oluşturulmuş iç kapsamda (inner scope), üçüncü `let` ifadesi de `x`’i gölgeler ve önceki değeri 2 ile çarparak `x`’e 12 değerini verir. O kapsam sona erdiğinde, iç gölgeleme biter ve `x` tekrar 6 olur. Programı çalıştırdığımızda şu çıktıyı alırız:

```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

Gölgeleme, bir değişkeni `mut` olarak işaretlemekten farklıdır çünkü `let` anahtar sözcüğünü kullanmadan yanlışlıkla yeniden atama yapmaya çalışırsak derleme zamanı hatası alırız. `let` kullanarak bir değer üzerinde birkaç dönüşüm gerçekleştirebiliriz, ancak bu dönüşümler tamamlandıktan sonra değişken değiştirilemez (immutable) olarak kalır.

`mut` ile gölgeleme arasındaki diğer fark, `let` anahtar sözcüğünü tekrar kullandığımızda aslında yeni bir değişken oluşturduğumuz için değerin türünü (type) değiştirebilmemizdir. Ancak yine de aynı adı kullanabiliriz. Örneğin, programımız bir kullanıcıdan bazı metinler arasına kaç boşluk bırakmak istediğini boşluk karakterleri girerek belirtmesini isterse ve biz bu girdiyi daha sonra sayı olarak saklamak istersek:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

İlk `spaces` değişkeni bir `string` türüdür, ikinci `spaces` değişkeni ise bir `number` türüdür. Böylece gölgeleme, `spaces_str` ve `spaces_num` gibi farklı adlar bulma zorunluluğunu ortadan kaldırır; bunun yerine daha basit olan `spaces` adını tekrar kullanabiliriz. Ancak bunu `mut` ile yapmaya çalışırsak derleme zamanı hatası alırız:

```rust
// Bu kod derlenmez!
let mut spaces = "   ";
spaces = spaces.len();
```

Hata, bir değişkenin türünü değiştirmemize izin verilmediğini söyler:

```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`
```

```
For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` (bin "variables") due to 1 previous error
```

Artık değişkenlerin nasıl çalıştığını incelediğimize göre, sahip olabilecekleri farklı veri türlerine (data types) göz atalım.
