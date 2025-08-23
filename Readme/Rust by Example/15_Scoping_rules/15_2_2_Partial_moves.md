## 🧩 Kısmi Taşımalar (partial moves)

Tek bir değişkenin ayrıştırılması (destructuring) sırasında hem **taşıma ile (by-move)** hem de **referans ile (by-reference)** desen bağlamaları aynı anda kullanılabilir. Bu durumda değişkenin **kısmi taşınması (partial move)** gerçekleşir; yani değişkenin bazı parçaları taşınırken, diğer parçaları kalır.

Böyle bir durumda, üst değişken artık bütün olarak kullanılamaz; ancak yalnızca referansla alınan (ve taşınmayan) parçalar hâlâ kullanılabilir.

`Drop` trait’ini uygulayan türlerden kısmi taşıma yapılamaz, çünkü `drop` metodu değişkeni bütün olarak kullanmaya devam eder.

```rust
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    // Hata! `Drop` trait’ini uygulayan türlerden kısmi taşıma yapılamaz
    //impl Drop for Person {
    //    fn drop(&mut self) {
    //        println!("Dropping the person struct {:?}", self)
    //    }
    //}
    // TODO ^ Bu satırların yorumunu kaldırmayı deneyin

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` person’dan taşındı, ancak `age` referansla alındı
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Hata! `person` kısmi taşındığı için artık bütün olarak kullanılamaz
    //println!("The person struct is {:?}", person);

    // `person` bütün olarak kullanılamaz, ancak `person.age` kullanılabilir
    println!("The person's age from person struct is {}", person.age);
}
```

👉 Bu örnekte `age` değişkenini öbekte (heap) saklıyoruz. Eğer yukarıdaki `ref` kaldırılırsa, `person.age`’in sahipliği `age`’e aktarılır ve hata oluşur. Eğer `Person.age` yığında (stack) saklansaydı, `ref` gerekli olmazdı; çünkü `age` tanımı `person.age`’den veriyi taşıyarak değil, kopyalayarak alabilirdi.
