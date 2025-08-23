## 🔄 if let

Bazı durumlarda, özellikle `enum` eşleştirmelerinde, `match` kullanımı hantal olabilir. Örneğin:

```rust
// `optional` tipini `Option<i32>` yap
let optional = Some(7);

match optional {
    Some(i) => println!("This is a really long string and `{:?}`", i),
    _ => {},
    // ^ `match` tüm durumları kapsamak zorundadır. Boş bir case yazmak alan kaybı gibi görünüyor, değil mi?
};
```

Bu durumda `if let` çok daha temiz bir çözüm sunar ve ayrıca başarısızlık (failure) durumlarını da belirtmeye izin verir:

```rust
fn main() {
    // Hepsi `Option<i32>` tipinde
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` ifadesi: "Eğer `let`, `number`’ı `Some(i)` olarak parçalayabilirse bloğu çalıştır."
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // Başarısızlık için else kullanılabilir:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Alternatif başarısızlık koşulu sağlama
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}
```

---

Aynı şekilde `if let`, herhangi bir `enum` değeriyle de kullanılabilir:

```rust
// Örnek enum
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    // Örnek değişkenler
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // a değişkeni Foo::Bar ile eşleşir
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    
    // b değişkeni Foo::Bar ile eşleşmez → hiçbir şey yazdırılmaz
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    
    // c değişkeni Foo::Qux ile eşleşir ve değer taşır
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // `if let` ile binding de yapılabilir
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}
```

---

`if let`’in bir diğer avantajı, parametre almayan `enum` varyantlarını da eşleştirmesidir.
Bu, `enum` `PartialEq` implement etmemiş veya türetmemiş (derive) olsa bile geçerlidir.
Örneğin `Foo::Bar == a` derleme hatasına sebep olurken, `if let` sorunsuz çalışır.

### 🏆 Zorluk: Aşağıdaki örneği `if let` ile düzeltin

```rust
// Bu enum özellikle PartialEq implement etmez veya türetmez.
// Bu nedenle Foo::Bar == a karşılaştırması derleme hatasına sebep olur.
enum Foo { Bar }

fn main() {
    let a = Foo::Bar;

    // a değişkeni Foo::Bar ile eşleşir
    if let Foo::Bar = a {
        println!("a is foobar");
    }
}
```

👉 Burada `if let` sayesinde karşılaştırma operatörüne gerek kalmadan, doğrudan `enum` varyantı ile eşleştirme yapılır.
