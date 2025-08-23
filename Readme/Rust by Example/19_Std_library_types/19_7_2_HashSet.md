## 🧩 HashSet

Bir **HashSet**, yalnızca anahtarlarla ilgilendiğimiz bir **HashMap** olarak düşünülebilir (`HashSet<T>` aslında `HashMap<T, ()>` etrafında bir sarmalayıcıdır).

👉 “Bunun ne faydası var? Anahtarları bir `Vec` içinde de tutabilirim.”

Fark şudur:
Bir **HashSet**, **tekrar eden öğelerin olmamasını garanti eder**. Bu, her küme (set) koleksiyonunun sağladığı bir sözleşmedir. `HashSet` bunun bir implementasyonudur (bkz: `BTreeSet`).

Eğer zaten mevcut bir değeri `HashSet`’e eklerseniz (yeni değer eskisiyle eşitse ve aynı hash’e sahipse), yeni değer eskisinin yerini alır.

Bu, bir şeyden **asla birden fazla olmamasını** istediğinizde veya zaten sahip olup olmadığınızı bilmek istediğinizde idealdir.

---

### 🔑 HashSet’in dört temel işlemi

Tüm bu işlemler bir yineleyici (iterator) döndürür:

* `union`: her iki kümedeki benzersiz tüm öğeleri döndürür.
* `difference`: yalnızca birinci kümede olup ikinci kümede olmayanları döndürür.
* `intersection`: yalnızca her iki kümede ortak olanları döndürür.
* `symmetric_difference`: yalnızca birinde bulunan, diğerinde olmayan öğeleri döndürür.

---

### 📌 Örnek

```rust
use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` eğer değer zaten varsa false döndürür.
    assert!(b.insert(4), "Value 4 is already in set B!");
    // FIXME ^ Bu satır yorum satırına alınmalı

    b.insert(5);

    // Eğer koleksiyonun eleman türü `Debug` uygularsa,
    // koleksiyon da `Debug` uygular.
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // [1, 2, 3, 4, 5] (rastgele sırada) yazdırır
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // [1] yazdırır
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // [2, 3, 4] (rastgele sırada) yazdırır
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // [1, 5] yazdırır
    println!("Symmetric Difference: {:?}",
             a.symmetric_difference(&b).collect::<Vec<&i32>>());
}
```

👉 Bu örnek, `HashSet`’in küme işlemlerini (`union`, `difference`, `intersection`, `symmetric_difference`) nasıl kullandığını göstermektedir.
