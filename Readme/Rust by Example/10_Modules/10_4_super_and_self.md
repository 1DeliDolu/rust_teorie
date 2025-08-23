## 🧭 `super` ve `self` Anahtar Sözcükleri (super and self)

`super` ve `self` anahtar sözcükleri, öğelere erişirken belirsizliği gidermek ve yolları (paths) gereksiz yere sabitlemeyi (hardcoding) önlemek için kullanılabilir.

```rust
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }
    
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    
    pub fn indirect_call() {
        // Bu kapsamdan `function` adındaki tüm fonksiyonlara erişelim!
        print!("called `my::indirect_call()`, that\n> ");
        
        // `self` anahtar sözcüğü mevcut modül kapsamını ifade eder – burada `my`.
        // `self::function()` çağırmak ile `function()` doğrudan çağırmak aynı sonucu verir,
        // çünkü ikisi de aynı fonksiyona karşılık gelir.
        self::function();
        function();
        
        // `self` aynı zamanda `my` içindeki başka bir modüle erişmek için de kullanılabilir:
        self::cool::function();
        
        // `super` anahtar sözcüğü üst kapsamı (yani `my` modülünün dışını) ifade eder.
        super::function();
        
        // Bu, *crate* kapsamındaki `cool::function` fonksiyonuna bağlanır.
        // Burada crate kapsamı en dış kapsama karşılık gelir.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
```

👉 Özet:

* `self` → bulunduğun mevcut modül (`my`)
* `super` → bir üst kapsam (`crate` içindeki `my`’in dışı)
* `crate` → en dış kapsam (kök seviye)
