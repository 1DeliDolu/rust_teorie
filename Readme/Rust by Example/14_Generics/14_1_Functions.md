## 🔧 Fonksiyonlar (Functions)

Aynı kurallar fonksiyonlar için de geçerlidir: Bir tür `T`, `<T>` ile birlikte yazıldığında jenerik (generic) olur.

Jenerik fonksiyonlar kullanılırken bazen tür parametrelerini **açıkça belirtmek** gerekir. Bu durum, fonksiyonun çağrıldığı yerde dönüş türü jenerik olduğunda veya derleyicinin gerekli tür parametrelerini çıkarsamak için yeterli bilgiye sahip olmadığı durumlarda ortaya çıkar.

Tür parametreleri açıkça belirtilmiş bir fonksiyon çağrısı şu şekilde görünür:

```rust
fun::<A, B, ...>()
```

### 📝 Örnek

```rust
struct A;          // Somut tür `A`.
struct S(A);       // Somut tür `S`.
struct SGen<T>(T); // Jenerik tür `SGen`.

// Aşağıdaki fonksiyonların tümü kendilerine verilen değişkenin sahipliğini alır
// ve hemen kapsamdan çıktıkları için değişkeni serbest bırakır.

// `reg_fn` fonksiyonunu tanımlıyoruz. Argüman olarak `S` alır.
// `<T>` bulunmadığı için bu jenerik bir fonksiyon değildir.
fn reg_fn(_s: S) {}

// `gen_spec_t` fonksiyonu `SGen<A>` alır.
// Burada `A` tipi açıkça verilmiştir. Ancak `gen_spec_t` için `<T>` tanımlı olmadığından
// bu fonksiyon jenerik değildir.
fn gen_spec_t(_s: SGen<A>) {}

// `gen_spec_i32` fonksiyonu `SGen<i32>` alır.
// Burada `i32` açıkça verilmiştir. `i32` jenerik bir tür olmadığından
// bu fonksiyon da jenerik değildir.
fn gen_spec_i32(_s: SGen<i32>) {}

// `generic` fonksiyonu `SGen<T>` alır.
// `<T>` önceden tanımlandığı için bu fonksiyon `T` üzerinde jeneriktir.
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Jenerik olmayan fonksiyonların kullanımı
    reg_fn(S(A));          // Somut tür.
    gen_spec_t(SGen(A));   // Dolaylı olarak belirtilmiş `A` tür parametresi.
    gen_spec_i32(SGen(6)); // Dolaylı olarak belirtilmiş `i32` tür parametresi.

    // Tür parametresinin açıkça belirtilmesi (`char`)
    generic::<char>(SGen('a'));

    // Tür parametresinin dolaylı olarak belirtilmesi (`char`)
    generic(SGen('c'));
}
```
