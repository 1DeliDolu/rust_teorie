## 🔧 Modülerliği ve Hata Yönetimini İyileştirmek İçin Refaktör (Refactoring to Improve Modularity and Error Handling)

Programımızı iyileştirmek için, programın yapısıyla ve potansiyel hataları ele alış biçimiyle ilgili dört sorunu düzelteceğiz. İlk olarak, `main` fonksiyonumuz (main function) şu anda iki görevi yerine getiriyor: argümanları ayrıştırıyor (parsing arguments) ve dosyaları okuyor (reading files). Programımız büyüdükçe, `main` fonksiyonunun ele aldığı ayrı görevlerin sayısı artacaktır. Bir fonksiyonun sorumlulukları arttıkça, onu kavramak zorlaşır, test etmek güçleşir ve parçalarından birini bozmadan değişiklik yapmak zorlaşır. En iyisi, işlevselliği ayırmak ve her fonksiyonun tek bir görevden sorumlu olmasını sağlamaktır.

Bu konu ikinci sorunla da bağlantılıdır: Her ne kadar `query` ve `file_path` programımızın yapılandırma değişkenleri (configuration variables) olsa da `contents` gibi değişkenler programın mantığını yürütmek için kullanılır. `main` uzadıkça kapsama (scope) almamız gereken değişken sayısı artacaktır; kapsamda ne kadar çok değişken olursa, her birinin amacını takip etmek o kadar zorlaşır. Amaçlarını netleştirmek için yapılandırma değişkenlerini tek bir yapıda gruplandırmak en iyisidir.

Üçüncü sorun, dosyayı okuma başarısız olduğunda bir hata mesajı yazdırmak için `expect` kullandık, ancak hata mesajı yalnızca “Should have been able to read the file” yazıyor. Bir dosyayı okumak çeşitli şekillerde başarısız olabilir: örneğin dosya eksik olabilir ya da dosyayı açmak için iznimiz olmayabilir. Şu anda durum ne olursa olsun her şey için aynı hata mesajını yazdırırdık ve bu da kullanıcıya herhangi bir bilgi vermezdi!

Dördüncü olarak, bir hatayı ele almak için `expect` kullanıyoruz ve kullanıcı programımızı yeterli sayıda argüman belirtmeden çalıştırırsa, problemi açıkça açıklamayan bir Rust “dizi sınırları dışında” hatası (index out of bounds error) alacaktır. Hata yönetimi (error handling) mantığının değişmesi gerekirse gelecekteki bakım yapanların yalnızca tek bir yere bakması için tüm hata yönetimi kodunun tek bir yerde olması en iyisi olur. Hata yönetimi kodunun tek bir yerde toplanması, son kullanıcılarımıza anlamlı mesajlar yazdırdığımızdan emin olmamızı da sağlar.

Bu dört sorunu projemizi refaktör (refactoring) ederek ele alalım.

## 🧩 İkili Projelerde Kaygıların Ayrılması (Separation of Concerns for Binary Projects)
## 📦 Yapılandırma Değerlerini Gruplama (Grouping Configuration Values)

`parse_config` fonksiyonunu biraz daha iyileştirmek için küçük bir adım daha atabiliriz. Şu anda bir **tuple** döndürüyoruz, ancak döndürdüğümüz tuple’ı hemen tek tek parçalara ayırıyoruz. Bu durum, henüz doğru soyutlamaya (abstraction) ulaşmadığımızın bir işaretidir.

Bir diğer geliştirme göstergesi ise `parse_config` içindeki *config* ifadesidir. Bu, döndürdüğümüz iki değerin ilişkili olduğunu ve tek bir yapılandırma değerinin (configuration value) parçası olduğunu ima ediyor. Ancak biz bu anlamı yalnızca iki değeri bir tuple içine koyarak ifade ediyoruz. Bunun yerine, bu iki değeri tek bir `struct` içine yerleştirebilir ve alanlarına (fields) anlamlı isimler verebiliriz. Bu, gelecekte kodu bakım yapacak kişilerin farklı değerlerin birbirleriyle nasıl ilişkili olduğunu ve amaçlarını daha kolay anlamasını sağlar.

---

### 📄 Dosya Adı: `src/main.rs`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // --snip--
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

12-6: `parse_config` fonksiyonunu bir `Config` struct örneği döndürecek şekilde refaktör etme

---

Burada `Config` adında, `query` ve `file_path` alanlarına sahip bir struct tanımladık. Artık `parse_config` fonksiyonunun imzası, `Config` tipinde bir değer döndürdüğünü gösteriyor. Fonksiyon gövdesinde, daha önce `args` içindeki `String` değerlerine referans döndürürken şimdi `Config` struct’ı, bu değerlerin *sahipli* (owned) kopyalarını içeriyor.

