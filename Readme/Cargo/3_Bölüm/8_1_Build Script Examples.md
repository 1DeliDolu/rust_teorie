## 🛠️ Derleme Betiği Örnekleri (build script examples)

Aşağıdaki bölümler, derleme betiklerinin (build scripts) nasıl yazılacağına dair bazı örnekleri göstermektedir.

Bazı yaygın derleme betiği işlevsellikleri, crates.io üzerindeki crate’ler (crates) aracılığıyla bulunabilir. Nelerin mevcut olduğunu görmek için `build-dependencies` (build-dependencies) anahtar sözcüğüne göz atın. Aşağıdaki liste, bazı popüler crate’lerin (crates) örnek bir seçkisidir1:

* **bindgen** — C kütüphaneleri için Rust FFI bağlamalarını (FFI bindings) otomatik olarak üretir.
* **cc** — C/C++/assembly derler.
* **pkg-config** — `pkg-config` yardımcı programını kullanarak sistem kütüphanelerini algılar.
* **cmake** — Yerel bir kütüphaneyi derlemek için `cmake` derleme aracını çalıştırır.
* **autocfg**, **rustc\_version**, **version\_check** — Bu crate’ler (crates), mevcut `rustc` (rustc) bilgilerine — örneğin derleyicinin sürümü — dayalı koşullu derleme (conditional compilation) uygulamanın yollarını sağlar.

## 🧬 Kod Üretimi (code generation)

Bazı Cargo paketlerinin (packages) çeşitli nedenlerle derlenmeden hemen önce kod üretilmesine ihtiyaç duyar. Burada derleme betiğinin (build script) bir parçası olarak bir kütüphane çağrısı üreten basit bir örnek üzerinden ilerleyeceğiz.

Önce bu paketin dizin yapısına bakalım:

```
.
├── Cargo.toml
├── build.rs
└── src
    └── main.rs

1 directory, 3 files
```

Burada bir `build.rs` derleme betiğimiz (build script) ve `main.rs` içinde ikili dosyamız (binary) olduğunu görebiliriz. Bu paketin temel bir bildirimi (manifest) vardır:

```toml
# Cargo.toml

[package]
name = "hello-from-generated-code"
version = "0.1.0"
edition = "2024"
```

Derleme betiğinin (build script) içinde neler olduğuna bakalım:

```rust
// build.rs

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        "
    ).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
```

Burada dikkat edilmesi gereken birkaç nokta vardır:

* Betik, çıktı dosyalarının nereye yerleştirilmesi gerektiğini keşfetmek için `OUT_DIR` ortam değişkenini (environment variable) kullanır. Girdi dosyalarının nerede bulunacağını bulmak için sürecin geçerli çalışma dizinini (current working directory) kullanabilir, ancak bu örnekte herhangi bir girdi dosyamız yoktur.
* Genel olarak, derleme betikleri (build scripts) `OUT_DIR` dışındaki herhangi bir dosyayı değiştirmemelidir. İlk bakışta sorun yok gibi görünse de, bu tür bir crate’i (crate) bağımlılık olarak kullandığınızda sorunlara yol açar; çünkü `.cargo/registry` içindeki kaynakların değişmez (immutable) olduğuna dair örtük bir değişmezlik varsayımı vardır. `cargo`, paketlerken bu tür betiklere izin vermez.
* Bu betik nispeten basittir çünkü yalnızca küçük bir üretilmiş dosya yazar. Örneğin, bir C başlık dosyasından veya başka bir dil tanımından bir Rust modülü (Rust module) üretmek gibi daha karmaşık işlemlerin yapılabileceğini düşünebilirsiniz.
* `rerun-if-changed` (rerun-if-changed) talimatı, derleme betiğinin (build script) yalnızca betiğin kendisi değişirse yeniden çalıştırılması gerektiğini `Cargo`’ya söyler. Bu satır olmadan, paketteki herhangi bir dosya değiştiğinde `Cargo` derleme betiğini otomatik olarak çalıştıracaktır. Kod üretiminiz bazı girdi dosyaları kullanıyorsa, bunların her birinin listesini burada yazdırmanız gerekir.

