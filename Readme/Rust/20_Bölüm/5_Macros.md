## 🧩 Makrolar (macros)

Bu kitap boyunca `println!` gibi makroları kullandık, fakat makronun ne olduğunu ve nasıl çalıştığını tam olarak incelemedik. **Makro (macro)** terimi, Rust’ta üç türden oluşan bir özellik ailesini ifade eder:

* **Deklaratif makrolar (declarative macros)** → `macro_rules!` ile tanımlanır.
* **Prosedürel makrolar (procedural macros):**

  * `#[derive]` ile özel türetme makroları (custom derive macros)
  * Öğe (item) üzerinde kullanılabilen özel öznitelikler tanımlayan **attribute-like macros**
  * Fonksiyon çağrılarına benzeyen fakat argüman olarak verilen **token**’lar üzerinde çalışan **function-like macros**

Bunları tek tek ele alacağız. Ancak önce, fonksiyonlarımız zaten varken neden makrolara ihtiyaç duyduğumuzu görelim.

---

## ⚖️ Makrolar ve Fonksiyonlar Arasındaki Fark (macros vs functions)

Temelde, makrolar **başka kod üreten kod** yazmanın bir yoludur. Buna **metaprogramlama (metaprogramming)** denir.

Örneğin:

* Ek C’de (Appendix C) tartışılan `derive` özniteliği, çeşitli trait implementasyonlarını sizin için otomatik üretir.
* `println!` ve `vec!` makroları da kullandığınızdan daha fazla kod üretir.

Metaprogramlama, yazmanız ve bakımını yapmanız gereken kod miktarını azaltır. Fonksiyonlar da benzer bir rol oynar, fakat makroların ekstra gücü vardır:

* Fonksiyon imzaları (function signatures) parametre sayısı ve türlerini açıkça belirtmek zorundadır.
* Makrolar ise değişken sayıda parametre alabilir. Örneğin:

  * `println!("hello")` → tek argüman
  * `println!("hello {}", name)` → iki argüman
* Makrolar, derleyici kodu yorumlamadan **önce** genişletilir. Bu nedenle bir makro, bir türe `trait` implementasyonu ekleyebilir.
* Fonksiyonlar yalnızca çalışma zamanında çağrılır, bu yüzden `trait` implementasyonu yapamazlar.

**Dezavantajı:**
Makro tanımları, fonksiyon tanımlarına kıyasla daha karmaşıktır, çünkü **Rust kodu yazan Rust kodu** yazmanız gerekir. Bu dolaylılık, makroları okumayı, anlamayı ve bakımını daha zor hale getirir.

Bir diğer önemli fark, fonksiyonları dosyada istediğiniz yerde tanımlayıp çağırabilmenize karşın, makroların çağrılmadan önce tanımlanması veya scope içine alınması gerekmesidir.

---

## 🛠️ `macro_rules!` ile Deklaratif Makrolar (declarative macros)

Rust’ta en çok kullanılan makro türü, **deklaratif makrolardır**. Bunlara:

* **“macros by example”**,
* **“macro\_rules! macros”**,
* veya kısaca **“macros”** da denir.

Bunlar, temelinde **`match` ifadesine** benzer şekilde çalışır:

* `match` ifadeleri → bir değeri alır, desenlerle karşılaştırır ve uygun kodu çalıştırır.
* Makrolar → Rust kaynak kodunu alır, desenlerle karşılaştırır ve uygun kodu derleme sırasında genişletir.

Bir makro tanımlamak için `macro_rules!` kullanılır. Örneğin `vec!` makrosunun tanımına bakalım. Bu makro, belirli değerleri içeren yeni bir `Vec` oluşturur.

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

Bunu fonksiyonla yapamayız, çünkü fonksiyon çağrılmadan önce kaç argüman olacağını veya türlerini bilemeyiz.

---

## 📦 `vec!` Makrosunun Basitleştirilmiş Tanımı

Aşağıda, `vec!` makrosunun basitleştirilmiş bir tanımı verilmiştir (Listing 20-35):

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

