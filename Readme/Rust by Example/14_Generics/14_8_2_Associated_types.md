## 🧩 İlişkili Türler (associated types)

“**İlişkili türler** (associated types)” kullanımı, iç türleri bir özelliğin (trait) içine **çıktı türleri** (output types) olarak yerelleştirerek kodun genel **okunabilirliğini** (readability) artırır. Özellik (trait) tanımı için sözdizimi (syntax) aşağıdaki gibidir:

```rust
// `A` ve `B`, `type` anahtar sözcüğü ile özellik (trait) içinde tanımlanır.
// (Not: Bu bağlamda `type`, takma adlar (aliases) için kullanılan `type`'tan farklıdır).
trait Contains {
    type A;
    type B;

    // Bu yeni türlere jenerik olarak atıf yapmak için güncellenmiş sözdizimi.
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}
```

**Contains** özelliğini (trait) kullanan işlevlerin (functions) artık **A** veya **B**’yi hiç ifade etmek zorunda olmadığını unutmayın:

```rust
// İlişkili türler kullanılmadan
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// İlişkili türler kullanılarak
fn difference<C: Contains>(container: &C) -> i32 { ... }
```

Bir önceki bölümdeki örneği ilişkili türleri (associated types) kullanarak yeniden yazalım:

```rust
struct Container(i32, i32);

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
trait Contains {
    // Define generic types here which methods will be able to utilize.
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also valid here.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}
```
