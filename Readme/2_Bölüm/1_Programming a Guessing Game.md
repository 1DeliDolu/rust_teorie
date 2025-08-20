## 🎮 Bir Tahmin Oyunu Programlama (programming a guessing game)

Hadi Rust’a, birlikte uygulamalı bir projeyle giriş yapalım! Bu bölüm, gerçek bir programda nasıl kullanılacaklarını göstererek birkaç yaygın Rust kavramını tanıtıyor. `let` anahtar sözcüğü (let), `match` deyimi (match), yöntemler (methods), ilişkilendirilmiş fonksiyonlar (associated functions), harici crate’ler (external crates) ve daha fazlasını öğreneceksiniz! Sonraki bölümlerde bu fikirleri daha ayrıntılı inceleyeceğiz. Bu bölümde ise yalnızca temelleri pratik edeceksiniz.

Klasik bir başlangıç programlama problemini uygulayacağız: bir tahmin oyunu. İşleyiş şöyle: program 1 ile 100 arasında rastgele bir tamsayı üretir. Ardından oyuncudan bir tahmin girmesini ister. Tahmin girildikten sonra, program tahminin çok düşük mü yoksa çok yüksek mi olduğunu belirtir. Tahmin doğruysa, oyun tebrik mesajı yazdırır ve sonlanır.
## 🛠️ Yeni Bir Proje Kurma (setting up a new project)

Yeni bir proje kurmak için, 1. bölümde oluşturduğunuz `projects` dizinine gidin ve Cargo kullanarak yeni bir proje oluşturun:

```
$ cargo new guessing_game
$ cd guessing_game
```

İlk komut olan `cargo new`, projenin adını (`guessing_game`) ilk argüman olarak alır. İkinci komut, yeni projenin dizinine geçiş yapar.

Oluşturulan `Cargo.toml` dosyasına bakın:

**Dosya adı: Cargo.toml**

```
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
```

1. bölümde gördüğünüz gibi, `cargo new` sizin için bir “Hello, world!” programı üretir. `src/main.rs` dosyasına göz atın:

**Dosya adı: src/main.rs**

```
fn main() {
    println!("Hello, world!");
}
```

Şimdi bu “Hello, world!” programını `cargo run` komutu ile derleyelim ve aynı adımda çalıştıralım:

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/guessing_game`
Hello, world!
```

`run` komutu, bir proje üzerinde hızlı yinelemeler yapmanız gerektiğinde oldukça kullanışlıdır. Bu oyunda da olduğu gibi, her yinelemeyi hızlıca test edip bir sonrakine geçerken işimizi kolaylaştırır.

`src/main.rs` dosyasını yeniden açın. Yazacağınız tüm kod bu dosyada olacak.
 ## 🔤 Bir Tahmini İşleme (processing a guess)

Tahmin oyunu programının ilk kısmı kullanıcıdan girdi isteyecek, bu girdiyi işleyecek ve girdinin beklenen biçimde olup olmadığını kontrol edecektir. Başlamak için, oyuncunun bir tahmin girmesine izin vereceğiz. Aşağıdaki 2-1 numaralı listeyi `src/main.rs` dosyasına yazın.

**Dosya adı: src/main.rs**

