## 👋 Merhaba, Cargo! (Hello, Cargo!)

Cargo, Rust’un yapı sistemi (build system) ve paket yöneticisidir (package manager). Çoğu Rustacean bu aracı Rust projelerini yönetmek için kullanır çünkü Cargo sizin için birçok işi halleder; örneğin kodunuzu derlemek, kodunuzun ihtiyaç duyduğu kütüphaneleri indirmek ve bu kütüphaneleri derlemek. (Kodunuzun ihtiyaç duyduğu kütüphanelere bağımlılıklar (dependencies) denir.)

En basit Rust programlarının, şu ana kadar yazdığımız gibi, herhangi bir bağımlılığı yoktur. Eğer “Hello, world!” projesini Cargo ile derlemiş olsaydık, Cargo yalnızca kodunuzu derleme kısmını kullanacaktı. Daha karmaşık Rust programları yazdıkça bağımlılıklar ekleyeceksiniz ve bir projeye Cargo ile başlarsanız bağımlılık eklemek çok daha kolay olacaktır.

Çünkü Rust projelerinin büyük çoğunluğu Cargo kullanır, bu kitabın geri kalanı da sizin Cargo kullandığınızı varsayar. Eğer Rust’u “Kurulum” (Installation) bölümünde tartışılan resmi kurucular (installer) ile yüklediyseniz Cargo da Rust ile birlikte yüklenir. Rust’u başka yollarla yüklediyseniz, Cargo’nun kurulu olup olmadığını terminalinizde şu komutu girerek kontrol edin:

```
$ cargo --version
```

👉 Bu komut, Cargo’nun kurulu olup olmadığını ve varsa sürüm numarasını gösterir.

Eğer bir sürüm numarası görüyorsanız, Cargo yüklüdür! Eğer `command not found` gibi bir hata görürseniz, kullandığınız kurulum yönteminin belgelerine bakarak Cargo’yu ayrı olarak nasıl yükleyeceğinizi öğrenin.


## 📦 Cargo ile Proje Oluşturma (Creating a Project with Cargo)

Hadi Cargo kullanarak yeni bir proje oluşturalım ve bunun ilk “Hello, world!” projemizden nasıl farklı olduğuna bakalım. Projeler dizininize (veya kodunuzu saklamayı seçtiğiniz yere) geri gidin. Ardından, herhangi bir işletim sisteminde şu komutları çalıştırın:

```
$ cargo new hello_cargo
$ cd hello_cargo
```

👉 İlk komut, `hello_cargo` adında yeni bir dizin ve proje oluşturur. Projemize `hello_cargo` adını verdik ve Cargo, dosyalarını aynı isimde bir dizin içinde oluşturur.

`hello_cargo` dizinine girin ve dosyaları listeleyin. Cargo’nun bizim için iki dosya ve bir dizin oluşturduğunu göreceksiniz: bir `Cargo.toml` dosyası ve içinde `main.rs` dosyası bulunan bir `src` dizini.

Ayrıca, Cargo yeni bir Git deposu (`git repository`) başlatır ve bir `.gitignore` dosyası ekler. Eğer `cargo new` komutunu mevcut bir Git deposu içinde çalıştırırsanız, Git dosyaları oluşturulmaz; bu davranışı `cargo new --vcs=git` kullanarak değiştirebilirsiniz.

Not: Git, yaygın kullanılan bir sürüm kontrol sistemidir (version control system). `cargo new` komutunu `--vcs` bayrağı ile farklı bir sürüm kontrol sistemi veya hiçbiri olmadan kullanabilirsiniz. Kullanılabilir seçenekleri görmek için `cargo new --help` çalıştırın.

Metin düzenleyicinizde `Cargo.toml` dosyasını açın. Şuna benzer görünmelidir:

**Dosya adı: Cargo.toml**

