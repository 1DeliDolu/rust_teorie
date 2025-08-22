## 📦 Diziler ve Dilimler (arrays and slices)

Bir **dizi** (array), aynı türden `T` nesnelerin bitişik bellekte depolandığı bir koleksiyondur. Diziler köşeli parantezler `[]` ile oluşturulur ve derleme zamanında bilinen uzunlukları, tür imzasının bir parçasıdır: `[T; length]`.

**Dilimler** (slices) dizilere benzer, ancak uzunlukları derleme zamanında bilinmez. Bunun yerine, bir dilim iki sözcüklük bir nesnedir:

* ilk sözcük veriye bir işaretçidir (pointer),
* ikinci sözcük dilimin uzunluğudur.

Sözcük boyutu `usize` ile aynıdır ve işlemci mimarisi tarafından belirlenir (ör. x86-64 üzerinde 64 bit). Dilimler bir dizinin bir bölümünü ödünç almak için kullanılabilir ve tür imzası `&[T]` şeklindedir.

```rust
use std::mem;

// Bu fonksiyon bir dilimi ödünç alır.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Sabit boyutlu dizi (tür imzası gereksizdir).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Tüm elemanlar aynı değerle başlatılabilir.
    let ys: [i32; 500] = [0; 500];

    // İndeksleme 0’dan başlar.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len`, dizideki eleman sayısını döndürür.
    println!("Number of elements in array: {}", xs.len());

    // Diziler stack üzerinde allocate edilir.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Diziler otomatik olarak dilim olarak ödünç alınabilir.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Dilimler bir dizinin bölümünü gösterebilir.
    // Biçimi: [starting_index..ending_index].
    // `starting_index` → dilimin ilk elemanı.
    // `ending_index` → son elemandan bir sonraki pozisyon.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    // Boş dilim örneği `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Aynı ama daha ayrıntılı

    // Dizilere güvenli erişim `.get` ile yapılabilir.
    // `.get`, bir `Option` döndürür.
    // Aşağıdaki gibi eşleştirilebilir ya da `.expect()` ile
    // hata mesajı verilerek program sonlandırılabilir.
    for i in 0..xs.len() + 1 { // Hata: bir eleman fazla!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Sabit uzunluklu dizide sınır dışı indeksleme derleme zamanı hatasıdır.
    //println!("{}", xs[5]);
    // Dilimde sınır dışı indeksleme çalışma zamanı hatasıdır.
    //println!("{}", xs[..][5]);
}
```

👉 Bu örnek, dizilerin ve dilimlerin nasıl oluşturulduğunu, ödünç alındığını, güvenli erişim yöntemlerini ve hata durumlarını göstermektedir.
