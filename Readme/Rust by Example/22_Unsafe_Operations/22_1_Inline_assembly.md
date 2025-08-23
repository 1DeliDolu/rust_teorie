## 🖥️ Satır İçi Assembly (inline assembly)

Rust, `asm!` makrosu aracılığıyla satır içi assembly desteği sağlar. Bu makro, derleyici tarafından üretilen assembly çıktısına elle yazılmış assembly talimatlarının gömülmesine olanak tanır. Genellikle buna ihtiyaç duyulmaz, ancak gerekli performans veya zamanlama başka türlü sağlanamıyorsa kullanılabilir. Düşük seviye donanım birimlerine erişim (örneğin çekirdek kodunda) da bu işlevselliği gerektirebilir.

Not: Buradaki örnekler x86/x86-64 assembly ile verilmiştir, ancak diğer mimariler de desteklenmektedir.

Satır içi assembly şu mimarilerde desteklenmektedir:

* x86 ve x86-64
* ARM
* AArch64
* RISC-V

---

## 🔑 Temel Kullanım (basic usage)

En basit örnekle başlayalım:

```rust
use std::arch::asm;

unsafe {
    asm!("nop");
}
```

Bu, derleyici tarafından üretilen assembly’ye bir `NOP` (no operation) talimatı ekleyecektir. Tüm `asm!` çağrılarının `unsafe` blok içinde olması gerekir çünkü bu makro rastgele talimatlar ekleyebilir ve çeşitli garantileri bozabilir. Eklenen talimatlar, `asm!` makrosunun ilk argümanında string literal olarak listelenir.

---

## 📥📤 Girdiler ve Çıktılar (inputs and outputs)

Şimdi, gerçekten veri üzerinde işlem yapan bir örneğe bakalım:

```rust
use std::arch::asm;

let x: u64;
unsafe {
    asm!("mov {}, 5", out(reg) x);
}
assert_eq!(x, 5);
```

Bu kod, `u64` türündeki `x` değişkenine `5` değerini yazar. Buradaki string literal aslında bir şablon stringtir. Şablon, Rust format string kurallarıyla yönetilir, fakat argümanların görünümü biraz farklıdır.

* Bir değişkenin **girdi mi yoksa çıktı mı** olduğunu belirtmemiz gerekir. Burada çıktı olduğu için `out` kullanılmıştır.
* Hangi tür register’da kullanılacağını da belirtmek gerekir. Bu örnekte `reg` ile herhangi bir genel amaçlı register seçilmiştir.

Başka bir örnekte hem girdi hem de çıktı kullanalım:

```rust
use std::arch::asm;

let i: u64 = 3;
let o: u64;
unsafe {
    asm!(
        "mov {0}, {1}",
        "add {0}, 5",
        out(reg) o,
        in(reg) i,
    );
}
assert_eq!(o, 8);
```

Bu kod, `i` değişkenine `5` ekleyip sonucu `o` değişkenine yazar.

Öne çıkan noktalar:

* `asm!` çok satırlı string şablonlarını destekler.
* Girdiler `in`, çıktılar `out` ile belirtilir.
* Argüman numarası veya ismi kullanılabilir (`{0}`, `{1}` gibi).

Daha basitleştirilmiş versiyon:

```rust
use std::arch::asm;

let mut x: u64 = 3;
unsafe {
    asm!("add {0}, 5", inout(reg) x);
}
assert_eq!(x, 8);
```

Burada `inout` hem girdi hem de çıktı olarak kullanılmıştır.

Ayrı giriş-çıkış değişkenleri de belirtilebilir:

```rust
use std::arch::asm;

let x: u64 = 3;
let y: u64;
unsafe {
    asm!("add {0}, 5", inout(reg) x => y);
}
assert_eq!(y, 8);
```

---

## 🕒 Geç Çıktı Operantları (late output operands)

Derleyici, `out` için register ayırırken temkinlidir ve bu register’ın diğer argümanlarla paylaşılmasına izin vermez. Performans için `lateout` kullanılabilir. Bu, yalnızca tüm girdiler tükendikten sonra yazılacak çıktılar için uygundur. `inlateout` varyantı da mevcuttur.

