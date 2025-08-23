## 🛠️ RAII (Resource Acquisition Is Initialization)

Rust’ta değişkenler yalnızca yığıtta (stack) veri tutmakla kalmaz; aynı zamanda kaynakların (resources) da sahibi olurlar. Örneğin, `Box<T>` yığında değil, öbekte (heap) bellek tahsis eder ve ona sahip olur. Rust, RAII (Resource Acquisition Is Initialization) kuralını uygular; yani bir nesne kapsam (scope) dışına çıktığında, onun yıkıcı (destructor) fonksiyonu çağrılır ve sahip olduğu kaynaklar serbest bırakılır.

Bu davranış, kaynak sızıntısı (resource leak) hatalarına karşı koruma sağlar; bu sayede belleği manuel olarak serbest bırakmanız ya da bellek sızıntıları konusunda endişelenmeniz gerekmez. İşte hızlı bir örnek:

```rust
// raii.rs
fn create_box() {
    // Öbekte (heap) bir tamsayı ayır
    let _box1 = Box::new(3i32);

    // `_box1` burada yok edilir ve bellek serbest bırakılır
}

fn main() {
    // Öbekte (heap) bir tamsayı ayır
    let _box2 = Box::new(5i32);

    // İç içe kapsam (nested scope):
    {
        // Öbekte (heap) bir tamsayı ayır
        let _box3 = Box::new(4i32);

        // `_box3` burada yok edilir ve bellek serbest bırakılır
    }

    // Eğlencesine bir sürü kutu oluştur
    // Belleği manuel olarak serbest bırakmaya gerek yok!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` burada yok edilir ve bellek serbest bırakılır
}
```

👉 Bu örnekte, her `Box` kapsamdan çıktığında otomatik olarak yok edilir ve bellek sızıntısı olmaz.

Bellek hatalarını kontrol etmek için `valgrind` kullanabiliriz:

```
$ rustc raii.rs && valgrind ./raii
==26873== Memcheck, a memory error detector
==26873== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==26873== Using Valgrind-3.9.0 and LibVEX; rerun with -h for copyright info
==26873== Command: ./raii
==26873==
==26873== HEAP SUMMARY:
==26873==     in use at exit: 0 bytes in 0 blocks
==26873==   total heap usage: 1,013 allocs, 1,013 frees, 8,696 bytes allocated
==26873==
==26873== All heap blocks were freed -- no leaks are possible
==26873==
==26873== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 2 from 2)
```

👉 Çıktıda görüldüğü gibi hiçbir bellek sızıntısı yoktur.

---

## 🧹 Yıkıcı (Destructor)

Rust’ta yıkıcı kavramı `Drop` trait’i aracılığıyla sağlanır. Bir kaynak kapsam dışına çıktığında bu yıkıcı çağrılır. Her tür için `Drop` trait’ini uygulamak zorunlu değildir; yalnızca kendi özel yıkıcı mantığınıza ihtiyaç duyuyorsanız bunu uygularsınız.

Aşağıdaki örnek, `Drop` trait’inin nasıl çalıştığını göstermektedir. `main` fonksiyonundaki değişken kapsam dışına çıktığında özel yıkıcı tetiklenecektir:

```rust
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}
```

👉 Bu kod çalıştırıldığında önce `"Made a ToDrop!"`, ardından kapsam sona erdiğinde `"ToDrop is being dropped"` yazdırılır.
