## 🔄 if let ve let else ile Kısa Kontrol Akışı (concise control flow)

`if let` sözdizimi, `if` ve `let` ifadelerini birleştirerek, bir desen (pattern) ile eşleşen değerleri işlerken diğerlerini yok saymak için daha az ayrıntılı bir yol sağlar. `Listing 6-6`’da gösterilen programda, `config_max` değişkenindeki `Option<u8>` değeriyle eşleşme yapılır ancak yalnızca değer `Some` varyantı ise kod yürütülmek istenir.

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
```

Listing 6-6: Yalnızca değer `Some` olduğunda kod çalıştırmaya önem veren bir `match` ifadesi.

Eğer değer `Some` ise, `Some` varyantı içindeki değer `max` değişkenine bağlanarak yazdırılır. `None` değeri ile hiçbir şey yapmak istemeyiz. Ancak `match` ifadesinin tamamlanabilmesi için yalnızca bir varyant işlenmiş olsa bile ` _ => ()` eklememiz gerekir, bu da gereksiz tekrarlayan kod (boilerplate) anlamına gelir.

Bunun yerine, bunu daha kısa şekilde `if let` ile yazabiliriz. Aşağıdaki kod, `Listing 6-6`’daki `match` ile aynı davranışı gösterir:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

`if let` sözdizimi, bir desen (pattern) ile bir ifadeyi eşittir işaretiyle ayırır. Bu, `match` ile aynı şekilde çalışır: ifade `match`’e verilir ve desen ilk kolu (arm) temsil eder. Bu durumda desen `Some(max)`’tir ve `max`, `Some` içindeki değere bağlanır. Daha sonra `if let` bloğunun içinde `max` aynı `match` kolunda olduğu gibi kullanılabilir. Kod yalnızca değer desene uyuyorsa çalışır.

`if let` kullanmak daha az yazım, daha az girinti ve daha az tekrar eden kod demektir. Ancak bu durumda, `match`’in sunduğu tüm olasılıkları ele alma (exhaustive checking) zorunluluğunu kaybedersiniz. `match` mi yoksa `if let` mi kullanılacağı, yaptığınız işe ve kısalığın, kapsamlı kontrolü kaybetmeye değer olup olmadığına bağlıdır.

Başka bir deyişle, `if let`, yalnızca bir desenle eşleştiğinde kod çalıştıran ve diğer tüm değerleri yok sayan `match` için bir sözdizimsel kısayol (syntax sugar) olarak düşünülebilir.

Bir `if let` ifadesine `else` de ekleyebiliriz. `else` ile birlikte gelen kod bloğu, aynı `match` ifadesinde ` _` koluna karşılık gelen kod bloğuyla aynıdır. `Listing 6-4`’te tanımlanan `Coin` enum’unu hatırlayın, burada `Quarter` varyantı ayrıca bir `UsState` değeri de tutuyordu. Eğer gördüğümüz çeyrek olmayan tüm paraları saymak ve aynı zamanda çeyreklerin eyalet bilgisini duyurmak istersek, bunu şu şekilde bir `match` ifadesiyle yapabiliriz:

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}
```

Veya aynı işi bir `if let` ve `else` ifadesiyle şöyle yapabiliriz:

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```
## 😀 let...else ile “Mutlu Yol”da Kalmak (happy path)

Yaygın bir desen, bir değer mevcut olduğunda bazı hesaplamalar yapmak ve aksi durumda varsayılan bir değer döndürmektir. `UsState` değerine sahip paralar (coins) örneğimizle devam edersek, eğer çeyrek üzerindeki eyaletin ne kadar eski olduğuna bağlı olarak komik bir şey söylemek istersek, `UsState` üzerinde eyaletin yaşını kontrol eden bir yöntem tanımlayabiliriz:

```rust
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}
```

Daha sonra, `if let` kullanarak paranın türü üzerinde eşleşme yapabilir, koşul bloğunun içinde bir `state` değişkeni tanımlayabiliriz. Bu `Listing 6-7`’de gösterilmiştir:

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
```

Listing 6-7: `if let` içinde iç içe koşullar kullanarak bir eyaletin 1900 yılında mevcut olup olmadığını kontrol etme.

Bu yöntem işi görür, ancak işi `if let` bloğunun içine taşıdığı için, yapılacak işler daha karmaşık olduğunda üst düzey dalların (branches) ilişkisini takip etmek zor olabilir. Ayrıca ifadelerin değer üretmesinden faydalanarak, `if let`’ten `state` elde edebilir veya erken dönüş yapabiliriz. (Benzerini `match` ile de yapabilirdiniz.) Bu yaklaşım `Listing 6-8`’de gösterilmektedir:

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

Listing 6-8: `if let` kullanarak bir değer üretmek veya erken dönüş yapmak.

Bu da kendi içinde biraz kafa karıştırıcıdır! `if let`’in bir kolu bir değer üretirken, diğeri fonksiyondan tamamen çıkış yapmaktadır.

Bu yaygın deseni daha güzel ifade etmek için Rust `let...else` sözdizimini sunar. `let...else` sözdizimi, sol tarafta bir desen (pattern), sağ tarafta ise bir ifade alır; `if let`’e çok benzer, ancak `if` kolu yoktur, yalnızca `else` kolu vardır. Eğer desen eşleşirse, desenin içindeki değer dış kapsamda bağlanır. Eğer eşleşmezse, program `else` koluna girer ve bu kol fonksiyondan çıkış yapmak zorundadır.

`Listing 6-9`’da, `Listing 6-8`’in `if let` yerine `let...else` ile nasıl göründüğünü görebilirsiniz:

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

Listing 6-9: Fonksiyon akışını netleştirmek için `let...else` kullanımı.

Dikkat edin ki bu yöntem, fonksiyonun ana gövdesinde “mutlu yol”da kalmayı sağlar; `if let`’te olduğu gibi iki farklı dal için önemli ölçüde farklı kontrol akışı bulunmaz.

Eğer programınızda bir `match` ifadesiyle anlatılamayacak kadar ayrıntılı mantık varsa, `if let` ve `let...else`’in de Rust araç kutunuzda bulunduğunu unutmayın.
## 📋 Özet (summary)

Artık, `enum`’ları kullanarak belirli bir kümeden değer alabilen özel türler (custom types) oluşturmayı öğrendik. Standart kütüphanedeki `Option<T>` türünün, hata yapmayı önlemek için tür sisteminden nasıl yararlanmanıza yardımcı olduğunu gördük. `enum` değerleri veri içerdiğinde, kaç durumu ele almanız gerektiğine bağlı olarak bu değerleri çıkarmak ve kullanmak için `match` veya `if let` kullanabileceğinizi öğrendik.

Artık Rust programlarınız, alanınıza özgü kavramları `struct` ve `enum` kullanarak ifade edebilir. API’nizde özel türler oluşturmak, tür güvenliğini (type safety) sağlar: Derleyici, her fonksiyonun yalnızca beklediği türde değerleri alacağından emin olur.

Kullanıcılarınıza iyi organize edilmiş, kullanımı kolay ve yalnızca gerçekten ihtiyaç duyacakları şeyleri ortaya çıkaran bir API sunmak için şimdi Rust’ın modüllerine (modules) geçelim.

