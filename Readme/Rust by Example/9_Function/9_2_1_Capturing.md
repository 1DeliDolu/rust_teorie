## 📦 Yakalama (capturing)

Kapanışlar (closures) doğaları gereği esnektir ve kapanışın çalışmasını sağlamak için gerek duyulan işlevselliği, ek açıklama (annotation) olmadan yerine getirirler. Bu, yakalamanın (capturing) kullanım durumuna göre esnek şekilde uyum sağlamasına olanak tanır; bazen taşıyarak (move), bazen ödünç alarak (borrow). Kapanışlar değişkenleri şu yollarla yakalayabilir:

* Referansla (`&T`)
* Değiştirilebilir referansla (`&mut T`)
* Değerle (`T`)

Varsayılan olarak kapanışlar değişkenleri referansla yakalar ve yalnızca gerektiğinde diğer yolları kullanır.

```rust
fn main() {
    use std::mem;
    
    let color = String::from("green");

    // `color`’ı yazdıran bir kapanış. Hemen `color`’ı (&) ödünç alır
    // ve hem ödünç almayı hem de kapanışı `print` değişkenine saklar.
    // Bu, `print` son kez kullanılana kadar ödünç alınmış olarak kalır.
    //
    // `println!` yalnızca argümanları değiştirilemez referansla gerektirir,
    // bu nedenle daha kısıtlayıcı bir şey dayatmaz.
    let print = || println!("`color`: {}", color);

    // Kapanışı ödünç alma ile çağır.
    print();

    // `color` tekrar değiştirilemez şekilde ödünç alınabilir çünkü
    // kapanış yalnızca `color`’a değiştirilemez bir referans tutar.
    let _reborrow = &color;
    print();

    // `print` son kez kullanıldıktan sonra taşıma (move) veya yeniden ödünç alma mümkündür.
    let _color_moved = color;


    let mut count = 0;
    // `count`’u artıran bir kapanış ya `&mut count` ya da `count` alabilir.
    // Ancak `&mut count` daha az kısıtlayıcı olduğundan onu alır.
    // Hemen `count`’u ödünç alır.
    //
    // İçeride `&mut` saklandığı için `inc` üzerinde `mut` gereklidir.
    // Bu nedenle kapanış çağrıldığında `count` değişir ve bu da `mut` ister.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Kapanışı değiştirilebilir ödünç alma ile çağır.
    inc();

    // Kapanış daha sonra çağrılacağı için hâlâ `count`’u değiştirilebilir şekilde ödünç alır.
    // Yeniden ödünç almaya çalışmak hata verecektir.
    // let _reborrow = &count; 
    // ^ TODO: bu satırı yorumdan çıkarıp deneyin.
    inc();

    // Kapanış artık `&mut count`’u ödünç almak zorunda değildir.
    // Bu nedenle yeniden ödünç almak hatasızdır.
    let _count_reborrowed = &mut count; 

    
    // Kopyalanamayan (non-copy) bir tür.
    let movable = Box::new(3);

    // `mem::drop` türü `T` ister, bu nedenle değerle almak zorundadır.
    // Kopyalanabilen bir tür olsaydı kapanışa kopyalanırdı ve asıl değer kalırdı.
    // Kopyalanamayan tür taşınmak zorundadır, bu yüzden `movable` kapanışa hemen taşınır.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` değişkeni tüketir, bu yüzden yalnızca bir kez çağrılabilir.
    consume();
    // consume();
    // ^ TODO: bu satırı yorumdan çıkarıp deneyin.
}
```

## 🔒 `move` ile yakalama

Dikey çizgiler (`||`) öncesinde `move` kullanmak, kapanışın yakalanan değişkenlerin sahipliğini (ownership) almasını zorunlu kılar:

```rust
fn main() {
    // `Vec` kopyalanamayan (non-copy) semantiğe sahiptir.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ Yukarıdaki satır yorumdan çıkarılırsa derleme hatası olur
    // çünkü borrow checker taşınmış bir değişkenin tekrar kullanılmasına izin vermez.
    
    // Kapanışın imzasından `move` kaldırılırsa kapanış `haystack`’i
    // değiştirilemez (immutable) ödünç alır. Bu durumda `haystack`
    // hâlâ kullanılabilir olur ve yukarıdaki satırı yorumdan çıkarmak hata vermez.
}
```
