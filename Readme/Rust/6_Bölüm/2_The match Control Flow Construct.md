## 🔀 match Kontrol Akışı Yapısı (control flow construct)

Rust, bir değeri bir dizi desen (pattern) ile karşılaştırmanıza ve hangi desen eşleşirse ona göre kod çalıştırmanıza olanak tanıyan son derece güçlü bir kontrol akışı yapısı olan `match` anahtar sözcüğüne sahiptir. Desenler (patterns), sabit değerlerden, değişken adlarından, joker karakterlerden (wildcards) ve daha birçok şeyden oluşabilir; tüm farklı desen türleri ve işlevleri Bölüm 19’da ele alınmaktadır. `match` ifadesinin gücü, desenlerin ifade gücünden ve derleyicinin tüm olası durumların ele alındığını doğrulamasından gelir.

Bir `match` ifadesini, sikkeleri boyutlarına göre ayıran bir makineye benzetebilirsiniz: Sikkeler bir ray boyunca kayar, farklı boyutlardaki deliklerin üzerinden geçer ve uygun olan ilk delikten düşer. Aynı şekilde, değerler de bir `match` ifadesindeki desenlerin üzerinden geçer ve “uyduğu” ilk desenin içine girer, ardından o desene bağlı kod bloğu yürütülür.

Sikkelerden bahsetmişken, hadi onları `match` için örnek olarak kullanalım! Bilinmeyen bir Amerikan sikkesini alan, aynı ayırma makinesi gibi hangi sikke olduğunu belirleyen ve değerini cent cinsinden döndüren bir fonksiyon yazabiliriz. Bu, Liste 6-3’te gösterilmiştir.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Liste 6-3: Bir `enum` ve `enum` varyantlarını desen olarak kullanan bir `match` ifadesi

Şimdi `value_in_cents` fonksiyonundaki `match` yapısını inceleyelim. Önce `match` anahtar sözcüğünü ve ardından bir ifade yazarız, bu durumda ifade `coin` değeridir. Bu, `if` ile kullanılan koşullu ifadelere benzer görünmektedir, ancak büyük bir fark vardır: `if` koşulunun Boolean bir değer döndürmesi gerekirken, `match` ifadesi herhangi bir türle çalışabilir. Buradaki `coin` türü, ilk satırda tanımladığımız `Coin` enum’udur.

Sonrasında `match` kolları (arms) gelir. Bir kol (arm) iki bölümden oluşur: bir desen (pattern) ve çalıştırılacak kod. İlk kolda desen `Coin::Penny` değeridir, ardından deseni çalıştırılacak koddan ayıran `=>` operatörü bulunur. Bu durumda çalıştırılacak kod yalnızca `1` değeridir. Her kol bir sonraki koldan virgül ile ayrılır.

Bir `match` ifadesi çalıştırıldığında, ortaya çıkan değer sırayla her kolun deseniyle karşılaştırılır. Bir desen değere uyarsa, o desenle ilişkili kod çalıştırılır. Eğer desen eşleşmezse, çalıştırma bir sonraki kola devam eder; tıpkı sikkeleri ayıran makinedeki gibi. İstediğimiz kadar kol ekleyebiliriz: Liste 6-3’teki `match` ifadesinin dört kolu vardır.

Her kolun kodu bir ifadedir ve eşleşen koldaki ifadenin döndürdüğü değer, tüm `match` ifadesinin sonucu olur.

Kısaysa genellikle `match` kolunda süslü parantezler kullanmayız, tıpkı Liste 6-3’te her kolun yalnızca bir değer döndürmesi gibi. Ancak bir `match` kolunda birden fazla satır kod çalıştırmak isterseniz süslü parantezler kullanmanız gerekir; bu durumda koldan sonra gelen virgül isteğe bağlıdır. Örneğin, aşağıdaki kod, her `Coin::Penny` çağrıldığında “Lucky penny!” yazdırır, ancak bloktaki son değer olan `1` yine döndürülür:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
## 🧩 Değerlere Bağlanan Desenler (patterns that bind to values)

`match` kollarının (arms) bir başka faydalı özelliği, eşleşen desenin değerlerinin bir kısmına bağlanabilmeleridir. Bu, `enum` varyantlarından değer çıkarmamızı sağlar.

Örneğin, `enum` varyantlarımızdan birini içine veri alacak şekilde değiştirelim. 1999’dan 2008’e kadar Amerika Birleşik Devletleri, 50 eyaletin her biri için farklı tasarımlara sahip çeyrek (quarter) sikkeler bastı. Diğer hiçbir sikke eyalet tasarımı almadı, dolayısıyla yalnızca çeyreklerin bu ek değeri vardır. Bu bilgiyi `enum` içine eklemek için, Liste 6-4’te yaptığımız gibi `Quarter` varyantını içinde bir `UsState` değeri saklayacak şekilde değiştirebiliriz.

