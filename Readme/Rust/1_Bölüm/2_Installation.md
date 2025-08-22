## 💾 Kurulum (Installation)

İlk adım Rust’ı kurmaktır. Rust’ı, Rust sürümlerini ve ilişkili araçları yönetmek için kullanılan bir komut satırı aracı (command line tool) olan **rustup** üzerinden indireceğiz. İndirme için internet bağlantısına ihtiyacınız olacak.

Not: Eğer herhangi bir nedenle `rustup` kullanmak istemezseniz, daha fazla seçenek için **Other Rust Installation Methods** sayfasına bakınız.

Aşağıdaki adımlar, Rust derleyicisinin (compiler) en son kararlı (stable) sürümünü kurar. Rust’ın kararlılık garantileri (stability guarantees), bu kitaptaki derlenen örneklerin, daha yeni Rust sürümleriyle de derlenmeye devam etmesini sağlar. Çıktı, sürümler arasında biraz farklı olabilir çünkü Rust sık sık hata mesajlarını ve uyarılarını geliştirir. Başka bir deyişle, bu adımları kullanarak kuracağınız herhangi bir yeni, kararlı Rust sürümü bu kitabın içeriğiyle beklendiği şekilde çalışmalıdır.

### ⌨️ Komut Satırı Gösterimi (Command Line Notation)

Bu bölümde ve kitabın geri kalanında, terminalde kullanılan bazı komutları göstereceğiz. Terminale girmeniz gereken satırların tümü `$` karakteri ile başlar. `$` karakterini yazmanıza gerek yoktur; bu sadece her komutun başlangıcını göstermek için kullanılan komut satırı istemidir. `$` ile başlamayan satırlar genellikle bir önceki komutun çıktısını gösterir.

Ek olarak, **PowerShell**’e özgü örnekler `$` yerine `>` kullanır.

## 🐧 Linux veya macOS’te rustup Kurulumu (Installing rustup on Linux or macOS)

Eğer Linux veya macOS kullanıyorsanız, bir terminal açın ve aşağıdaki komutu girin:

```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

👉 Bu komut, bir betik (script) indirir ve `rustup` aracının kurulumunu başlatır; bu araç Rust’ın en son kararlı (stable) sürümünü kurar.

Kurulum sırasında sizden parolanız istenebilir. Kurulum başarılı olursa şu satır görünecektir:

```
Rust is installed now. Great!
```

Ayrıca bir **bağlayıcıya (linker)** ihtiyacınız olacak. Bağlayıcı, Rust’ın derlenen çıktılarını tek bir dosyada birleştirmek için kullandığı bir programdır. Büyük olasılıkla sisteminizde zaten yüklüdür. Eğer bağlayıcı hataları alırsanız, genellikle bir bağlayıcıyı da içeren bir **C derleyicisi (C compiler)** kurmalısınız. C derleyicisi ayrıca faydalıdır çünkü bazı yaygın Rust paketleri C koduna bağımlıdır ve bir C derleyicisine ihtiyaç duyar.

* **macOS’te**, bir C derleyicisini şu komutla kurabilirsiniz:

```
$ xcode-select --install
```

* **Linux kullanıcıları**, dağıtımlarının belgelerine göre genellikle **GCC** veya **Clang** kurmalıdır.
  Örneğin, Ubuntu kullanıyorsanız `build-essential` paketini kurabilirsiniz.

## 🪟 Windows’ta rustup Kurulumu (Installing rustup on Windows)

Windows’ta [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) adresine gidin ve Rust kurulum talimatlarını izleyin. Kurulumun bir aşamasında sizden **Visual Studio** kurmanız istenecektir. Bu, programları derlemek için gerekli olan **linker** ve yerel kütüphaneleri sağlar. Bu adım için daha fazla yardıma ihtiyacınız olursa şu bağlantıya bakın:
[https://rust-lang.github.io/rustup/installation/windows-msvc.html](https://rust-lang.github.io/rustup/installation/windows-msvc.html)

Kitabın geri kalanında kullanılan komutlar hem **cmd.exe** hem de **PowerShell** üzerinde çalışır. Eğer belirli farklar varsa, hangi ortamı kullanmanız gerektiğini açıklayacağız.

---

## 🔧 Sorun Giderme (Troubleshooting)

Rust’ın doğru kurulup kurulmadığını kontrol etmek için bir kabuk (shell) açın ve şu satırı girin:

```
$ rustc --version
```

En son kararlı sürümün sürüm numarasını, commit hash’ini ve commit tarihini aşağıdaki formatta görmelisiniz:

```
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Bu bilgiyi görüyorsanız, Rust başarıyla kurulmuş demektir! Eğer görmüyorsanız, Rust’ın `%PATH%` sistem değişkenine eklenip eklenmediğini kontrol edin:

