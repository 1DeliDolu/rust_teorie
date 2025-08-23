## 🔄 while

`while` anahtar kelimesi, bir koşul doğru olduğu sürece döngü çalıştırmak için kullanılabilir.

Şimdi ünlü **FizzBuzz** problemini bir `while` döngüsü ile yazalım:

```rust
fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}
```

👉 Bu örnekte `while n < 101` koşulu doğru olduğu sürece döngü çalışıyor ve 1’den 100’e kadar sayılar için FizzBuzz kuralları uygulanıyor.
