## 🛠️ macro\_rules!

Rust güçlü bir makro sistemi sağlar ve bu sistem metaprogramlama (metaprogramming) için kullanılabilir. Önceki bölümlerde gördüğünüz gibi, makrolar fonksiyonlara benzerler, ancak adlarının sonunda `!` bulunur. Fakat bir fonksiyon çağrısı üretmek yerine, makrolar kaynak koda açılır (expand edilir) ve programın geri kalanıyla birlikte derlenir. Ancak C ve diğer dillerdeki makrolardan farklı olarak, Rust makroları dizge ön işleme (string preprocessing) ile değil, soyut sözdizim ağaçlarına (abstract syntax trees) açılır. Bu nedenle beklenmedik öncelik (precedence) hataları ortaya çıkmaz.

Makrolar `macro_rules!` makrosu kullanılarak oluşturulur.

```rust
// Bu, `say_hello` adında basit bir makrodur.
macro_rules! say_hello {
    // `()` makronun herhangi bir argüman almadığını gösterir.
    () => {
        // Makro, bu bloğun içeriğine açılacaktır.
        println!("Hello!")
    };
}

fn main() {
    // Bu çağrı `println!("Hello!")` olarak açılacaktır.
    say_hello!()
}
```

## 🤔 Peki makrolar neden kullanışlıdır?

* **Kendini tekrar etme (Don't repeat yourself)**: Birçok durumda farklı türlerle (types) ama benzer işlevsiliğe ihtiyaç duyabilirsiniz. Makro yazmak, aynı kodu tekrar tekrar yazmaktan kaçınmak için faydalıdır. (Daha sonra buna değinilecektir)

* **Alan-özel diller (Domain-specific languages)**: Makrolar, belirli bir amaç için özel sözdizimleri tanımlamanıza izin verir. (Daha sonra buna değinilecektir)

* **Değişken sayıda argüman alan arayüzler (Variadic interfaces)**: Bazen değişken sayıda argüman alan bir arayüz tanımlamak isteyebilirsiniz. Örneğin `println!`, biçimlendirme dizesine (format string) bağlı olarak herhangi bir sayıda argüman alabilir. (Daha sonra buna değinilecektir)
