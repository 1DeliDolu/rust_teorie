## 🗂️ Cargo Çalışma Alanları (Cargo Workspaces)

Bölüm 12’de, bir ikili crate (binary crate) ve bir kütüphane crate’i (library crate) içeren bir paket inşa etmiştik. Projeniz geliştikçe, kütüphane crate’inizin büyüdüğünü ve paketinizi birden fazla kütüphane crate’ine ayırmak istediğinizi görebilirsiniz. Cargo, birlikte geliştirilen birden fazla ilgili paketi yönetmenize yardımcı olabilecek **çalışma alanı (workspace)** adında bir özellik sunar.

---

## 🏗️ Bir Çalışma Alanı Oluşturma (Creating a Workspace)

Bir çalışma alanı, aynı `Cargo.lock` dosyasını ve çıktı dizinini paylaşan paketler kümesidir. Küçük bir örnek proje yaparak bir çalışma alanı oluşturalım. Basit kodlar kullanacağız, böylece çalışma alanının yapısına odaklanabiliriz.

Burada yaygın kullanılan bir yapıyı göstereceğiz: bir çalışma alanı içinde bir ikili crate ve iki kütüphane crate’i olacak. İkili crate ana işlevselliği sağlayacak ve iki kütüphaneye bağımlı olacak. Kütüphanelerden biri `add_one`, diğeri `add_two` fonksiyonu sağlayacak. Bu üç crate aynı çalışma alanının parçası olacak.

Öncelikle çalışma alanı için yeni bir dizin oluşturalım:

```
$ mkdir add
$ cd add
```

Ardından, `add` dizininde tüm çalışma alanını yapılandıracak `Cargo.toml` dosyasını oluşturalım. Bu dosyada `[package]` bölümü olmayacak. Bunun yerine `[workspace]` bölümüyle başlayacak ve çalışma alanına üye eklememizi sağlayacak. Ayrıca çözümleyici (resolver) algoritmasının en güncel sürümünü kullanmak için `resolver = "3"` ayarlıyoruz.

Dosya adı: `Cargo.toml`

```toml
[workspace]
resolver = "3"
```

Şimdi, `add` dizini içinde `cargo new` komutunu çalıştırarak `adder` adında bir ikili crate oluşturalım:

```
$ cargo new adder
    Creating binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

Bir çalışma alanı içinde `cargo new` çalıştırmak, yeni paketi otomatik olarak `Cargo.toml` içindeki `members` anahtarına ekler:

```toml
[workspace]
resolver = "3"
members = ["adder"]
```

Bu noktada `cargo build` ile çalışma alanını derleyebiliriz. `add` dizinindeki dosya yapısı şu şekilde olacaktır:

```
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Çalışma alanının üst seviyede tek bir `target` dizini vardır; derlenen tüm çıktılar buraya yerleştirilir. `adder` paketinin kendi `target` dizini yoktur. Hatta `adder` dizininde `cargo build` çalıştırsak bile derlenen çıktılar `add/target` içine konur. Cargo, çalışma alanındaki crate’ler birbirine bağımlı olduğu için `target` dizinini paylaşacak şekilde yapılandırır. Eğer her crate’in kendi `target` dizini olsaydı, her biri diğerlerini tekrar tekrar derlemek zorunda kalırdı. Paylaşılan tek bir `target` dizini sayesinde gereksiz derlemelerden kaçınılır.

---

## ➕ Çalışma Alanına İkinci Paketi Eklemek (Creating the Second Package in the Workspace)

Şimdi, çalışma alanına başka bir üye paket ekleyelim ve ona `add_one` adını verelim. Bunun için yeni bir kütüphane crate’i oluşturalım:

```
$ cargo new add_one --lib
    Creating library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

Üst düzey `Cargo.toml` artık `add_one` yolunu da `members` listesine ekleyecek:

Dosya adı: `Cargo.toml`

```toml
[workspace]
resolver = "3"
members = ["adder", "add_one"]
```

Artık `add` dizinimiz şu yapıya sahip:

```
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

`add_one/src/lib.rs` dosyasına `add_one` fonksiyonunu ekleyelim:

Dosya adı: `add_one/src/lib.rs`

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Şimdi ikili paketimiz (`adder`) bu kütüphaneye (`add_one`) bağımlı olacak. Bunun için `adder/Cargo.toml` dosyasına bir yol bağımlılığı (path dependency) eklemeliyiz.

Dosya adı: `adder/Cargo.toml`

```toml
[dependencies]
add_one = { path = "../add_one" }
```

Cargo, çalışma alanındaki crate’lerin birbirine bağımlı olduğunu varsaymaz, bu nedenle bağımlılık ilişkilerini açıkça belirtmemiz gerekir.

Sonra, `adder` crate’inde `add_one` fonksiyonunu kullanalım. Bunun için `adder/src/main.rs` dosyasını açıp `main` fonksiyonunu şu şekilde değiştirelim (Liste 14-7):

