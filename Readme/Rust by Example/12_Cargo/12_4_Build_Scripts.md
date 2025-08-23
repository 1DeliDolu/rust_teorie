## 🛠️ Derleme Betikleri (Build Scripts)

Bazen `cargo` ile yapılan normal bir derleme yeterli olmaz. Örneğin, `crate`’inizin başarılı bir şekilde derlenebilmesi için ön koşullara ihtiyaç olabilir — kod üretimi (code generation) veya derlenmesi gereken yerel kod (native code) gibi. Bu sorunu çözmek için `Cargo` tarafından çalıştırılabilen **derleme betikleri (build scripts)** kullanılır.

Bir pakete derleme betiği eklemek için, `Cargo.toml` içinde şu şekilde belirtilebilir:

```toml
[package]
...
build = "build.rs"
```

Aksi halde, Cargo varsayılan olarak proje dizininde bir `build.rs` dosyası arar.

### 🔧 Bir Derleme Betiği Nasıl Kullanılır

Derleme betiği, paketteki diğer şeylerden önce derlenecek ve çalıştırılacak başka bir Rust dosyasıdır. Bu nedenle, `crate`’inizin ön koşullarını karşılamak için kullanılabilir.

Cargo, betiğe bazı girdileri çevre değişkenleri (environment variables) aracılığıyla sağlar. Bu değişkenler [burada](https://doc.rust-lang.org/cargo/reference/environment-variables.html) tanımlanmıştır.

Betiğin çıktısı `stdout` üzerinden sağlanır. Yazdırılan tüm satırlar `target/debug/build/<pkg>/output` içine yazılır. Ayrıca, `cargo:` ile başlayan satırlar doğrudan Cargo tarafından yorumlanır ve böylece paketin derleme parametrelerini tanımlamak için kullanılabilir.

Daha fazla ayrıntı ve örnekler için **Cargo specification** belgesine göz atabilirsiniz.
