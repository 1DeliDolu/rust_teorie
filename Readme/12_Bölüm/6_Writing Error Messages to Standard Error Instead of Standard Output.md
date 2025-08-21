## 🖥️ Hata Mesajlarını Standart Çıkış (stdout) Yerine Standart Hata (stderr)’ya Yazma

Şu anda tüm çıktılarımızı `println!` makrosunu kullanarak terminale yazıyoruz. Çoğu terminalde iki tür çıktı vardır: genel bilgiler için standart çıkış (standard output, `stdout`) ve hata mesajları için standart hata (standard error, `stderr`). Bu ayrım, kullanıcıların bir programın başarılı çıktısını bir dosyaya yönlendirmesine, ancak hata mesajlarını ekranda görmeye devam etmesine olanak tanır.

`println!` makrosu yalnızca standart çıkışa yazabilir, bu nedenle hata mesajlarını standart hataya yazmak için başka bir şey kullanmamız gerekir.

---

## 🔍 Hataların Nereye Yazıldığını Kontrol Etme

Önce, `minigrep` tarafından yazdırılan içeriğin şu anda standart çıkışa nasıl gönderildiğini, standart hataya yazmak istediğimiz hata mesajları da dahil olmak üzere gözlemleyelim. Bunu yapmak için standart çıkış akışını bir dosyaya yönlendirip, bilerek bir hata oluşturalım. Standart hata akışını yönlendirmeyeceğiz, bu yüzden standart hataya gönderilen her şey ekranda görünmeye devam edecek.

Komut satırı programlarının hata mesajlarını standart hata akışına göndermesi beklenir. Böylece standart çıkış akışı bir dosyaya yönlendirilse bile hata mesajlarını ekranda görebiliriz. Programımız şu anda doğru davranmıyor: hata mesajı çıktısının bir dosyaya kaydedildiğini göreceğiz!

Bu davranışı göstermek için programı `>` ile çalıştırıp standart çıkış akışını yönlendirmek istediğimiz dosya yolunu (`output.txt`) verelim. Argüman geçmeyeceğiz, bu bir hata oluşturmalıdır:

```bash
$ cargo run > output.txt
```

`>` sözdizimi, kabuğa standart çıkışın içeriğini ekrana değil `output.txt` dosyasına yazmasını söyler. Beklediğimiz hata mesajını ekranda görmedik, bu da dosyaya yazıldığını gösterir. `output.txt` dosyasının içeriği şu şekildedir:

```
Problem parsing arguments: not enough arguments
```

Evet, hata mesajımız standart çıkışa yazılıyor. Oysa bu tür hata mesajlarının standart hataya yazılması çok daha faydalıdır, böylece yalnızca başarılı çalışmalardan elde edilen veriler dosyaya girer. Bunu değiştireceğiz.

---

## 🛠️ Hataları Standart Hataya Yazma

Hata mesajlarının nasıl yazıldığını değiştirmek için 12-24 numaralı listedeki kodu kullanacağız. Bu bölümde daha önce yaptığımız yeniden düzenleme sayesinde, hata mesajlarını yazdıran tüm kod `main` fonksiyonunda. Standart kütüphane, hata mesajlarını standart hata akışına yazan `eprintln!` makrosunu sağlar. Bu yüzden, hataları yazdırmak için `println!` kullandığımız iki yeri `eprintln!` ile değiştireceğiz.

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

📌 Bu kod, hata mesajlarını `stdout` yerine `stderr` akışına yazdırır.

Şimdi programı tekrar aynı şekilde çalıştıralım, yani argüman olmadan ve standart çıkışı `>` ile yönlendirerek:

```bash
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Artık hata mesajını ekranda görüyoruz ve `output.txt` boş. Bu, komut satırı programlarından beklenen davranıştır.

Programı bu kez hata oluşturmayan argümanlarla çalıştırıp yine standart çıkışı dosyaya yönlendirelim:

```bash
$ cargo run -- to poem.txt > output.txt
```

Ekrana hiçbir çıktı yazılmayacak ve `output.txt` şu sonucu içerecek:

**Dosya adı:** `output.txt`

```
Are you nobody, too?
How dreary to be somebody!
```

Bu, artık başarılı çıktılar için standart çıkışı, hatalar içinse standart hatayı kullandığımızı gösterir.

---

## 📋 Özet

Bu bölümde şimdiye kadar öğrendiğiniz bazı önemli kavramları gözden geçirdik ve Rust’ta yaygın G/Ç (I/O) işlemlerini nasıl yapacağınızı gördük. Komut satırı argümanları, dosyalar, ortam değişkenleri ve hata yazdırmak için `eprintln!` makrosunu kullanarak artık komut satırı uygulamaları yazmaya hazırsınız. Önceki bölümlerdeki kavramlarla birleştirildiğinde, kodunuz iyi organize edilecek, verileri uygun veri yapılarında etkili bir şekilde saklayacak, hataları düzgün bir şekilde yönetecek ve iyi test edilmiş olacaktır.

Bir sonraki bölümde, işlevsel dillerden etkilenmiş bazı Rust özelliklerini inceleyeceğiz: kapanışlar (closures) ve yineleyiciler (iterators).
