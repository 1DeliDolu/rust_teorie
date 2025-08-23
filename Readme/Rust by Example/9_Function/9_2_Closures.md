## 🔒 Kapanışlar (closures)

Kapanışlar (closures), çevreleyen ortamı yakalayabilen fonksiyonlardır. Örneğin, `x` değişkenini yakalayan bir kapanış:

```
|val| val + x
```

Kapanışların sözdizimi ve yetenekleri, onları anlık (on the fly) kullanım için oldukça elverişli kılar. Bir kapanış çağırmak, fonksiyon çağırmaya tamamen benzer. Ancak, hem giriş hem de dönüş türleri çıkarımsal olarak belirlenebilir ve giriş değişken isimleri mutlaka belirtilmelidir.

Kapanışların diğer özellikleri şunlardır:

* Giriş değişkenlerinin etrafında `()` yerine `||` kullanılır.
* Tek satırlık ifadelerde gövde sınırlayıcı `{}` isteğe bağlıdır (aksi durumda zorunludur).
* Dış ortam değişkenlerini yakalayabilme yeteneğine sahiptirler.

```rust
fn main() {
    let outer_var = 42;
    
    // Normal bir fonksiyon, çevreleyen ortam içindeki değişkenlere başvuramaz
    //fn function(i: i32) -> i32 { i + outer_var }
    // TODO: Yukarıdaki satırın yorumunu kaldırın ve derleyici hatasını görün.
    // Derleyici, bunun yerine bir kapanış tanımlamanızı önerir.

    // Kapanışlar anonimdir, burada onları referanslara bağlıyoruz.
    // Açıklama (annotation) fonksiyon açıklaması ile aynıdır ancak isteğe bağlıdır,
    // tıpkı gövdeyi saran `{}` gibi. Bu isimsiz fonksiyonlar
    // uygun şekilde adlandırılmış değişkenlere atanır.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;

    // Kapanışları çağırma.
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // Bir kapanışın türü bir kez çıkarıldığında, başka bir tür ile yeniden çıkarılamaz.
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: Yukarıdaki satırın yorumunu kaldırın ve derleyici hatasını görün.

    // Hiç argüman almayan ve bir `i32` döndüren bir kapanış.
    // Dönüş türü çıkarımsaldır.
    let one = || 1;
    println!("closure returning one: {}", one());
}
```
