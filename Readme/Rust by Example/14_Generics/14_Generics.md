## 🌐 Generics

**Generics**, türleri ve işlevsellikleri daha geniş durumlara genelleme konusudur. Bu, kod tekrarını azaltmak için son derece faydalıdır ancak kimi zaman karmaşık bir sözdizimi gerektirebilir. Özellikle, jenerik (generic) tanımlarken hangi türler üzerinde geçerli olduğunu dikkatle belirtmek gerekir. Generics’in en basit ve en yaygın kullanımı **tür parametreleri (type parameters)** için yapılır.

Bir tür parametresi, **açılı parantezler** (`<...>`) ve **UpperCamelCase** yazımı ile belirtilir: `<Aaa, Bbb, ...>`.
Genellikle **jenerik tür parametreleri** `<T>` ile ifade edilir. Rust’ta, bir veya daha fazla jenerik tür parametresi `<T>` kabul eden her şey "generic" olarak adlandırılır. Jenerik tür parametresi olarak belirtilen her şey **generic**, diğer her şey ise **somut (concrete)** kabul edilir.

Örneğin, herhangi bir türden `T` argümanı alan `foo` adında jenerik bir fonksiyon tanımlamak için:

```rust
fn foo<T>(arg: T) { ... }
```

Burada `T`, `<T>` ile jenerik tür parametresi olarak belirtildiği için `(arg: T)` kullanımında jenerik kabul edilir. Bu, `T` daha önce bir `struct` olarak tanımlanmış olsa bile geçerlidir.

### 📝 Örnek – Sözdiziminin Kullanımı

```rust
// Somut (concrete) bir tür `A`.
struct A;

// `Single` türü tanımlanırken, `A` ilk kez `<A>` olmadan kullanıldı.
// Bu nedenle `Single`, somut bir türdür ve yukarıda tanımlanan `A`’yı kullanır.
struct Single(A);
//            ^ Burada `Single`’ın `A` türünü ilk kullanımı var.

// Burada `<T>`, `T`’nin ilk kullanımından önce geldiği için `SingleGen` jenerik bir türdür.
// Jenerik tür parametresi `T`, her şey olabilir. Yukarıda tanımlanan somut tür `A` dahil.
struct SingleGen<T>(T);

fn main() {
    // `Single` somuttur ve açıkça `A` alır.
    let _s = Single(A);
    
    // `SingleGen<char>` türünden `_char` adında bir değişken oluşturuluyor
    // ve ona `SingleGen('a')` değeri veriliyor.
    // Burada `SingleGen`’in tür parametresi açıkça belirtilmiş.
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen`’in tür parametresi dolaylı (implicit) olarak da belirtilebilir:
    let _t    = SingleGen(A);   // Yukarıda tanımlanan `A`’yı kullanır.
    let _i32  = SingleGen(6);   // `i32` türünü kullanır.
    let _char = SingleGen('a'); // `char` türünü kullanır.
}
```
