## 👻 Hayalet Tür Parametreleri (phantom type parameters)

Bir **hayalet tür parametresi (phantom type parameter)**, çalışma zamanında (runtime) görünmeyen ancak yalnızca derleme zamanında (compile time) statik olarak kontrol edilen bir parametredir.

Veri türleri, işaretçi (marker) olarak veya derleme zamanında tür denetimi yapmak amacıyla fazladan generic tür parametreleri kullanabilir. Bu ek parametreler herhangi bir depolama alanı tutmaz ve çalışma zamanı davranışına sahip değildir.

Aşağıdaki örnekte, `std::marker::PhantomData` yapısını hayalet tür parametresi kavramıyla birleştirerek farklı veri türlerini içeren tuple’lar oluşturuyoruz:

```rust
use std::marker::PhantomData;

// `A` üzerinde generic olan, gizli parametre `B` içeren bir hayalet tuple struct.
#[derive(PartialEq)] // Bu tür için eşitlik testine izin verir.
struct PhantomTuple<A, B>(A, PhantomData<B>);

// `A` üzerinde generic olan, gizli parametre `B` içeren bir hayalet struct.
#[derive(PartialEq)] // Bu tür için eşitlik testine izin verir.
struct PhantomStruct<A, B> { 
    first: A, 
    phantom: PhantomData<B> 
}

// Not: Generic tür `A` için depolama alanı ayrılır, ancak `B` için ayrılmaz.
// Bu nedenle `B`, hesaplamalarda kullanılamaz.
```

```rust
fn main() {
    // Burada `f32` ve `f64` gizli parametrelerdir.
    // PhantomTuple türü `<char, f32>` olarak belirtilmiştir.
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // PhantomTuple türü `<char, f64>` olarak belirtilmiştir.
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Tür `<char, f32>` olarak belirtilmiştir.
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // Tür `<char, f64>` olarak belirtilmiştir.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // Derleme zamanı hatası! Tür uyumsuzluğu nedeniyle karşılaştırılamaz:
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);

    // Derleme zamanı hatası! Tür uyumsuzluğu nedeniyle karşılaştırılamaz:
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}
```