```rust
#[derive(Debug)] // state değerini inceleyebilmek için
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

Liste 6-4: `Quarter` varyantının ayrıca bir `UsState` değeri taşıdığı bir `Coin` enum’u

Diyelim ki bir arkadaşımız 50 eyaletin tüm çeyreklerini toplamaya çalışıyor. Biz bozuk paraları türlerine göre ayırırken, her çeyrek için eyalet adını da söyleyeceğiz ki eğer arkadaşımızda yoksa koleksiyonuna ekleyebilsin.

Bu kodun `match` ifadesinde, `Coin::Quarter` varyantı ile eşleşen desene `state` adlı bir değişken ekliyoruz. Bir `Coin::Quarter` eşleştiğinde, `state` değişkeni o çeyreğin eyalet değerine bağlanır. Ardından bu değişkeni o kola ait kodda kullanabiliriz:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

Eğer `value_in_cents(Coin::Quarter(UsState::Alaska))` çağrısını yapsaydık, `coin` değeri `Coin::Quarter(UsState::Alaska)` olurdu. Bu değer, `match` kollarıyla karşılaştırıldığında, `Coin::Quarter(state)` desenine ulaşana kadar hiçbirine uymaz. O noktada `state` değişkeni `UsState::Alaska` değerine bağlanır. Daha sonra bu bağı `println!` ifadesinde kullanabiliriz ve böylece `Quarter` varyantının içindeki `UsState` değerini almış oluruz.
## 🎯 Option<T> ile Eşleştirme (matching with Option<T>)

Önceki bölümde, `Option<T>` kullanırken `Some` durumunun içindeki `T` değerini almak istemiştik; `Option<T>`’yi de `match` ile ele alabiliriz, tıpkı `Coin` enum’unda yaptığımız gibi! Bu kez sikkeleri karşılaştırmak yerine `Option<T>` varyantlarını karşılaştıracağız, fakat `match` ifadesinin çalışma şekli aynı kalır.

Diyelim ki bir `Option<i32>` alan ve eğer içinde bir değer varsa bu değere `1` ekleyen bir fonksiyon yazmak istiyoruz. Eğer içinde bir değer yoksa, fonksiyon `None` döndürmeli ve hiçbir işlem yapmaya çalışmamalıdır.

`match` sayesinde bu fonksiyon çok kolay yazılır ve Liste 6-5’teki gibi görünür:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Liste 6-5: `Option<i32>` üzerinde `match` ifadesi kullanan bir fonksiyon

Şimdi `plus_one` fonksiyonunun ilk çalıştırılmasını daha detaylı inceleyelim. `plus_one(five)` çağırdığımızda, `plus_one` fonksiyonunun gövdesindeki `x` değişkeni `Some(5)` değerine sahip olur. Ardından bu değeri her bir `match` kolu ile karşılaştırırız:

```rust
None => None,
```

`Some(5)` değeri `None` deseniyle eşleşmez, bu yüzden bir sonraki kola geçilir:

```rust
Some(i) => Some(i + 1),
```

`Some(5)` değeri `Some(i)` ile eşleşir mi? Evet! Varyantlar aynıdır. `i`, `Some` içindeki değere bağlanır, yani `i = 5` olur. Ardından kola ait kod çalıştırılır; `i` değerine 1 eklenir ve toplam `6` içeren yeni bir `Some` oluşturulur.

Şimdi Liste 6-5’teki ikinci çağrıyı düşünelim: `plus_one(None)`. Bu durumda `x`, `None`’dur. `match` içine girildiğinde ilk kola bakılır:

```rust
None => None,
```

Eşleşme gerçekleşir! Eklenebilecek bir değer yoktur, bu yüzden program durur ve `=>`’nun sağındaki `None` döndürülür. İlk kol eşleştiği için diğer kollar artık kontrol edilmez.

`match` ve `enum`’ları birleştirmek birçok durumda faydalıdır. Bu deseni Rust kodlarında çok sık göreceksiniz: bir `enum` üzerinde `match` yapmak, içteki veriye bir değişken bağlamak ve buna göre kod çalıştırmak. İlk başta biraz karışık gelse de alıştığınızda, bu özelliği tüm dillerde olsun isteyeceksiniz. Rust kullanıcılarının favorilerinden biridir.
## ✅ Eşleşmeler Tüketicidir (matches are exhaustive)

`match` hakkında konuşmamız gereken bir diğer nokta, kolların (arms) desenlerinin tüm olasılıkları kapsaması gerektiğidir. Örneğin, aşağıdaki hatalı `plus_one` fonksiyonunu ele alalım; bu kod derlenmez:

```rust
// Bu kod derlenmez!
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

