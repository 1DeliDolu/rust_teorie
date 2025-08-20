## 🔀 Kontrol Akışı (control flow)

Bir koşul doğru olduğunda belirli kodların çalıştırılması ve bir koşul doğru olduğu sürece bazı kodların tekrar tekrar çalıştırılması, çoğu programlama dilinde temel yapı taşlarıdır. Rust kodunun çalıştırılma akışını (flow of execution) kontrol etmenizi sağlayan en yaygın yapılar `if` ifadeleri (if expressions) ve döngülerdir (loops).

---

## ❓ if İfadeleri (if expressions)

Bir `if` ifadesi, koşullara bağlı olarak kodunuzu dallandırmanızı sağlar. Bir koşul belirtirsiniz ve “Eğer bu koşul sağlanıyorsa şu kod bloğunu çalıştır, sağlanmıyorsa çalıştırma” dersiniz.

Projeniz klasöründe `branches` adında yeni bir proje oluşturun ve `src/main.rs` dosyasına şu kodu yazın:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

Tüm `if` ifadeleri `if` anahtar sözcüğü ile başlar ve ardından bir koşul gelir. Bu örnekte, koşul `number` değişkeninin 5’ten küçük olup olmadığını kontrol ediyor. Koşul doğru olduğunda çalıştırılacak kod bloğunu hemen süslü parantezler içinde belirtiyoruz. `if` ifadelerindeki bu koşullara bağlı kod bloklarına bazen **kol (arm)** denir; bu terimi, 2. bölümde “Tahmini Gizli Sayı ile Karşılaştırmak” kısmında `match` ifadeleri için de görmüştük.

İsteğe bağlı olarak, koşul yanlış olduğunda çalıştırılacak alternatif bir blok sağlamak için `else` ifadesi ekleyebiliriz. Eğer `else` eklemezseniz ve koşul yanlış olursa, program `if` bloğunu atlar ve bir sonraki koda devam eder.

Bu kodu çalıştırmayı deneyin; çıktısı şöyle olmalıdır:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
condition was true
```

Şimdi `number` değerini, koşulu yanlış yapacak şekilde değiştirelim:

```rust
let number = 7;
```

Programı yeniden çalıştırın ve çıktıyı gözlemleyin:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
condition was false
```

---

## ⚠️ Koşulların Türü: bool

Burada önemli bir nokta, koşulun türünün `bool` olması gerektiğidir. Eğer koşul bir `bool` değilse, hata alırsınız. Örneğin şu kodu deneyin:

**Dosya adı:** `src/main.rs`

```rust
// Bu kod derlenmez!
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

Bu durumda `if` koşulu `3` değerine dönüşür ve Rust hata verir:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

Hata mesajı, Rust’ın bir `bool` beklediğini fakat bir tamsayı (integer) bulduğunu belirtir. Ruby ve JavaScript gibi dillerden farklı olarak, Rust `bool` olmayan türleri otomatik olarak `bool`’a dönüştürmez. Her zaman açıkça `if` için bir `bool` sağlamalısınız.

Eğer, örneğin, `if` kod bloğunun yalnızca sayı `0`’a eşit değilse çalışmasını istiyorsak, `if` ifadesini şu şekilde yazabiliriz:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

Bu kodu çalıştırmak şu çıktıyı verecektir:

```
number was something other than zero
```
 ## 🔢 else if ile Birden Fazla Koşul (handling multiple conditions with else if)

Birden fazla koşulu, `if` ve `else` yapısını `else if` ile birleştirerek kullanabilirsiniz. Örneğin:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Bu programın dört olası yolu vardır. Çalıştırıldığında şu çıktıyı göreceksiniz:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
number is divisible by 3
```

Program çalıştığında, sırayla her `if` ifadesini kontrol eder ve koşulu doğru olan ilk bloğu çalıştırır. Dikkat edin, 6 aynı zamanda 2’ye bölünebilmesine rağmen `number is divisible by 2` mesajını görmüyoruz; hatta `else` bloğundaki metin de çalışmıyor. Bunun nedeni, Rust’ın **sadece ilk doğru koşulun bloğunu çalıştırması** ve bulduktan sonra diğerlerini kontrol etmemesidir.

