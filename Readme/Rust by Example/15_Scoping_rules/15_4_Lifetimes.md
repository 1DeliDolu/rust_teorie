## ⏳ Yaşam Süreleri (lifetimes)

Bir yaşam süresi (lifetime), derleyicinin (özellikle borrow checker’ın) tüm ödünç almaların geçerli olmasını sağlamak için kullandığı bir yapıdır. Özellikle, bir değişkenin yaşam süresi oluşturulduğunda başlar ve yok edildiğinde sona erer. Yaşam süreleri (lifetimes) ve kapsamlar (scopes) genellikle birlikte anılsa da aynı şey değildir.

Örneğin, bir değişkeni `&` ile ödünç aldığımız durumu ele alalım. Ödünç alma, nerede tanımlandığına göre belirlenen bir yaşam süresine sahiptir. Sonuç olarak, ödünç alma, ödünç veren yok edilmeden önce biterse geçerlidir. Ancak ödünç almanın kapsamı, referansın nerede kullanıldığına bağlıdır.

Aşağıdaki örnekte ve bu bölümün geri kalanında yaşam sürelerinin kapsamlarla nasıl ilişkili olduğunu ve ikisinin nasıl farklılaştığını göreceğiz.

```rust
// Yaşam süreleri, her değişkenin oluşturulup yok edildiğini gösteren çizgilerle belirtilmiştir.
// `i`, kapsamı hem `borrow1` hem de `borrow2`'yu tamamen kapsadığı için en uzun yaşam süresine sahiptir. 
// `borrow1` ve `borrow2` sürelerinin birbirine göre uzunluğu önemsizdir çünkü birbirinden ayrıdırlar.
fn main() {
    let i = 3; // `i` için yaşam süresi başlar. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` yaşam süresi başlar. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1` biter. ─────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` yaşam süresi başlar. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` biter. ─────────────────────────────────┘│
    //                                                     │
}   // `i` yaşam süresi biter. ──────────────────────────────┘ 
```

Dikkat edilmelidir ki yaşam sürelerini etiketlemek için herhangi bir ad veya tür atanmaz. Bu durum, yaşam sürelerinin nasıl kullanılabileceğini sınırlamaktadır; ilerleyen kısımlarda bunu göreceğiz.
