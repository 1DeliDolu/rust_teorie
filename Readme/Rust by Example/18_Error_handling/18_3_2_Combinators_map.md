## 🔗 Birleştiriciler: map (Combinators: map)

`match`, `Option` değerlerini ele almak için geçerli bir yöntemdir. Ancak, özellikle yalnızca giriş olduğunda geçerli olan işlemlerle sıkça karşılaşıldığında, kullanımı yorucu olabilir. Bu gibi durumlarda, kontrol akışını modüler bir şekilde yönetmek için **birleştiriciler (combinators)** kullanılabilir.

`Option` türünün yerleşik bir metodu olan `map()`, basitçe `Some -> Some` ve `None -> None` dönüşümü yapan bir birleştiricidir. Birden fazla `map()` çağrısı zincirleme kullanılabilir ve bu, daha da esnek bir çözüm sağlar.

Aşağıdaki örnekte, `process()` fonksiyonu önceki tüm fonksiyonların yerini alırken oldukça derli toplu kalır:

```rust
#![allow(dead_code)]

#[derive(Debug)] enum Food { Apple, Carrot, Potato }

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

// Yemekleri soymak. Eğer yiyecek yoksa `None` döner.
// Aksi halde soyulmuş yiyecek döner.
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None       => None,
    }
}

// Yemekleri doğramak. Eğer yiyecek yoksa `None` döner.
// Aksi halde doğranmış yiyecek döner.
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None               => None,
    }
}

// Yemekleri pişirmek. Burada durumları ele almak için `match` yerine `map()` kullanıyoruz.
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// Soyma, doğrama ve pişirme işlemlerini sırayla yapan bir fonksiyon.
// Birden fazla `map()` çağrısını zincirleyerek kodu basitleştiriyoruz.
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// Yemek yemeye çalışmadan önce yiyecek olup olmadığını kontrol et!
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None       => println!("Oh no! It wasn't edible."),
    }
}

fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // Şimdi daha basit görünen `process()` fonksiyonunu deneyelim.
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}
```

👉 Bu örnekte, `map()` kullanarak zincirleme dönüşümler yapılır ve `None` durumunda işlemler otomatik olarak sona erer. Bu, kodu hem daha okunabilir hem de daha kompakt hale getirir.