`main` içindeki `args` değişkeni argüman değerlerinin sahibidir ve sadece `parse_config` fonksiyonuna ödünç veriyordu (borrow). Eğer `Config` bu değerlerin sahipliğini almaya çalışsaydı, Rust’ın ödünç alma kurallarını ihlal ederdik.

`String` verisini yönetmenin birkaç yolu vardır. En kolayı, her ne kadar biraz verimsiz olsa da, `clone` metodunu çağırmaktır. Bu, `Config` örneği için verinin tam bir kopyasını oluşturur. Bu yöntem daha fazla zaman ve bellek kullanır, ancak kodumuzu çok daha basit hale getirir çünkü referansların ömürlerini (lifetimes) yönetmek zorunda kalmayız. Bu durumda, biraz performanstan vazgeçip sadelik kazanmak değerli bir tercihtir.

---

## ⚖️ `clone` Kullanımındaki Denge (The Trade-Offs of Using clone)

Birçok Rust geliştiricisinin (`Rustaceans`) sahiplik sorunlarını çözmek için `clone` kullanmaktan kaçınma eğilimi vardır, çünkü bunun çalışma zamanı maliyeti (runtime cost) vardır. Chapter 13’te bu tür durumlarda daha verimli yöntemleri nasıl kullanabileceğinizi öğreneceksiniz. Ancak şimdilik birkaç küçük `string` kopyalamak sorun değildir, çünkü bu kopyaları yalnızca bir kez oluşturacaksınız ve hem dosya yolu hem de arama dizesi çok küçüktür. İlk aşamada kodu fazla optimize etmeye çalışmak yerine biraz verimsiz ama çalışan bir program yazmak daha iyidir. Rust konusunda daha deneyimli olduğunuzda, başlangıçta en verimli çözümle ilerlemek daha kolay olacaktır, fakat şimdilik `clone` kullanmak gayet kabul edilebilir.

---

`main` fonksiyonunu güncelledik, artık `parse_config` tarafından döndürülen `Config` örneğini `config` adında bir değişkende saklıyoruz. Daha önce `query` ve `file_path` değişkenlerini doğrudan kullanan kodu da `Config` struct’ının alanlarını kullanacak şekilde güncelledik.

Artık kodumuz, `query` ve `file_path`’in ilişkili olduğunu ve amaçlarının programın çalışma biçimini yapılandırmak olduğunu çok daha açık bir şekilde ifade ediyor. Bu değerlere ihtiyaç duyan her kod parçası, `config` örneğinden ilgili alanlara bakarak onları bulabileceğini biliyor.

---

## 🏗️ Config için Yapıcı Fonksiyon (Creating a Constructor for Config)

Şu ana kadar komut satırı argümanlarını ayrıştırmaktan sorumlu mantığı `main`’den çıkardık ve `parse_config` fonksiyonuna koyduk. Bu, `query` ve `file_path` değerlerinin ilişkili olduğunu görmemize yardımcı oldu. Ardından bu ilişkiyi kodumuzda yansıtmak için `Config` struct’ını ekledik.

Artık `parse_config` fonksiyonunun amacı bir `Config` örneği oluşturmak olduğuna göre, `parse_config`’i basit bir fonksiyon olmaktan çıkarıp `Config` struct’ına bağlı bir `new` fonksiyonu yapabiliriz. Bu değişiklik kodu daha idiyomatik hale getirir. Standart kütüphane türlerinden örnekler oluşturmak için `String::new` çağırabildiğimiz gibi, artık `Config::new` çağırarak `Config` örnekleri oluşturabileceğiz.

---

