## 📝 Önce Bildirme (declare first)

Değişken bağlamaları (variable bindings) önce bildirilebilir ve daha sonra başlatılabilir. Ancak tüm değişken bağlamaları kullanılmadan önce başlatılmalıdır; çünkü derleyici, başlatılmamış bir değişken bağlamasının kullanımına izin vermez. Bu durum tanımsız davranışa (undefined behavior) yol açar.

Bir değişken bağlamasının önce bildirilip sonra başlatılması yaygın değildir. Bildirim ile başlatma birbirinden ayrıldığında, okuyucunun değişkenin nerede başlatıldığını bulması daha zor olur. Genellikle değişken bağlaması, kullanılacağı yere yakın bir yerde bildirilir ve başlatılır.

```rust
fn main() {
    // Bir değişken bağlaması bildir
    let a_binding;

    {
        let x = 2;

        // Bağlamayı başlat
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Hata! Başlatılmamış bağlamanın kullanımı
    println!("another binding: {}", another_binding);
    // FIXME ^ Bu satırı yorum satırı yapın

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
```
