## 👋 Merhaba, Dünya! (Hello, World!)

Artık Rust’ı kurduğunuza göre, ilk Rust programınızı yazma zamanı geldi. Yeni bir dili öğrenirken ekrana **Hello, world!** metnini yazdıran küçük bir program yazmak geleneksel bir adımdır. Biz de burada aynısını yapacağız!

Not: Bu kitap komut satırı (command line) hakkında temel bir aşinalık varsayar. Rust, düzenleme, araçlar veya kodunuzun nerede bulunacağı konusunda herhangi bir özel gereklilik getirmez. Eğer komut satırı yerine bir tümleşik geliştirme ortamı (IDE - Integrated Development Environment) kullanmayı tercih ediyorsanız, favori IDE’nizi kullanabilirsiniz. Günümüzde birçok IDE, belirli bir seviyede Rust desteğine sahiptir; detaylar için IDE’nin belgelerine göz atın. Rust ekibi, `rust-analyzer` aracılığıyla mükemmel IDE desteği sağlamaya odaklanmaktadır. Daha fazla ayrıntı için Ek D’ye bakınız.

## 📂 Proje Dizini Oluşturma (Creating a Project Directory)

Öncelikle Rust kodunuzu saklayacağınız bir dizin oluşturacaksınız. Kodunuzun nerede bulunduğu Rust için önemli değildir, ancak bu kitaptaki alıştırmalar ve projeler için ev dizininizde (home directory) bir `projects` dizini oluşturmanızı ve tüm projelerinizi burada tutmanızı öneriyoruz.

Bir terminal açın ve aşağıdaki komutları girerek önce `projects` dizinini, ardından bu dizin içinde **Hello, world!** projesine ait bir dizin oluşturun.

Linux, macOS ve Windows PowerShell için şu komutları girin:

```
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

👉 Bu komutlar, `projects` adında bir dizin ve içinde `hello_world` dizinini oluşturur.

Windows CMD için şu komutları girin:

```
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

👉 Bu komutlar, Windows kullanıcı dizininizde `projects` klasörünü ve içinde `hello_world` klasörünü oluşturur.


## 📝 Bir Rust Programı Yazmak ve Çalıştırmak (Writing and Running a Rust Program)

Sonraki adımda yeni bir kaynak dosyası (source file) oluşturun ve adını `main.rs` koyun. Rust dosyaları her zaman `.rs` uzantısı ile biter. Dosya adında birden fazla kelime kullanıyorsanız, kelimeleri ayırmak için alt çizgi (`_`) kullanmak gelenektir. Örneğin, `helloworld.rs` yerine `hello_world.rs` kullanmalısınız.

Şimdi oluşturduğunuz `main.rs` dosyasını açın ve aşağıdaki kodu girin (Liste 1-1).

**Dosya adı: main.rs**

```rust
fn main() {
    println!("Hello, world!");
}
```

📄 **Liste 1-1: "Hello, world!" yazdıran bir program**

Dosyayı kaydedin ve terminal penceresine geri dönün (`~/projects/hello_world` dizininde).
Linux veya macOS için aşağıdaki komutları girin:

```
$ rustc main.rs
$ ./main
Hello, world!
```

👉 Bu komutlar dosyayı derler (`rustc`) ve ardından çalıştırır (`./main`).

Windows için `./main` yerine `.\main` kullanın:

```
> rustc main.rs
> .\main
Hello, world!
```

👉 Bu komutlar Windows’ta aynı şekilde dosyayı derleyip çalıştırır.

İşletim sisteminiz ne olursa olsun, terminale **Hello, world!** yazısı basılmalıdır. Eğer bu çıktıyı görmüyorsanız, yardım için Kurulum bölümündeki “Sorun Giderme” kısmına geri dönün.

Eğer **Hello, world!** yazısı ekrana basıldıysa, tebrikler! Artık resmi olarak bir Rust programı yazdınız. Bu da sizi bir **Rust programcısı** yapar — hoş geldiniz!

## 🧩 Bir Rust Programının Anatomisi (Anatomy of a Rust Program)

Şimdi bu **Hello, world!** programını ayrıntılı bir şekilde inceleyelim. İşte ilk parça:

```rust
fn main() {

}
```

Bu satırlar `main` adlı bir fonksiyon (function) tanımlar. `main` fonksiyonu özeldir: çalıştırılabilir (executable) her Rust programında ilk çalışan koddur. Burada ilk satır, parametre almayan ve hiçbir şey döndürmeyen `main` adlı bir fonksiyon bildirir. Eğer parametre olsaydı, parantezler `()` içine yazılacaktı.

Fonksiyon gövdesi (`function body`) süslü parantezler `{}` içine alınır. Rust, tüm fonksiyon gövdelerinin süslü parantezlerle çevrilmesini zorunlu kılar. İyi bir yazım tarzı, açılış süslü parantezi fonksiyon bildirimi ile aynı satıra koymak ve araya bir boşluk eklemektir.