### 📄 Dosya Adı: `src/main.rs`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
```

12-7: `parse_config` fonksiyonunu `Config::new` haline dönüştürme

---

`main` içinde daha önce `parse_config` çağırdığımız yerde artık `Config::new` çağırıyoruz. Ayrıca `parse_config`’in adını `new` olarak değiştirdik ve bir `impl` bloğu içine taşıdık; böylece `new` fonksiyonunu `Config` ile ilişkilendirmiş olduk.

Şimdi bu kodu tekrar derleyip çalıştırarak doğru şekilde çalıştığını doğrulayabilirsiniz.


## 📦 Yapılandırma Değerlerini Gruplama (Grouping Configuration Values)

`parse_config` fonksiyonunu biraz daha iyileştirmek için küçük bir adım atabiliriz. Şu anda bir **tuple** döndürüyoruz, ancak bu tuple’ı hemen parçalarına ayırıyoruz. Bu durum, henüz doğru soyutlamaya ulaşmadığımızın bir işaretidir.

Bir diğer gösterge, `parse_config` içindeki *config* ifadesidir. Bu, döndürdüğümüz iki değerin ilişkili olduğunu ve tek bir yapılandırma değerinin (configuration value) parçası olduğunu ima eder. Ancak biz bu anlamı yalnızca iki değeri bir tuple içine koyarak ifade ediyoruz. Bunun yerine, bu iki değeri bir `struct` içine koyup alanlarına (fields) anlamlı isimler verebiliriz. Bu, gelecekte kodun bakımını yapanların, değerlerin nasıl ilişkili olduğunu ve ne amaçla kullanıldıklarını daha kolay anlamalarını sağlar.

---

### 📄 Dosya Adı: `src/main.rs`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // --snip--
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

12-6: `parse_config` fonksiyonunu bir `Config` struct örneği döndürecek şekilde refaktör etme

---

Burada `Config` adında, `query` ve `file_path` alanlarına sahip bir struct tanımladık. Artık `parse_config` fonksiyonu bir `Config` döndürüyor. Fonksiyon gövdesinde, daha önce `args` içindeki `String` değerlerine referans döndürürken şimdi `Config`, bu değerlerin **sahipli** (`owned`) kopyalarını içeriyor.

`main` içindeki `args` değişkeni argüman değerlerinin sahibidir ve yalnızca `parse_config` fonksiyonuna ödünç veriyordu (`borrow`). Eğer `Config`, bu değerlerin sahipliğini almaya çalışsaydı, Rust’ın ödünç alma kurallarını ihlal ederdik.

`String` verisini yönetmenin çeşitli yolları vardır. En kolayı, biraz verimsiz olsa da, `clone` metodunu çağırmaktır. Bu yöntem, `Config` örneğinin verinin tam bir kopyasına sahip olmasını sağlar. Bu, referansların ömürlerini (`lifetimes`) yönetme gerekliliğini ortadan kaldırdığı için kodu oldukça basitleştirir. Bu durumda, biraz performanstan vazgeçip sadelik kazanmak değerli bir tercihtir.

---

## ⚖️ `clone` Kullanımının Dengesi (The Trade-Offs of Using clone)

Birçok Rust geliştiricisi (`Rustaceans`), sahiplik sorunlarını çözmek için `clone` kullanmaktan kaçınır, çünkü çalışma zamanı maliyeti vardır. Ancak Chapter 13’te bu tür durumlarda daha verimli yöntemleri öğreneceksiniz. Şimdilik, birkaç küçük `string`’i kopyalamak sorun değildir; bu kopyalar yalnızca bir kez yapılacak ve hem dosya yolu hem de arama dizesi küçüktür. İlk aşamada fazla optimize etmeye çalışmak yerine biraz verimsiz ama çalışan bir program yazmak daha iyidir.

---

`main` fonksiyonunda artık `parse_config`’in döndürdüğü `Config` örneğini `config` adında bir değişkende saklıyoruz. Daha önce `query` ve `file_path` değişkenlerini doğrudan kullanan kodu da `Config` struct’ının alanlarını kullanacak şekilde güncelledik.

Artık kodumuz, `query` ve `file_path`’in ilişkili olduğunu ve amaçlarının programın çalışma biçimini yapılandırmak olduğunu daha net gösteriyor. Bu değerlere ihtiyaç duyan her kod parçası, `config` örneğindeki ilgili alanlardan onları bulabiliyor.

---

## 🏗️ Config için Yapıcı Fonksiyon (Creating a Constructor for Config)

Şu ana kadar, komut satırı argümanlarını ayrıştırma işini `main`’den çıkarıp `parse_config` fonksiyonuna koyduk. Bu, `query` ve `file_path` değerlerinin ilişkili olduğunu fark etmemizi sağladı. Ardından bu ilişkiyi kodda yansıtmak için `Config` struct’ını ekledik.

Artık `parse_config` fonksiyonunun amacı bir `Config` örneği oluşturmak olduğuna göre, bu fonksiyonu basit bir fonksiyon olmaktan çıkarıp `Config` struct’ına bağlı bir `new` fonksiyonuna dönüştürebiliriz. Bu, kodu daha idiyomatik hale getirir. Standart kütüphane türlerinde örnekler oluşturmak için `String::new` çağırabildiğimiz gibi, artık `Config::new` çağırarak `Config` örnekleri oluşturabileceğiz.

---

### 📄 Dosya Adı: `src/main.rs`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
```

