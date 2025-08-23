## 💥 panic!

`panic!` makrosu, bir panik üretmek ve yığının (stack) çözülmesini (unwinding) başlatmak için kullanılabilir. Yığın çözülürken çalışma zamanı, iş parçacığının (thread) sahip olduğu tüm kaynakları, ilgili nesnelerin yıkıcılarını (destructor) çağırarak serbest bırakır.

Tek iş parçacıklı (single-threaded) programlarla çalıştığımız için, `panic!` programın panik mesajını bildirmesine ve çıkmasına neden olur.

```rust
// Tamsayı bölmenin (/) yeniden uygulanması
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Sıfıra bölme panik tetikler
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

// `main` görevi
fn main() {
    // Öbekte ayrılmış tamsayı
    let _x = Box::new(0i32);

    // Bu işlem bir görev hatası tetikleyecek
    division(3, 0);

    println!("This point won't be reached!");

    // `_x` bu noktada yok edilmelidir
}
```

👉 Bu örnekte görüldüğü gibi, `panic!` bellek sızıntısına neden olmaz.

---

### 🔍 Bellek sızıntısı testi (valgrind)

```bash
$ rustc panic.rs && valgrind ./panic
==4401== Memcheck, a memory error detector
==4401== Copyright (C) ...
==4401== Command: ./panic
==4401== 
thread '<main>' panicked at 'division by zero', panic.rs:5
==4401== 
==4401== HEAP SUMMARY:
==4401==     in use at exit: 0 bytes in 0 blocks
==4401==   total heap usage: 18 allocs, 18 frees, 1,648 bytes allocated
==4401== 
==4401== All heap blocks were freed -- no leaks are possible
==4401== 
==4401== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```

👉 Çıktıda görüldüğü üzere, `panic!` sırasında tüm heap blokları serbest bırakılmıştır; yani **bellek sızıntısı (memory leak) yoktur**.
