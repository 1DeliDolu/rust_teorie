## 🔀 Dönüşüm (conversion)

İlkel türler (primitive types), tür dönüştürme (casting) ile birbirine dönüştürülebilir.

Rust, özel türler (custom types) (örneğin `struct` ve `enum`) arasındaki dönüşümü **trait** kullanarak ele alır. Genel dönüşümler için `From` ve `Into` trait’leri kullanılır. Bununla birlikte, özellikle `String` türüne veya `String` türünden yapılan dönüşümlerde daha özel trait’ler bulunmaktadır.
