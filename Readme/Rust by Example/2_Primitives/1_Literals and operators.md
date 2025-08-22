## 🔢 Literaller ve Operatörler (literals and operators)

Tamsayılar (integers) `1`, kayan nokta sayılar (floats) `1.2`, karakterler (characters) `'a'`, dizgiler (strings) `"abc"`, boolean’lar (booleans) `true` ve birim türü (unit type) `()` literaller (literals) kullanılarak ifade edilebilir.

Tamsayılar ayrıca önekler kullanılarak farklı tabanlarda da ifade edilebilir:

* Onaltılık (hexadecimal) → `0x`
* Sekizlik (octal) → `0o`
* İkilik (binary) → `0b`

Okunabilirliği artırmak için sayısal literallerin içine alt çizgi (`_`) eklenebilir. Örneğin:

* `1_000` ile `1000` aynıdır.
* `0.000_001` ile `0.000001` aynıdır.

Rust ayrıca bilimsel gösterimi (scientific E-notation) destekler. Örneğin:

* `1e6` → `1.000.000`
* `7.6e-4` → `0.00076`
  Bu türlerin varsayılan tipi `f64`’tür.

Kullandığımız literallerin türünü derleyiciye belirtmemiz gerekir. Şimdilik:

* `u32` soneki (suffix), işaretsiz 32 bit tamsayı (unsigned 32-bit integer)
* `i32` soneki (suffix), işaretli 32 bit tamsayı (signed 32-bit integer)
  anlamına gelir.

Rust’taki operatörler ve öncelikleri, C-benzeri (C-like) dillere benzerdir.

```rust
fn main() {
    // Tamsayı toplama
    println!("1 + 2 = {}", 1u32 + 2);

    // Tamsayı çıkarma
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ `1i32` yerine `1u32` deneyin ve neden türün önemli olduğunu görün

    // Bilimsel gösterim
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Kısa devre yapan (short-circuiting) mantıksal işlemler
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bit düzeyinde (bitwise) işlemler
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Okunabilirliği artırmak için alt çizgi kullanın!
    println!("One million is written as {}", 1_000_000u32);
}
```

👉 Bu örnek, Rust’ta literallerin farklı türlerde nasıl yazılabileceğini ve operatörlerin nasıl çalıştığını göstermektedir.