Not: Rust projeleri arasında standart bir kod stili (style) kullanmak isterseniz, kodunuzu belirli bir stile göre biçimlendiren `rustfmt` adlı otomatik biçimlendirme aracı vardır (detaylar için Ek D’ye bakın). Rust ekibi, bu aracı Rust’un standart dağıtımıyla birlikte sunmuştur; tıpkı `rustc` gibi bu aracın da bilgisayarınızda yüklü olması gerekir!

`main` fonksiyonunun gövdesi şu kodu içerir:

```rust
println!("Hello, world!");
```

Bu satır, küçük programımızdaki tüm işi yapar: ekrana metin yazdırır. Burada dikkat edilmesi gereken üç önemli ayrıntı vardır.

1. `println!` bir Rust **makrosu**nu (macro) çağırır. Eğer bir fonksiyon çağırıyor olsaydı, `println` (ünlem işareti olmadan) yazılırdı. Rust makroları, Rust sözdizimini (syntax) genişletmek için kod üreten kod yazmanın bir yoludur. Bunları 20. bölümde daha ayrıntılı inceleyeceğiz. Şimdilik bilmeniz gereken, `!` işareti kullanıldığında normal bir fonksiyon yerine makro çağırıldığını ve makroların her zaman fonksiyonlarla aynı kurallara uymadığını bilmektir.

2. `"Hello, world!"` adlı bir **string** görüyorsunuz. Bu string, `println!` makrosuna argüman (argument) olarak verilir ve ekrana yazdırılır.

3. Satırı noktalı virgül (`;`) ile bitiriyoruz. Bu, ifadenin (expression) bittiğini ve bir sonrakine geçmeye hazır olunduğunu belirtir. Rust kodundaki satırların çoğu noktalı virgül ile biter.

## ⚙️ Derleme ve Çalıştırma Ayrı Adımlardır (Compiling and Running Are Separate Steps)

Yeni bir program çalıştırdınız, şimdi sürecin her adımını inceleyelim.

Bir Rust programını çalıştırmadan önce, onu **Rust derleyicisi (compiler)** ile derlemeniz gerekir. Bunun için `rustc` komutunu girip kaynak dosyanızın (source file) adını verirsiniz, örneğin:

```
$ rustc main.rs
```

Eğer C veya C++ geçmişiniz varsa, bunun `gcc` veya `clang` kullanımına benzediğini fark edeceksiniz. Başarılı bir derlemeden sonra Rust, ikili (binary) bir çalıştırılabilir (executable) dosya üretir.

Linux, macOS ve Windows PowerShell üzerinde shell içinde `ls` komutunu girerek çalıştırılabilir dosyayı görebilirsiniz:

```
$ ls
main  main.rs
```

Linux ve macOS’ta iki dosya görürsünüz. PowerShell ile Windows’ta ise, CMD kullanırken göreceğiniz üç dosyayı görürsünüz. Windows CMD’de şu komutu girersiniz:

```
> dir /B %= /B seçeneği sadece dosya adlarını gösterir =%
main.exe
main.pdb
main.rs
```

Burada `.rs` uzantılı kaynak dosyası, çalıştırılabilir dosya (`main.exe` Windows’ta, diğer tüm platformlarda sadece `main`) ve Windows kullanıyorsanız hata ayıklama (debugging) bilgisi içeren `.pdb` uzantılı dosya görüntülenir. Buradan sonra `main` ya da `main.exe` dosyasını şu şekilde çalıştırırsınız:

```
$ ./main   # Windows'ta .\main
```

Eğer `main.rs` dosyanız **Hello, world!** programıysa, bu satır terminalinize **Hello, world!** yazısını basar.

Ruby, Python veya JavaScript gibi dinamik dillere daha aşina iseniz, programı derleme ve çalıştırmanın ayrı adımlar olmasına alışkın olmayabilirsiniz. Rust, **önceden derlenen (ahead-of-time compiled)** bir dildir; yani bir programı derleyip ortaya çıkan çalıştırılabilir dosyayı başkasına verebilirsiniz ve onlar Rust kurulu olmasa bile bu programı çalıştırabilir. Ancak `.rb`, `.py` veya `.js` dosyası verirseniz, karşı tarafın sırasıyla Ruby, Python veya JavaScript yorumlayıcısının kurulu olması gerekir. Bu dillerde programı derlemek ve çalıştırmak için tek bir komut yeterlidir. Dil tasarımında her şey bir ödünleşmedir (trade-off).

Sadece `rustc` kullanarak derleme yapmak basit programlar için uygundur, ancak projeniz büyüdükçe tüm seçenekleri yönetmek ve kodunuzu paylaşmayı kolaylaştırmak isteyeceksiniz. Bir sonraki bölümde, gerçek dünyadaki Rust programlarını yazmanıza yardımcı olacak **Cargo** aracını tanıyacağız.
