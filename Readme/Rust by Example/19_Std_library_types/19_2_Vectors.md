## 📊 Vektörler (Vectors)

Vektörler yeniden boyutlandırılabilir dizilerdir (re-sizable arrays). Dilimlerde (slices) olduğu gibi boyutları derleme zamanında bilinmez, ancak istendiğinde büyüyüp küçülebilirler. Bir vektör 3 parametreyle temsil edilir:

* veriye işaretçi (pointer to the data)
* uzunluk (length)
* kapasite (capacity)

`capacity`, vektör için ne kadar belleğin ayrıldığını gösterir. Vektör, uzunluk kapasiteden küçük olduğu sürece büyüyebilir. Bu eşik aşıldığında, vektör daha büyük bir kapasiteyle yeniden ayrılır (reallocated).

```rust
fn main() {
    // Yineleyiciler (iterators) vektörlere toplanabilir
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // `vec!` makrosu bir vektör başlatmak için kullanılabilir
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Vektörün sonuna yeni bir eleman ekleme
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Hata! Değişmez (immutable) vektörler büyütülemez
    collected_iterator.push(0);
    // FIXME ^ Bu satırı yorum satırı yapın

    // `len` metodu, vektörde şu anda depolanan eleman sayısını döndürür
    println!("Vector length: {}", xs.len());

    // İndeksleme köşeli parantezlerle yapılır (indeksleme 0’dan başlar)
    println!("Second element: {}", xs[1]);

    // `pop`, vektörün son elemanını kaldırır ve döndürür
    println!("Pop last element: {:?}", xs.pop());

    // Aralık dışı indeksleme panic oluşturur
    println!("Fourth element: {}", xs[3]);
    // FIXME ^ Bu satırı yorum satırı yapın

    // `Vector` kolayca yinelenebilir
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // Bir `Vector` yinelenirken, aynı zamanda `i` değişkeninde
    // konum numarası da alınabilir
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // `iter_mut` sayesinde değiştirilebilir (mutable) `Vector`ler
    // her değerin güncellenmesine izin verecek şekilde yinelenebilir
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}
```

👉 Daha fazla `Vec` metodu `std::vec` modülünde bulunabilir.
