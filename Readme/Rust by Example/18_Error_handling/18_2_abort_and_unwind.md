## 🔄 Abort ve Unwind (abort and unwind)

Önceki bölüm, hata yönetim mekanizması `panic`i göstermektedir. Farklı kod yolları, `panic` ayarına bağlı olarak koşullu derlenebilir. Mevcut değerler `unwind` ve `abort`tur.

Limonata örneği üzerine kurarak, farklı kod satırlarını çalıştırmak için `panic` stratejisini açıkça kullanıyoruz:

```rust
fn drink(beverage: &str) {
    // Çok fazla şekerli içecek içmemelisiniz.
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!!");
        } else {
            println!("Spit it out!!!!");
        }
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
}
```

👉 Bu örnekte, `panic` stratejisi `abort` veya `unwind` değerine göre farklı çıktılar üretir.

Başka bir örnek olarak, `drink()` fonksiyonunu yeniden yazarak `unwind` anahtar kelimesini açıkça kullanıyoruz:

```rust
#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!!!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!!!!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
}
```

👉 Burada, `ah()` fonksiyonunun hangi sürümünün çağrılacağı `panic` stratejisine bağlıdır.

`panic` stratejisi, komut satırında `abort` veya `unwind` kullanılarak ayarlanabilir:

```bash
rustc lemonade.rs -C panic=abort
```

👉 Bu komut, derlemeyi `abort` stratejisi ile yapar.
