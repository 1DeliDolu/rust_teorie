## 🔄 Yineleyicilerle (iterators) Bir Dizi Öğeyi İşleme

Yineleyici (iterator) deseni, bir öğe dizisi üzerinde sırayla işlem yapmanıza olanak tanır. Bir yineleyici, her öğe üzerinde dolaşma ve dizinin ne zaman bittiğini belirleme mantığından sorumludur. Yineleyicileri kullandığınızda bu mantığı yeniden uygulamanız gerekmez.

Rust’ta yineleyiciler **tembeldir (lazy)**, yani tüketilene kadar hiçbir şey yapmazlar. Örneğin, 13-10 numaralı listede `Vec<T>` türü üzerinde tanımlı `iter` metodunu çağırarak bir yineleyici oluşturuyoruz. Bu kod kendi başına faydalı bir iş yapmaz.

**Dosya adı:** `src/main.rs`

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

📌 Bu kod yalnızca `v1` vektörü üzerinde bir yineleyici oluşturur ve `v1_iter` değişkenine atar.

---

## 📌 Döngüde Yineleyici Kullanmak

3. bölümde (Liste 3-5) gördüğümüz gibi, bir `for` döngüsü aslında perde arkasında bir yineleyici oluşturur ve tüketir. 13-11 numaralı listede ise yineleyicinin oluşturulmasını ve kullanımını açıkça ayırıyoruz:

**Dosya adı:** `src/main.rs`

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {val}");
}
```

📌 Burada `for` döngüsü, `v1_iter` yineleyicisini tüketerek her öğeyi sırayla ekrana yazdırır.

---

## 🔍 Yineleyici Olmayan Dillerle Karşılaştırma

Standart kütüphanesinde yineleyici olmayan bir dilde aynı işlevi yazmak için:

* 0’dan başlayan bir sayaç tutar,
* bu sayaçla diziyi indeksleyerek değerleri alır,
* sayaç dizinin uzunluğuna ulaşana kadar artırırdınız.

Rust yineleyicilerle bu mantığı sizin yerinize yönetir, tekrar eden ve hata yapmaya müsait kodu ortadan kaldırır. Ayrıca yalnızca vektör gibi indekslenebilir yapılarla değil, çok çeşitli dizilerle aynı mantığı kullanma esnekliği sağlar.

---

## 🛠️ `Iterator` Özelliği (trait) ve `next` Metodu

Tüm yineleyiciler, standart kütüphanede tanımlanmış `Iterator` adlı özelliği uygular. Tanımı şöyledir:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // varsayılan implementasyonlu metodlar gizlenmiştir
}
```

* `type Item` → Yineleyicinin üreteceği öğe türü (ilişkili tür – associated type).
* `fn next` → Her çağrıldığında yineleyiciden bir öğe döndürür. Öğeler bittiğinde `None` döner.

---

## 📌 `next` Metodunu Doğrudan Çağırma

Yineleyiciler üzerinde `next` metodunu doğrudan çağırabiliriz. 13-12 numaralı liste, bir vektörden oluşturulan yineleyici üzerinde `next` çağrıldığında dönen değerleri göstermektedir:

**Dosya adı:** `src/lib.rs`

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

📌 Burada `v1_iter`’i `mut` yapmamız gerekti çünkü `next`, yineleyicinin dahili durumunu değiştirir. Her çağrı, sıradaki öğeyi tüketir.

Notlar:

* `for` döngüsüyle kullandığımızda, döngü yineleyicinin sahipliğini alır ve onu perde arkasında değiştirilebilir hale getirir.
* `iter()` → değiştirilemez referanslar (`&T`) üzerinde yineleyici üretir.
* `into_iter()` → sahipliği alarak değerleri (`T`) döndürür.
* `iter_mut()` → değiştirilebilir referanslar (`&mut T`) üzerinde yineleyici üretir.

## 🔄 Yineleyiciyi (iterator) Tüketen Metodlar

`Iterator` özelliği (trait), standart kütüphanede birçok **varsayılan metod** ile tanımlanmıştır. Bu metodların bazıları, kendi içinde `next` metodunu çağırır; bu nedenle `Iterator` özelliğini uygularken (`impl Iterator`) `next` metodunu tanımlamanız gerekir.

`next` metodunu çağıran metodlara **tüketici adaptörler (consuming adapters)** denir çünkü bu metodlar, yineleyiciyi kullanıp tüketir.

Örneğin, `sum` metodu yineleyicinin sahipliğini alır, `next`’i tekrarlı şekilde çağırarak tüm öğeleri toplar ve toplamı döndürür.

**Dosya adı:** `src/lib.rs`

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

📌 Burada `sum` çağrıldıktan sonra `v1_iter` artık kullanılamaz çünkü `sum` onu tüketmiştir.

---

## 🔄 Yeni Yineleyiciler Üreten Metodlar

Yineleyici adaptörleri (iterator adapters), yineleyiciyi tüketmez, onun yerine farklı özelliklerde yeni bir yineleyici döndürür.

Örneğin `map`, her öğe üzerinde çalıştırılacak bir kapanış alır ve her öğeyi dönüştürerek yeni bir yineleyici üretir.

**Dosya adı:** `src/main.rs`
(Bu kod istenen çıktıyı vermez.)

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

📌 Bu kod uyarı üretir çünkü yineleyiciler **tembeldir (lazy)** ve `map`’in ürettiği yeni yineleyici hiç tüketilmemektedir.

```
warning: unused `Map` that must be used
= note: iterators are lazy and do nothing unless consumed
```

---

## 📌 `collect` ile Yineleyiciyi Tüketmek

`map` ile oluşturulan yeni yineleyiciyi tüketmek için `collect` metodunu kullanabiliriz. Bu metod yineleyiciyi tüketir ve sonuçları bir koleksiyona dönüştürür.

**Dosya adı:** `src/main.rs`

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

📌 Burada `v2` artık `v1`’in her öğesinin 1 artırılmış halini içeren yeni bir vektördür.

`map` bir kapanış aldığı için, her öğe üzerinde istediğimiz işlemi tanımlayabiliriz. Ayrıca birden fazla adaptörü zincirleme kullanarak oldukça karmaşık işlemleri okunabilir şekilde gerçekleştirebiliriz. Ancak, yineleyiciler tembel olduğundan zincirlenen adaptörlerden sonuç almak için mutlaka tüketici metodlardan biri çağrılmalıdır (`collect`, `sum`, `for` döngüsü vb.).

---

## 🔍 Ortamı Yakalayan Kapanışlarla Yineleyici Kullanımı

Birçok yineleyici adaptörü, argüman olarak kapanış alır. Bu kapanışlar genellikle ortamlarından değer yakalar.

Örneğin `filter` metodu, her öğe için `bool` döndüren bir kapanış alır. Kapanış `true` dönerse öğe tutulur, `false` dönerse atılır.

13-16 numaralı listede, `shoe_size` değişkenini ortamdan yakalayan bir kapanış ile `filter` kullanıyoruz:

**Dosya adı:** `src/lib.rs`

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

📌 `shoes_in_size` fonksiyonu:

* Ayakkabı listesinin sahipliğini (`Vec<Shoe>`) ve bir ayakkabı numarasını parametre olarak alır.
* Ortamdan `shoe_size`’ı yakalayan kapanışla `filter` çağırır.
* Sonucu `collect` ile yeni bir vektöre toplar ve döndürür.

Test, yalnızca belirtilen numaradaki ayakkabıların döndüğünü doğrular.
