## 🧺 Tuple’lar (tuples)

Bir **tuple**, farklı türlerden değerlerin bir koleksiyonudur. Tuple’lar parantezler `()` kullanılarak oluşturulur ve her tuple, tür imzası `(T1, T2, ...)` olan bir değerdir. Burada `T1`, `T2` tuple üyelerinin türleridir.

Fonksiyonlar birden fazla değer döndürmek için tuple kullanabilir, çünkü tuple herhangi sayıda değer tutabilir.

```rust
// Tuple’lar fonksiyon argümanı ve dönüş değeri olarak kullanılabilir.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` ile tuple üyeleri değişkenlere bağlanabilir.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// Aktivite için kullanılacak struct.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // Farklı türlerden oluşan bir tuple.
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Tuple değerleri indeksleme ile alınabilir.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuple’lar, başka tuple’ların üyesi olabilir.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuple’lar yazdırılabilir.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // Ancak uzun tuple’lar (12’den fazla eleman) yazdırılamaz.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Yukarıdaki 2 satırı yorumdan çıkarın ve derleyici hatasını görün

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // Tek elemanlı tuple oluşturmak için virgül gerekir,
    // aksi halde yalnızca parantez içine alınmış bir değer olur.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuple parçalanarak (destructuring) bağlamalar oluşturulabilir.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}
```

👉 Bu örnek tuple’ların nasıl tanımlandığını, yazdırıldığını, fonksiyonlara aktarılıp döndürüldüğünü göstermektedir.

---

## 📝 Aktivite

Özet: Yukarıdaki örnekteki `Matrix` struct’ına `fmt::Display` trait’i ekleyin. Böylece `{:?}` debug formatı yerine `{}` display formatını kullandığınızda şu çıktıyı göreceksiniz:

```
( 1.1 1.2 )
( 2.1 2.2 )
```

`reverse` fonksiyonunu şablon olarak kullanarak bir `transpose` fonksiyonu ekleyin. Bu fonksiyon bir `Matrix` argümanı alacak ve iki elemanın yer değiştirdiği yeni bir `Matrix` döndürecek.

Örneğin:

```rust
println!("Matrix:\n{}", matrix);
println!("Transpose:\n{}", transpose(matrix));
```

Çıktısı şu şekilde olacaktır:

```
Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
```