```
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

👉 Bu dosya TOML (Tom’s Obvious, Minimal Language) formatındadır, bu da Cargo’nun yapılandırma formatıdır.

İlk satır `[package]`, bir bölüm başlığıdır ve ardından gelen satırların bir paketi yapılandırdığını belirtir. Dosyaya daha fazla bilgi ekledikçe başka bölümler de ekleyeceğiz.

Sonraki üç satır, Cargo’nun programınızı derlemek için ihtiyaç duyduğu yapılandırma bilgilerini ayarlar: `name`, `version` ve kullanılacak Rust sürümü olan `edition`. `edition` anahtarını Ek E’de (Appendix E) tartışacağız.

Son satır `[dependencies]`, projenizin bağımlılıklarını listeleyeceğiniz bir bölümün başlangıcıdır. Rust’ta kod paketlerine crate denir. Bu proje için başka crate’lere ihtiyacımız yok, ancak 2. bölümdeki ilk projede ihtiyaç duyacağız; o zaman bu dependencies bölümünü kullanacağız.

Şimdi `src/main.rs` dosyasını açın ve göz atın:

**Dosya adı: src/main.rs**

```
fn main() {
    println!("Hello, world!");
}
```

👉 Cargo, sizin için bir “Hello, world!” programı oluşturdu; tıpkı Liste 1-1’de yazdığımız gibi! Şu ana kadarki farklar, Cargo’nun kodu `src` dizinine koyması ve üst dizinde bir `Cargo.toml` yapılandırma dosyası bulunmasıdır.

Cargo, kaynak dosyalarınızın (`source files`) `src` dizini içinde olmasını bekler. En üst seviye proje dizini yalnızca `README` dosyaları, lisans bilgileri, yapılandırma dosyaları ve kodunuzla ilgili olmayan diğer şeyler içindir. Cargo kullanmak, projelerinizi düzenlemenize yardımcı olur. Her şeyin bir yeri vardır ve her şey yerindedir.

Eğer Cargo kullanmayan bir projeye başladıysanız, bizim ilk “Hello, world!” projesinde olduğu gibi, onu Cargo kullanan bir projeye dönüştürebilirsiniz. Proje kodunu `src` dizinine taşıyın ve uygun bir `Cargo.toml` dosyası oluşturun. Bu `Cargo.toml` dosyasını elde etmenin kolay bir yolu `cargo init` çalıştırmaktır; bu komut, dosyayı sizin için otomatik olarak oluşturacaktır.

## ⚙️ Bir Cargo Projesini Derlemek ve Çalıştırmak (Building and Running a Cargo Project)

Şimdi Cargo ile “Hello, world!” programını derleyip çalıştırdığımızda nelerin farklı olduğuna bakalım! `hello_cargo` dizinindeyken projenizi şu komutla derleyin:

```
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

👉 Bu komut, yürütülebilir bir dosyayı `target/debug/hello_cargo` (Windows’ta `target\debug\hello_cargo.exe`) içinde oluşturur; mevcut dizininizde değil. Varsayılan derleme bir `debug build` olduğu için Cargo ikili dosyayı (`binary`) `debug` adındaki dizine koyar. Çalıştırmak için şu komutu kullanabilirsiniz:

```
$ ./target/debug/hello_cargo   # veya Windows’ta .\target\debug\hello_cargo.exe
Hello, world!
```

👉 Her şey yolunda giderse, `Hello, world!` terminale yazdırılacaktır.
`cargo build` komutunu ilk çalıştırmanız aynı zamanda en üst dizinde `Cargo.lock` adında yeni bir dosya da oluşturur. Bu dosya, projenizde kullanılan bağımlılıkların kesin sürümlerini takip eder. Bu projenin bağımlılığı olmadığından dosya oldukça boştur. Bu dosyayı asla elle değiştirmenize gerek yoktur; Cargo içeriğini sizin için yönetir.

Bir projeyi `cargo build` ile derleyip `./target/debug/hello_cargo` ile çalıştırdık. Ancak `cargo run` komutunu kullanarak hem derlemeyi hem de programı tek bir adımda çalıştırabiliriz:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

👉 `cargo run`, önce `cargo build` çalıştırıp ardından oluşan ikili dosyayı çalıştırmaya eşdeğerdir. Bu yüzden çoğu geliştirici `cargo run` kullanır.

Bu sefer Cargo’nun derleme çıktısını göstermediğine dikkat edin. Cargo, dosyaların değişmediğini fark ettiği için yeniden derlemedi, sadece var olan ikiliyi çalıştırdı. Eğer kaynak kodunuzu değiştirseydiniz, Cargo projeyi çalıştırmadan önce yeniden derleyecek ve şu çıktıyı görecektiniz:

```
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo ayrıca `cargo check` adında bir komut da sağlar. Bu komut, kodunuzun derlenebilir olduğundan hızlıca emin olur ama yürütülebilir dosya üretmez:

```
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

