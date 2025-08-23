## 🛠️ Metotlar (methods)

Metotlar, fonksiyonlarda olduğu gibi benzer şekilde yaşam süresi (lifetime) anotasyonları alırlar:

```rust
struct Owner(i32);

impl Owner {
    // Bağımsız bir fonksiyondaki gibi yaşam süreleri anotasyonlanır.
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
```
