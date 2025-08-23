## 🥤 Option ve Unwrap (Option & unwrap)

Önceki örnekte, programın istediğimiz zaman hataya düşmesini sağlayabileceğimizi gösterdik. Programımıza şekerli limonata içersek `panic` etmesini söyledik. Peki ya bir içecek bekleyip hiç içecek alamazsak? Bu da en az onun kadar kötü olurdu, bu yüzden ele alınması gerekir!

Bunu limonata örneğinde yaptığımız gibi boş string (`""`) ile test edebilirdik. Ancak Rust kullandığımız için, bir içecek olmadığında derleyicinin bize işaret etmesini sağlayabiliriz.

`std` kütüphanesinde bulunan `Option<T>` adlı `enum`, yokluğun bir olasılık olduğu durumlarda kullanılır. İki farklı "seçenek" (option) içerir:

* `Some(T)`: T türünden bir eleman bulundu
* `None`: Hiçbir eleman bulunamadı

Bu durumlar ya `match` ile açıkça (explicitly) ele alınabilir ya da `unwrap` ile örtük (implicitly) şekilde ele alınabilir. Örtük kullanım, içteki elemanı döndürür veya `panic` oluşturur.

`panic` çıktısı `expect` ile özelleştirilebilir. Ancak `unwrap`, açıkça yapılan işlemlere göre daha az anlamlı bir çıktı bırakır. Aşağıdaki örnekte, açıkça ele alma yöntemi daha kontrollü bir sonuç üretir, fakat istenirse `panic` etme olanağını da korur.

```rust
// Yetişkin her şeyi görmüştür ve her içecekle iyi başa çıkar.
// Tüm içecekler `match` kullanılarak açıkça ele alınır.
fn give_adult(drink: Option<&str>) {
    // Her durum için yapılacak eylemi belirt.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No drink? Oh well."),
    }
}

// Diğerleri, şekerli içecek içmeden önce `panic` eder.
// Tüm içecekler `unwrap` kullanılarak örtük şekilde ele alınır.
fn drink(drink: Option<&str>) {
    // `unwrap`, `None` aldığında bir `panic` döndürür.
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
```

👉 Bu örnekte, `give_adult` tüm durumları güvenli bir şekilde işlerken, `drink` fonksiyonu `unwrap` ile `None` aldığında `panic` üretir.
