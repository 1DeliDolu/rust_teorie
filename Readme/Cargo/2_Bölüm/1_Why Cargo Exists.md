## ❓ Neden Cargo Var (Why Cargo Exists)

### 📌 Ön Bilgiler (Preliminaries)

Rust’ta bir kütüphane (library) ya da çalıştırılabilir program (executable program) **crate** olarak adlandırılır. Crate’ler, Rust derleyicisi olan `rustc` ile derlenir. Rust’a başlayan çoğu kişinin ilk karşılaştığı kaynak kod, klasik “hello world” programıdır. Bu program doğrudan `rustc` ile şu şekilde derlenir:

```
$ rustc hello.rs
$ ./hello
Hello, world!
```

👉 Burada dosya adını açıkça belirtmek gerektiğine dikkat edin. Farklı bir programı `rustc` ile derlemek isterseniz, farklı bir komut satırı çağrısı kullanmanız gerekir. Ayrıca belirli derleyici bayrakları (compiler flags) ya da harici bağımlılıklar eklemeniz gerekirse, komut daha da özel (ve karmaşık) hale gelecektir.

Dahası, çoğu ciddi programın dış kütüphanelere bağımlılığı olacaktır. Bu durumda bu bağımlılıklar transitif olarak başka bağımlılıklara da ihtiyaç duyar. Tüm gerekli bağımlılıkların doğru sürümlerini elde etmek ve güncel tutmak, elle yapıldığında zor ve hata yapmaya açık bir süreçtir.

Sadece crate’ler ve `rustc` ile çalışmak yerine, daha üst seviye bir “paket” (package) soyutlaması ve bir paket yöneticisi (package manager) kullanmak bu zorlukları ortadan kaldırır.

---

### 📦 Cargo Sahneye Giriyor (Enter: Cargo)

**Cargo**, Rust’ın paket yöneticisidir (package manager). Rust paketlerinin bağımlılıklarını tanımlamasına izin veren ve her zaman tekrarlanabilir (repeatable) bir derleme elde etmenizi sağlayan bir araçtır.

Bu amacı gerçekleştirmek için Cargo dört şey yapar:

1. Çeşitli paket bilgilerini içeren iki metadata dosyası tanımlar.
2. Paketinizin bağımlılıklarını indirir ve derler.
3. `rustc` ya da başka bir derleme aracını doğru parametrelerle çağırarak paketinizin derlenmesini sağlar.
4. Rust paketleriyle çalışmayı kolaylaştırmak için bazı kurallar (conventions) getirir.

Büyük ölçüde Cargo, verilen bir programı veya kütüphaneyi derlemek için gereken komutları standartlaştırır. Daha sonra göreceğimiz gibi, aynı komut isimlerinden bağımsız olarak farklı çıktıları derlemek için kullanılabilir. `rustc`’yi doğrudan çağırmak yerine `cargo build` gibi daha genel bir komutu çağırabilir ve Cargo’nun doğru `rustc` çağrısını oluşturmasına izin verebilirsiniz.

Ayrıca, Cargo tanımladığınız tüm bağımlılıkları bir kayıt defterinden (registry) otomatik olarak indirir ve gerektiğinde derlemenize dahil eder.

Bir Cargo tabanlı projeyi derlemeyi öğrendiğinizde, aslında tüm Cargo tabanlı projeleri nasıl derleyeceğinizi öğrenmiş olursunuz demek çok da abartılı sayılmaz.