* `#[macro_export]` → Bu öznitelik, makronun bulunduğu crate scope içine alındığında kullanılabilir hale gelmesini sağlar. Olmazsa makro dışarıdan çağrılamaz.
* `macro_rules!` → Makro tanımının başladığını belirtir.
* `vec` → Makronun adı (`!` olmadan).

### 🧩 Desen Yapısı

* `( $( $x:expr ),* )` → Deseni tanımlar.

  * `$x:expr` → Herhangi bir Rust ifadesini (`expr`) eşleştirir.
  * `,` → Araya virgül gelmesi gerektiğini belirtir.
  * `*` → Bu desenin sıfır veya daha fazla kez tekrarlanabileceğini belirtir.

Makro çağrıldığında, `$x` her eşleşen ifadeyle değiştirilir.
Örneğin `vec![1, 2, 3];` çağrısı şu koda dönüştürülür:

```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

Bu makro sayesinde, herhangi sayıda ve türde argüman alıp bir `Vec` oluşturabiliriz.

---

## 📚 Daha Fazla Kaynak

Makro yazmayı öğrenmek için:

* **Resmi Rust belgelerine** bakabilirsiniz.
* Ayrıca Daniel Keep tarafından başlatılıp Lukas Wirth tarafından sürdürülen **“The Little Book of Rust Macros”** kitabı da faydalı bir kaynaktır.


## 🛠️ Özniteliklerden Kod Üreten Prosedürel Makrolar (procedural macros for generating code from attributes)

Makroların ikinci biçimi \*\*prosedürel makrolar (procedural macros)\*\*dır. Bunlar bir fonksiyona daha çok benzer (bir tür prosedürdür).

* Deklaratif makrolar (`macro_rules!`) → desenlere göre eşleştirme yapıp kodu değiştirme.
* Prosedürel makrolar → kodu girdi olarak alır, üzerinde işlem yapar ve çıktı olarak yeni kod üretir.

Prosedürel makroların üç türü vardır:

1. **Custom derive** makroları (`#[derive]`)
2. **Attribute-like** makrolar (özel öznitelikler tanımlar)
3. **Function-like** makrolar (fonksiyon çağrısına benzer görünür)

Hepsi benzer şekilde çalışır.

---

## 📦 Prosedürel Makrolar İçin Özel Crate

Prosedürel makro tanımları, **özel crate türünde** kendi crate’i içinde bulunmak zorundadır. Bu, teknik nedenlerden kaynaklanır.

Basit bir örnek (Listing 20-36):

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

* Girdi → `TokenStream` (işlenecek Rust kodu)
* Çıktı → `TokenStream` (üretilen Rust kodu)
* `proc_macro` → Rust ile birlikte gelen crate, **token akışı** üzerinde işlem yapmayı sağlar.

---

## ✨ Custom Derive Makro Yazmak

Şimdi `hello_macro` adında bir crate oluşturalım. Bunun içinde:

* `HelloMacro` adında bir trait,
* `hello_macro` adında bir fonksiyon bulunacak.

Kullanıcılar, kendi türlerine elle implementasyon yazmak yerine, `#[derive(HelloMacro)]` kullanarak otomatik implementasyon alacak.

Örnek kullanıcı kodu (Listing 20-37):

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

**Beklenen çıktı:**

```
Hello, Macro! My name is Pancakes!
```

---

## 📐 Trait Tanımı (Listing 20-38)

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

Kullanıcılar manuel olarak şu şekilde implementasyon yapabilir (Listing 20-39):

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

Ancak her tür için bunu tekrar yazmak zahmetli olur. Ayrıca Rust, çalışma zamanında tip ismi öğrenmeye yarayan **reflection** desteğine sahip değildir. Bu nedenle, tip adını derleme zamanında eklemek için **makroya ihtiyaç vardır.**

---

## 📂 Prosedürel Makro Crate’i Oluşturmak

Yeni crate açalım:

```
$ cargo new hello_macro_derive --lib
```

Ardından `hello_macro_derive/Cargo.toml` içine şu eklenir:

```toml
[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"
```

* `syn` → Rust kodunu ayrıştırır (parse eder).
* `quote` → Ayrıştırılan yapıyı tekrar Rust koduna çevirir.

---

## 🔧 Prosedürel Makro Tanımı (Listing 20-40)