Çok fazla `else if` kullanmak kodunuzu karmaşıklaştırabilir. Eğer birden fazla koşulunuz varsa, kodunuzu yeniden düzenlemeyi düşünebilirsiniz. Bu tür durumlar için Rust’ın güçlü bir dallanma yapısı olan `match`, 6. bölümde anlatılacaktır.

---

## 📝 let İçinde if Kullanımı (using if in a let statement)

`if` bir ifade (expression) olduğundan, onu bir `let` deyiminin sağ tarafında kullanarak sonucunu bir değişkene atayabiliriz. Bu, Liste 3-2’de gösterilmiştir:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

**Liste 3-2:** Bir `if` ifadesinin sonucunu bir değişkene atamak

`number` değişkeni, `if` ifadesinin sonucuna göre bir değere bağlanır. Bu kodu çalıştırın ve sonucu görün:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
The value of number is: 5
```

Unutmayın: Kod blokları, içlerindeki son ifadeye dönüşür. Tek başına sayılar da birer ifadedir. Bu durumda, tüm `if` ifadesinin değeri, hangi bloğun çalıştırıldığına bağlıdır.

Bu, her kolun (`if` ve `else`) dönüş değerlerinin **aynı türde** olması gerektiği anlamına gelir. Liste 3-2’de hem `if` kolu hem de `else` kolu `i32` türünde değerler döndürmektedir.

Eğer türler uyumsuz olursa hata alırsınız. Örneğin:

**Dosya adı:** `src/main.rs`

```rust
// Bu kod derlenmez!
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```

Bu kodu derlediğinizde şu hatayı alırsınız:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this
```

`if` bloğundaki ifade bir tamsayıya (`i32`) dönüşürken, `else` bloğundaki ifade bir string’e (`&str`) dönüşmektedir. Bu çalışmaz, çünkü bir değişken yalnızca tek bir türe sahip olabilir ve Rust derleyici zamanı (compile time) sırasında bu türü kesin olarak bilmek zorundadır.

`number` değişkeninin türünü bilmek, derleyicinin her kullanım noktasında türün geçerli olup olmadığını doğrulamasına olanak tanır. Eğer tür ancak çalışma zamanında (runtime) belirlenmiş olsaydı, derleyici çok daha karmaşık hale gelir ve kod hakkında daha az garanti verebilirdi.
## 🔁 Döngülerle Tekrar (repetition with loops)

Bir kod bloğunu birden fazla kez çalıştırmak çoğu zaman faydalıdır. Bu görev için Rust, döngüler (loops) sağlar. Döngüler, gövdesindeki kodu çalıştırır, sona gelir ve hemen baştan tekrar etmeye başlar.

Bu yapıları denemek için `loops` adında yeni bir proje oluşturalım.

Rust’ta üç çeşit döngü vardır: `loop`, `while` ve `for`. Şimdi bunların her birini inceleyelim.

---

## ♾️ loop ile Kod Tekrarlama (repeating code with loop)

`loop` anahtar sözcüğü, Rust’a bir kod bloğunu sonsuza kadar tekrar etmesini veya açıkça durdurmasını söyleyene kadar çalıştırmasını bildirir.

Örneğin, `loops` klasörünüzdeki `src/main.rs` dosyasını şu şekilde değiştirin:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

Bu programı çalıştırdığımızda, `again!` çıktısını sürekli göreceğiz. Programı manuel olarak durdurana kadar devam eder. Çoğu terminal, sürekli döngüye takılan bir programı durdurmak için `ctrl-c` kısayolunu destekler. Deneyin:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

`^C` işareti, `ctrl-c` tuşlarına bastığınız yeri gösterir. Döngü hangi noktada kesme sinyalini aldıysa, `^C` işaretinden sonra `again!` ifadesini görüp görmemeniz değişebilir.

Neyse ki, Rust ayrıca döngüden çıkmak için kod düzeyinde bir yol sunar. Döngü içinde `break` anahtar sözcüğünü kullanarak programın ne zaman döngüyü durduracağını belirtebilirsiniz. Bunu, 2. bölümde tahmin oyununda kullanıcı doğru sayıyı bildiğinde programdan çıkmak için yapmıştık.