12-7: `parse_config` fonksiyonunu `Config::new` olarak değiştirme

---

`main` içinde daha önce `parse_config` çağırdığımız yerde artık `Config::new` çağırıyoruz. Ayrıca `parse_config`’in adını `new` olarak değiştirdik ve bir `impl` bloğunun içine taşıdık; böylece `new` fonksiyonu `Config` ile ilişkilendirilmiş oldu.

Şimdi bu kodu derleyip çalıştırarak doğru şekilde çalıştığını doğrulayabilirsiniz.


## 🧩 Yapılandırma Değerlerini Gruplama (Grouping Configuration Values)

`parse_config` fonksiyonunu daha da iyileştirmek için küçük bir adım daha atabiliriz. Şu anda bir ikili (tuple) (tuple) döndürüyoruz, ancak hemen ardından bu ikiliyi tekrar bireysel parçalara ayırıyoruz. Bu durum, muhtemelen henüz doğru soyutlamaya (abstraction) (abstraction) sahip olmadığımızın bir işaretidir.

Geliştirme için başka bir gösterge, `parse_config` içindeki *config* kısmıdır; bu, döndürdüğümüz iki değerin ilişkili olduğunu ve tek bir yapılandırma değeri (configuration value) (configuration value) içinde yer aldığını ima eder. Şu anda bu anlamı, iki değeri yalnızca bir ikili (tuple) içinde gruplayarak veri yapısında yansıtmıyoruz; bunun yerine bu iki değeri tek bir yapı (struct) (struct) içine koyacak ve yapı alanlarına (fields) (fields) anlamlı adlar vereceğiz. Bunu yapmak, bu kodun gelecekteki bakıcılarının farklı değerlerin birbirleriyle nasıl ilişkili olduğunu ve amaçlarını daha kolay anlamasını sağlayacaktır.

Aşağıdaki 12-6 numaralı liste, `parse_config` fonksiyonundaki iyileştirmeleri göstermektedir.

### 📄 Dosya Adı: `src/main.rs`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // --snip--
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

Liste 12-6: `parse_config` fonksiyonunu bir `Config` yapı (struct) örneği döndürecek şekilde yeniden düzenleme (refactoring)

`query` ve `file_path` adlı alanlara (fields) sahip `Config` adlı bir yapı (struct) ekledik. Artık `parse_config` imzası, bir `Config` değeri döndürdüğünü belirtmektedir. `parse_config` gövdesinde, daha önce `args` içindeki `String` değerlerine başvuran `string slice`lar (string slices) (string slices) döndürürken, şimdi `Config`’i sahipli (owned) (owned) `String` değerlerini içerecek şekilde tanımlıyoruz. `main` içindeki `args` değişkeni, argüman değerlerinin sahibidir ve `parse_config` fonksiyonuna yalnızca ödünç (borrow) (borrow) vermektedir; bu da `Config`’in `args` içindeki değerlere sahiplik almaya çalışması durumunda Rust’ın ödünç alma kurallarını (borrowing rules) (borrowing rules) ihlal edeceğimiz anlamına gelir.

`String` verisini yönetmenin bir dizi yolu vardır; en kolayı, her ne kadar biraz verimsiz olsa da, değerler üzerinde `clone` (clone) çağrısı yapmaktır. Bu, `Config` örneğinin sahip olacağı verinin tam bir kopyasını oluşturur; bu da `string` verisine referans saklamaya göre daha fazla zaman ve bellek kullanır. Ancak veriyi klonlamak, referansların ömürlerini (lifetimes) (lifetimes) yönetmek zorunda kalmayacağımız için kodumuzu oldukça yalın hale getirir; bu durumda, sadelik kazanmak adına biraz performanstan vazgeçmek makul bir değiş tokuştur (trade-off) (trade-off).

### ⚖️ `clone` Kullanımının Değiş Tokuşları (The Trade-Offs of Using clone)

Birçok Rust geliştiricisi, çalışma zamanı maliyeti (runtime cost) (runtime cost) nedeniyle sahiplik problemlerini `clone` ile çözmekten kaçınma eğilimindedir. Chapter 13’te bu tür durumlarda daha verimli yöntemleri nasıl kullanacağınızı öğreneceksiniz. Ancak şimdilik ilerlemeye devam edebilmek için birkaç `string` kopyalamak sorun değildir; çünkü bu kopyaları yalnızca bir kez yapacaksınız ve dosya yolu ile arama dizesi çok küçüktür. İlk geçişte kodu aşırı optimize etmeye çalışmaktansa, biraz verimsiz ama çalışan bir programın olması daha iyidir. Rust ile daha fazla deneyim kazandıkça, en verimli çözümle başlamak daha kolay olacaktır; ancak şimdilik `clone` çağırmak gayet kabul edilebilir.