Şimdi de kütüphanenin (library) içine göz atalım:

```rust
// src/main.rs

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("{}", message());
}
```

Asıl sihir burada gerçekleşir. Kütüphane, derleyici tarafından tanımlanan `include!` makrosunu (include! macro) `concat!` ve `env!` makrolarıyla (macros) birlikte kullanarak üretilen dosyayı (`hello.rs`) crate’in (crate) derlemesine dahil eder.

Burada gösterilen yapı kullanılarak, crate’ler (crates) derleme betiğinden (build script) herhangi sayıda üretilmiş dosyayı dahil edebilirler.

## 🧩 Yerel Bir Kütüphane Derleme (building a native library)

Bazen bir paketin parçası olarak yerel C veya C++ kodunun derlenmesi gerekebilir. Bu, Rust crate’i (crate) derlenmeden önce yerel bir kütüphaneyi (native library) derlemek için derleme betiğinden (build script) yararlanmanın bir başka mükemmel kullanım alanıdır. Örnek olarak, “Hello, World!” yazdırmak için C’ye çağrı yapan bir Rust kütüphanesi (library) oluşturacağız.

Yukarıdaki gibi, önce paket yerleşimine bakalım:

```
.
├── Cargo.toml
├── build.rs
└── src
    ├── hello.c
    └── main.rs

1 directory, 4 files
```

Öncekine oldukça benziyor! Sırada bildirim (manifest) var:

```toml
# Cargo.toml

[package]
name = "hello-world-from-c"
version = "0.1.0"
edition = "2024"
```

Şimdilik herhangi bir derleme bağımlılığı (build dependency) kullanmayacağız, bu yüzden şimdi derleme betiğine (build script) bakalım:

```rust
// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/hello.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo::rustc-link-search=native={}", out_dir);
    println!("cargo::rustc-link-lib=static=hello");
    println!("cargo::rerun-if-changed=src/hello.c");
}
```

Bu derleme betiği (build script), C dosyamızı bir nesne dosyasına derleyerek (gcc çağrısı yaparak) başlar ve ardından bu nesne dosyasını statik bir kütüphaneye (static library) dönüştürür (`ar` çağrısı yaparak). Son adımda ise çıktımızın `out_dir` içinde olduğunu ve derleyicinin crate’i (crate) `libhello.a` ile statik olarak bağlaması gerektiğini (`-l static=hello` bayrağı aracılığıyla) `Cargo`’ya geri bildirir.

Bu sabit kodlanmış yaklaşımla ilgili birkaç sakınca olduğuna dikkat edin:

* `gcc` komutunun kendisi platformlar arasında taşınabilir değildir. Örneğin Windows platformlarında `gcc` bulunmaması muhtemeldir ve hatta tüm Unix platformlarında `gcc` bulunmayabilir. `ar` komutu da benzer bir durumdadır.
* Bu komutlar çapraz derlemeyi (cross-compilation) dikkate almaz. Örneğin Android gibi bir platform için çapraz derleme yapıyorsak, `gcc`’nin bir ARM çalıştırılabilir dosyası üretmesi muhtemel değildir.

Ancak endişeye gerek yok; işte burada bir `build-dependencies` (build-dependencies) girdisi yardım eder! Cargo ekosistemi, bu tür görevleri çok daha kolay, taşınabilir ve standart hale getiren bir dizi pakete sahiptir. Hadi crates.io’daki `cc` crate’ini (cc crate) deneyelim. Önce `Cargo.toml` içine `build-dependencies` bölümüne ekleyin:

```toml
[build-dependencies]
cc = "1.0"
```

Ve derleme betiğini (build script) bu crate’i kullanacak şekilde yeniden yazın:

```rust
// build.rs

fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
    println!("cargo::rerun-if-changed=src/hello.c");
}
```

`cc` crate’i (cc crate), C kodu için bir dizi derleme betiği (build script) gereksinimini soyutlar:

