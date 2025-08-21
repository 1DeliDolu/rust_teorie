## 📦 Paketler (packages) ve Crate’ler (crates)

İlk olarak modül sisteminin paketler (packages) ve crate’ler (crates) kısımlarını ele alacağız.

Bir **crate**, Rust derleyicisinin (compiler) aynı anda dikkate aldığı en küçük kod birimidir. Hatta `cargo` yerine `rustc` çalıştırıp tek bir kaynak dosyası verdiğinizde (1. bölümdeki “Bir Rust Programı Yazmak ve Çalıştırmak” kısmında yaptığımız gibi), derleyici o dosyayı bir crate olarak değerlendirir. Crate’ler modüller (modules) içerebilir, ve bu modüller crate ile birlikte derlenen diğer dosyalarda tanımlanabilir; bunu sonraki bölümlerde göreceğiz.

Bir crate iki biçimde olabilir: **binary crate** veya **library crate**.

* **Binary crate**’ler derlenip çalıştırılabilen yürütülebilir programlardır, örneğin komut satırı uygulamaları veya sunucular. Her binary crate, çalıştırıldığında ne olacağını tanımlayan `main` adında bir fonksiyon içermelidir. Şimdiye kadar oluşturduğumuz tüm crate’ler binary crate olmuştur.
* **Library crate**’ler ise `main` fonksiyonu içermez ve yürütülebilir dosyaya derlenmez. Bunun yerine, birden fazla projeyle paylaşılacak işlevsellik tanımlarlar. Örneğin, 2. bölümde kullandığımız `rand` crate, rastgele sayı üretme işlevselliği sağlar. Rust kullanıcıları (“Rustaceans”) çoğu zaman “crate” derken library crate’i kastederler ve bu terimi genel programlamadaki “kütüphane” (library) kavramıyla birbirinin yerine kullanırlar.

**Crate root**, Rust derleyicisinin başlangıç noktası olarak aldığı ve crate’in kök modülünü (root module) oluşturan kaynak dosyadır (modülleri ayrıntılı olarak “Kapsam ve Gizliliği Kontrol Etmek İçin Modüllerin Tanımlanması” kısmında açıklayacağız).

Bir **package**, bir veya daha fazla crate içeren ve belirli bir işlevsellik sağlayan bir pakettir. Bir package, içinde o crate’lerin nasıl derleneceğini açıklayan bir `Cargo.toml` dosyası içerir. Örneğin `Cargo`, aslında bir package’tir: içinde komut satırı aracını çalıştıran binary crate vardır. Ayrıca `Cargo` package’i, bu binary crate’in bağımlı olduğu bir library crate de içerir. Diğer projeler, `Cargo` komut satırı aracının kullandığı mantığı aynı şekilde kullanabilmek için `Cargo` library crate’ine bağımlı olabilirler.

Bir package istediğiniz kadar çok binary crate içerebilir, ancak en fazla bir tane library crate içerebilir. Ayrıca bir package, en az bir crate içermek zorundadır; bu crate ister library ister binary olabilir.

---

## ⚙️ Bir Paket Oluşturma Süreci

Bir paket oluşturduğumuzda ne olduğunu inceleyelim. Önce şu komutu giriyoruz:

```
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

`cargo new my-project` çalıştırdıktan sonra, `ls` ile `Cargo`’nun neler oluşturduğunu görüyoruz. Proje dizininde, bize bir package sağlayan `Cargo.toml` dosyası var. Ayrıca `main.rs` dosyasını içeren bir `src` dizini mevcut.

`Cargo.toml` dosyasını bir metin düzenleyici ile açtığınızda, `src/main.rs`’ten bahsedilmediğini göreceksiniz. Bunun sebebi, `Cargo`’nun şu sözleşmeyi (convention) takip etmesidir:

* `src/main.rs`, paketin adıyla aynı isme sahip binary crate’in crate root dosyasıdır.
* Eğer package dizininde `src/lib.rs` varsa, paket aynı adı taşıyan bir library crate içerir ve `src/lib.rs` onun crate root dosyasıdır.
  `Cargo`, crate root dosyalarını `rustc`’ye iletir ve bu şekilde library veya binary’yi derler.

---

## 🧩 Paket Yapısının Örnekleri

Burada yalnızca `src/main.rs` içeren bir paketimiz var, yani `my-project` adında yalnızca bir binary crate var.

Eğer bir paket `src/main.rs` ve `src/lib.rs` içeriyorsa, aynı ada sahip hem bir binary hem de bir library crate’e sahip olur.

Bir paket, `src/bin` dizinine dosyalar eklenerek birden fazla binary crate içerebilir: bu dizindeki her dosya ayrı bir binary crate olacaktır.