`main` fonksiyonunu, `parse_config` tarafından döndürülen `Config` örneğini `config` adlı bir değişkene yerleştirecek şekilde güncelledik ve önceden ayrı `query` ve `file_path` değişkenlerini kullanan kodu, artık `Config` yapısındaki alanları kullanacak şekilde düzenledik.

Artık kodumuz, `query` ve `file_path`’in ilişkili olduğunu ve amaçlarının programın nasıl çalışacağını yapılandırmak olduğunu daha net bir şekilde ifade ediyor. Bu değerleri kullanan herhangi bir kod, amaçlarına göre adlandırılmış alanlarda, `config` örneği içinde onları bulacağını bilir.

---

## 🏗️ Config için Bir Yapıcı Oluşturma (Creating a Constructor for Config)

Şimdiye kadar, komut satırı argümanlarını ayrıştırmaktan sorumlu mantığı `main` içinden çıkarıp `parse_config` fonksiyonuna yerleştirdik. Bunu yapmak, `query` ve `file_path` değerlerinin ilişkili olduğunu görmemize yardımcı oldu ve bu ilişkiyi kodumuzda ifade ettik. Ardından, `query` ve `file_path`’in ilişkili amacını adlandırmak ve `parse_config` fonksiyonundan alan adları (field names) olarak döndürmek için bir `Config` yapı (struct) ekledik.

Artık `parse_config` fonksiyonunun amacı bir `Config` örneği oluşturmak olduğuna göre, `parse_config`’i sıradan bir fonksiyon olmaktan çıkarıp `Config` yapısıyla ilişkilendirilmiş `new` (constructor) (new/constructor) adlı bir fonksiyona dönüştürebiliriz. Bu değişiklik kodu daha idiyomatik yapar. Standart kütüphane türlerinin örneklerini, örneğin `String`, `String::new` çağırarak oluşturabiliriz. Benzer şekilde, `parse_config`’i `Config` ile ilişkilendirilmiş bir `new` fonksiyonuna dönüştürerek `Config` örneklerini `Config::new` çağırarak oluşturabiliriz. 12-7 numaralı liste, yapmamız gereken değişiklikleri göstermektedir.

### 📄 Dosya Adı: `src/main.rs`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
```

Liste 12-7: `parse_config`’i `Config::new` şeklinde değiştirme

`main` içinde daha önce `parse_config` çağırdığımız yerde artık `Config::new` çağırıyoruz. `parse_config` adını `new` olarak değiştirdik ve bir `impl` bloğu içine taşıdık; bu da `new` fonksiyonunu `Config` ile ilişkilendirir. Kodun çalıştığından emin olmak için bu kodu yeniden derlemeyi deneyin.
## 🧰 Hata Yönetimini Düzeltme (error handling) (Fixing the Error Handling)

Şimdi hata yönetimimizi düzeltmek üzerinde çalışacağız. `args` vektöründeki 1 veya 2 numaralı indeksteki değerlere erişmeye çalışmanın, vektör üçten az öğe içeriyorsa programın paniklemesine (panic) neden olacağını hatırlayın. Programı hiçbir argüman olmadan çalıştırmayı deneyin; çıktısı şöyle görünecektir:

```bash
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`

thread 'main' panicked at src/main.rs:27:21:
index out of bounds: the len is 1 but the index is 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`index out of bounds: the len is 1 but the index is 1` satırı, programcılara yönelik bir hata mesajıdır. Son kullanıcılarımızın ne yapmaları gerektiğini anlamalarına yardımcı olmaz. Bunu şimdi düzeltelim.

## 📝 Hata Mesajını İyileştirme (Improving the Error Message)

Liste 12-8’de, dilimdeki (slice) yeterli uzunluğu, 1 ve 2 numaralı indekslere erişmeden önce doğrulayacak bir kontrolü yeni (new) fonksiyonuna ekliyoruz. Eğer dilim yeterince uzun değilse, program panikler (panic) ve daha iyi bir hata mesajı gösterir.

### 📄 Dosya Adı: `src/main.rs`

```rust
    // --snip--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        // --snip--