Dosya adı: `adder/src/main.rs`

```rust
fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
```

Liste 14-7: `adder` crate’inde `add_one` kütüphane crate’ini kullanma

Şimdi üst düzey `add` dizininde `cargo build` çalıştırarak çalışma alanını derleyelim:

```
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

İkili crate’i çalıştırmak için `add` dizininden, hangi paketin çalıştırılacağını `-p` argümanı ile belirtebiliriz:

```
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

👉 Bu, `adder/src/main.rs` dosyasındaki kodu çalıştırır ve bu kod `add_one` crate’ine bağımlıdır.


## 📦 Çalışma Alanında Harici Bir Pakete Bağımlı Olmak (Depending on an External Package in a Workspace)

Çalışma alanında her crate’in dizininde ayrı bir `Cargo.lock` dosyası yerine, yalnızca üst düzeyde bir `Cargo.lock` dosyası bulunur. Bu, tüm crate’lerin aynı bağımlılık sürümlerini kullanmasını garanti eder.

Örneğin, `adder/Cargo.toml` ve `add_one/Cargo.toml` dosyalarına `rand` paketini eklersek, Cargo ikisini de tek bir `rand` sürümüne çözümler ve bunu `Cargo.lock` içine kaydeder. Böylece tüm crate’ler aynı bağımlılıkları kullanır ve birbirleriyle uyumlu kalır.

Şimdi `add_one` crate’inde `rand` paketini kullanalım. Bunun için `add_one/Cargo.toml` dosyasına bağımlılığı ekleyelim:

Dosya adı: `add_one/Cargo.toml`

```toml
[dependencies]
rand = "0.8.5"
```

Sonra `add_one/src/lib.rs` dosyasına `use rand;` ekleyelim. `add` dizininde `cargo build` çalıştırdığımızda Cargo `rand` crate’ini indirir ve derler. Ancak henüz `rand`’ı kullanmadığımız için bir uyarı alırız:

```
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
```

Artık üst düzey `Cargo.lock`, `add_one` crate’inin `rand` bağımlılığını içerir. Ancak, `rand` çalışma alanında bir yerde kullanılıyor olsa bile, diğer crate’lerde onu kullanamayız; bunun için onların `Cargo.toml` dosyalarına da `rand` bağımlılığını eklememiz gerekir.

Örneğin, `adder/src/main.rs` dosyasına `use rand;` eklersek şu hatayı alırız:

```
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

Bunu düzeltmek için `adder/Cargo.toml` dosyasına da `rand` bağımlılığını eklemeliyiz. Ardından Cargo, `Cargo.lock` içinde `adder` için `rand` bağımlılığını kaydeder, ancak ek kopya indirmez. Cargo, tüm crate’lerin uyumlu `rand` sürümlerini kullandığından emin olur.

👉 Eğer crate’ler aynı bağımlılığın uyumsuz sürümlerini belirtirse, Cargo her birini çözümler, ancak mümkün olduğunca az sürüm kullanmaya çalışır.

---

## 🧪 Çalışma Alanına Test Ekleme (Adding a Test to a Workspace)

Şimdi `add_one::add_one` fonksiyonuna bir test ekleyelim:

Dosya adı: `add_one/src/lib.rs`

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

Artık üst düzey `add` dizininde `cargo test` çalıştırabiliriz. Bu komut, çalışma alanındaki tüm crate’lerin testlerini çalıştırır:

```
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests
```

👉 Çıktının ilk kısmı, `add_one` crate’indeki `it_works` testinin geçtiğini gösterir. İkinci kısımda `adder` crate’inde hiç test olmadığı belirtilir.

Belirli bir crate için test çalıştırmak istersek, `-p` bayrağını kullanarak crate adını belirtebiliriz:

```
$ cargo test -p add_one
```

👉 Bu, yalnızca `add_one` crate’inin testlerini çalıştırır.

---

## 📤 Workspace Crate’lerini Yayınlamak (Publishing Crates in a Workspace)

Çalışma alanındaki crate’leri `crates.io`’ya yayınlamak isterseniz, her crate’i ayrı ayrı yayınlamanız gerekir. Tıpkı `cargo test` gibi, `cargo publish` ile de `-p` bayrağını kullanarak yayınlamak istediğiniz crate’in adını belirtebilirsiniz.

---

## 📈 Sonuç (Conclusion)

Ek bir alıştırma olarak, `add_one` crate’ini eklediğimiz şekilde bir `add_two` crate’i ekleyebilirsiniz.

Projeniz büyüdükçe bir çalışma alanı kullanmayı düşünün:

* Kodunuzu tek bir büyük yığın yerine daha küçük, anlaşılır bileşenlerle yönetebilirsiniz.
* Ayrıca, crate’ler sık sık birlikte değiştiriliyorsa, aynı çalışma alanında tutmak koordinasyonu kolaylaştırır.
