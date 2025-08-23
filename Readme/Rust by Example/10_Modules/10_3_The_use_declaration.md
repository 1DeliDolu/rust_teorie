## 📥 `use` Bildirimi (the use declaration)

`use` bildirimi, tam bir yolu (full path) yeni bir ada bağlamak için kullanılabilir, bu da erişimi kolaylaştırır. Genellikle şu şekilde kullanılır:

```rust
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

fn main() {
    my_first_function();
}
```

👉 Bu örnekte `deeply::nested` içindeki öğeler doğrudan isimleriyle kullanılabilir.

---

`as` anahtar sözcüğü, içe aktarılan (import edilen) bir öğeyi farklı bir ada bağlamak için kullanılabilir:

```rust
// `deeply::nested::function` yolunu `other_function` adına bağla
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // `deeply::nested::function`’a daha kolay erişim
    other_function();

    println!("Entering block");
    {
        // Bu, `use deeply::nested::function as function` ile eşdeğerdir.
        // Bu `function()` dıştakini gölgeleyecektir.
        use crate::deeply::nested::function;

        // `use` bildirimleri yerel (local) kapsama sahiptir.
        // Bu durumda, `function()` gölgelemesi yalnızca bu blok içinde geçerlidir.
        function();

        println!("Leaving block");
    }

    function();
}
```

👉 Burada görüldüğü gibi, `use` bildirimleri **yalnızca tanımlandıkları kapsamda** (scope) geçerlidir ve aynı isimdeki dıştaki fonksiyonları **gölgeleyebilir (shadowing)**.
