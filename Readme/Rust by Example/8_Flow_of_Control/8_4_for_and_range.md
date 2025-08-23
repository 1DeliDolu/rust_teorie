## 🔂 for Döngüleri (for loops)

### 🔢 for ve range

`for in` yapısı bir `Iterator` üzerinde yineleme (iterate) yapmak için kullanılabilir. Bir yineleyici (iterator) oluşturmanın en kolay yollarından biri aralık (range) gösterimidir: `a..b`.
Bu ifade, `a` (dahil) ile `b` (hariç) arasındaki değerleri birer adım artışla üretir.

FizzBuzz’u `while` yerine `for` ile yazalım:

```rust
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

Alternatif olarak, `a..=b` ifadesi her iki ucu da dahil eden bir aralık üretir. Yukarıdaki örnek şu şekilde de yazılabilir:

```rust
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```

👉 `..` üst sınırı hariç tutar, `..=` ise üst sınırı da dahil eder.

---

### 🔗 for ve iteratörler (for and iterators)

`for in` yapısı, bir `Iterator` ile çeşitli yollarla etkileşim kurabilir. `Iterator` trait bölümünde açıklandığı gibi, varsayılan olarak `for` döngüsü koleksiyon üzerine `into_iter` uygular. Ancak bu, koleksiyonları iteratöre dönüştürmenin tek yolu değildir.

* `iter`
* `into_iter`
* `iter_mut`

hepsi koleksiyonu bir iteratöre dönüştürür fakat veriye farklı açılardan bakış sağlar.

---

#### 📌 `iter`

Her yinelemede koleksiyondaki elemanları ödünç alır (borrow). Böylece koleksiyon döngüden sonra kullanılabilir halde kalır.

```rust
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
}
```

👉 `iter` sadece ödünç aldığı için `names` koleksiyonu döngüden sonra hâlâ kullanılabilir.

---

#### 📌 `into_iter`

Koleksiyonu **tüketir (consume)**. Her yinelemede verinin kendisi sağlanır. Koleksiyon taşındığı (moved) için döngüden sonra artık mevcut değildir.

```rust
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}
```

👉 Burada `names` koleksiyonu döngü tarafından taşındığı için tekrar kullanılamaz.

---

#### 📌 `iter_mut`

Her yinelemede koleksiyondaki elemanları **değiştirilebilir (mutable) ödünç** alır. Böylece koleksiyonun elemanları yerinde değiştirilebilir.

```rust
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
```

👉 `iter_mut` sayesinde koleksiyonun elemanları döngü içinde değiştirilebiliyor.

---

### 📋 Özet

* `iter` → Elemanları ödünç alır, koleksiyon bozulmaz.
* `into_iter` → Koleksiyonu tüketir, döngüden sonra kullanılamaz.
* `iter_mut` → Elemanları değiştirilebilir şekilde ödünç alır, koleksiyon yerinde değiştirilebilir.
