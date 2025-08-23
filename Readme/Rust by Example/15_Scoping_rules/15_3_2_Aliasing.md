## 🔀 Aliasing (çift yönlü erişim kuralları)

Veriler **sınırsız sayıda değiştirilemez şekilde (`&T`) ödünç alınabilir**, ancak değiştirilemez ödünçleme devam ederken aynı veriler **değiştirilebilir (`&mut T`)** olarak ödünç alınamaz.

Buna karşılık, **aynı anda yalnızca bir değiştirilebilir ödünçleme** mümkündür. Değiştirilebilir referans kullanıldıktan sonra, orijinal veri tekrar ödünç alınabilir.

```rust
struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Verilere hem referanslarla hem de orijinal sahip üzerinden erişilebilir
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // Hata! `point` değiştirilemez ödünç alındığı için aynı anda değiştirilebilir
    // şekilde ödünç alınamaz.
    // let mutable_borrow = &mut point;
    // TODO ^ Bu satırın yorumunu kaldırmayı deneyin

    // Ödünç alınmış değerler tekrar kullanılabilir
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // Artık değiştirilemez referanslar kullanılmadığından,
    // değiştirilebilir referans almak mümkün
    let mutable_borrow = &mut point;

    // Değiştirilebilir referans aracılığıyla veriyi değiştir
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Hata! `point` şu anda değiştirilebilir ödünç alındığı için
    // değiştirilemez şekilde ödünç alınamaz
    // let y = &point.y;
    // TODO ^ Bu satırın yorumunu kaldırmayı deneyin

    // Hata! `println!` makrosu değiştirilemez referans aldığı için
    // `point` doğrudan yazdırılamaz
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ Bu satırın yorumunu kaldırmayı deneyin

    // Sorun yok! Değiştirilebilir referans, `println!` için
    // değiştirilemez gibi kullanılabilir
    println!("Point has coordinates: ({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // Değiştirilebilir referans artık kullanılmadığından,
    // tekrar değiştirilemez referans almak mümkündür
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}
```

👉 Bu örnekte Rust’un **aliasing kuralları** görülüyor:

* Aynı anda **birden fazla `&T`** (değiştirilemez referans) olabilir.
* Aynı anda **yalnızca bir `&mut T`** (değiştirilebilir referans) olabilir.
* `&T` ve `&mut T` aynı anda var olamaz.
