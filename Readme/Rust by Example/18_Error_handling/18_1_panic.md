## 💥 Panic (panic)

Göreceğimiz en basit hata yönetim mekanizması `panic`tir. Bir hata mesajı yazdırır, stack çözülmesini (unwinding) başlatır ve genellikle programdan çıkar. Burada hata koşulumuzda açıkça `panic` çağrısı yapıyoruz:

```rust
fn drink(beverage: &str) {
    // Çok fazla şekerli içecek içmemelisiniz.
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}
```

👉 İlk `drink` çağrısı çalışır. İkincisi `panic` ile programı sonlandırır ve bu yüzden üçüncü çağrı hiç çalışmaz.
