## 🏗️ Yapılar (structs)

Benzer şekilde, bir `struct` parçalanabilir (destructured) olarak kullanılabilir:

```rust
fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Struct içindeki değerleri değiştirip ne olduğunu deneyebilirsiniz
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // struct parçalanabilir ve değişkenler yeniden adlandırılabilir,
        // sıranın önemi yoktur
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // bazı değişkenler yok sayılabilir:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // bu hata verecektir: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // Bir struct’ı parçalamak için match bloğuna gerek yoktur:
    let Foo { x : x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    // Parçalama iç içe (nested) struct’larla da çalışır:
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar { foo: Foo { x: nested_x, y: nested_y } } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}
```

👉 Bu örnekte `struct` alanları `match` içinde veya doğrudan parçalanarak (`destructuring`) kullanılabilir. Ayrıca iç içe geçmiş (nested) yapılar da aynı şekilde çözümlenebilir.