👉 Neden yürütülebilir dosya istemeyebilirsiniz? Çünkü çoğu zaman `cargo check`, `cargo build`’den çok daha hızlıdır; çünkü ikili oluşturma adımını atlar. Eğer kod yazarken sürekli olarak çalışmanızı kontrol etmek istiyorsanız, `cargo check` süreci hızlandırır. Böylece birçok Rustacean programlarını yazarken periyodik olarak `cargo check` çalıştırır, daha sonra yürütülebilir dosyayı kullanmaya hazır olduklarında `cargo build` çalıştırırlar.

### 📋 Özet (Recap)

Şimdiye kadar Cargo hakkında şunları öğrendik:

* `cargo new` ile bir proje oluşturabiliriz.
* `cargo build` ile projeyi derleyebiliriz.
* `cargo run` ile projeyi tek adımda derleyip çalıştırabiliriz.
* `cargo check` ile ikili oluşturmadan yalnızca hataları kontrol edebiliriz.
* Cargo, çıktıları kodumuzun olduğu dizine değil, `target/debug` dizinine kaydeder.

Cargo kullanmanın ek bir avantajı, komutların hangi işletim sisteminde çalışıyor olursanız olun aynı olmasıdır. Bu noktadan sonra artık Linux, macOS veya Windows için ayrı komutlar vermeyeceğiz.


## 🚀 Yayın İçin Derleme (Building for Release)

Projeniz nihayet yayınlamaya hazır olduğunda, `cargo build --release` komutunu kullanarak optimizasyonlarla derleyebilirsiniz. Bu komut, yürütülebilir dosyayı `target/debug` yerine `target/release` dizininde oluşturur.

👉 Optimizasyonlar, Rust kodunuzun daha hızlı çalışmasını sağlar; ancak bu ayarların etkinleştirilmesi derleme süresini uzatır. İşte bu yüzden iki farklı profil vardır:

* **Geliştirme profili (development profile):** Hızlı ve sık yeniden derlemeler için.
* **Yayın profili (release profile):** Kullanıcıya vereceğiniz, tekrar tekrar derlenmeyecek ve mümkün olduğunca hızlı çalışacak nihai program için.

Eğer kodunuzun çalışma süresini ölçüyorsanız (benchmarking), mutlaka `cargo build --release` çalıştırın ve `target/release` içindeki yürütülebilir dosya ile ölçüm yapın.


## 📏 Alışılmış Bir Standart Olarak Cargo (Cargo as Convention)

Basit projelerde Cargo, yalnızca `rustc` kullanmaya kıyasla çok fazla avantaj sağlamaz; ancak programlarınız daha karmaşık hale geldikçe değerini kanıtlar. Programlar birden fazla dosyaya bölündüğünde veya bağımlılık (dependency) gerektiğinde, derlemeyi Cargo’nun yönetmesine izin vermek çok daha kolaydır.

`hello_cargo` projesi basit olsa da, artık Rust kariyerinizin geri kalanında kullanacağınız gerçek araçların çoğunu kullanmaktadır. Hatta, mevcut herhangi bir projede çalışmak için aşağıdaki komutlarla Git üzerinden kodu indirebilir, o projenin dizinine geçebilir ve derleyebilirsiniz:

```
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

👉 Daha fazla bilgi için Cargo’nun belgelerine göz atabilirsiniz.


## 📚 Özet (Summary)

Rust yolculuğunuza harika bir başlangıç yaptınız! Bu bölümde şunları öğrendiniz:

* `rustup` kullanarak Rust’ın en son kararlı sürümünü yüklemek
* Rust’ı daha yeni bir sürüme güncellemek
* Yerel olarak yüklenmiş belgeleri açmak
* `rustc` kullanarak doğrudan bir “Hello, world!” programı yazmak ve çalıştırmak
* Cargo standartlarını kullanarak yeni bir proje oluşturmak ve çalıştırmak

👉 Artık Rust kodunu okumaya ve yazmaya alışmak için daha kapsamlı bir program inşa etmenin tam zamanı. Bu nedenle 2. bölümde bir tahmin oyunu programı (guessing game) geliştireceğiz. Eğer önce Rust’taki yaygın programlama kavramlarının nasıl çalıştığını öğrenmek isterseniz, önce 3. bölüme bakabilir ve sonra 2. bölüme geri dönebilirsiniz.
