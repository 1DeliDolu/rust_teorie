## ➕ Operatör Aşırı Yükleme (operator overloading)

Rust’ta birçok **operatör (operator)**, **özellikler (traits)** aracılığıyla aşırı yüklenebilir (**overload**). Yani, bazı operatörler girdilerine göre farklı görevler yerine getirebilir. Bu mümkündür çünkü operatörler aslında **yöntem çağrılarının (method calls)** sözdizimsel şekeridir (**syntactic sugar**).

Örneğin, `a + b` ifadesindeki `+` operatörü aslında `a.add(b)` yöntemini çağırır. Bu `add` yöntemi, `Add` trait’inin bir parçasıdır. Dolayısıyla `+` operatörü, `Add` trait’ini uygulayan herhangi bir tür tarafından kullanılabilir.

`Add` gibi operatör aşırı yükleme sağlayan trait’lerin listesi `core::ops` modülünde bulunabilir.

```rust
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// `std::ops::Add` trait’i, `+` operatörünün işlevselliğini belirtmek için kullanılır.
// Burada `Add<Bar>` yani sağ tarafta (RHS) `Bar` türü ile toplama trait’i uygulanır.
// Aşağıdaki blok şu işlemi gerçekleştirir: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// Türler ters çevrildiğinde, değişmeli olmayan (non-commutative) toplama uygulanmış olur.
// Burada `Add<Foo>` yani sağ tarafta `Foo` türü ile toplama trait’i uygulanır.
// Bu blok şu işlemi gerçekleştirir: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
```