* Uygun derleyiciyi çağırır (Windows için MSVC (MSVC), MinGW için `gcc` (gcc), Unix platformları için `cc` (cc) vb.).
* `TARGET` değişkenini (TARGET variable) göz önünde bulundurarak kullanılan derleyiciye uygun bayrakları geçirir.
* `OPT_LEVEL`, `DEBUG` vb. gibi diğer ortam değişkenlerinin (environment variables) tümü otomatik olarak ele alınır.
* `stdout` çıktısı (stdout output) ve `OUT_DIR` konumları da `cc` kütüphanesi (library) tarafından yönetilir.

Burada, ortak derleme bağımlılıklarına (build dependencies) mümkün olduğunca çok işlevsellik devretmenin, tüm derleme betiklerinde (build scripts) mantığı çoğaltmak yerine ne kadar büyük faydalar sağlayabildiğini görmeye başlıyoruz!

Vaka çalışmasına geri dönersek, `src` dizininin içeriğine kısaca göz atalım:

```c
// src/hello.c

#include <stdio.h>

void hello() {
    printf("Hello, World!\n");
}
```

```rust
// src/main.rs

// Note the lack of the `#[link]` attribute. We’re delegating the responsibility
// of selecting what to link over to the build script rather than hard-coding
// it in the source file.
unsafe extern { fn hello(); }

fn main() {
    unsafe { hello(); }
}
```

Ve işte bu kadar! Bu, bir Cargo paketi içinden derleme betiğini (build script) kullanarak bazı C kodlarını derleme örneğimizi tamamlamalıdır. Bu aynı zamanda, bir derleme bağımlılığı (build dependency) kullanmanın birçok durumda neden kritik olabileceğini ve hatta çok daha özlü olabileceğini de gösterir!

Ayrıca, bir derleme betiğinin (build script) yalnızca derleme süreci için (build time) bir crate’i (crate) nasıl bağımlılık olarak kullanabileceğine, ancak çalışma zamanında (runtime) crate’in kendisi için kullanılmayabileceğine dair kısa bir örnek görmüş olduk.

## 🧷 Sistem Kütüphanelerine Bağlama (linking to system libraries)

Bu örnek, bir sistem kütüphanesine (system library) nasıl bağlanılacağını ve bu kullanım durumunu desteklemek için derleme betiğinin (build script) nasıl kullanıldığını göstermektedir.

Rust crate’lerinin (crates) oldukça sık, işlevselliğine bağlanmak veya sadece bir uygulama ayrıntısı olarak kullanmak için sistemde sağlanan yerel bir kütüphaneye bağlanmak istemesi olağandır. Bunu platformdan bağımsız bir şekilde gerçekleştirmek, oldukça incelikli bir problemdir. Mümkünse, bunu tüketiciler için olabildiğince kolaylaştırmak adına işin büyük kısmını dışarıya devretmek en iyisidir.

Bu örnekte, sistemin `zlib` kütüphanesine bağlanan bir bağlama (binding) oluşturacağız. Bu, veri sıkıştırma sağlayan ve çoğu Unix benzeri sistemde yaygın olarak bulunan bir kütüphanedir. Bu zaten `libz-sys` crate’i (libz-sys crate) içinde paketlenmiştir, ancak bu örnek için son derece basitleştirilmiş bir sürüm yapacağız. Tam örnek için kaynak koda göz atın.

Kütüphanenin konumunu bulmayı kolaylaştırmak için `pkg-config` crate’ini (pkg-config crate) kullanacağız. Bu crate (crate), bir kütüphane hakkında bilgi keşfetmek için sistemin `pkg-config` yardımcı programını kullanır. Kütüphaneyi bağlamak için gerekenleri `Cargo`’ya otomatik olarak bildirir. Bu muhtemelen yalnızca `pkg-config` kurulu Unix benzeri sistemlerde çalışacaktır. Önce bildirimi (manifest) ayarlayarak başlayalım:

```toml
# Cargo.toml

[package]
name = "libz-sys"
version = "0.1.0"
edition = "2024"
links = "z"

