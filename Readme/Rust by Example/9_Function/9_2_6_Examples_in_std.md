## 🔄 Dönüştürme (conversion)

İlkel türler (primitive types) tür dönüştürme (casting) yoluyla birbirine dönüştürülebilir.

Rust, özel türler (custom types) yani `struct` ve `enum` arasındaki dönüşümleri *trait* kullanarak ele alır. Genel dönüşümler `From` ve `Into` *trait*lerini kullanır. Ancak, daha yaygın durumlar için özellikle `String` türüne ve `String` türünden yapılan dönüşümlerde daha özel *trait*ler bulunur.