```

Liste 12-8: Argüman sayısı için bir kontrol ekleme

Bu kod, Liste 9-13’te yazdığımız `Guess::new` fonksiyonuna benzer; orada değer argümanı geçerli değer aralığının dışında olduğunda `panic!` çağırmıştık. Burada bir değer aralığını kontrol etmek yerine, `args` uzunluğunun en az 3 olduğunu kontrol ediyoruz ve fonksiyonun geri kalanı bu koşulun sağlandığı varsayımıyla çalışabilir. `args` üçten az öğeye sahipse, bu koşul doğru olur ve programı derhal sonlandırmak için `panic!` makrosunu çağırırız.

Bu birkaç ek satırla `new` içinde, programı yine argümansız çalıştırıp hatanın şimdi nasıl göründüğüne bakalım:

```bash
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`

thread 'main' panicked at src/main.rs:26:13:
not enough arguments
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Bu çıktı daha iyidir: artık makul bir hata mesajımız var. Ancak kullanıcılarımıza vermek istemediğimiz fazladan bilgiler de var. Liste 9-13’te kullandığımız teknik burada en iyisi olmayabilir: Chapter 9’da tartışıldığı gibi, `panic!` çağrısı kullanım hatalarından ziyade programlama hataları için daha uygundur. Bunun yerine, Chapter 9’da öğrendiğiniz diğer tekniği kullanacağız — başarıyı veya hatayı belirten bir `Result` (Result) döndürmek.

## 🔁 `panic!` Çağırmak Yerine `Result` Döndürme (Returning a Result Instead of Calling panic!)

Bunun yerine, başarılı durumda bir `Config` (Config) örneği içeren ve hata durumunda problemi tanımlayan bir `Result` değeri döndürebiliriz. Ayrıca fonksiyon adını `new`’den `build`’e değiştireceğiz, çünkü birçok programcı `new` fonksiyonlarının asla başarısız olmayacağını bekler. `Config::build` (Config::build) `main` ile iletişim kurarken, bir sorun olduğunu belirtmek için `Result` türünü kullanabiliriz. Ardından `main`’i, `panic!` çağrısının neden olduğu `thread 'main'` ve `RUST_BACKTRACE` hakkındaki metinler olmadan, `Err` (Err) varyantını kullanıcılarımız için daha pratik bir hataya dönüştürecek şekilde değiştirebiliriz.

Liste 12-9, artık `Config::build` olarak adlandırdığımız fonksiyonun dönüş değerinde yapmamız gereken değişiklikleri ve bir `Result` döndürmek için gerekli gövdeyi gösterir. Dikkat edin: bunu `main`’i de güncelleyene kadar derleyemezsiniz; bir sonraki listede bunu yapacağız.

### 📄 Dosya Adı: `src/main.rs`

*Bu kod derlenmez! (This code does not compile!)*

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

Liste 12-9: `Config::build`’ten bir `Result` döndürme

`build` fonksiyonumuz, başarı durumunda bir `Config` örneği ve hata durumunda bir string literal (string literal) içeren bir `Result` döndürür. Hata değerlerimiz her zaman `'static` ömürlü (lifetime) string literal’ler olacaktır.

Fonksiyon gövdesinde iki değişiklik yaptık: kullanıcı yeterli argüman geçmediğinde `panic!` çağırmak yerine artık bir `Err` değeri döndürüyoruz ve `Config` dönüş değerini `Ok` içine sarıyoruz. Bu değişiklikler, fonksiyonun yeni tip imzasına (type signature) uymasını sağlar.

`Config::build`’ten bir `Err` değeri döndürmek, `main` fonksiyonunun `build` fonksiyonundan döndürülen `Result` değerini ele almasına ve hata durumunda süreci daha temiz bir şekilde sonlandırmasına olanak tanır.

## 📣 `Config::build`’i Çağırma ve Hataları Ele Alma (Calling Config::build and Handling Errors)

Hata durumunu ele almak ve kullanıcı dostu bir mesaj yazdırmak için, `main`’i `Config::build` tarafından döndürülen `Result`’ı işleyecek şekilde güncellememiz gerekir; bu, Liste 12-10’da gösterilmiştir. Ayrıca, komut satırı aracını sıfırdan farklı (nonzero) bir hata koduyla (exit status code) sonlandırma sorumluluğunu `panic!`’ten alıp bunu elle uygulayacağız. Sıfırdan farklı bir çıkış durumu (exit status), programımızı çağıran sürece programın bir hata durumuyla sonlandığını bildirmek için bir konvansiyondur.

### 📄 Dosya Adı: `src/main.rs`

```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
```

Liste 12-10: Bir `Config` oluşturma başarısız olursa bir hata koduyla çıkma

