## 🧩 `ref` Deseni (the ref pattern)

`let` bağlamasıyla desen eşleme (pattern matching) veya ayrıştırma (destructuring) yapılırken, bir yapının ya da tuple’ın alanlarına referans almak için `ref` anahtar sözcüğü kullanılabilir.

Aşağıdaki örnek, `ref` kullanımının faydalı olduğu durumları göstermektedir:

```rust
#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
    let c = 'Q';

    // Atamanın sol tarafında `ref` kullanmak,
    // sağ tarafta `&` kullanmaya denktir.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref`, bir struct ayrıştırılırken de kullanılabilir.
    let _copy_of_x = {
        // `ref_to_x`, `point` içindeki `x` alanına bir referanstır.
        let Point { x: ref ref_to_x, y: _ } = point;

        // `point` içindeki `x` alanının bir kopyasını döndür.
        *ref_to_x
    };

    // `point`’in değiştirilebilir bir kopyası
    let mut mutable_point = point;

    {
        // `ref` ile birlikte `mut` kullanılarak değiştirilebilir referans alınabilir.
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // `mutable_point`’in `y` alanını değiştirilebilir referans ile değiştir.
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // İçinde işaretçi bulunan değiştirilebilir bir tuple
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    
    {
        // `mutable_tuple`’ı ayrıştırarak `last` değerini değiştir.
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    
    println!("tuple is {:?}", mutable_tuple);
}
```

👉 Bu örnekte:

* `ref` kullanmak, sağ tarafta `&` ile referans almaya eşdeğerdir.
* `ref mut` ile alanlara değiştirilebilir referans alınabilir.
* Bu yöntem, özellikle **struct** ve **tuple** ayrıştırmalarında alanlara doğrudan referans erişimi sağlamak için kullanışlıdır.