```rust
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Rust kodunu bir söz dizimi ağacına (AST) dönüştür.
    let ast = syn::parse(input).unwrap();

    // Trait implementasyonunu üret.
    impl_hello_macro(&ast)
}
```

Burada:

* `hello_macro_derive` → dışarıya açık prosedürel makro.
* `impl_hello_macro` → ayrıştırılmış kod üzerinde işlem yapacak yardımcı fonksiyon.

---

## ⚙️ Kod Üretimi (Listing 20-42)

```rust
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}
```

* `ast.ident` → struct ismini (`Pancakes`) alır.
* `quote!` → Rust kodu üretir.
* `stringify!(#name)` → `#name` ifadesini derleme zamanında stringe çevirir (`"Pancakes"`).

---

## 🚀 Kullanım

Yeni proje açalım:

```
$ cargo new pancakes
```

`Cargo.toml` içine şu bağımlılıklar eklenir:

```toml
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

`src/main.rs` içine (Listing 20-37) yazılır ve çalıştırılır:

```
$ cargo run
Hello, Macro! My name is Pancakes!
```

Sonuç: `#[derive(HelloMacro)]` makrosu, `HelloMacro` trait’inin implementasyonunu otomatik olarak ekledi.

---

## ⏭️ Sonraki Adım

Bir sonraki bölümde, diğer prosedürel makro türlerinin (attribute-like ve function-like) custom derive makrolardan nasıl farklılaştığını inceleyeceğiz.

## 🏷️ Attribute-Like Makrolar (attribute-like macros)

**Attribute-like makrolar**, `custom derive` makrolarına benzer. Ancak `derive` yalnızca **struct** ve **enum**’lara uygulanabilirken, attribute-like makrolar çok daha esnektir: fonksiyonlara veya diğer ögelere de uygulanabilir.

Örnek: bir web uygulama çatısında (framework) kullanılan `#[route]` özniteliğini düşünelim:

```rust
#[route(GET, "/")]
fn index() {
```

Bu `#[route]` özniteliği, framework tarafından bir prosedürel makro olarak tanımlanır. Makro tanımı şöyle olur:

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

* `attr: TokenStream` → özniteliğin içeriği (`GET, "/"`).
* `item: TokenStream` → özniteliğin eklendiği öğe (örneğin `fn index() { ... }`).

Attribute-like makrolar, `custom derive` makrolarıyla aynı şekilde çalışır:

* `proc-macro` crate türünde tanımlanır.
* Bir fonksiyon ile kod üretilir.

---

## 📞 Function-Like Makrolar (function-like macros)

**Function-like makrolar**, fonksiyon çağrılarına benzer şekilde tanımlanır.

* `macro_rules!` makrolarına benzer şekilde değişken sayıda argüman alabilirler.
* Ancak `macro_rules!` yalnızca `match` benzeri sözdizimiyle tanımlanır.
* Function-like makrolar ise bir `TokenStream` parametresi alır ve üzerinde Rust kodu ile işlem yapar.

Örnek: SQL sorgusunu işleyen `sql!` makrosu:

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

Bu makro, verilen SQL ifadesini ayrıştırır ve sözdiziminin doğru olduğunu kontrol eder. Tanımı şöyle yapılır:

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

Fonksiyon imzası `custom derive` makrosuna benzer: parantez içindeki token’ları alır, işler ve üretilen Rust kodunu döndürür.

---

## 📋 Özet (summary)

Artık nadiren kullanacağınız ama özel durumlarda faydalı olacak bazı Rust özelliklerini gördünüz:

* Deklaratif makrolar (`macro_rules!`)
* Prosedürel makrolar (custom derive, attribute-like, function-like)

Bu konular biraz karmaşık olsa da, hata mesajlarında veya başkalarının kodunda karşılaştığınızda ne olduklarını tanıyabileceksiniz. Bu bölümü, karşılaştığınızda başvurabileceğiniz bir **referans** olarak düşünebilirsiniz.

---

## ⏭️ Sonraki Adım

Bir sonraki bölümde, kitap boyunca öğrendiğimiz her şeyi bir araya getirip **son bir proje** üzerinde uygulayacağız!