```
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

**Liste 2-1: Kullanıcıdan bir tahmin alıp ekrana yazdıran kod**

Bu kod birçok bilgi içeriyor, bu yüzden satır satır üzerinden geçelim. Kullanıcıdan girdi almak ve sonucu çıktı olarak yazdırmak için, giriş/çıkış kütüphanesi olan `io`’yu kapsamımıza (scope) dahil etmemiz gerekir. `io` kütüphanesi, standart kütüphaneden (`std`) gelir:

```
use std::io;
```

Varsayılan olarak Rust, her programın kapsamına aldığı standart kütüphanede tanımlanmış bir öğe kümesine sahiptir. Bu küme *prelude* olarak adlandırılır ve standart kütüphane belgelerinde tamamını görebilirsiniz.

Kullanmak istediğiniz bir tür *prelude* içinde değilse, onu açıkça `use` ifadesiyle kapsamınıza dahil etmeniz gerekir. `std::io` kütüphanesini kullanmak, kullanıcı girdisi kabul etme özelliği de dahil olmak üzere birçok faydalı işlev sunar.

1. bölümde gördüğünüz gibi, `main` fonksiyonu (main function) programın giriş noktasıdır:

```
fn main() {
```

`fn` sözdizimi yeni bir fonksiyon tanımlar; parantezler `()` parametre olmadığını belirtir; süslü parantez `{` ise fonksiyonun gövdesini başlatır.

Yine 1. bölümde öğrendiğiniz gibi, `println!` bir makrodur (macro) ve ekrana bir dize (string) yazdırır:

```
    println!("Guess the number!");

    println!("Please input your guess.");
```

Bu kod, oyunun ne olduğunu belirten ve kullanıcıdan bir girdi isteyen bir mesaj ekrana yazdırır.
## 📦 Değerleri Değişkenlerle Saklama (storing values with variables)

Şimdi kullanıcı girdisini saklamak için bir değişken oluşturacağız, şöyle:

```
let mut guess = String::new();
```

İşte program ilginçleşmeye başladı! Bu küçük satırda birçok şey oluyor. `let` deyimini (let statement) değişken oluşturmak için kullanıyoruz. İşte başka bir örnek:

```
let apples = 5;
```

Bu satır, `apples` adında yeni bir değişken oluşturur ve ona `5` değerini bağlar. Rust’ta değişkenler varsayılan olarak değiştirilemezdir (immutable), yani bir değişkene değer verdikten sonra bu değer değiştirilemez. Bu kavramı 3. bölümdeki “Değişkenler ve Değiştirilebilirlik” (Variables and Mutability) kısmında ayrıntılı olarak tartışacağız. Bir değişkeni değiştirilebilir yapmak için (mutable), değişken adının önüne `mut` ekleriz:

```
let apples = 5; // değiştirilemez
let mut bananas = 5; // değiştirilebilir
```

Not: `//` sözdizimi (syntax), satırın sonuna kadar devam eden bir yorum (comment) başlatır. Rust yorumların içindeki her şeyi yok sayar. Yorumları da 3. bölümde daha ayrıntılı inceleyeceğiz.

Tahmin oyunu programına dönersek, artık `let mut guess` ifadesinin `guess` adında değiştirilebilir bir değişken tanıttığını biliyorsunuz. Eşittir işareti (`=`), Rust’a bu değişkene bir değer bağlamak istediğimizi söyler. Eşittirin sağında, `guess` değişkenine bağlanan değer vardır; bu da `String::new` fonksiyonunu çağırmanın sonucudur. `String::new`, yeni bir `String` örneği döndüren bir fonksiyondur. `String`, standart kütüphanede bulunan, büyüyebilen ve UTF-8 kodlamalı bir metin türüdür.

`::` sözdizimi, `::new` satırında, `new` fonksiyonunun `String` türüne ait bir ilişkilendirilmiş fonksiyon (associated function) olduğunu belirtir. İlişkilendirilmiş fonksiyon, bir türe (bu durumda `String`) uygulanmış fonksiyondur. Bu `new` fonksiyonu, yeni ve boş bir string oluşturur. `new` fonksiyonunu birçok türde bulacaksınız, çünkü bu, bir türün yeni bir değerini oluşturan fonksiyonlar için yaygın bir isimdir.

Kısacası, `let mut guess = String::new();` satırı, şu anda yeni ve boş bir `String` örneğine bağlı olan değiştirilebilir bir değişken oluşturmuştur. Oh!
## ⌨️ Kullanıcı Girdisini Alma (receiving user input)

Hatırlarsanız, programın ilk satırında standart kütüphaneden giriş/çıkış (input/output) işlevselliğini `use std::io;` ifadesiyle dahil etmiştik. Şimdi `io` modülünden `stdin` fonksiyonunu çağıracağız; bu, kullanıcı girdisini işlememizi sağlar:

```
    io::stdin()
        .read_line(&mut guess)
```

Programın başında `use std::io;` ifadesiyle `io` modülünü içe aktarmamış olsaydık, bu fonksiyon çağrısını yine de `std::io::stdin` şeklinde yazarak kullanabilirdik. `stdin` fonksiyonu, terminalinizdeki standart girişe erişimi temsil eden bir tür olan `std::io::Stdin` örneğini döndürür.

Sonraki satır olan `.read_line(&mut guess)`, standart giriş erişimi üzerinde `read_line` metodunu çağırarak kullanıcıdan girdi alır. Ayrıca `read_line` metoduna argüman olarak `&mut guess` geçiriyoruz; bu da kullanıcı girdisinin hangi string içinde saklanacağını belirtir. `read_line` metodunun tam görevi, kullanıcının standart girişe yazdığı her şeyi alıp bir string’e eklemektir (içeriğini silmeden). Bu nedenle, bu string’i argüman olarak geçiririz. String argümanının değiştirilebilir (mutable) olması gerekir ki metod içeriğini değiştirebilsin.

`&` işareti, bu argümanın bir *referans* (reference) olduğunu gösterir. Referanslar, veriyi bellekte kopyalamak zorunda kalmadan kodunuzun birden çok kısmının aynı veriye erişmesini sağlar. Referanslar karmaşık bir özelliktir, ancak Rust’ın büyük avantajlarından biri referansları güvenli ve kolay kullanabilmesidir. Bu programı bitirmek için tüm detayları bilmenize gerek yok. Şimdilik bilmeniz gereken, değişkenler gibi referansların da varsayılan olarak değiştirilemez (immutable) olduğudur. Bu nedenle, değiştirilebilir hale getirmek için `&guess` yerine `&mut guess` yazmanız gerekir. (Referanslar 4. bölümde çok daha ayrıntılı açıklanacaktır.)

## ⚠️ Olası Hataları Result ile Ele Alma (handling potential failure with result)

Hâlâ aynı kod satırı üzerinde çalışıyoruz. Şimdi üçüncü kısmını tartışıyoruz, ama bunun hâlâ tek bir mantıksal satırın parçası olduğunu unutmayın. Sıradaki kısım şu metottur:

```
        .expect("Failed to read line");
```

Bu kodu şöyle de yazabilirdik:

```
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Ancak bu kadar uzun bir satır okumayı zorlaştırır, bu yüzden onu bölmek en iyisidir. `.method_name()` sözdizimiyle bir metot çağırırken uzun satırları bölebilmek için yeni satır ve boşluk eklemek çoğu zaman akıllıcadır. Şimdi bu satırın ne yaptığını inceleyelim.

Daha önce belirtildiği gibi, `read_line` kullanıcıdan gelen girdiyi kendisine verdiğimiz string’e ekler; ama aynı zamanda bir `Result` değeri döndürür. `Result`, *enum* (enumeration) adı verilen, birden fazla olası durumda bulunabilen bir türdür. Bu olası durumların her birine *variant* (varyant) denir.

6. bölümde *enum*’ları daha ayrıntılı inceleyeceğiz. `Result` türlerinin amacı, hata yönetimi bilgisini kodlamaktır.

`Result`’ın varyantları `Ok` ve `Err`’dir. `Ok` varyantı işlemin başarılı olduğunu gösterir ve başarıyla üretilen değeri içerir. `Err` varyantı işlemin başarısız olduğunu gösterir ve hatanın nasıl veya neden oluştuğuna dair bilgiyi içerir.

Her türün değerlerinde olduğu gibi, `Result` değerlerinin de metotları vardır. Bir `Result` örneği, çağırabileceğiniz bir `expect` metoduna sahiptir. Eğer bu `Result` örneği bir `Err` değeriyse, `expect` programın çökmesine sebep olur ve argüman olarak verdiğiniz mesajı ekrana yazdırır. Eğer `read_line` metodu bir `Err` döndürürse, bu muhtemelen altta yatan işletim sisteminden kaynaklanan bir hata olur. Eğer bu `Result` örneği bir `Ok` değeriyse, `expect` `Ok`’un tuttuğu değeri alır ve size sadece o değeri döndürür. Bu durumda, döndürülen değer kullanıcının girdisinin byte cinsinden uzunluğudur.

`expect` çağrısı yapmazsanız, program yine derlenir ama bir uyarı alırsınız:

```
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
10 |     let _ = io::stdin().read_line(&mut guess);
   |     +++++++
```

```
warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
```

Rust, `read_line` tarafından döndürülen `Result` değerini kullanmadığınız konusunda sizi uyarır; bu da programın olası bir hatayı ele almadığını gösterir.

Bu uyarıyı bastırmanın doğru yolu gerçekten hata yönetimi kodu yazmaktır; ama bizim durumumuzda, bir sorun çıktığında bu programın doğrudan çökmesini istediğimiz için `expect` kullanabiliriz. Hatalardan kurtulmayı (recovering from errors) ise 9. bölümde öğreneceksiniz.
 ## 🖨️ Değerleri println! Yer Tutucuları ile Yazdırma (printing values with println! placeholders)

Kapanış süslü parantezi dışında, şimdiye kadarki kodda tartışmamız gereken yalnızca bir satır daha var:

```
    println!("You guessed: {guess}");
```

Bu satır, artık kullanıcının girdisini içeren string’i ekrana yazdırır. `{}` süslü parantezler bir *yer tutucu*dur (placeholder): `{}`’yi bir değeri yerinde tutan küçük yengeç kıskacı gibi düşünebilirsiniz. Bir değişkenin değerini yazdırırken değişken adı süslü parantezlerin içine yazılabilir. Bir ifadenin (expression) değerlendirilmesinin sonucunu yazdırırken, format string’in içine boş süslü parantezler yerleştirirsiniz, ardından format string’den sonra her boş yer tutucuya karşılık gelen ifadeleri virgülle ayırarak yazarsınız. `println!` makrosuyla bir değişken ve bir ifadenin sonucunu tek seferde yazdırmak şöyle görünür:

```
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

Bu kod şunu yazdırır:
`x = 5 and y + 2 = 12`

---

## 🧪 İlk Kısmı Test Etme (testing the first part)

Şimdi tahmin oyununun ilk kısmını test edelim. `cargo run` komutu ile çalıştırın:

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

Bu noktada oyunun ilk kısmı tamamlanmış oluyor: Klavyeden girdi alıyor ve ardından bunu ekrana yazdırıyoruz.

 ## 🔢 Gizli Bir Sayı Üretme (generating a secret number)

Sıradaki adım, kullanıcının tahmin etmeye çalışacağı gizli sayıyı üretmek. Gizli sayı her seferinde farklı olmalı ki oyun birden fazla kez oynandığında eğlenceli kalsın. Oyunun çok zor olmaması için 1 ile 100 arasında rastgele bir sayı kullanacağız.

Rust, henüz standart kütüphanesinde rastgele sayı üretme işlevselliğini içermiyor. Ancak Rust ekibi bu işlevi sağlayan `rand` adlı bir crate sunuyor.
## 📦 Daha Fazla İşlevsellik için Bir Crate Kullanma (using a crate to get more functionality)

Unutmayın ki bir *crate*, Rust kaynak kodu dosyalarının bir koleksiyonudur. Şimdiye kadar inşa ettiğimiz proje bir *binary crate*’tir, yani çalıştırılabilir bir dosyadır. `rand` crate’i ise bir *library crate*’tir; yani başka programlarda kullanılmak üzere yazılmış kod içerir ve tek başına çalıştırılamaz.

Harici crate’lerin yönetiminde Cargo’nun gücü ortaya çıkar. `rand` crate’ini kullanacak kodu yazmadan önce, `Cargo.toml` dosyasını değiştirip `rand` crate’ini bir bağımlılık olarak eklememiz gerekir. Bu dosyayı açın ve Cargo’nun sizin için oluşturduğu `[dependencies]` başlığının altına şu satırı ekleyin. Burada verilen sürüm numarasını birebir yazmanız önemlidir; aksi halde bu öğreticideki kod örnekleri çalışmayabilir:

**Dosya adı: Cargo.toml**

```
[dependencies]
rand = "0.8.5"
```

`Cargo.toml` dosyasında, bir başlığın altında yer alan her şey, yeni bir bölüm başlayana kadar o bölümün parçasıdır. `[dependencies]` kısmında, projenizin hangi harici crate’lere bağımlı olduğunu ve bu crate’lerin hangi sürümlerini istediğinizi belirtirsiniz. Bu durumda `rand` crate’ini semantik sürüm belirtimi (semantic version specifier) `0.8.5` ile ekledik. Cargo, Semantik Sürümleme’yi (Semantic Versioning, kısaca *SemVer*) anlar; bu, sürüm numaralarının yazımı için standarttır. Aslında `0.8.5`, `^0.8.5`’in kısaltmasıdır; yani en az `0.8.5` olan ama `0.9.0`’dan küçük tüm sürümler kabul edilir.

Cargo, bu sürümlerin `0.8.5` ile uyumlu bir genel API’ye sahip olduğunu varsayar. Bu sayede, burada yazdığımız kodla uyumlu olacak en güncel *patch* sürümünü elde etmiş olursunuz. `0.9.0` veya üzeri sürümler ise aynı API’ye sahip olma garantisini vermez.

Şimdi, herhangi bir kod değişikliği yapmadan projeyi derleyelim. Aşağıda Liste 2-2’de gösterildiği gibi:

```
$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```

**Liste 2-2: `rand` crate’i bağımlılık olarak eklendikten sonra `cargo build` çıktısı**

Siz farklı sürüm numaraları görebilirsiniz (ama SemVer sayesinde tümü kodla uyumlu olacaktır) ve farklı satırlar olabilir (işletim sistemine bağlı olarak) ve satırların sırası da farklılık gösterebilir.

Harici bir bağımlılık eklediğimizde Cargo, o bağımlılığın ihtiyaç duyduğu her şeyin en güncel sürümünü *registry*’den getirir. *Registry*, Crates.io’daki verilerin bir kopyasıdır. Crates.io, Rust ekosistemindeki geliştiricilerin açık kaynak Rust projelerini yayınladıkları yerdir.

Registry güncellendikten sonra Cargo, `[dependencies]` bölümünü kontrol eder ve listelenmiş olup daha önce indirilmemiş crate’leri indirir. Bu durumda biz yalnızca `rand`’ı eklemiş olsak da, Cargo `rand`’ın çalışabilmesi için ihtiyaç duyduğu diğer crate’leri de indirir. İndirme tamamlandıktan sonra Rust bu crate’leri derler ve ardından bağımlılıklarla birlikte projeyi derler.

Eğer hemen ardından hiçbir değişiklik yapmadan `cargo build` çalıştırırsanız, `Finished` satırı dışında herhangi bir çıktı almazsınız. Cargo, bağımlılıkların zaten indirildiğini ve derlendiğini bilir; `Cargo.toml` dosyanızda da bir değişiklik yoktur. Ayrıca kodunuzda da bir değişiklik yapmadığınız için onu da yeniden derlemez. Yapacak bir iş olmadığında sadece sonlanır.

Eğer `src/main.rs` dosyasını açıp ufak bir değişiklik yapar, kaydedip tekrar derlerseniz yalnızca şu iki satırı görürsünüz:

```
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

Bu satırlar, Cargo’nun yalnızca `src/main.rs` dosyasındaki küçük değişikliği derlemeye dahil ettiğini gösterir. Bağımlılıklarınız değişmediği için Cargo, zaten indirilmiş ve derlenmiş olanları yeniden kullanabileceğini bilir.


## 📑 Cargo.lock Dosyası ile Tekrarlanabilir Derlemeleri Sağlama (ensuring reproducible builds with the Cargo.lock file)

Cargo, sizin veya başkasının kodunuzu her derlediğinde aynı çıktıyı yeniden üretebilmenizi sağlayan bir mekanizmaya sahiptir: Siz farklı bir şey belirtmedikçe, Cargo yalnızca belirttiğiniz bağımlılık sürümlerini kullanır.

Örneğin, gelecek hafta `rand` crate’inin `0.8.6` sürümü çıkarsa ve bu sürüm önemli bir hata düzeltmesi içerse ama aynı zamanda kodunuzu bozacak bir gerileme (regression) barındırsa, Rust bunu yönetebilmek için ilk kez `cargo build` çalıştırdığınızda `Cargo.lock` dosyasını oluşturur. Artık `guessing_game` dizininde bu dosyaya sahibiz.

Bir projeyi ilk kez derlediğinizde, Cargo kriterlere uyan tüm bağımlılık sürümlerini belirler ve bunları `Cargo.lock` dosyasına yazar. Daha sonra projeyi yeniden derlediğinizde Cargo, `Cargo.lock` dosyasının var olduğunu görür ve sürümleri yeniden hesaplamak yerine orada belirtilen sürümleri kullanır. Bu da size otomatik olarak tekrarlanabilir (reproducible) bir derleme sağlar. Başka bir deyişle, `Cargo.lock` dosyası sayesinde projeniz siz açıkça yükseltme yapana kadar `0.8.5` sürümünde kalacaktır.

`Cargo.lock` dosyası tekrarlanabilir derlemeler için önemli olduğundan, genellikle projenizdeki diğer kodlarla birlikte sürüm kontrolüne dahil edilir.
## ⬆️ Yeni Bir Sürüm İçin Crate Güncelleme (updating a crate to get a new version)

Bir crate’i güncellemek istediğinizde, Cargo `update` komutunu sağlar. Bu komut `Cargo.lock` dosyasını yok sayar ve `Cargo.toml` içindeki tanımlamanıza uyan en güncel sürümleri bulur. Daha sonra bu sürümleri `Cargo.lock` dosyasına yazar. Bu durumda Cargo yalnızca `0.8.5`’ten büyük ve `0.9.0`’dan küçük sürümlere bakacaktır.

Eğer `rand` crate’i `0.8.6` ve `0.9.0` sürümlerini yayımlamışsa, `cargo update` komutunu çalıştırdığınızda şu çıktıyı görürsünüz:

```
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.9.0)
```

Cargo `0.9.0` sürümünü yok sayar. Bu noktada, `Cargo.lock` dosyanızda artık `rand` crate’inin `0.8.6` sürümünü kullandığınızı belirten bir değişiklik fark edersiniz. Eğer `rand`’in `0.9.0` sürümünü veya `0.9.x` serisinden bir sürümü kullanmak istiyorsanız, `Cargo.toml` dosyanızı şu şekilde güncellemeniz gerekir:

```
[dependencies]
rand = "0.9.0"
```

Sonraki sefer `cargo build` çalıştırdığınızda Cargo, kullanılabilir crate’lerin kaydını günceller ve `rand` için gereksinimlerinizi belirttiğiniz yeni sürüme göre tekrar değerlendirir.

Cargo ve ekosistemi hakkında söylenecek daha çok şey var; bunları 14. bölümde tartışacağız. Şimdilik bilmeniz gereken bu kadar. Cargo, kütüphaneleri tekrar kullanmayı çok kolaylaştırır; bu sayede Rust geliştiricileri (Rustaceans) çok sayıda paketten oluşan daha küçük projeler yazabilirler.

---

## 🎲 Rastgele Bir Sayı Üretme (generating a random number)

Şimdi `rand` crate’ini kullanarak tahmin edilecek sayıyı üretmeye başlayalım. Bir sonraki adım, `src/main.rs` dosyasını aşağıdaki 2-3 numaralı listedeki gibi güncellemektir.

**Dosya adı: src/main.rs**

```
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

**Liste 2-3: Rastgele sayı üretmek için kod ekleme**

Önce şu satırı ekledik:

```
use rand::Rng;
```

`Rng` trait’i, rastgele sayı üreteçlerinin (random number generators) uyguladığı metotları tanımlar ve bu metotları kullanabilmek için trait’in kapsamda olması gerekir. *Trait*’ler 10. bölümde ayrıntılı olarak incelenecektir.

Ardından ortada iki satır ekledik. İlk satırda, `rand::thread_rng` fonksiyonunu çağırıyoruz. Bu fonksiyon bize kullanacağımız rastgele sayı üretecini verir: mevcut yürütme iş parçacığına (thread) özgü olan ve işletim sistemi tarafından başlatılan bir üreteç. Sonrasında bu üreteç üzerinde `gen_range` metodunu çağırıyoruz. Bu metod, `use rand::Rng;` ifadesiyle kapsamımıza aldığımız `Rng` trait’i tarafından tanımlanmıştır. `gen_range` bir aralık ifadesini (range expression) argüman olarak alır ve bu aralıkta rastgele bir sayı üretir. Burada kullandığımız aralık ifadesi `start..=end` biçimindedir ve alt ile üst sınırları kapsayıcıdır. Bu nedenle 1 ile 100 arasında bir sayı istemek için `1..=100` yazmamız gerekir.

Not: Hangi trait’leri kullanacağınızı ve hangi metot veya fonksiyonları çağıracağınızı bilmek her zaman kolay değildir; bu yüzden her crate, onu nasıl kullanacağınıza dair belgelerle birlikte gelir. Cargo’nun güzel bir özelliği de `cargo doc --open` komutunu çalıştırarak tüm bağımlılıkların belgelerini yerelde oluşturup tarayıcıda açabilmesidir. Örneğin `rand` crate’inde başka işlevlere ilgi duyarsanız `cargo doc --open` komutunu çalıştırın ve sol kenar çubuğunda `rand`’e tıklayın.

Eklenen ikinci satır gizli sayıyı ekrana yazdırır. Bu, programı geliştirirken test edebilmek için kullanışlıdır; ancak final sürümde silinecektir. Program başlar başlamaz cevabı yazdırırsa pek oyun sayılmaz!

Şimdi programı birkaç kez çalıştırmayı deneyin:

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4
```

```
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

Her çalıştırmada farklı rastgele sayılar elde etmelisiniz ve bunların tümü 1 ile 100 arasında olmalıdır. Harika iş! 🎉
## 🔍 Tahmini Gizli Sayı ile Karşılaştırma (comparing the guess to the secret number)

Artık elimizde kullanıcı girdisi ve rastgele bir sayı var; şimdi bunları karşılaştırabiliriz. Bu adım, 2-4 numaralı listede gösterilmiştir. Ancak bu kod henüz derlenmeyecektir; nedenini açıklayacağız.

**Dosya adı: src/main.rs**
**⚠️ Bu kod derlenmez!**

```
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

**Liste 2-4: İki sayıyı karşılaştırmanın olası sonuçlarını ele alma**

---

### 📌 Ordering Enum’u Kullanmak

Öncelikle standart kütüphaneden `std::cmp::Ordering` adlı türü kapsamımıza aldık. `Ordering` türü bir başka *enum*’dur ve `Less`, `Greater`, `Equal` adlı varyantlara sahiptir. İki değeri karşılaştırdığınızda ortaya çıkabilecek üç olası sonuç bunlardır.

Sonrasında en alta `Ordering` türünü kullanan beş yeni satır ekledik. `cmp` metodu iki değeri karşılaştırır ve karşılaştırılabilir herhangi bir şey üzerinde çağrılabilir. Argüman olarak karşılaştırmak istediğiniz değerin referansını alır: burada `guess` ile `secret_number` karşılaştırılıyor. Bu metod, `use` ifadesiyle kapsamımıza aldığımız `Ordering` enum’unun bir varyantını döndürür. Daha sonra, hangi `Ordering` varyantının döndüğüne göre ne yapılacağını belirlemek için `match` ifadesini kullanırız.

---

### 📌 Match İfadesi

Bir `match` ifadesi *kollardan* (arms) oluşur. Bir kol, eşleşecek bir desen (pattern) ve bu desene uyan bir değer geldiğinde çalıştırılacak koddur. Rust, `match` içine verilen değeri alır ve sırasıyla her kolun desenine bakar. Desenler ve `match` yapısı Rust’ın güçlü özelliklerindendir: Kodunuzun karşılaşabileceği pek çok durumu ifade etmenizi sağlar ve hepsini ele aldığınızdan emin olur. Bu özellikler 6. ve 19. bölümlerde ayrıntılı olarak incelenecektir.

Bir örnek üzerinden gidelim. Diyelim ki kullanıcı `50` tahmin etti ve rastgele üretilen gizli sayı `38` oldu.

Kod `50` ile `38`’i karşılaştırdığında `cmp` metodu `Ordering::Greater` döndürür çünkü `50`, `38`’den büyüktür. `match` ifadesi `Ordering::Greater` değerini alır ve kollara bakmaya başlar. İlk kol `Ordering::Less`’tir; bu eşleşmez, bu yüzden o kol atlanır. İkinci kol `Ordering::Greater`’dır ve bu eşleşir! Böylece bu kola ait kod çalışır ve ekrana **Too big!** yazdırılır. İlk başarılı eşleşmeden sonra `match` ifadesi sona erer, son kol değerlendirilmez.

---

### ⚠️ Derleme Hatası

Ancak Liste 2-4’teki kod derlenmez. Denediğinizde şu hata alırsınız:

```
error[E0308]: mismatched types
  --> src/main.rs:23:21
   |
23 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
   |                 |
   |                 arguments to this method are incorrect
```

Bu hata türlerin uyuşmadığını belirtir. Rust güçlü ve statik bir tip sistemine sahiptir, ancak aynı zamanda tip çıkarımı (type inference) da yapar. `let mut guess = String::new()` yazdığımızda, Rust bunun bir `String` olacağını çıkarsamıştı. Öte yandan `secret_number` bir sayı tipidir. Rust’ta 1 ile 100 arasında olabilecek birkaç sayı türü vardır: `i32` (32 bit tam sayı), `u32` (işaretsiz 32 bit tam sayı), `i64` ve diğerleri. Başka türlü belirtilmediğinde Rust varsayılan olarak `i32` kullanır; bu yüzden `secret_number` da `i32`’dir. Ancak Rust, bir `String` ile bir sayıyı karşılaştıramaz.

---

### 📌 String’i Sayıya Çevirmek

Sonuçta, kullanıcı girdisini `String` olarak aldıktan sonra bunu bir sayı tipine çevirmemiz gerekir. Bunun için `main` fonksiyonuna şu satırı ekleriz:

**Dosya adı: src/main.rs**

```
    // --snip--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

Yeni eklenen satır şudur:

```
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Burada yine `guess` adında bir değişken oluşturuyoruz. Ama zaten `guess` adında bir değişkenimiz vardı, değil mi? Evet, ama Rust aynı ada sahip önceki değerin üzerine yenisini tanımlamamıza (shadowing) izin verir. Shadowing, örneğin `guess_str` ve `guess` gibi iki farklı isim kullanmak yerine aynı ismi yeniden kullanmamıza olanak tanır. Bu özellik özellikle bir değeri bir türden başka bir türe dönüştürürken sıkça kullanılır (3. bölümde ayrıntısıyla ele alınacak).

`guess.trim().parse()` ifadesini bu yeni değişkene bağlıyoruz. Buradaki `guess`, orijinal `String` değerini ifade eder. `trim` metodu, baştaki ve sondaki boşlukları temizler. Kullanıcı `read_line` için enter tuşuna bastığında, girdiye bir satır sonu karakteri eklenir (`\n` veya Windows’ta `\r\n`). Örneğin kullanıcı `5` yazıp enter’a bastığında `guess` şu hale gelir: `"5\n"`. `trim` bu `\n` veya `\r\n` karakterini kaldırır ve sadece `"5"` kalır.

`parse` metodu ise string’i başka bir türe dönüştürür. Burada string’i sayıya dönüştürüyoruz. Rust’a tam olarak hangi sayı türünü istediğimizi `let guess: u32` ifadesiyle belirtmeliyiz. Buradaki `u32`, işaretsiz 32 bit tam sayıdır ve küçük pozitif sayılar için iyi bir varsayılan seçimdir. Ayrıca bu durumda `u32` kullandığımız için Rust, `secret_number`’ı da `u32` olarak değerlendirecektir. Böylece artık karşılaştırma iki aynı tür arasında yapılabilir.

---

### ⚠️ parse ile Hata İhtimali

`parse` yalnızca mantıksal olarak sayıya çevrilebilecek karakterlerde çalışır, bu nedenle hata ihtimali vardır. Örneğin string `"A👍%"` gibi bir değer içerirse, bu sayıya çevrilemez. Bu yüzden `parse` de bir `Result` değeri döndürür, tıpkı `read_line` gibi. Burada da `expect` metodunu kullanıyoruz. Eğer `parse` bir `Err` döndürürse program çöker ve mesajı ekrana yazdırır. Eğer `Ok` dönerse, sayı alınır ve kullanılır.

---

### 🏃 Programı Çalıştırma

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Güzel! Tahminin öncesinde boşluk karakterleri olsa bile program girdiyi temizleyip doğru değerlendirdi.

Şimdi programı birkaç kez çalıştırarak farklı durumları test edin: doğru tahmin, çok büyük tahmin, çok küçük tahmin.

Artık oyunun büyük kısmı çalışıyor, ancak kullanıcı yalnızca bir tahmin yapabiliyor. Bunu bir döngü (loop) ekleyerek değiştirelim!
## 🔁 Birden Fazla Tahmine İzin Vermek İçin Döngü Kullanma (allowing multiple guesses with looping)

`loop` anahtar sözcüğü sonsuz bir döngü oluşturur. Kullanıcıya sayıyı tahmin etmek için daha fazla şans vermek amacıyla bir döngü ekleyeceğiz:

**Dosya adı: src/main.rs**

```rust
    // --snip--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

Gördüğünüz gibi, tahmin isteminden (guess input prompt) sonraki her şeyi bir döngünün içine aldık. Döngü içindeki satırların her birini dört boşluk daha içeriden yazmaya dikkat edin ve programı tekrar çalıştırın.

Artık program sonsuza kadar yeni bir tahmin isteyecektir. Bu ise yeni bir sorun doğuruyor: Kullanıcı oyunu bırakamıyor gibi görünüyor!

Kullanıcı her zaman `ctrl-c` klavye kısayolunu kullanarak programı kesebilir. Ama başka bir yol daha var: “Tahmini Gizli Sayı ile Karşılaştırma” bölümünde `parse` metodunu anlatırken değindiğimiz gibi, kullanıcı sayı olmayan bir şey girerse program çöker. Bunu kullanarak kullanıcıya oyundan çıkma imkânı tanıyabiliriz. Örneğin:

```
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit

thread 'main' panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`quit` yazmak oyunu sonlandırır; ancak fark edeceğiniz üzere, sayı olmayan herhangi bir giriş de aynı sonucu verir. Bu pek de ideal bir durum değildir; oyunun, aynı zamanda doğru sayı tahmin edildiğinde de durmasını isteriz.
## 🏆 Doğru Tahmin Sonrası Çıkış (quitting after a correct guess)

Oyuncu doğru tahminde bulunduğunda oyundan çıkması için programa `break` ifadesini ekleyelim:

**Dosya adı: src/main.rs**

```rust
        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

`You win!` mesajından sonra eklenen `break` satırı, kullanıcı gizli sayıyı doğru tahmin ettiğinde döngüden çıkmasını sağlar. Döngüden çıkmak aynı zamanda `main` fonksiyonunun da bitmesi anlamına gelir; yani program tamamen sonlanır.

---

## 🚫 Geçersiz Girdiyi Ele Alma (handling invalid input)

Oyunun davranışını daha da iyileştirmek için, kullanıcı sayı olmayan bir şey girdiğinde programın çökmesi yerine girdiyi yok sayalım ve kullanıcıya yeniden tahmin etme hakkı verelim. Bunu yapmak için `guess` değerini `String`’den `u32`’ye dönüştürdüğümüz satırı değiştireceğiz. Aşağıda 2-5 numaralı listede gösterildiği gibi:

**Dosya adı: src/main.rs**

```rust
        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // --snip--
```

**Liste 2-5: Sayı olmayan girdiyi yok sayıp, kullanıcıdan yeni tahmin isteme**

Burada `expect` çağrısı yerine bir `match` ifadesi kullanıyoruz; yani hatada çökme davranışını bırakıp hatayı yönetiyoruz. `parse` metodunun bir `Result` döndürdüğünü hatırlayın; `Result` enum’u `Ok` ve `Err` varyantlarına sahiptir. Daha önce `cmp` metodunun sonucunu işlerken olduğu gibi, burada da `match` kullanıyoruz.

* Eğer `parse` string’i başarıyla sayıya çevirebilirse, `Ok` döner. Bu durumda `num` değişkeni elde edilir ve yeni `guess` değişkenine atanır.
* Eğer `parse` string’i sayıya çeviremezse, `Err` döner. Bu durumda ikinci kol `Err(_)` ile eşleşir. `_` (alt çizgi) tüm değerleri yakalayan (catch-all) bir desendir. Burada, içindeki bilgilere bakmaksızın tüm `Err` değerlerini karşılamak istediğimizi söylüyoruz. Program bu durumda `continue` çalıştırır, yani döngünün sonraki adımına geçip yeniden tahmin ister. Böylece program, `parse` sırasında oluşabilecek tüm hataları yok sayar.

---

## ✅ Tamamlanmış Kod (final version)

Artık programımız beklendiği gibi çalışıyor. Son bir küçük ayarla oyunu tamamlıyoruz: Program hâlâ gizli sayıyı ekrana yazdırıyor. Bu test için faydalıydı, ama oyunu bozuyor. Bu yüzden gizli sayıyı yazdıran `println!` satırını silmeliyiz. Tam kod aşağıdaki gibidir:

**Dosya adı: src/main.rs**

```rust
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

**Liste 2-6: Tahmin oyununun tamamlanmış kodu**

---

## 📋 Özet (summary)

Bu projede Rust’ın birçok yeni kavramını uygulamalı olarak öğrendiniz:

* `let` deyimi,
* `match` ifadeleri,
* fonksiyonlar,
* harici crate kullanımı,
  ve daha fazlası.

Sonraki bölümlerde bu kavramları daha ayrıntılı inceleyeceksiniz.

* **3. bölüm**: Çoğu programlama dilinde bulunan kavramlar (değişkenler, veri türleri, fonksiyonlar) ve bunların Rust’ta kullanımı.
* **4. bölüm**: Rust’ı diğer dillerden ayıran özellik olan *sahiplik* (ownership).
* **5. bölüm**: *Struct*’lar ve yöntem (method) sözdizimi.
* **6. bölüm**: *Enum*’ların nasıl çalıştığı.

Tebrikler, tahmin oyununu başarıyla tamamladınız! 🎉