[build-dependencies]
pkg-config = "0.3.16"
```

`package` tablosuna `links` anahtarını (links key) dahil ettiğimize dikkat edin. Bu, `libz` kütüphanesine bağlandığımızı `Cargo`’ya söyler. “Başka bir sys crate’i kullanma” (using another sys crate) bölümünde bunun nasıl kullanılacağına dair bir örnek göreceksiniz.

Derleme betiği (build script) oldukça basittir:

```rust
// build.rs

fn main() {
    pkg_config::Config::new().probe("zlib").unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
```

Basit bir FFI bağlaması (FFI binding) ile örneği tamamlayalım:

```rust
// src/lib.rs

use std::os::raw::{c_uint, c_ulong};

unsafe extern "C" {
    pub fn crc32(crc: c_ulong, buf: *const u8, len: c_uint) -> c_ulong;
}

#[test]
fn test_crc32() {
    let s = "hello";
    unsafe {
        assert_eq!(crc32(0, s.as_ptr(), s.len() as c_uint), 0x3610a686);
    }
}
```

`cargo build -vv` çalıştırarak derleme betiğinden (build script) gelen çıktıyı görün. `libz`’in zaten yüklü olduğu bir sistemde çıktı aşağıdakine benzer görünebilir:

```
[libz-sys 0.1.0] cargo::rustc-link-search=native=/usr/lib
[libz-sys 0.1.0] cargo::rustc-link-lib=z
[libz-sys 0.1.0] cargo::rerun-if-changed=build.rs
```

Harika! `pkg-config`, kütüphaneyi bulma ve `Cargo`’ya nerede olduğunu söyleme işinin tamamını yaptı.

Paketlerin, kütüphanenin kaynaklarını içermesi ve sistemde bulunamazsa veya bir özellik (feature) ya da ortam değişkeni ayarlandığında statik olarak derlemesi olağandışı değildir. Örneğin gerçek `libz-sys` crate’i, sistem kütüphanesi yerine kaynaktan derlemek için `LIBZ_SYS_STATIC` ortam değişkenini (environment variable) veya `static` özelliğini (feature) kontrol eder. Daha eksiksiz bir örnek için kaynağa göz atın.

## 🔗 Başka Bir Sys Crate’i Kullanma (using another sys crate)

`links` anahtarını (links key) kullanırken, crate’ler (crates) kendisine bağımlı olan diğer crate’ler tarafından okunabilen üstveriler (metadata) ayarlayabilir. Bu, crate’ler arasında bilgi iletişimini sağlayan bir mekanizma sunar. Bu örnekte, gerçek `libz-sys` crate’inden `zlib` kullanan bir C kütüphanesi oluşturacağız.

`zlib`’e bağımlı bir C kütüphaneniz varsa, `libz-sys` crate’inden yararlanarak onu otomatik olarak bulabilir veya derleyebilirsiniz. Bu, `zlib`’in genellikle yüklü olmadığı Windows gibi platformlar için mükemmel bir çapraz platform (cross-platform) desteğidir. `libz-sys`, diğer paketlere `zlib` için başlık dosyalarını nerede bulacaklarını söylemek üzere `include` üstverisini (include metadata) ayarlar. Derleme betiğimiz (build script), `DEP_Z_INCLUDE` ortam değişkeni (environment variable) ile bu üstveriyi okuyabilir. İşte bir örnek:

```toml
# Cargo.toml

[package]
name = "zuser"
version = "0.1.0"
edition = "2024"

[dependencies]
libz-sys = "1.0.25"

[build-dependencies]
cc = "1.0.46"
```

Burada `libz-sys`’i dahil ettik; bu, nihai kütüphanede yalnızca bir `libz` kullanılmasını sağlar ve derleme betiğimizden (build script) ona erişmemize olanak tanır:

```rust
// build.rs

fn main() {
    let mut cfg = cc::Build::new();
    cfg.file("src/zuser.c");
    if let Some(include) = std::env::var_os("DEP_Z_INCLUDE") {
        cfg.include(include);
    }
    cfg.compile("zuser");
    println!("cargo::rerun-if-changed=src/zuser.c");
}
```

`libz-sys` tüm ağır işleri yaparken, C kaynak kodu artık `zlib` başlığını içerebilir ve bu başlığı, yüklü olmadığı sistemlerde bile bulabilmelidir.

```c
// src/zuser.c

#include "zlib.h"

// … rest of code that makes use of zlib.
```

## ⚙️ Koşullu Derleme (conditional compilation)

Bir derleme betiği (build script), derleme zamanında denetlenebilecek koşulları etkinleştirebilen `rustc-cfg` talimatlarını (rustc-cfg instructions) yayabilir. Bu örnekte, `openssl` crate’inin OpenSSL kütüphanesinin birden çok sürümünü desteklemek için bunu nasıl kullandığına bakacağız.

`openssl-sys` crate’i (openssl-sys crate) OpenSSL kütüphanesini derlemeyi ve bağlamayı uygular. Birden çok farklı uygulamayı (LibreSSL gibi) ve birden çok sürümü destekler. Diğer derleme betiklerine (build scripts) bilgi geçirebilmek için `links` anahtarından (links key) yararlanır. Geçirdiği şeylerden biri de `version_number` anahtarı (version\_number key) olup, algılanan OpenSSL sürümüdür. Derleme betiğindeki (build script) kod kabaca şu şekildedir:

```rust
println!("cargo::metadata=version_number={openssl_version:x}");
```

Bu talimat, `openssl-sys`’e doğrudan bağımlı olan crate’lerde `DEP_OPENSSL_VERSION_NUMBER` ortam değişkeninin (environment variable) ayarlanmasına neden olur.

Daha üst düzey arayüz sağlayan `openssl` crate’i (openssl crate), `openssl-sys`’i bir bağımlılık olarak belirtir. `openssl` derleme betiği (build script), `DEP_OPENSSL_VERSION_NUMBER` ortam değişkeniyle `openssl-sys` derleme betiği (build script) tarafından üretilen sürüm bilgisini okuyabilir. Bunu bazı `cfg` değerleri üretmek için kullanır:

```rust
// (portion of build.rs)

println!("cargo::rustc-check-cfg=cfg(ossl101,ossl102)");
println!("cargo::rustc-check-cfg=cfg(ossl110,ossl110g,ossl111)");

if let Ok(version) = env::var("DEP_OPENSSL_VERSION_NUMBER") {
    let version = u64::from_str_radix(&version, 16).unwrap();

    if version >= 0x1_00_01_00_0 {
        println!("cargo::rustc-cfg=ossl101");
    }
    if version >= 0x1_00_02_00_0 {
        println!("cargo::rustc-cfg=ossl102");
    }
    if version >= 0x1_01_00_00_0 {
        println!("cargo::rustc-cfg=ossl110");
    }
    if version >= 0x1_01_00_07_0 {
        println!("cargo::rustc-cfg=ossl110g");
    }
    if version >= 0x1_01_01_00_0 {
        println!("cargo::rustc-cfg=ossl111");
    }
}
```

Bu `cfg` değerleri, koşullu olarak kodu dahil etmek için `cfg` özniteliği (cfg attribute) veya `cfg` makrosuyla (cfg macro) kullanılabilir. Örneğin, `SHA3` desteği OpenSSL 1.1.1’de eklendi; bu nedenle daha eski sürümler için koşullu olarak dışlanır:

```rust
// (portion of openssl crate)

#[cfg(ossl111)]
pub fn sha3_224() -> MessageDigest {
    unsafe { MessageDigest(ffi::EVP_sha3_224()) }
}
```

Elbette, bunu kullanırken dikkatli olmak gerekir; çünkü ortaya çıkan ikili dosyayı (binary) derleme ortamına daha da bağımlı hale getirir. Bu örnekte, ikili dosya başka bir sisteme dağıtılırsa, aynı paylaşılan kütüphanelere sahip olmayabilir ve bu da sorunlara yol açabilir.

Bu liste bir tavsiye anlamına gelmez. Projeniz için hangisinin uygun olduğunu görmek üzere bağımlılıklarınızı değerlendirin. ↩