* **Windows CMD** için:

```
> echo %PATH%
```

* **PowerShell** için:

```
> echo $env:Path
```

* **Linux ve macOS** için:

```
$ echo $PATH
```

Her şey doğru görünüyor ama Rust hâlâ çalışmıyorsa, yardım alabileceğiniz birçok yer vardır. Rust topluluğu sayfasında diğer **Rustacean** (Rust kullanıcılarının kendilerine taktıkları eğlenceli lakap) kişilerle nasıl iletişim kurabileceğinizi öğrenebilirsiniz.

---

## 🔄 Güncelleme ve Kaldırma (Updating and Uninstalling)

Rust `rustup` üzerinden kurulduktan sonra, yeni bir sürüme güncellemek oldukça kolaydır. Kabuk üzerinden şu komutu çalıştırın:

```
$ rustup update
```

Rust ve `rustup`’ı kaldırmak için şu komutu çalıştırın:

```
$ rustup self uninstall
```
## 📖 Yerel Dokümantasyon (Local Documentation)

Rust kurulumu ayrıca dokümantasyonun yerel bir kopyasını da içerir, böylece çevrimdışı okuyabilirsiniz. Yerel dokümantasyonu tarayıcınızda açmak için şu komutu çalıştırın:

```
$ rustup doc
```

👉 Bu komut, Rust dokümantasyonunu yerel olarak açar.

Standart kütüphane tarafından sağlanan bir tür (type) veya fonksiyon hakkında emin olmadığınızda, ne işe yaradığını veya nasıl kullanılacağını öğrenmek için **API (application programming interface) dokümantasyonu**nu kullanın!

---

## 📝 Metin Editörleri ve Entegre Geliştirme Ortamları (Text Editors and Integrated Development Environments)

Bu kitap, Rust kodunu yazarken hangi araçları kullandığınız konusunda herhangi bir varsayımda bulunmaz. Hemen hemen her metin editörü işinizi görecektir! Ancak, birçok metin editörü ve **IDE (Integrated Development Environment)** Rust için yerleşik destek sunar. Rust web sitesindeki araçlar sayfasında birçok editör ve IDE’nin oldukça güncel bir listesini her zaman bulabilirsiniz.

---

## 🌐 Bu Kitapla Çevrimdışı Çalışmak (Working Offline with This Book)

Bazı örneklerde, standart kütüphanenin ötesinde Rust paketleri kullanacağız. Bu örnekler üzerinde çalışmak için ya internet bağlantınız olmalı ya da bu bağımlılıkları (dependencies) önceden indirmeniz gerekir. Önceden indirmek için aşağıdaki komutları çalıştırabilirsiniz. (Cargo’nun ne olduğunu ve bu komutların her birinin ne işe yaradığını ileride ayrıntılı olarak açıklayacağız.)

```
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0
```

👉 Bu komutlar, bu paketlerin indirilmesini önbelleğe alır, böylece daha sonra indirmenize gerek kalmaz.

Bu komutu çalıştırdıktan sonra `get-dependencies` klasörünü saklamanız gerekmez. Bu komutu çalıştırdıysanız, kitabın geri kalanındaki tüm cargo komutlarında `--offline` bayrağını kullanabilir ve böylece ağ üzerinden erişmek yerine önbelleğe alınmış sürümleri kullanabilirsiniz.
