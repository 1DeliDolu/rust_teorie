## 🍏 Option’ları Açma ve Varsayılanlarla Çalışma (Unpacking options and defaults)

Bir `Option` değerini açıp, eğer `None` ise bir **varsayılan** değere geri dönmenin birden fazla yolu vardır. İhtiyacımıza uygun olanı seçmek için şunları göz önünde bulundurmalıyız:

* **Erken (eager) mi yoksa tembel (lazy) değerlendirme mi istiyoruz?**
* **Orijinal boş değeri korumak mı yoksa yerinde (in place) değiştirmek mi istiyoruz?**

---

### 🔗 `or()`: Zincirlenebilir, erken değerlendirme, boş değeri korur

`or()` zincirlenebilir ve argümanını **erken (eagerly)** değerlendirir. Bu yüzden `or` içine verilen değişken taşınır (moved).

```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);
    // first_available_fruit: Some(Orange)

    // `or` argümanını taşır (move).
    // Yukarıdaki örnekte, `or(orange)` bir `Some` döndürdü, bu yüzden `or(apple)` çalışmadı.
    // Ama `apple` değişkeni yine de taşınmış oldu ve artık kullanılamaz.
}
```

---

### 🟢 `or_else()`: Zincirlenebilir, tembel değerlendirme, boş değeri korur

`or_else` de zincirlenebilir, ancak argümanını **tembel (lazy)** şekilde değerlendirir. Yani sadece gerekirse closure çağrılır.

```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit);
    // Providing kiwi as fallback
    // first_available_fruit: Some(Kiwi)
}
```

---

### ✏️ `get_or_insert()`: Erken değerlendirme, boş değeri yerinde değiştirir

`Option`’ın bir değer içerdiğinden emin olmak için `get_or_insert` kullanılabilir. Bu metot, boş değeri yerinde değiştirir. Ancak parametresi **erken (eager)** değerlendirilir, bu yüzden taşınır (moved).

```rust
#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);
    // first_available_fruit is: Apple
    // my_fruit is: Some(Apple)
}
```

---

### ⚡ `get_or_insert_with()`: Tembel değerlendirme, boş değeri yerinde değiştirir

`get_or_insert_with`, doğrudan bir değer vermek yerine **closure** alır. Bu closure sadece gerektiğinde çağrılır.

```rust
#[derive(Debug)] 
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit
        .get_or_insert_with(get_lemon_as_fallback);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);
    // Providing lemon as fallback
    // first_available_fruit is: Lemon
    // my_fruit is: Some(Lemon)

    // Eğer Option zaten bir değer içeriyorsa, closure çağrılmaz
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple is: {:?}", should_be_apple);
    println!("my_apple is unchanged: {:?}", my_apple);
    // should_be_apple is: Apple
    // my_apple is unchanged: Some(Apple)
}
```

---

👉 Özetle:

* `or()` → eager, zincirlenebilir, boşu korur
* `or_else()` → lazy, zincirlenebilir, boşu korur
* `get_or_insert()` → eager, boşu yerinde değiştirir
* `get_or_insert_with()` → lazy, boşu yerinde değiştirir

İster misiniz ben size bunları bir tablo halinde **eager vs lazy** ve **in-place vs immutable** kıyaslamasıyla da özetleyeyim?
