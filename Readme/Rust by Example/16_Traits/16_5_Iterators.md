## 🔄 Yineleyiciler (iterators)

**`Iterator` trait’i**, diziler gibi koleksiyonlar üzerinde yineleyicileri (iterators) uygulamak için kullanılır.

Bu trait’in yalnızca bir yöntemi zorunludur: bir sonraki elemanı döndüren `next`. Bu yöntem, ya manuel olarak bir `impl` bloğu içinde tanımlanabilir ya da diziler ve aralıklar (**ranges**) gibi yapılarda otomatik olarak sağlanır.

Kolaylık sağlaması için, `for` yapısı bazı koleksiyonları `.into_iter()` yöntemi aracılığıyla otomatik olarak yineleyiciye dönüştürür.

```rust
struct Fibonacci {
    curr: u32,
    next: u32,
}

// `Fibonacci` için `Iterator` uygula.
// `Iterator` trait’i yalnızca `next` elemanı için bir yöntem
// ve dönüş tipini bildirmek için bir `associated type` gerektirir.
impl Iterator for Fibonacci {
    // Bu türe Self::Item diyerek atıfta bulunabiliriz
    type Item = u32;

    // Burada diziyi `.curr` ve `.next` kullanarak tanımlıyoruz.
    // Dönüş tipi `Option<T>`’dir:
    //     * Yineleyici bittiğinde `None` döndürülür.
    //     * Aksi halde bir sonraki değer `Some` içine sarılıp döndürülür.
    // Dönüş tipinde Self::Item kullanıyoruz, böylece türü değiştirsek bile
    // fonksiyon imzalarını güncellememiz gerekmez.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        // Fibonacci dizisinin bir sonu olmadığı için bu yineleyici asla `None`
        // döndürmez; her zaman `Some` döner.
        Some(current)
    }
}

// Fibonacci dizisi üreteci döndürür
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    // `0..3`, 0, 1 ve 2 üreten bir `Iterator`’dür.
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for`, bir `Iterator` üzerinden `None` döndürülene kadar ilerler.
    // Her `Some` değeri açılır (unwrap) ve bir değişkene bağlanır (burada `i`).
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // `take(n)` yöntemi, bir `Iterator`’ü ilk `n` terimle sınırlar.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // `skip(n)` yöntemi, bir `Iterator`’ün ilk `n` terimini atlar.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // `iter` yöntemi, bir dizi/dilim üzerinde `Iterator` üretir.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
```
