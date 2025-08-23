## 🔑 Girdi Parametreleri (as input parameters)

Rust kapanışların (closure) değişkenleri nasıl yakalayacağına çoğunlukla anlık karar verir ve bu sırada tür ek açıklamasına (type annotation) gerek yoktur. Ancak bu belirsizliğe fonksiyon yazarken izin verilmez. Bir kapanış fonksiyonun girdi parametresi olarak alındığında, kapanışın tam türü birkaç trait’ten biriyle ek açıklama yapılmalıdır. Bu trait’ler kapanışın yakalanan değerle ne yaptığına göre belirlenir. Kısıtlama derecesine göre sıralama şöyledir:

* `Fn`: kapanış yakalanan değeri referansla (`&T`) kullanır
* `FnMut`: kapanış yakalanan değeri değiştirilebilir referansla (`&mut T`) kullanır
* `FnOnce`: kapanış yakalanan değeri değerle (`T`) kullanır

Derleyici, her değişken için mümkün olan en az kısıtlayıcı yöntemi seçecektir.

Örneğin, bir parametre `FnOnce` olarak ek açıklama yapıldığında, kapanışın `&T`, `&mut T` veya `T` ile yakalaması mümkündür. Ancak derleyici, kapanış içindeki kullanımına göre son kararı verir.

Bunun sebebi, eğer bir taşıma (move) mümkünse, her türlü ödünç alma (`&T` veya `&mut T`) da mümkün olmalıdır. Ancak bunun tersi doğru değildir. Eğer parametre `Fn` olarak ek açıklanırsa, `&mut T` veya `T` ile yakalama mümkün değildir. Sadece `&T` geçerlidir.

Aşağıdaki örnekte `Fn`, `FnMut` ve `FnOnce` kullanımını değiştirip sonucu gözlemleyin:

```rust
// Bir kapanışı argüman olarak alan ve çağıran fonksiyon.
// <F> "Genel tür parametresi" (generic type parameter) olduğunu gösterir.
fn apply<F>(f: F) where
    // Kapanış girdi almaz ve hiçbir şey döndürmez.
    F: FnOnce() {
    // ^ TODO: Bunu `Fn` veya `FnMut` olarak değiştirmeyi deneyin.

    f();
}

// Bir kapanışı alan ve `i32` döndüren fonksiyon.
fn apply_to_3<F>(f: F) -> i32 where
    // Kapanış bir `i32` alır ve bir `i32` döndürür.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // Kopyalanamayan (non-copy) bir tür.
    // `to_owned`, ödünç alınan veriden sahip olunan veri oluşturur.
    let mut farewell = "goodbye".to_owned();

    // İki değişkeni yakala: `greeting` referansla, `farewell` değerle.
    let diary = || {
        // `greeting` referansla: `Fn` gerektirir.
        println!("I said {}.", greeting);

        // Değişiklik yapılması `farewell`’in
        // değiştirilebilir referansla yakalanmasına neden olur. Artık `FnMut` gerekir.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // `drop` çağrılması `farewell`’i
        // değerle yakalamaya zorlar. Artık `FnOnce` gerekir.
        mem::drop(farewell);
    };

    // Kapanışı uygulayan fonksiyonu çağır.
    apply(diary);

    // `double`, `apply_to_3`’ün trait sınırını karşılar.
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
```
