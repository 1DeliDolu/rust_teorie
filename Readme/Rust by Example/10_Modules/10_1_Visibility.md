## 👁️ Görünürlük (visibility)

Varsayılan olarak, bir modüldeki öğelerin görünürlüğü **özel (private)**’dir. Ancak bu durum `pub` belirleyicisiyle (modifier) değiştirilebilir. Yalnızca bir modülün **genel (public)** öğelerine modül kapsamı dışından erişilebilir.

```rust
// `my_mod` adında bir modül
mod my_mod {
    // Modüllerdeki öğeler varsayılan olarak özel görünürlüğe sahiptir.
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Varsayılan görünürlüğü değiştirmek için `pub` kullanılır.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Öğeler aynı modül içindeki diğer öğelere erişebilir,
    // özel olsalar bile.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modüller iç içe de tanımlanabilir
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // `pub(in path)` ile tanımlanan fonksiyonlar yalnızca verilen yol (path) içinde görünürdür.
        // `path`, bir üst veya atadan gelen bir modül olmalıdır.
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // `pub(self)` yalnızca mevcut modül içinde görünürdür,
        // yani özel bırakmakla aynıdır.
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // `pub(super)` yalnızca üst modül içinde görünürdür.
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` bir fonksiyonun yalnızca mevcut crate içinde görünür olmasını sağlar
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // İç içe modüller de aynı görünürlük kurallarını takip eder
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Üst öğe özel olduğunda, alt öğe daha geniş kapsam için
        // görünür ilan edilse bile görünürlük kısıtlanır.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // Modüller aynı ada sahip öğeler arasında ayırt etme imkanı sağlar.
    function();
    my_mod::function();

    // Genel öğelere, iç içe modüller dahil, ebeveyn modül dışından erişilebilir.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // `pub(crate)` öğelerine aynı crate içindeki her yerden erişilebilir
    my_mod::public_function_in_crate();

    // `pub(in path)` yalnızca belirtilen modül içinde çağrılabilir
    // Hata! `public_function_in_my_mod` özeldir
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Bu satırı açmayı deneyin

    // Bir modülün özel öğelerine doğrudan erişilemez,
    // genel bir modülün içinde olsalar bile:

    // Hata! `private_function` özeldir
    //my_mod::private_function();
    // TODO ^ Bu satırı açmayı deneyin

    // Hata! `private_function` özeldir
    //my_mod::nested::private_function();
    // TODO ^ Bu satırı açmayı deneyin

    // Hata! `private_nested` özel bir modüldür
    //my_mod::private_nested::function();
    // TODO ^ Bu satırı açmayı deneyin

    // Hata! `private_nested` özel bir modüldür
    //my_mod::private_nested::restricted_function();
    // TODO ^ Bu satırı açmayı deneyin
}
```
