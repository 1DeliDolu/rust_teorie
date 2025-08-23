## 📜 Where Hükümleri (Where clauses)

Bir sınır (bound), bir türün ilk geçtiği yerde yazılmak yerine, açılı `{` süslü parantezden hemen önce kullanılan **`where` hükmü (where clause)** ile de ifade edilebilir. Ayrıca `where` hükümleri yalnızca tür parametrelerine değil, **herhangi bir türe** sınır uygulayabilir.

### Ne zaman faydalıdır?

* **Tür parametreleri ile sınırların ayrı ayrı yazılması daha açık olduğunda:**

```rust
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
```

`where` hükmü ile:

```rust
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

* **Normal sözdizimiyle ifade edilemeyen durumlarda daha açıklayıcı olduğunda:**

Aşağıdaki örnek, yalnızca `where` hükmü ile ifade edilebilir:

```rust
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// Normalde `T: Debug` yazmamız gerekirdi, ancak burada asıl yazdırılan `Option<T>`.
// Bu nedenle doğru sınır `Option<T>: Debug` olmalıdır ve bunun için `where` gerekir.
impl<T> PrintInOption for T where
    Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
```

Burada:

* `PrintInOption` trait’i jenerik olarak uygulanır.
* `where Option<T>: Debug` şartı ile yalnızca `Option<T>` `Debug` implementasyonu varsa geçerli olur.
* Bu sayede `vec![1, 2, 3]` `Option<Vec<i32>>` olarak yazdırılabilir.