Ayrıca tahmin oyununda `continue` kullandık. `continue`, döngüde bu adımın kalan kısmını atlayıp bir sonraki yinelemeye geçmesini söyler.

---

## 🔙 Döngülerden Değer Döndürmek (returning values from loops)

Döngülerden bir kullanım şekli, başarısız olabileceğini bildiğiniz bir işlemi tekrar etmektir, örneğin bir iş parçacığının görevini bitirip bitirmediğini kontrol etmek. Bazen de bu işlemin sonucunu döngünün dışındaki koda iletmeniz gerekir.

Bunu yapmak için, `break` ifadesi ile döngüyü durdururken döndürmek istediğiniz değeri yazabilirsiniz. Bu değer, döngünün dışına aktarılır ve kullanılabilir. Örneğin:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

Bu örnekte:

* Döngüden önce `counter` adlı bir değişken tanımlar ve `0` ile başlatırız.
* Daha sonra, döngünün döndürdüğü değeri saklamak için `result` adında bir değişken tanımlarız.
* Döngünün her adımında `counter` değişkenini 1 artırırız.
* `counter` değeri `10` olduğunda, `break` ile birlikte `counter * 2` değerini döndürürüz.
* Döngüden çıkan bu değer, `result` değişkenine atanır.
* Son olarak `result` ekrana yazdırılır; bu örnekte sonuç `20` olacaktır.

Ayrıca, bir döngü içinden `return` kullanarak da çıkabilirsiniz. Fark şu ki:

* `break` sadece **mevcut döngüden** çıkar.
* `return` ise her zaman **mevcut fonksiyondan** çıkar.
## 🏷️ Döngü Etiketleri ile Birden Fazla Döngüyü Ayırt Etmek (loop labels to disambiguate between multiple loops)

İç içe döngüleriniz (nested loops) olduğunda, `break` ve `continue` komutları o anda bulunduğunuz en içteki döngüye uygulanır. Ancak, hangi döngünün durdurulacağını veya devam ettirileceğini açıkça belirtmek için döngülere **etiket (label)** verebilirsiniz.

Döngü etiketleri tek tırnak (`'`) ile başlar. İşte iki iç içe döngüye sahip bir örnek:

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

* Dıştaki döngü `'counting_up` etiketiyle işaretlenmiştir ve `count` değerini 0’dan 2’ye kadar artırır.
* Etiketsiz olan iç döngü ise 10’dan 9’a kadar geri sayar.
* Etiket belirtmeyen `break`, yalnızca **iç döngüden** çıkar.
* `break 'counting_up;` ifadesi ise **dış döngüyü** sonlandırır.

