## 🧩 Sorun (the problem)

Bir **kapsayıcı tür** (container type) üzerinde **jenerik** (generic) olan bir **özellik** (trait), tür belirtimi gerektirir — özelliğin kullanıcıları tüm jenerik türleri belirtmek zorundadır.

Aşağıdaki örnekte, **Contains** özelliği (trait) **A** ve **B** jenerik türlerinin kullanımına izin verir. Özellik (trait) ardından **Container** türü için uygulanır (impl) ve **A** ile **B** için **i32** belirtilir; böylece **fn difference()** ile kullanılabilir.

**Contains** jenerik (generic) olduğu için, **fn difference()** için tüm jenerik türleri açıkça belirtmeye zorlanırız. Pratikte, **A** ve **B**’nin girdi **C** tarafından belirlendiğini ifade edebilmenin bir yolunu isteriz. Bir sonraki bölümde göreceğiniz gibi, **ilişkili türler** (associated types) tam olarak bu yeteneği sağlar.

```rust
struct Container(i32, i32);

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
    fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
}

impl Contains<i32, i32> for Container {
    // True if the numbers stored are equal.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number.
    fn first(&self) -> i32 { self.0 }

    // Grab the last number.
    fn last(&self) -> i32 { self.1 }
}

// `C` contains `A` and `B`. In light of that, having to express `A` and
// `B` again is a nuisance.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> {
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
