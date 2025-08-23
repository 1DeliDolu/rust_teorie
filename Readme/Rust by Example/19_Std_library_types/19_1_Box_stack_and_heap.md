## 📦 Box, yığın (stack) ve öbek (heap)

Rust’ta tüm değerler varsayılan olarak **yığın** (stack) üzerinde ayrılır. Değerler, `Box<T>` oluşturarak **öbekte** (heap) ayrılabilir. Bir `Box`, öbekte ayrılmış `T` türünden bir değere işaret eden akıllı bir işaretçidir (smart pointer). Bir `Box` kapsam (scope) dışına çıktığında, yıkıcı (destructor) çağrılır, iç nesne yok edilir ve öbekteki bellek serbest bırakılır.

Kutulanmış (boxed) değerler `*` operatörüyle **dereference** edilerek kullanılabilir; bu, bir dolaylılık katmanını (indirection) kaldırır.

```rust
use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// Bir dikdörtgen (Rectangle), sol üst ve sağ alt köşe noktalarıyla tanımlanabilir
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Bu noktayı öbekte ayırır ve işaretçisini döndürür
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // (tüm tür açıklamaları gereksizdir)
    // Yığında ayrılmış değişkenler
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    // Öbekte ayrılmış dikdörtgen
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // Fonksiyon çıktıları kutulanabilir
    let boxed_point: Box<Point> = Box::new(origin());

    // Çift dolaylılık
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point yığında {} bayt yer kaplar",
             mem::size_of_val(&point));
    println!("Rectangle yığında {} bayt yer kaplar",
             mem::size_of_val(&rectangle));

    // box boyutu == işaretçi boyutu
    println!("Boxed point yığında {} bayt yer kaplar",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle yığında {} bayt yer kaplar",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box yığında {} bayt yer kaplar",
             mem::size_of_val(&box_in_a_box));

    // `boxed_point` içindeki veriyi `unboxed_point` içine kopyalar
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point yığında {} bayt yer kaplar",
             mem::size_of_val(&unboxed_point));
}
```

👉 Bu örnek, `Box` kullanarak yığın (stack) ve öbek (heap) bellek yönetiminin nasıl işlediğini göstermektedir.
