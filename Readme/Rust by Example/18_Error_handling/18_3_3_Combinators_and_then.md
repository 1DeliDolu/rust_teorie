## 🔗 Birleştiriciler: and\_then (Combinators: and\_then)

`map()`, `match` ifadelerini basitleştirmek için zincirlenebilir bir yöntem olarak açıklanmıştı. Ancak, `map()` bir `Option<T>` döndüren fonksiyon üzerinde kullanıldığında, sonuç `Option<Option<T>>` olur. Birden fazla çağrı zincirlendiğinde bu kafa karıştırıcı hale gelebilir. İşte burada bazı dillerde **flatmap** olarak bilinen başka bir birleştirici olan `and_then()` devreye girer.

`and_then()`, içindeki değeri argüman olarak alıp fonksiyonu çağırır ve sonucu döndürür. Eğer `Option` `None` ise, doğrudan `None` döner.

Aşağıdaki örnekte, `cookable_v3()` bir `Option<Food>` sonucuna ulaşır. Eğer `map()` kullansaydık, sonuç `Option<Option<Food>>` olacaktı ve bu `eat()` için geçersiz olurdu.

```rust
#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// Sushi yapmak için malzemelerimiz yok.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// Cordon Bleu dışında tüm yemeklerin tarifine sahibiz.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

// Bir yemeği yapabilmek için hem tarif hem de malzeme gerekir.
// Bu mantığı `match` zinciriyle ifade edebiliriz:
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None       => None,
        Some(food) => have_ingredients(food),
    }
}

// Daha derli toplu olarak `and_then()` ile yazılabilir:
fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

// Aksi halde `Option<Option<Food>>` değerini `flatten()` ile açmamız gerekirdi:
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
```

👉 Bu örnekte `and_then()`, `Option<Option<T>>` yerine doğrudan `Option<T>` döndürerek daha temiz ve anlaşılır bir akış sağlar.
