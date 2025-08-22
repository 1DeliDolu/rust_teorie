## 🔄 From ve Into (From and Into)

`From` ve `Into` trait’leri doğal olarak birbiriyle bağlantılıdır ve bu aslında implementasyonlarının bir parçasıdır. Eğer bir türü `B`’den `A`’ya dönüştürebiliyorsak, `A`’dan `B`’ye dönüştürmenin de mümkün olması beklenir.

### 🏗️ From

`From` trait’i, bir türün kendisini başka bir türden nasıl oluşturacağını tanımlar. Böylece birçok tür arasında dönüştürme yapmak için çok basit bir mekanizma sağlar. Standart kütüphane, ilkel (primitive) ve yaygın türlerin dönüşümü için bu trait’in pek çok implementasyonunu içerir.

Örneğin, bir `str` kolayca `String`’e dönüştürülebilir:

```rust
let my_str = "hello";
let my_string = String::from(my_str);
```

Kendi türümüz için dönüşüm tanımlamak da mümkündür:

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
```

### 🔁 Into

`Into` trait’i, `From` trait’inin tam tersidir. Bir türün başka bir türe nasıl dönüştürüleceğini tanımlar.

`into()` çağrısı genellikle sonuç türünün belirtilmesini gerektirir, çünkü çoğu zaman derleyici bu türü kendi başına çıkaramaz.

```rust
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

fn main() {
    let int = 5;
    // Tür açıklamasını kaldırmayı deneyin
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
```

### 🔄 From ve Into Birlikte Kullanımı

`From` ve `Into`, birbirini tamamlayacak şekilde tasarlanmıştır. İki trait’i birden implement etmeye gerek yoktur. Eğer türünüz için `From` implementasyonu yaptıysanız, gerektiğinde `Into` otomatik olarak onu çağırır.

Ancak bunun tersi doğru değildir: `Into` trait’ini implement etmek, otomatik olarak `From` implementasyonunu sağlamaz.

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// `From` tanımla
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // `Into` kullan
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
```
