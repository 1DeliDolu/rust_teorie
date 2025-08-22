## 🚀 I/O Projemizi İyileştirme (Improving Our I/O Project)

Yineleyiciler (iterator) hakkındaki bu yeni bilgilerle, Bölüm 12’deki G/Ç (I/O) projemizi yineleyicileri kullanarak kodun bazı kısımlarını daha anlaşılır ve özlü hale getirerek iyileştirebiliriz. Yineleyicilerin, `Config::build` işlevi (function) ve `search` işlevinin uygulanmasını nasıl iyileştirebileceğine bakalım.

## 🧹 Bir Klonu Kaldırma: Bir Yineleyici Kullanma (Removing a clone Using an Iterator)

Liste 12-6’da, bir `String` diliminden (slice) kesit alıp, dilimde indeksleme (indexing) yaparak değerleri klonlayıp (clone) `Config` yapısının (struct) bir örneğini oluşturmuştuk; böylece `Config` bu değerlere sahip olabiliyordu (sahiplik/ownership). Liste 13-17’de, `Config::build` işlevinin Liste 12-23’teki hâlini yeniden ürettik.

Filename: src/lib.rs

```rust
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

Listing 13-17: Listing 12-23’teki `Config::build` işlevinin yeniden üretimi

O zamanlar, verimsiz `clone` çağrılarını dert etmememizi, bunları ileride kaldıracağımızı söylemiştik. İşte o zaman şimdi!

Burada `clone`’a ihtiyaç duyduk çünkü parametre `args` içinde `String` öğeleri bulunan bir dilim (slice) var, ancak `build` işlevi `args`’a sahip değil (ödünç alma/borrowing). Bir `Config` örneğinin sahipliğini döndürebilmek için, `Config`’in `query` ve `file_path` alanlarındaki değerleri klonlayarak, `Config` örneğinin kendi değerlerine sahip olmasını sağladık.

Yineleyiciler (iterator) hakkındaki yeni bilgimizle, `build` işlevini bir dilimi ödünç almak yerine, argüman olarak bir yineleyicinin sahipliğini alacak şekilde değiştirebiliriz. Dilimin uzunluğunu denetleyip belirli konumlara indeksleme yapmak yerine yineleyici işlevselliğini kullanacağız. Bu, `Config::build` işlevinin ne yaptığını netleştirecek; çünkü yineleyici değerlere erişecek.

`Config::build` yineleyicinin sahipliğini alıp indeksleme gibi ödünç alan işlemleri kullanmayı bıraktığında, `String` değerlerini `clone` çağırmadan ve yeni bir tahsis (allocation) yapmadan yineleyiciden `Config` içine taşıyabiliriz (move).

## 🔄 Döndürülen Yineleyiciyi Doğrudan Kullanma (Using the Returned Iterator Directly)

G/Ç projenizin `src/main.rs` dosyasını açın; şu şekilde görünmelidir:

Filename: src/main.rs

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
}
```

Önce, Liste 12-24’teki `main` fonksiyonunun başlangıcını bu kez bir yineleyici kullanan Liste 13-18’deki kodla değiştireceğiz. `Config::build`’i de güncelleyene kadar bu derlenmeyecek.

Filename: src/main.rs
This code does not compile!

```rust
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
}
```

Listing 13-18: `env::args`’ın dönüş değerini `Config::build`’e aktarma

`env::args` işlevi bir yineleyici (iterator) döndürür! Yineleyici değerlerini bir vektöre toplayıp ardından `Config::build`’e bir dilim (slice) geçirmek yerine, şimdi `env::args`’ın döndürdüğü yineleyicinin sahipliğini doğrudan `Config::build`’e iletiyoruz.

Sırada `Config::build` tanımını güncellemek var. G/Ç projenizin `src/lib.rs` dosyasında, `Config::build` imzasını Liste 13-19’daki gibi değiştirelim. İşlev gövdesini de güncellememiz gerektiğinden bu hâliyle hâlâ derlenmeyecek.