Bu listede, henüz ayrıntılı olarak ele almadığımız bir yöntemi kullandık: `unwrap_or_else` (unwrap\_or\_else); bu yöntem standart kütüphanede `Result<T, E>` (Result\<T, E>) üzerinde tanımlıdır. `unwrap_or_else` kullanmak, panic olmayan özel hata yönetimi (non-`panic!` error handling) tanımlamamıza olanak tanır. `Result` bir `Ok` değeri ise, bu yöntemin davranışı `unwrap`’a (unwrap) benzer: `Ok`’un sardığı iç değeri döndürür. Ancak değer bir `Err` ise, bu yöntem `closure` (closure) içindeki kodu çağırır; bu, `unwrap_or_else`’e argüman olarak tanımlayıp verdiğimiz anonim bir fonksiyondur. Closure’ları (closures) Chapter 13’te daha ayrıntılı inceleyeceğiz. Şimdilik bilmeniz gereken, `unwrap_or_else`’in `Err`’in iç değerini — bu durumda, Liste 12-9’da eklediğimiz statik string `"not enough arguments"` — closure’a, dikey çizgiler (| |) arasında görünen `err` argümanı olarak ileteceğidir. Closure içindeki kod daha sonra çalıştığında bu `err` değerini kullanabilir.

Standart kütüphaneden `process`’i kapsama (scope) almak için yeni bir `use` satırı ekledik. Hata durumunda çalışacak closure içindeki kod yalnızca iki satırdır: `err` değerini yazdırır ve ardından `process::exit` çağrısını yapar. `process::exit` fonksiyonu programı hemen durdurur ve çıkış durumu kodu olarak geçirilen sayıyı döndürür. Bu, Liste 12-8’de kullandığımız `panic!` tabanlı ele almaya benzer, ancak artık tüm ek çıktıları görmeyiz. Deneyelim:

```bash
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

Harika! Bu çıktı, kullanıcılarımız için çok daha dostça.

## 🔄 `main` Fonksiyonundan Mantığı Ayırma (Extracting Logic from the main Function)

Yapılandırma ayrıştırmasını (configuration parsing) refaktör etmeyi bitirdiğimize göre, şimdi programın mantığına geçelim. “İkili Projelerde Kaygıların Ayrılması (Separation of Concerns for Binary Projects)” bölümünde belirttiğimiz gibi, `run` adlı bir fonksiyon çıkaracağız ve bu fonksiyon, yapılandırma ayarlama veya hata yönetimiyle ilgili olmayan tüm mantığı tutacak. İşimiz bittiğinde `main` fonksiyonu oldukça kısa olacak ve gözden geçirerek doğrulaması kolay hale gelecek. Ayrıca geri kalan mantık için testler yazabileceğiz.

Aşağıdaki Liste 12-11, küçük ve artımlı bir iyileştirme olarak `run` fonksiyonunu çıkarmayı göstermektedir.

---

### 📄 Dosya Adı: `src/main.rs`

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// --snip--
```

**Liste 12-11**: Program mantığının geri kalanını içeren `run` fonksiyonunu çıkarma

---

`run` fonksiyonu artık `main`’deki geri kalan tüm mantığı içeriyor, yani dosyayı okumayla başlıyor. `run` fonksiyonu bir `Config` örneğini argüman olarak alıyor.

---

## 📤 `run` Fonksiyonundan Hata Döndürme (Returning Errors from the run Function)

Program mantığını `run` fonksiyonuna ayırdıktan sonra, hata yönetimini de iyileştirebiliriz. Liste 12-9’da `Config::build` ile yaptığımız gibi, `expect` çağırarak paniklemek yerine, bir şeyler ters gittiğinde `Result<T, E>` döndürmesini sağlayacağız. Böylece hata yönetimi mantığını `main` içinde, kullanıcı dostu bir şekilde birleştirebiliriz.

### 📄 Dosya Adı: `src/main.rs`

```rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
```

**Liste 12-12**: `run` fonksiyonunu `Result` döndürecek şekilde değiştirme

---

Burada üç önemli değişiklik yaptık:

1. `run` fonksiyonunun dönüş tipini `Result<(), Box<dyn Error>>` olarak değiştirdik. Önceden `()` (unit type) döndürüyordu, bunu da başarı durumunda `Ok(())` olarak koruduk.
2. Hata türü için `Box<dyn Error>` kullandık (başta `std::error::Error`’u `use` ifadesiyle kapsam içine aldık). Bu, fonksiyonun `Error` trait’ini uygulayan herhangi bir türü döndürebileceği anlamına gelir.
3. `expect` yerine `?` operatörünü kullandık. Hata oluşursa `panic!` yapmak yerine, `?` hatayı çağırana geri döndürür.

