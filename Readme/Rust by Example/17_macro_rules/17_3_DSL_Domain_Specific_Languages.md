## 🧩 Alan-Özel Diller (Domain Specific Languages - DSLs)

Bir **DSL (Domain Specific Language)**, Rust makrosu içine gömülmüş küçük bir "dil"dir. Bu, tamamen geçerli Rust’tır çünkü makro sistemi normal Rust yapılarıyla açılır (expand edilir), fakat küçük bir dil gibi görünür. Bu sayede, özel bir işlevsellik için (belirli sınırlar içinde) **kısa ve sezgisel bir sözdizimi** tanımlamanız mümkün olur.

Örneğin, küçük bir hesap makinesi API’si tanımlamak istediğimizi varsayalım. Bir ifade (expression) verip çıktısını konsola yazdırmak isteyelim:

```rust
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // Türleri işaretsiz tamsayıya zorla
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate! {
        eval 1 + 2 // hehehe `eval` Rust anahtar kelimesi değildir!
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
```

### 📤 Çıktı:

```
1 + 2 = 3
(1 + 2) * (3 / 4) = 0
```

Bu çok basit bir örnektir, ancak `lazy_static` veya `clap` gibi çok daha karmaşık arayüzler bu yöntemle geliştirilmiştir.

Ayrıca, makrodaki **iki çift süslü paranteze** dikkat edin. Dıştakiler `macro_rules!` sözdiziminin bir parçasıdır ve `()` ya da `[]` ile birlikte kullanılabilir.
