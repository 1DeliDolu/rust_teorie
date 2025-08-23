## ⛔ Sapmayan Fonksiyonlar (diverging functions)

**Sapmayan fonksiyonlar** (diverging functions) asla dönmezler. Bunlar `!` ile işaretlenir, ki bu da boş bir türdür (empty type).

```rust
fn foo() -> ! {
    panic!("This call never returns.");
}
```

Diğer tüm türlerin aksine, bu tür örneklenemez (instantiate) çünkü bu türün alabileceği değer kümesi boştur. Burada dikkat edilmesi gereken nokta, `()` türünden farklı olduğudur. `()` türü tam olarak bir tek değere sahiptir.

Örneğin, aşağıdaki fonksiyon normal şekilde döner, her ne kadar dönüş değerinde herhangi bir bilgi olmasa da:

```rust
fn some_fn() {
    ()
}

fn main() {
    let _a: () = some_fn();
    println!("This function returns and you can see this line.");
}
```

Buna karşılık, aşağıdaki fonksiyon hiçbir zaman kontrolü çağırana geri vermez:

```rust
#![feature(never_type)]

fn main() {
    let x: ! = panic!("This call never returns.");
    println!("You will never see this line!");
}
```

Bu soyut bir kavram gibi görünse de, aslında oldukça faydalıdır ve sıkça kullanışlı olur. Bu türün en büyük avantajı, **herhangi bir türe dönüştürülebilmesidir**. Bu da özellikle `match` kollarında belirli bir türün gerektiği durumlarda çok yönlülük sağlar. Bu esneklik, aşağıdaki gibi kod yazmamıza imkân tanır:

```rust
fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Bu match ifadesinin dönüş türü u32 olmalıdır,
            // çünkü "addition" değişkeni u32 türündedir.
            let addition: u32 = match i % 2 == 1 {
                // "i" değişkeni u32 türündedir, bu sorun değil.
                true => i,
                // "continue" ifadesi u32 döndürmez, ama sorun yok,
                // çünkü asla dönmez ve bu yüzden match ifadesinin tür
                // gereksinimlerini ihlal etmez.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
```

Ayrıca, sonsuza kadar dönen fonksiyonların (`loop {}`) dönüş türü de budur. Örneğin: ağ sunucuları veya süreci sonlandıran fonksiyonlar (`exit()`).