Artık `run` başarı durumunda `Ok(())` döndürüyor. Bu, `run`’ı yalnızca yan etkileri (side effects) için çağırdığımızı belirtmek için idiyomatik bir yoldur.

---

Bu kodu çalıştırdığınızda derlenecek, ancak şu uyarıyı göreceksiniz:

```bash
warning: unused `Result` that must be used
  --> src/main.rs:19:5
   |
19 |     run(config);
   |     ^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
```

Rust, `Result` değerini göz ardı ettiğimizi ve bunun bir hata içerebileceğini söylüyor. Bu da derleyicinin bize hata yönetimini eklememiz gerektiğini hatırlattığı anlamına geliyor. Şimdi bunu düzelteceğiz.

---

## 🛠️ `run` Fonksiyonundan Dönen Hataları `main` İçinde Ele Alma (Handling Errors Returned from run in main)

Hataları kontrol edip ele almak için `Config::build` ile Liste 12-10’da kullandığımız tekniğe benzer bir yöntem kullanacağız, fakat küçük bir farkla:

### 📄 Dosya Adı: `src/main.rs`

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

Burada `unwrap_or_else` yerine `if let` kullandık. Çünkü `run`, başarı durumunda `()` döndürür ve `()` bizim için anlamlı bir değer değildir. Bizim tek derdimiz hata olup olmadığını kontrol etmektir. Eğer `Err` dönerse, hata mesajını yazdırır ve `process::exit(1)` ile çıkarız.

---

## 📚 Kodu Bir Kütüphane Crate’ine Bölmek (Splitting Code into a Library Crate)

`minigrep` projemiz şu ana kadar iyi gidiyor! Şimdi `src/main.rs` dosyasını bölecek ve bazı kodları `src/lib.rs` dosyasına koyacağız. Böylece kodu test edebiliriz ve `src/main.rs` dosyasının sorumlulukları daha az olur.

Artık metin aramadan sorumlu kodu `src/lib.rs` içinde tanımlayacağız. Bu, bizim (veya `minigrep` kütüphanemizi kullanan başkalarının) arama fonksiyonunu sadece `minigrep` binary’sinden değil, başka bağlamlardan da çağırmasına izin verecek.

---

### 📄 Dosya Adı: `src/lib.rs`

*Bu kod derlenmez!*

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    unimplemented!();
}
```

**Liste 12-13**: `src/lib.rs` içinde `search` fonksiyonunu tanımlama

---

Burada `pub` anahtar kelimesini kullanarak `search` fonksiyonunu kütüphanemizin genel API’sine dahil ettik. Artık bir kütüphane crate’imiz var; bu kodu binary crate’imizden kullanabiliriz ve test de edebiliriz!

---

Şimdi `src/lib.rs` içinde tanımladığımız `search` fonksiyonunu binary crate’in kapsamına dahil edip çağırmamız gerekiyor.

### 📄 Dosya Adı: `src/main.rs`

```rust
// --snip--
use minigrep::search;

fn main() {
    // --snip--
}

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```

**Liste 12-14**: `src/main.rs` içinde `minigrep` kütüphane crate’indeki `search` fonksiyonunu kullanma

---

`use minigrep::search` satırını ekleyerek `search` fonksiyonunu binary crate kapsamına aldık. Artık `run` fonksiyonu, dosya içeriğini yazdırmak yerine `search` fonksiyonunu çağırıyor, `config.query` ve `contents` değerlerini argüman olarak veriyor ve dönen satırları yazdırıyor.

Bu noktada, `main` fonksiyonundaki `println!` çağrılarını kaldırmamız gerekir; program sadece arama sonuçlarını yazdırmalıdır (hata yoksa).

Not: `search` fonksiyonu sonuçları döndürmeden önce bir vektörde topluyor. Bu, büyük dosyalarda sonuçların ekranda yavaş görünmesine neden olabilir. Çünkü sonuçlar bulunur bulunmaz yazdırılmaz. Chapter 13’te yineleyiciler (iterators) kullanarak bunu nasıl iyileştirebileceğimizi tartışacağız.

---

Phew! Epey iş yaptık, ama gelecekte başarı için sağlam bir temel oluşturduk. Artık hataları yönetmek çok daha kolay ve kodumuz daha modüler. Bundan sonra yapacağımız işlerin çoğu `src/lib.rs` dosyasında olacak.

Şimdi bu yeni modülerlikten yararlanarak eskiden zor olacak ama artık kolay hale gelen bir şey yapalım: **testler yazalım!** ✅
