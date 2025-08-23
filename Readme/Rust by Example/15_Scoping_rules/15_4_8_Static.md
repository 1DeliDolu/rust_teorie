## ♾️ `'static` Yaşam Süresi

Rust’ta bazı ayrılmış (reserved) yaşam süresi isimleri vardır. Bunlardan biri `'static`’tir. İki durumda karşınıza çıkabilir:

```rust
// `'static` yaşam süresine sahip bir referans:
let s: &'static str = "hello world";

// Trait bound içinde `'static`:
fn generic<T>(x: T) where T: 'static {}
```

Her ikisi de birbiriyle ilişkilidir ancak aralarında ince bir fark vardır. Bu fark, Rust öğrenirken kafa karışıklığına yol açabilir. Aşağıda her duruma ait örnekler verilmiştir:

```rust
// `'static` yaşam süresine sahip bir sabit oluştur.
static NUM: i32 = 18;

// `NUM`’a referans döndürür; burada `'static` yaşam süresi,
// girdinin yaşam süresine dönüştürülür.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Bir string literal oluştur ve yazdır:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // `static_string` kapsamdan çıktığında referans artık kullanılamaz,
        // ancak veri binary içinde kalır.
    }

    {
        // `coerce_static` için bir tamsayı oluştur:
        let lifetime_num = 9;

        // `NUM`, `lifetime_num`’un yaşam süresine dönüştürülür:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
```

---

## 🔗 Referans Yaşam Süresi

Bir referans yaşam süresi olarak `'static`, referansın işaret ettiği verinin programın geri kalan ömrü boyunca geçerli olduğunu belirtir. Ancak, yine de daha kısa bir yaşam süresine dönüştürülebilir.

`'static` yaşam süresine sahip bir değişken oluşturmanın iki yaygın yolu vardır ve ikisi de binary’nin salt-okunur (read-only) belleğinde depolanır:

1. `static` bildirimi ile bir sabit oluşturmak.
2. Bir string literal oluşturmak (türü `&'static str`’dir).

---

## 🧰 Dinamik `'static` Referanslar

`'static` referansların yalnızca programın kalan ömrü boyunca geçerli olması gerektiğinden, program çalışırken de oluşturulabilirler. Örneğin, `Box::leak` kullanarak dinamik olarak `'static` referanslar oluşturulabilir. Bu durumda referans tüm süre boyunca değil, yalnızca “sızdırıldığı” (leak) andan itibaren geçerlidir.

---

## 📌 Trait Bound Olarak `'static`

Bir trait bound olarak `'static`, türün hiçbir *non-static* (geçici) referans içermediği anlamına gelir. Yani alıcı, bu türü istediği kadar saklayabilir ve bırakana (drop) kadar hiçbir zaman geçersiz hale gelmez.

Bu, sahip olunan (owned) verilerin her zaman `'static` yaşam süresi sınırını geçtiğini, ancak bu verilere alınan referansların genellikle geçmediğini gösterir:

```rust
use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    // `i` sahiplenilmiştir ve referans içermez, bu yüzden `'static`’tir:
    let i = 5;
    print_it(i);

    // Ancak &i, yalnızca main() kapsamı boyunca geçerlidir,
    // bu yüzden `'static` değildir:
    print_it(&i);
}
```

Derleyici şu hatayı verecektir:

```
error[E0597]: `i` does not live long enough
  --> src/lib.rs:15:15
   |
15 |     print_it(&i);
   |     ---------^^--
   |     |         |
   |     |         borrowed value does not live long enough
   |     argument requires that `i` is borrowed for `'static`
16 | }
   | - `i` dropped here while still borrowed
```

---

Bkz:
Yaşam süreleri, trait sınırları ve referans yönetimi.