Çalıştırıldığında şu çıktıyı verir:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/loops`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

---

## 🔄 while ile Koşullu Döngüler (conditional loops with while)

Bir programın genellikle, döngü içinde bir koşulu değerlendirmesi gerekir. Koşul doğru olduğu sürece döngü çalışır; koşul yanlış olduğunda program `break` ile döngüyü durdurur.

Aslında, bu davranışı `loop`, `if`, `else` ve `break` kombinasyonu ile uygulayabilirsiniz. Ancak bu desen o kadar yaygındır ki, Rust bunun için özel bir yapı sunar: **while döngüsü**.

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

**Liste 3-3:** Bir koşul doğru olduğu sürece kodu çalıştırmak için `while` kullanmak

Bu yapı, `loop`, `if`, `else` ve `break` kullanmak zorunda kalındığında ortaya çıkan iç içe geçmiş (nested) yapıları ortadan kaldırır ve çok daha anlaşılırdır.

* Koşul doğru olduğu sürece kod çalışır.
* Koşul yanlış olduğunda döngüden çıkar.

Bu örnekte program `3, 2, 1` geri sayımı yapar ve ardından **“LIFTOFF!!!”** mesajını yazdırarak sona erer.
## 📚 Bir Koleksiyon Üzerinde for ile Döngü (looping through a collection with for)

Bir koleksiyonun (örneğin bir dizinin) elemanlarını döngüyle dolaşmak için `while` yapısını kullanabilirsiniz. Örneğin, Liste 3-4’teki döngü, dizideki `a` elemanlarının her birini yazdırır:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

**Liste 3-4:** Bir koleksiyonun elemanlarını `while` ile döngüde dolaşmak

Burada kod, dizinin elemanlarını sırayla sayarak dolaşır. 0. indexten başlar ve dizinin son elemanına kadar devam eder (yani `index < 5` artık doğru olmayana kadar). Bu kodu çalıştırdığınızda dizideki tüm elemanlar ekrana yazdırılır:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

Beklendiği gibi beş değerin tamamı terminalde görünür. `index` değeri 5’e ulaşsa da, döngü koşulu sayesinde program diziden altıncı bir değeri almaya çalışmaz.

Ancak bu yaklaşım hataya açık bir yöntemdir. Koşulu yanlış yazarsanız veya dizi boyutunu değiştirip koşulu güncellemeyi unutursanız, program hata verip panik yapabilir. Ayrıca bu yöntem yavaştır, çünkü derleyici her yinelemede dizinin sınırları içinde olup olmadığını kontrol eden çalışma zamanı kodu ekler.

---

## 🔄 for ile Daha Güvenli Döngü (for loop as an alternative)

Daha kısa ve güvenli bir alternatif olarak `for` döngüsünü kullanabilirsiniz. `for`, koleksiyondaki her bir eleman için kod çalıştırır. Aşağıdaki kod, Liste 3-5’te gösterilmiştir:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

**Liste 3-5:** Bir koleksiyonun elemanlarını `for` döngüsü ile dolaşmak

Bu kodu çalıştırdığınızda, Liste 3-4 ile aynı çıktıyı alırsınız. Daha da önemlisi, artık kod daha güvenli hale gelir ve dizinin sonuna geçip hata verme ya da elemanları eksik yazdırma ihtimali ortadan kalkar.

Ayrıca, `for` döngülerinden üretilen makine kodu daha verimli olabilir; çünkü her yinelemede `index` değeriyle dizinin uzunluğu karşılaştırılmak zorunda kalmaz.

`for` kullandığınızda, diziye eleman eklediğinizde veya çıkardığınızda başka hiçbir kodu değiştirmek zorunda kalmazsınız.

Güvenli ve kısa olmaları nedeniyle, Rust’ta en yaygın kullanılan döngü türü `for` döngüleridir.

---

## 🚀 Range ile Geri Sayım (countdown with for and range)

Bazı durumlarda kodu belirli sayıda çalıştırmak isteyebilirsiniz. Örneğin, Liste 3-3’te `while` ile geri sayım yapmıştık. Çoğu Rust geliştiricisi bunun için de `for` döngüsünü tercih eder. Bunun yolu, standart kütüphanenin sağladığı **Range** (aralık) yapısını kullanmaktır. `Range`, belirli bir sayıdan başlayıp diğerine kadar sıralı sayılar üretir.

Ayrıca `rev` metodu ile aralığı tersine çevirebiliriz. Örneğin:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

Bu kod, 3’ten 1’e doğru geri sayım yapar ve sonunda **“LIFTOFF!!!”** yazdırır.


## 📋 Özet (summary)

Başardınız! Bu bölüm oldukça kapsamlıydı: değişkenler (variables), skaler (scalar) ve bileşik (compound) veri türleri, fonksiyonlar (functions), yorumlar (comments), `if` ifadeleri (if expressions) ve döngüler (loops) hakkında öğrendiniz!

Bu bölümde ele alınan kavramları pekiştirmek için şu programları yazmayı deneyebilirsiniz:

* Fahrenheit ve Celsius arasında sıcaklık dönüştürme.
* `n`’inci Fibonacci sayısını üretme.
* “The Twelve Days of Christmas” adlı Noel şarkısının sözlerini, şarkıdaki tekrarları kullanarak yazdırma.

Hazır olduğunuzda, Rust’ta diğer programlama dillerinde yaygın olmayan bir kavrama geçeceğiz: **sahiplik (ownership)**.
