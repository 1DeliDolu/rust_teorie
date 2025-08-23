## 📝 `impl Trait`

`impl Trait` iki yerde kullanılabilir:

1. **Bir argüman türü (argument type) olarak**
2. **Bir dönüş türü (return type) olarak**

---

### 🔹 Argüman türü olarak

Fonksiyonunuz bir trait üzerinde genelse fakat belirli tür sizin için önemli değilse, fonksiyon bildirimini `impl Trait` ile basitleştirebilirsiniz.

Örneğin:

```rust
fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(',') 
                    .map(|entry| String::from(entry.trim())) 
                    .collect() 
            })
        })
        .collect() 
}
```

`parse_csv_document` geneldir ve `BufRead` trait’ini uygulayan herhangi bir türü (ör. `BufReader<File>` veya `[u8]`) alabilir. Ancak `R` yalnızca `src`’nin türünü belirtmek için kullanılır. Bu nedenle fonksiyon şu şekilde de yazılabilir:

```rust
fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(',') 
                    .map(|entry| String::from(entry.trim())) 
                    .collect() 
            })
        })
        .collect() 
}
```

⚠️ Not: Argüman türü olarak `impl Trait` kullanıldığında, fonksiyonun belirli bir tür ile çağrılacağını **açıkça belirtmeniz** mümkün değildir. Yani şu kullanım ikinci örnekte çalışmaz:

```rust
parse_csv_document::<std::io::Empty>(std::io::empty());
```

---

### 🔹 Dönüş türü olarak

Fonksiyonunuz bir trait’i uygulayan bir tür döndürüyorsa, dönüş tipini `-> impl MyTrait` şeklinde yazabilirsiniz. Bu, tür imzalarını oldukça sadeleştirir.

```rust
use std::iter;
use std::vec::IntoIter;

// Karmaşık dönüş tipi
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// `impl Trait` ile daha basit
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");
}
```

---

### 🔹 İsimsiz closure türleri

Bazı Rust türleri yazılamaz; örneğin her **closure**’ın kendi isimsiz somut türü vardır. `impl Trait` gelmeden önce, closure döndürmek için yığında (heap) bellek ayırmak gerekirdi. Artık bunu tamamen statik olarak yapabilirsiniz:

```rust
// `y` değerini girdisine ekleyen bir fonksiyon döndürür
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}
```

---

### 🔹 `map` ve `filter` ile `impl Trait`

Closure türlerinin ismi olmadığı için, `map` veya `filter` kullanan bir iterator döndürürken açık dönüş tipi yazılamaz. `impl Trait` bunu kolaylaştırır:

```rust
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn main() {
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}
```
