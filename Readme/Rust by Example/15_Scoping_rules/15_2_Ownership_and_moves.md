## 📦 Sahiplik ve Taşımalar (ownership and moves)

Değişkenler kendi kaynaklarını (resources) serbest bırakmaktan sorumlu oldukları için, her kaynağın yalnızca bir sahibi olabilir. Bu, kaynakların birden fazla kez serbest bırakılmasını engeller. Ancak her değişken kaynakların sahibi değildir (örneğin, referanslar (references) kaynaklara sahip değildir).

Atama (`let x = y`) veya fonksiyon argümanlarını değer ile (by value) geçirme (`foo(x)`) işlemleri yapıldığında, kaynakların sahipliği aktarılır. Rust terminolojisinde bu işleme **move (taşıma)** denir.

Kaynak taşındıktan sonra, önceki sahip artık kullanılamaz. Bu durum, asılı (dangling) işaretçilerin oluşmasını engeller.

```rust
// Bu fonksiyon öbekte (heap) tahsis edilen belleğin sahipliğini alır
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` yok edilir ve bellek serbest bırakılır
}

fn main() {
    // Yığında (stack) tahsis edilen tamsayı
    let x = 5u32;

    // `x`'i `y`'ye *kopyala* — hiçbir kaynak taşınmaz
    let y = x;

    // Her iki değer bağımsız olarak kullanılabilir
    println!("x is {}, and y is {}", x, y);

    // `a`, öbekte (heap) tahsis edilmiş bir tamsayıya işaret eder
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // `a`'yı `b`'ye *taşı*
    let b = a;
    // İşaretçi adresi `a`’dan `b`’ye kopyalanır (veri değil).
    // Her ikisi de aynı öbekteki veriye işaret eder, fakat artık
    // sahibi yalnızca `b`’dir.
    
    // Hata! `a` artık verilere erişemez, çünkü öbek belleğin sahibi değildir
    //println!("a contains: {}", a);
    // TODO ^ Bu satırı açmayı deneyin

    // Bu fonksiyon öbek belleğin sahipliğini `b`'den alır
    destroy_box(b);

    // Öbek belleği bu noktada serbest bırakıldığı için,
    // belleğe erişmeye çalışmak yasaktır
    // Hata! Yukarıdakiyle aynı nedenle
    //println!("b contains: {}", b);
    // TODO ^ Bu satırı açmayı deneyin
}
```

👉 Bu örnekte görüldüğü gibi, taşımadan (move) sonra önceki sahip (`a` veya `b`) artık kullanılamaz. Bu, Rust’un **çift serbest bırakma (double free)** ve **asılı işaretçi (dangling pointer)** hatalarını önlemesini sağlar.