Örnek (yanlış sonuç verebilecek senaryo):

```rust
use std::arch::asm;

let mut a: u64 = 4;
let b: u64 = 4;
let c: u64 = 4;
unsafe {
    asm!(
        "add {0}, {1}",
        "add {0}, {2}",
        inout(reg) a,
        in(reg) b,
        in(reg) c,
    );
}
assert_eq!(a, 12);
```

Eğer `inlateout` kullanılsaydı, optimize edilmiş durumda sonuç `16` olabilirdi.

Doğru kullanım örneği:

```rust
use std::arch::asm;

let mut a: u64 = 4;
let b: u64 = 4;
unsafe {
    asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);
}
assert_eq!(a, 8);
```

---

## 🎯 Açık Register Operantları (explicit register operands)

Bazı talimatlar yalnızca belirli register’larda çalışır. Bu durumda `eax`, `ebx` gibi açık register isimleri kullanılmalıdır.

```rust
use std::arch::asm;

let cmd = 0xd1;
unsafe {
    asm!("out 0x64, eax", in("eax") cmd);
}
```

Bu kod `cmd` değerini `eax` üzerinden `0x64` portuna yazar.

---

## 🗂️ Register Çarpımı Örneği (mul instruction)

```rust
use std::arch::asm;

fn mul(a: u64, b: u64) -> u128 {
    let lo: u64;
    let hi: u64;

    unsafe {
        asm!(
            "mul {}",
            in(reg) a,
            inlateout("rax") b => lo,
            lateout("rdx") hi
        );
    }

    ((hi as u128) << 64) + lo as u128
}
```

Bu, iki `u64` değeri çarpar ve `u128` sonucu üretir. Sonuç `rax:rdx` register ikilisine yazılır.

---

## 🛠️ Bozulmuş Register’lar (clobbered registers)

Bazı durumlarda assembly, doğrudan çıktı olarak kullanılmayan register’ları değiştirir. Bunlara **clobbered** denir. Derleyiciye bu register’ların değiştiğini bildirmek gerekir.

Örnek: `cpuid` ile CPU üretici bilgisini almak

```rust
use std::arch::asm;

fn main() {
    let mut name_buf = [0_u8; 12];

    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov [rdi], ebx",
            "mov [rdi + 4], edx",
            "mov [rdi + 8], ecx",
            "pop rbx",
            in("rdi") name_buf.as_mut_ptr(),
            inout("eax") 0 => _,
            out("ecx") _,
            out("edx") _,
        );
    }

    let name = core::str::from_utf8(&name_buf).unwrap();
    println!("CPU Manufacturer ID: {}", name);
}
```

---

## ⚙️ Ek Özellikler

* **Sembol operantları ve ABI clobbers** → `clobber_abi("C")` kullanılarak çağrı sözleşmesine göre register’ların bozulabileceği belirtilir.
* **Register şablon değiştiriciler** → `:h`, `:l` gibi eklerle register’ın alt bölümlerine erişilir.
* **Bellek adresi operantları** → Mimariye özgü sözdizimi ile kullanılmalıdır (`[ ]`).
* **Etiketler (labels)** → GNU assembler sayısal etiketleri kullanılmalı, ileri `f` ve geri `b` yönleri belirtilmelidir.
* **Seçenekler (options)** → `pure`, `nomem`, `nostack` gibi seçeneklerle derleyiciye daha fazla bilgi verilerek optimizasyonlar sağlanır.

Örnek:

```rust
use std::arch::asm;

let mut a: u64 = 4;
let b: u64 = 4;
unsafe {
    asm!(
        "add {0}, {1}",
        inlateout(reg) a, in(reg) b,
        options(pure, nomem, nostack),
    );
}
assert_eq!(a, 8);
```

---

👉 `asm!` makrosu Rust’ta düşük seviye donanım kontrolü ve performans kritik kod parçaları için güçlü ama dikkatli kullanılmalı olan bir özelliktir.