Filename: src/lib.rs
This code does not compile!

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // --snip--
```

Listing 13-19: `Config::build` imzasını bir yineleyici bekleyecek şekilde güncelleme

Standart kitaplıktaki `env::args` belgeleri, döndürdüğü yineleyicinin türünün `std::env::Args` olduğunu ve bu türün `Iterator` özelliğini (trait) uyguladığını (implements) ve `String` değerleri döndürdüğünü gösterir.

`Config::build` işlevinin imzasını, `args` parametresi için `&[String]` yerine, `impl Iterator<Item = String>` özellik sınırına (trait bound) sahip genel (generic) bir türe güncelledik. Bölüm 10’daki “Parametreler olarak Özellikler (Traits as Parameters)” kısmında tartıştığımız `impl Trait` sözdiziminin (syntax) bu kullanımı, `args`’ın `Iterator` özelliğini uygulayan ve `String` öğeleri döndüren herhangi bir tür olabileceği anlamına gelir.

`args`’ın sahipliğini aldığımız ve yineleyici üzerinde yineleme yaparak onu değiştireceğimiz için, `args` parametresini değiştirilebilir yapmak üzere `mut` anahtar sözcüğünü ekleyebiliriz.

## 🧩 İndeksleme Yerine Iterator Özellik Yöntemlerini Kullanma (Using Iterator Trait Methods Instead of Indexing)

Şimdi `Config::build` gövdesini düzelteceğiz. `args` `Iterator` özelliğini uyguladığına göre, üzerinde `next` yöntemini çağırabileceğimizi biliyoruz! Liste 13-20, Liste 12-23’teki kodu `next` yöntemini kullanacak şekilde güncelliyor.

Filename: src/lib.rs

```rust
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

Listing 13-20: `Config::build` gövdesini yineleyici yöntemlerini kullanacak şekilde değiştirme

`env::args`’ın döndürdüğü ilk değer programın adıdır. Bunu yok saymak ve bir sonraki değere geçmek istediğimiz için önce `next` çağırırız ve dönüş değerini kullanmayız. Sonra `Config`’in `query` alanına koymak istediğimiz değeri almak için `next` çağırırız. `next` `Some` döndürürse, değeri çıkarmak için bir `match` (eşleştirme) kullanırız. `None` döndürürse, yeterli argüman verilmediği anlamına gelir ve erken `Err` değeriyle döneriz. `file_path` değeri için de aynı işlemi yaparız.

## ✨ Yineleyici Adaptörleriyle (Iterator Adapters) Kodu Daha Açık Hale Getirme (Making Code Clearer with Iterator Adapters)

G/Ç (I/O) projemizdeki `search` işlevinde de yineleyicilerden yararlanabiliriz. Bu işlev, Liste 12-19’daki hâliyle Liste 13-21’de yeniden üretilmiştir:

Filename: src/lib.rs
Listing 13-21: Liste 12-19’daki `search` işlevinin uygulanması

Bu kodu, yineleyici adaptör (iterator adapter) yöntemlerini kullanarak daha özlü bir şekilde yazabiliriz. Bunu yapmak ayrıca, değiştirilebilir (mutable) bir ara `results` vektörüne ihtiyaç duymamızı da engeller. Fonksiyonel programlama tarzı, kodu daha açık hale getirmek için değiştirilebilir durumun (mutable state) miktarını en aza indirmeyi tercih eder. Değiştirilebilir durumu kaldırmak, gelecekte aramayı paralel olarak çalıştırmaya olanak sağlayabilir; çünkü `results` vektörüne eşzamanlı erişimi yönetmemiz gerekmeyecektir. Bu değişiklik Liste 13-22’de gösterilmektedir:

Filename: src/lib.rs

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

Listing 13-22: `search` işlevinin uygulanmasında yineleyici adaptör yöntemlerini kullanma

`search` işlevinin amacı, `contents` içindeki `query`’yi içeren tüm satırları döndürmektir. Liste 13-16’daki `filter` örneğine benzer şekilde, bu kod `filter` adaptörünü kullanarak yalnızca `line.contains(query)` ifadesi `true` döndüren satırları tutar. Ardından eşleşen satırları `collect` ile başka bir vektöre toplarız. Çok daha basit! Aynı değişikliği `search_case_insensitive` işlevinde de yineleyici yöntemleri kullanacak şekilde yapmaktan çekinmeyin.

## 🔄 Döngüler mi Yineleyiciler mi? (Choosing Between Loops or Iterators)

Sıradaki mantıklı soru şudur: Kendi kodunuzda hangi tarzı seçmelisiniz ve neden? Liste 13-21’deki orijinal uygulama mı, yoksa Liste 13-22’deki yineleyicileri kullanan sürüm mü? Çoğu Rust programcısı yineleyici tarzını tercih eder. İlk başta kavraması biraz zor olsa da, çeşitli yineleyici adaptörlerine ve onların ne yaptığına alıştıktan sonra yineleyiciler daha kolay anlaşılabilir hale gelir. Yeni vektörler oluşturmak ve döngülerle uğraşmak yerine, kod döngünün yüksek düzeyli amacına odaklanır. Bu da sıradan kod parçalarını soyutlar (abstract away) ve bu koda özgü kavramların —örneğin, yineleyicideki her öğenin geçmesi gereken filtreleme koşulu— daha net görünmesini sağlar.

Peki, bu iki uygulama gerçekten eşdeğer midir? Sezgisel varsayım, daha düşük seviyeli döngünün daha hızlı olacağıdır. Haydi performanstan bahsedelim.
