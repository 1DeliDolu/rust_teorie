## 📚 Ödünç Alma (borrowing)

Çoğu zaman veriye sahipliği (ownership) almadan erişmek isteriz. Bunu gerçekleştirmek için Rust, **ödünç alma (borrowing)** mekanizmasını kullanır. Nesneler değerle (`T`) geçirilmek yerine, referansla (`&T`) geçirilebilir.

Derleyici, **borrow checker** sayesinde referansların her zaman geçerli nesnelere işaret ettiğini derleme zamanında garanti eder. Yani, bir nesneye referanslar varken, o nesne yok edilemez.

```rust
// Bu fonksiyon bir kutunun (box) sahipliğini alır ve onu yok eder
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// Bu fonksiyon bir i32’yi ödünç alır
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // Öbekte (heap) bir i32 ve yığında (stack) bir i32 oluştur
    // Sayılarda okunabilirlik için rastgele alt çizgi kullanılabilir
    // 5_i32, 5i32 ile aynıdır
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Kutunun içeriğini ödünç al. Sahiplik alınmaz,
    // bu nedenle içerik tekrar ödünç alınabilir.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Kutunun içindeki veriye referans al
        let _ref_to_i32: &i32 = &boxed_i32;

        // Hata!
        // `boxed_i32` yok edilemez, çünkü iç değeri hâlâ ödünç alınmış durumda
        eat_box_i32(boxed_i32);
        // FIXME ^ Bu satırı yorum satırı yapın

        // İç değer yok edildikten sonra `_ref_to_i32`’yi ödünç almaya çalışmak
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` kapsam dışına çıkar ve artık ödünç alınmaz.
    }

    // Artık `boxed_i32` sahipliğini `eat_box_i32`’ye verebilir ve yok edilebilir
    eat_box_i32(boxed_i32);
}
```

👉 Bu örnekte, Rust’un ödünç alma kuralları sayesinde **geçersiz referans** (dangling reference) veya **çift serbest bırakma (double free)** gibi hatalar önlenmiş olur.
