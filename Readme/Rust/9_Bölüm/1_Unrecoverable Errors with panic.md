## 💥 `panic!` ile Kurtarılamaz Hatalar (unrecoverable errors)

Bazen kodunuzda kötü şeyler olur ve yapabileceğiniz hiçbir şey yoktur. Bu durumlarda Rust, `panic!` makrosuna sahiptir. Pratikte panik oluşmasının iki yolu vardır: kodunuzun panik oluşturmasına sebep olacak bir işlem yapmak (örneğin bir dizinin sonunun ötesine erişmek) ya da doğrudan `panic!` makrosunu çağırmak. Her iki durumda da programımızda panik meydana gelir. Varsayılan olarak bu panikler bir hata mesajı yazdırır, stack’i temizleyerek (unwind) programı sonlandırır. Bir ortam değişkeni (environment variable) aracılığıyla, panik oluştuğunda Rust’ın çağrı yığınını (call stack) da göstermesini sağlayabilirsiniz, böylece panik kaynağını bulmak kolaylaşır.

### 🔄 Panik Karşısında Stack’i Çözmek (unwinding) veya Durdurmak (abort)

Varsayılan olarak bir panik oluştuğunda program *unwinding* işlemine başlar; yani Rust stack üzerinde geri doğru ilerler ve her fonksiyondaki verileri temizler. Ancak geri gitmek ve temizlemek yoğun bir işlemdir. Bu nedenle Rust, alternatif olarak programı anında durdurma (abort) seçeneğini sunar; bu durumda program temizleme yapmadan sonlanır.

Bu durumda programın kullandığı bellek, işletim sistemi tarafından temizlenmek zorundadır. Eğer projenizde oluşan ikili dosyanın (binary) mümkün olduğunca küçük olmasını istiyorsanız, `Cargo.toml` dosyanızdaki uygun `[profile]` bölümüne `panic = 'abort'` ekleyerek panik durumunda *unwinding* yerine *abort* seçeneğine geçebilirsiniz. Örneğin, release modunda panikte abort yapmak için:

```toml
[profile.release]
panic = 'abort'
```

### 📝 Basit Bir panic! Çağrısı

**Dosya adı:** `src/main.rs`

```rust
// This code panics!
fn main() {
    panic!("crash and burn");
}
```

Çalıştırdığınızda şuna benzer bir çıktı alırsınız:

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`

thread 'main' panicked at src/main.rs:2:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Buradaki `panic!` çağrısı son iki satırdaki hata mesajını üretir. İlk satır panik mesajımızı ve panik oluşan kaynak kod konumunu gösterir: `src/main.rs:2:5` ifadesi, `src/main.rs` dosyamızın ikinci satırının beşinci karakterini işaret eder.

Bu örnekte işaret edilen satır bizim kodumuzun içindedir ve oraya gittiğimizde `panic!` makrosu çağrısını görürüz. Ancak başka durumlarda `panic!` çağrısı, bizim kodumuzun çağırdığı başka bir kütüphanede olabilir ve hata mesajında belirtilen satır numarası bizim değil, başkasının kodunu gösterir.

Bu durumda, panik kaynağını anlamak için fonksiyonların geri izini (backtrace) kullanırız.

### 📊 Vektörün Sınırlarının Ötesine Erişmek (Listing 9-1)

**Dosya adı:** `src/main.rs`

```rust
// This code panics!
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

Burada vektörümüzün 100. elemanına (indeks 99, çünkü indeksler sıfırdan başlar) erişmeye çalışıyoruz; ancak vektör yalnızca üç elemana sahip. Bu durumda Rust panik oluşturacaktır. `[]` operatörü bir eleman döndürmelidir, ancak geçersiz bir indeks verirseniz Rust burada döndürülebilecek doğru bir eleman bulamaz.

C dilinde, bir veri yapısının sonunun ötesini okumaya çalışmak tanımsız davranıştır (undefined behavior). Belleğin o konumunda ne varsa döndürülebilir, bu da güvenlik açıklarına yol açabilir. Rust ise böyle bir durumda yürütmeyi durdurur ve programın devam etmesine izin vermez.

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`

thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Bu hata, `main.rs` dosyasının 4. satırında, `v` içindeki 99. indekse erişmeye çalıştığımızı gösterir.

### 🔍 Backtrace Kullanımı (Listing 9-2)

Bir hata oluştuğunda, `RUST_BACKTRACE` ortam değişkenini `0` dışında bir değere ayarlayarak geri izleme (backtrace) alabiliriz:

```bash
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/.../std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/.../core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/.../core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at .../core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at .../core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at .../alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at .../core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Burada 6. satır bizim projemizdeki hatalı kısmı işaret ediyor: `src/main.rs` dosyasının 4. satırı. Dolayısıyla hata araştırmasına, geri izleme çıktısında kendi yazdığımız ilk satırdan başlamak gerekir.

Listing 9-1’de, bilerek panik oluşturacak kod yazdığımız için çözüm, vektör sınırları dışında bir elemana erişmeye çalışmamaktır.

Gelecekte kodunuz panik oluşturduğunda, hangi işlem hangi değerlerle yapılırken bu duruma yol açtığını ve bunun yerine kodun ne yapması gerektiğini araştırmanız gerekir.

---

Bir sonraki bölümde, hatalardan `Result` kullanarak nasıl kurtarılabileceğimizi inceleyeceğiz.