Burada `None` durumu ele alınmamıştır, bu yüzden kod hata verir. Neyse ki Rust bu hatayı yakalamayı bilir. Bu kodu derlemeye çalıştığımızda şu hatayı alırız:

```
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
 --> src/main.rs:3:15
  |
3 |         match x {
  |               ^ pattern `None` not covered
  |
note: `Option<i32>` defined here
 --> /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library/core/src/option.rs:572:1
 ::: /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library/core/src/option.rs:576:5
  |
  = note: not covered
  = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
4 ~             Some(i) => Some(i + 1),
5 ~             None => todo!(),
  |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` (bin "enums") due to 1 previous error
```

Rust, tüm olasılıkları kapsamadığımızı ve unuttuğumuz deseni (`None`) bile bildiğimizi söylüyor! Rust’taki `match` ifadeleri **tüketicidir (exhaustive)**: Kodun geçerli olabilmesi için her ihtimalin kapsanmış olması gerekir. Özellikle `Option<T>` durumunda, Rust bizim `None` durumunu açıkça ele almayı unutmamıza izin vermez ve böylece elimizde değer varmış gibi davranmamızı önler. Bu sayede, daha önce bahsedilen milyar dolarlık hata (null kullanımı hatası) imkânsız hale gelir.

## 🎭 Genel Desenler ve `_` Yer Tutucusu (catch-all patterns and the \_ placeholder)

`enum` kullanırken, bazı özel değerler için özel işlemler yapabilir, diğer tüm değerler için ise tek bir varsayılan işlem belirleyebiliriz. Örneğin, bir oyun yazdığımızı hayal edelim: Eğer zar atışında **3** gelirse oyuncu ilerlemez, bunun yerine yeni bir şapka kazanır. Eğer **7** gelirse, oyuncu bir şapkasını kaybeder. Diğer tüm değerlerde ise oyuncu, zar sayısı kadar kare ilerler. Bu mantığı uygulayan `match` ifadesi aşağıdaki gibidir (zar sonucu rastgele değil, sabit verilmiştir; diğer fonksiyonların gövdeleri boş bırakılmıştır):

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

İlk iki kolda desenler sabit değerlerdir: `3` ve `7`. Son kolda ise her diğer olası değeri kapsayan desen, `other` adlı değişkendir. Bu değer `move_player` fonksiyonuna parametre olarak gönderilerek kullanılır.

Bu kod derlenir, çünkü `u8` tipinin alabileceği tüm olası değerler tek tek listelenmemiş olsa da, son koldaki genel desen tüm değerleri kapsar. Böylece `match` ifadesinin **tüketici (exhaustive)** olma şartı sağlanır. Dikkat edilmesi gereken bir nokta, genel deseni en sona koymamız gerektiğidir; çünkü desenler sırayla değerlendirilir. Eğer genel deseni en başa koysaydık, diğer kollar asla çalıştırılamazdı. Rust, genel desenin ardından yeni kollar eklemeye kalkarsak bizi bu konuda uyarır.

Rust ayrıca, değeri kullanmak istemediğimiz durumlar için özel bir desen sunar: `_`. Bu özel desen, herhangi bir değerle eşleşir ama o değeri bir değişkene bağlamaz. Bu şekilde Rust’a, bu değeri kullanmayacağımızı söylemiş oluruz, dolayısıyla “kullanılmayan değişken” uyarısı almayız.

Şimdi oyunun kurallarını değiştirelim: Eğer zar atışında 3 veya 7 dışında bir şey gelirse, oyuncu tekrar zar atmalıdır. Bu durumda, artık genel desendeki değeri kullanmamıza gerek yoktur. Kodumuzu `other` yerine `_` kullanacak şekilde değiştirebiliriz:

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

Bu örnekte de **tüketicilik (exhaustiveness)** şartı sağlanır çünkü diğer tüm değerleri bilinçli olarak yok sayıyoruz. Yani hiçbir olasılığı unutmamış oluyoruz.

Son olarak, kuralları tekrar değiştirelim: Eğer zar atışında 3 veya 7 dışında bir şey gelirse, oyuncunun sırası boşa gider ve hiçbir şey olmaz. Bu durumu, `_` koluna **unit değeri** (`()` — boş tuple tipi) yazarak ifade edebiliriz:

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

Burada Rust’a açıkça şunu söylüyoruz: Önceki kollarla eşleşmeyen herhangi bir değeri kullanmayacağız ve bu durumda hiçbir kod çalıştırmak istemiyoruz.

Desenler (patterns) ve eşleştirme (matching) hakkında daha fazlasını Bölüm 19’da göreceğiz. Şimdilik, biraz daha sade kullanımlar için faydalı olabilecek `if let` sözdizimine geçiyoruz.
