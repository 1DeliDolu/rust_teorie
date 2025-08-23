## 🧩 Yüksek Mertebeli Fonksiyonlar (higher order functions)

Rust, **yüksek mertebeli fonksiyonlar** (Higher Order Functions, HOF) sağlar. Bunlar, bir veya daha fazla fonksiyon alan ve/veya daha faydalı bir fonksiyon üreten fonksiyonlardır. HOF’lar ve tembel yineleyiciler (lazy iterators), Rust’a işlevsel (functional) bir tat katar.

```rust
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the numbers with odd squares under 1000");
    let upper = 1000;

    // Buyurgan (imperative) yaklaşım
    // Biriktirici (accumulator) değişkeni tanımla
    let mut acc = 0;
    // Yinele: 0, 1, 2, ... sonsuza kadar
    for n in 0.. {
        // Sayının karesini al
        let n_squared = n * n;

        if n_squared >= upper {
            // Üst sınıra ulaşıldıysa döngüyü kır
            break;
        } else if is_odd(n_squared) {
            // Eğer tek ise değeri biriktir
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // İşlevsel (functional) yaklaşım
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                             // Tüm doğal sayıların kareleri
             .take_while(|&n_squared| n_squared < upper) // Üst sınırın altındakiler
             .filter(|&n_squared| is_odd(n_squared))     // Tek olanlar
             .sum();                                     // Toplamını al
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
```

`Option` ve `Iterator`, kendi paylarına düşen birçok yüksek mertebeli fonksiyonu uygular.
