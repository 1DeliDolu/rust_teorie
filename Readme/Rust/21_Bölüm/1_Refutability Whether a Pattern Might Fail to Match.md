## 🖥️ Tek İş Parçacıklı Bir Web Sunucusu Kurmak (single-threaded web server)

Önce tek iş parçacıklı bir web sunucusunu (web server) çalışır hâle getirmekle başlayacağız. Başlamadan önce, web sunucuları oluşturmada yer alan protokollere (protocol) hızlıca göz atalım. Bu protokollerin ayrıntıları bu kitabın kapsamı dışında kalsa da, kısa bir genel bakış ihtiyaç duyduğunuz bilgiyi sağlayacaktır.

Web sunucularında yer alan iki ana protokol, Üstveri Aktarım Protokolü (Hypertext Transfer Protocol, **HTTP**) ve İletim Denetim Protokolü (Transmission Control Protocol, **TCP**)’dir. Her iki protokol de istek-yanıt (request-response) modeline dayanır; yani bir istemci (client) istekleri başlatır ve bir sunucu (server) bu istekleri dinleyip istemciye bir yanıt (response) sağlar. Bu istek ve yanıtların içeriği protokoller tarafından tanımlanır.

TCP, bilgilerin bir sunucudan diğerine nasıl iletildiğinin ayrıntılarını tanımlayan daha alt düzey bir protokoldür (lower-level protocol), ancak bu bilgilerin ne olduğunu belirtmez. HTTP ise TCP’nin üzerine inşa edilir ve istekler ile yanıtların içeriğini tanımlar. Teknik olarak HTTP’yi başka protokollerle kullanmak da mümkündür; fakat vakaların büyük çoğunluğunda HTTP verilerini TCP üzerinden gönderir. Biz TCP ve HTTP istekleri ile yanıtlarının ham baytları (raw bytes) ile çalışacağız.

## 🔊 TCP Bağlantısını Dinlemek (listening to the TCP connection)

Web sunucumuzun bir TCP bağlantısını (TCP connection) dinlemesi gerekir; bu nedenle önce bu kısımla çalışacağız. Standart kitaplık (standard library) bunu yapmamıza imkân veren bir `std::net` modülü (module) sunar. Hadi her zamanki şekilde yeni bir proje oluşturalım:

```
$ cargo new hello
     Created binary (application) `hello` project
$ cd hello
```

Şimdi `src/main.rs` dosyasına, başlangıç için Liste 21-1’deki kodu girin. Bu kod, gelen TCP akışlarını (stream) dinlemek için yerel adres `127.0.0.1:7878` üzerinde bekler. Bir akış geldiğinde, `Connection established!` yazdırır.

**Dosya adı:** src/main.rs

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
```

Liste 21-1: Gelen akışları dinlemek ve bir akış aldığımızda bir mesaj yazdırmak

`TcpListener` kullanarak `127.0.0.1:7878` adresinde TCP bağlantılarını dinleyebiliriz. Adreste, iki nokta üst üstenin önceki kısmı bilgisayarınızı temsil eden bir IP adresidir (her bilgisayarda aynıdır ve özellikle yazarların bilgisayarını temsil etmez) ve `7878` ise bağlantı noktasıdır (port). Bu bağlantı noktasını iki nedenle seçtik: HTTP normalde bu bağlantı noktasında kabul edilmez, bu nedenle makinenizde çalışan başka bir web sunucusuyla çakışma olasılığı düşüktür; ayrıca `7878`, bir telefonda “rust” yazarken kullanılan tuşlara karşılık gelir.

Bu senaryoda `bind` işlevi (function), bir `new` işlevi gibi davranır ve yeni bir `TcpListener` örneği döndürür. İşlevin adına `bind` denir çünkü ağ (networking) bağlamında, dinlemek için bir bağlantı noktasına bağlanmaya “bağlama (binding)’’ denir.

`bind` işlevi `Result<T, E>` döndürür; bu, bağlamanın başarısız olabileceğini gösterir. Örneğin, `80` numaralı bağlantı noktasına bağlanmak yönetici ayrıcalıkları (administrator privileges) gerektirir (yönetici olmayanlar yalnızca `1023`’ten büyük bağlantı noktalarında dinleme yapabilir), dolayısıyla yönetici olmadan `80`’e bağlanmaya çalışırsak bağlama çalışmaz. Ayrıca programımızın iki örneğini çalıştırarak aynı bağlantı noktasında iki program dinliyorsa da bağlama çalışmaz. Yalnızca öğrenme amaçlı temel bir sunucu yazdığımız için bu tür hataları ele almayacağız; bunun yerine, hata oluştuğunda programı durdurmak için `unwrap` kullanacağız.

`TcpListener` üzerindeki `incoming` yöntemi (method), bize bir dizi akış (daha doğrusu, `TcpStream` türünde akışlar) veren bir yineleyici (iterator) döndürür. Tek bir akış, istemci ile sunucu arasındaki açık bir bağlantıyı (connection) temsil eder. Bir bağlantı, istemcinin sunucuya bağlandığı, sunucunun bir yanıt ürettiği ve sunucunun bağlantıyı kapattığı tam istek-yanıt sürecinin adıdır. Bu nedenle, istemcinin ne gönderdiğini görmek için `TcpStream`’den okuyacak ve veriyi istemciye geri göndermek için yanıtımızı akışa yazacağız. Genel olarak, bu `for` döngüsü her bağlantıyı sırayla işleyecek ve ele almamız için bir dizi akış üretecektir.

Şimdilik, akışı ele alışımız `unwrap` çağırarak akışta herhangi bir hata varsa programı sonlandırmaktan ibarettir; hata yoksa program bir mesaj yazdırır. Bir istemci sunucuya bağlandığında `incoming` yönteminden neden hatalar alabileceğimizin sebebi, aslında bağlantılar üzerinde yineleme yapmıyor olmamızdır. Bunun yerine, bağlantı girişimleri (connection attempts) üzerinde yineleme yapıyoruz. Bağlantı çeşitli nedenlerle başarılı olmayabilir ve bunların çoğu işletim sistemine özgüdür. Örneğin, birçok işletim sisteminin destekleyebileceği eşzamanlı açık bağlantı sayısına bir sınırı vardır; bu sayının ötesindeki yeni bağlantı girişimleri, açık bağlantılardan bazıları kapanana kadar hata üretir.

Haydi bu kodu çalıştırmayı deneyelim! Terminalde `cargo run` komutunu çalıştırın ve ardından bir web tarayıcısında `127.0.0.1:7878` adresini yükleyin. Sunucu şu anda hiçbir veri geri göndermediğinden, tarayıcı “Bağlantı sıfırlandı (Connection reset)” gibi bir hata mesajı gösterecektir. Ancak terminalinize baktığınızda, tarayıcının sunucuya bağlandığında yazdırılan birkaç mesaj görmelisiniz!

```
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

Bazen tek bir tarayıcı isteği için birden fazla mesaj görebilirsiniz; bunun nedeni, tarayıcının sayfa için bir istek yapmasının yanı sıra, tarayıcı sekmesinde görünen `favicon.ico` gibi başka kaynaklar için de istek yapması olabilir.

Ayrıca, sunucu herhangi bir veriyle yanıt vermediği için tarayıcının sunucuya birden çok kez bağlanmaya çalışması da mümkündür. Döngünün sonunda `stream` kapsam dışına çıktığında (goes out of scope) ve bırakıldığında (dropped), bağlantı `drop` uygulamasının (drop implementation) bir parçası olarak kapatılır. Tarayıcılar bazen kapatılan bağlantılarla geçici bir sorun olabileceği düşüncesiyle yeniden deneme yaparlar.

Tarayıcılar bazen, daha sonra istek göndereceklerse bunun daha hızlı gerçekleşmesi için herhangi bir istek göndermeden sunucuya birden fazla bağlantı da açarlar. Bu olduğunda, bizim sunucumuz, o bağlantı üzerinden herhangi bir istek olup olmadığına bakmaksızın her bağlantıyı görecektir. Örneğin, Chrome tabanlı tarayıcıların birçok sürümü bunu yapar; bu optimizasyonu gizli (private) gezinme modunu kullanarak devre dışı bırakabilir veya farklı bir tarayıcı kullanabilirsiniz.

Önemli nokta, bir TCP bağlantısının kontrolünü (handle) başarıyla elde etmiş olmamızdır!

Belirli bir kod sürümünü çalıştırmayı bitirdiğinizde programı `ctrl-c` ile durdurmayı unutmayın. Ardından, her bir kod değişikliği setinden sonra en yeni kodu çalıştırdığınızdan emin olmak için `cargo run` komutunu yeniden çalıştırarak programı başlatın.

## 🔍 İsteği Okuma (reading the request)

Tarayıcıdan gelen isteği (request) okumaya yönelik işlevselliği uygulayalım! Bir bağlantıyı (connection) önce edinme ve ardından bu bağlantıyla işlem yapma sorumluluklarını ayırmak için, bağlantıları işlemek üzere yeni bir işlev (function) başlatacağız. Bu yeni `handle_connection` işlevinde, TCP akışından (TCP stream) veri okuyacak ve tarayıcıdan gönderilen verileri görebilmek için yazdıracağız. Kodu, Liste 21-2’deki gibi değiştirin.

**Dosya adı:** src/main.rs

```rust
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
```

Liste 21-2: `TcpStream`’den okumak ve veriyi yazdırmak

Akıştan (stream) okumamızı ve yazmamızı sağlayan trait’lere (traits) ve tiplere (types) erişebilmek için `std::io::prelude` ve `std::io::BufReader`’ı kapsama (scope) alıyoruz. `main` işlevindeki (main function) `for` döngüsünde (for loop), artık bir bağlantı kurduğumuzu söyleyen bir mesaj yazdırmak yerine yeni `handle_connection` işlevini çağırıyor ve akışı (stream) ona iletiyoruz.

`handle_connection` işlevinde, akışa bir başvuru (reference) saran yeni bir `BufReader` örneği oluşturuyoruz. `BufReader`, bizim adımıza `std::io::Read` trait yöntemlerine (methods) yapılan çağrıları yöneterek arabelleğe alma (buffering) ekler.

Tarayıcının sunucumuza gönderdiği isteğin satırlarını toplamak için `http_request` adlı bir değişken oluşturuyoruz. Bu satırları bir vektörde (vector) toplamak istediğimizi `Vec<_>` tür açıklaması (type annotation) ekleyerek belirtiriz.

`BufReader`, `std::io::BufRead` trait’ini uygular (implements) ve bu trait `lines` yöntemini sağlar. `lines` yöntemi, veri akışında her yeni satır baytı (newline) gördüğünde akışı bölerek `Result<String, std::io::Error>` yineleyicisi (iterator) döndürür. Her bir `String`’i elde etmek için, her `Result`’ı `map` ve `unwrap` ile açarız. Veri geçerli UTF-8 değilse veya akıştan okuma sırasında bir sorun olduysa `Result` bir hata (error) olabilir. Üretim (production) bir program bu hataları daha zarif biçimde ele almalıdır, fakat basitlik için hata durumunda programı durdurmayı seçiyoruz.

Tarayıcı, bir HTTP isteğinin sonunu art arda iki yeni satır (newline) göndererek bildirir; bu nedenle, akıştan tek bir istek almak için, boş dizge (empty string) olan bir satır görene kadar satırları alırız. Satırları bir vektörde topladıktan sonra, tarayıcının sunucumuza gönderdiği talimatlara göz atabilmek için güzel hata ayıklama biçimlendirmesi (pretty debug formatting) kullanarak onları yazdırıyoruz.

Hadi bu kodu deneyelim! Programı başlatın ve bir web tarayıcısıyla tekrar bir istek yapın. Tarayıcıda hâlâ bir hata sayfası göreceğimizi unutmayın, ancak terminaldeki program çıktımız şimdi buna benzer görünecektir:

```
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

Kullandığınız tarayıcıya bağlı olarak çıktınız biraz farklı olabilir. Artık istek verilerini yazdırdığımıza göre, bir tarayıcı isteğinden neden birden fazla bağlantı aldığımızı, isteğin ilk satırında `GET`’ten sonra gelen yola (path) bakarak görebiliriz. Tekrarlanan bağlantıların hepsi `/` talep ediyorsa, programımızdan yanıt alamadığı için tarayıcının `/` yolunu tekrar tekrar almaya çalıştığını biliriz.

Hadi, tarayıcının programımızdan ne istediğini anlamak için bu istek verilerini parçalara ayıralım.

## 📄 Bir HTTP İsteğine Yakından Bakmak (a closer look at an HTTP request)

HTTP, metin tabanlı (text-based) bir protokoldür (protocol) ve bir istek (request) şu biçimi alır:

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

İlk satır, istemcinin (client) ne talep ettiğine dair bilgileri tutan **istek satırıdır** (request line). İstek satırının ilk bölümü, **GET** veya **POST** gibi kullanılan yöntemi (method) belirtir; bu yöntem, istemcinin isteği nasıl yaptığını açıklar. Bizim istemcimiz bir **GET** isteği kullandı; bu, bilgi talep ettiği anlamına gelir.

İstek satırının bir sonraki bölümü `/` olup, istemcinin talep ettiği **tekdüzen kaynak tanımlayıcıyı** (uniform resource identifier, **URI**) gösterir: bir URI neredeyse, ama tam olarak değil, bir **tekdüzen kaynak konumlayıcıya** (uniform resource locator, **URL**) denktir. URI ile URL arasındaki fark bu bölümdeki amacımız açısından önemli değildir; HTTP standardı URI terimini kullandığı için burada zihnimizde URI yerine URL düşünebiliriz.

Son bölüm, istemcinin kullandığı **HTTP sürümüdür** (HTTP version) ve ardından istek satırı bir **CRLF** dizisiyle (carriage return + line feed) sona erer. (CRLF, daktilo dönemlerinden kalma terimlerdir!) CRLF dizisi `\r\n` olarak da yazılabilir; burada `\r` carriage return, `\n` ise line feed anlamına gelir. CRLF dizisi, istek satırını geri kalan istek verilerinden ayırır. Dikkat edin, CRLF yazdırıldığında `\r\n` değil, yeni bir satır başlangıcı görürüz.

Şimdiye kadar programımızı çalıştırarak aldığımız istek satırı verilerine bakarsak: `GET` yöntem, `/` istek URI’si, `HTTP/1.1` ise sürümdür.

İstek satırından sonra, `Host:` ile başlayan geri kalan satırlar başlıklardır (headers). **GET** isteklerinin gövdesi (body) yoktur.

Farklı bir tarayıcıdan istek yapmayı veya `127.0.0.1:7878/test` gibi farklı bir adres talep etmeyi deneyerek, istek verilerinin nasıl değiştiğini görebilirsiniz.

Artık tarayıcının ne istediğini bildiğimize göre, ona veri geri gönderelim!

---

## 📨 Bir Yanıt Yazmak (writing a response)

Bir istemci isteğine yanıt olarak veri göndermeyi uygulayacağız. Yanıtların biçimi şöyledir:

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

İlk satır, yanıtın (response) kullandığı **HTTP sürümünü**, isteğin sonucunu özetleyen sayısal bir **durum kodunu** (status code) ve durum kodunu açıklayan metinsel bir **sebep ifadesini** (reason phrase) içerir. CRLF dizisinden sonra başlıklar (headers), bir CRLF dizisi daha ve ardından yanıt gövdesi (body) gelir.

İşte HTTP sürüm 1.1’i kullanan, 200 durum koduna ve `OK` sebep ifadesine sahip, başlıksız ve gövdesiz bir yanıt örneği:

```
HTTP/1.1 200 OK\r\n\r\n
```

Durum kodu **200**, standart başarı (success) yanıtıdır. Bu metin, küçük bir başarılı HTTP yanıtıdır. Bunu başarılı bir isteğe yanıt olarak akışa (stream) yazalım! `handle_connection` işlevinde istek verilerini yazdıran `println!` ifadesini kaldırın ve bunun yerine Liste 21-3’teki kodu ekleyin.

**Dosya adı:** src/main.rs

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
```

Liste 21-3: Akışa küçük bir başarılı HTTP yanıtı yazmak

İlk yeni satır, başarı mesajının verilerini tutan `response` değişkenini tanımlar. Ardından, dizge verilerini baytlara dönüştürmek için `as_bytes` çağırırız. `stream` üzerindeki `write_all` yöntemi bir `&[u8]` alır ve bu baytları doğrudan bağlantı üzerinden gönderir. `write_all` işlemi başarısız olabileceği için, daha önce olduğu gibi hatada programı sonlandırmak amacıyla `unwrap` kullanıyoruz. Yine, gerçek bir uygulamada burada hata yönetimi eklemeniz gerekir.

Bu değişikliklerle, kodumuzu çalıştırıp bir istek yapalım. Artık terminale herhangi bir veri yazdırmıyoruz, bu yüzden Cargo’nun çıktısı dışında bir şey görmeyeceğiz. Ancak `127.0.0.1:7878` adresini bir web tarayıcısında açtığınızda, artık hata yerine boş bir sayfa görmelisiniz. Az önce elle bir HTTP isteği almayı ve bir yanıt göndermeyi kodladınız!

## 📝 Gerçek HTML Döndürmek (returning real HTML)

Boş bir sayfadan fazlasını döndürme işlevselliğini uygulayalım. Proje dizininizin köküne (root), `src` dizinine değil, `hello.html` adında yeni bir dosya oluşturun. İçine istediğiniz HTML’yi koyabilirsiniz; Liste 21-4 bir olasılığı göstermektedir.

**Dosya adı:** hello.html

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
```

Liste 21-4: Yanıtta döndürmek için örnek HTML dosyası

Bu, bir başlık ve biraz metin içeren minimal bir HTML5 belgesidir. Sunucuya bir istek geldiğinde bunu döndürmek için, `handle_connection` işlevini Liste 21-5’te gösterildiği gibi değiştireceğiz: HTML dosyasını okuyacak, onu yanıtın gövdesine (body) ekleyecek ve gönderecek.

**Dosya adı:** src/main.rs

```rust
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
```

Liste 21-5: Yanıtın gövdesi olarak `hello.html` içeriğini göndermek

Burada `fs` modülünü kullanmak için `use` bildirimine ekledik. Bir dosyanın içeriğini bir dizgeye (string) okuma kodu size tanıdık gelmeli; G/Ç (I/O) projemizde, Liste 12-4’te benzerini yapmıştık.

Sonraki adımda, dosyanın içeriğini yanıt gövdesi olarak eklemek için `format!` makrosunu kullanıyoruz. Geçerli bir HTTP yanıtı olması için ayrıca `Content-Length` başlığını (header) ekliyoruz; bu, yanıt gövdesinin boyutuna ayarlanır (bizim durumumuzda `hello.html` boyutu).

Bu kodu `cargo run` ile çalıştırın ve tarayıcıda `127.0.0.1:7878` adresini yükleyin; HTML’nizin işlendiğini görmelisiniz!

Şu anda, `http_request` içindeki istek verilerini yok sayıyoruz ve koşulsuz olarak HTML dosyasını gönderiyoruz. Bu, tarayıcıda `127.0.0.1:7878/something-else` talep etseniz bile aynı HTML yanıtını alacağınız anlamına gelir. Şu an sunucumuz çok sınırlıdır ve çoğu web sunucusunun yaptığı işi yapmamaktadır. Amacımız, isteğe göre yanıtlarımızı özelleştirmek ve yalnızca `/` için düzgün bir istek geldiğinde HTML dosyasını döndürmektir.

---

## ✅ İsteği Doğrulama ve Seçici Yanıt Verme (validating the request and selectively responding)

Şu anda, web sunucumuz istemcinin ne talep ettiğine bakmaksızın HTML’yi döndürüyor. Tarayıcının `/` yolunu istediğini kontrol eden işlevsellik ekleyelim; eğer tarayıcı başka bir şey isterse, bir hata döndürelim. Bunun için `handle_connection` işlevini, Liste 21-6’daki gibi değiştirmemiz gerekiyor. Bu yeni kod, alınan isteğin içeriğini `/` için beklenenle karşılaştırır ve farklı durumları işlemek için `if` ve `else` blokları ekler.

**Dosya adı:** src/main.rs

```rust
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // some other request
    }
}
```

Liste 21-6: `/` isteklerini diğer isteklerden farklı işlemek

Artık yalnızca HTTP isteğinin ilk satırına bakıyoruz, bu yüzden tüm isteği bir vektöre okumak yerine, yineleyiciden (iterator) ilk öğeyi almak için `next` çağırıyoruz. İlk `unwrap`, `Option` değerini açar ve yineleyicide hiç öğe yoksa programı durdurur. İkinci `unwrap`, `Result` değerini açar ve Liste 21-2’de `map` içinde kullandığımız `unwrap` ile aynı etkiye sahiptir.

Sonrasında `request_line` değerini kontrol ediyoruz; eğer `/` yoluna yapılmış bir **GET** isteğiyle eşleşiyorsa, `if` bloğu HTML dosyamızın içeriğini döndürür.

Eğer `request_line` bu koşulu sağlamıyorsa, başka bir istek almışız demektir. Birazdan `else` bloğuna, diğer tüm isteklere yanıt verecek kod ekleyeceğiz.

Bu kodu şimdi çalıştırın ve `127.0.0.1:7878` isteği yapın; `hello.html` dosyasındaki HTML’yi almalısınız. Eğer `127.0.0.1:7878/something-else` gibi başka bir istek yaparsanız, Liste 21-1 ve 21-2’deki kodları çalıştırırken gördüğünüz bağlantı hatalarıyla aynı türden bir hata alacaksınız.

Şimdi, `else` bloğuna Liste 21-7’deki kodu ekleyelim. Bu kod, istenen içeriğin bulunamadığını bildiren **404** durum koduyla bir yanıt döndürecek. Ayrıca, tarayıcıda kullanıcıya gösterilecek bir hata sayfası da göndereceğiz.

**Dosya adı:** src/main.rs

```rust
    // --snip--
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
```

Liste 21-7: `/` dışındaki isteklere 404 kodu ve hata sayfası döndürmek

Burada yanıtın durum satırı **404 NOT FOUND** kodunu içeriyor. Yanıtın gövdesi ise `404.html` dosyasındaki HTML olacak. Bunun için `hello.html` dosyasının yanına bir `404.html` dosyası oluşturmanız gerekiyor; dilediğiniz HTML’yi koyabilirsiniz veya Liste 21-8’deki örneği kullanabilirsiniz.

**Dosya adı:** 404.html

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
```

Liste 21-8: 404 yanıtı için örnek hata sayfası içeriği

Bu değişikliklerle sunucunuzu tekrar çalıştırın. `127.0.0.1:7878` isteği `hello.html` içeriğini döndürmeli, diğer tüm istekler (ör. `127.0.0.1:7878/foo`) ise `404.html` dosyasındaki hata sayfasını döndürmelidir.

## 🔄 Küçük Bir Refaktör (a touch of refactoring)

Şu anda, `if` ve `else` bloklarında çok fazla tekrar var: her ikisi de dosya okuyor ve dosya içeriğini akışa (stream) yazıyor. Tek fark, **durum satırı** (status line) ve **dosya adı** (filename). Kodu daha özlü hale getirmek için bu farklılıkları ayrı `if` ve `else` satırlarına alalım; bu satırlar durum satırı ve dosya adı değerlerini değişkenlere atayacak. Daha sonra bu değişkenleri koşulsuz olarak dosyayı okumak ve yanıtı yazmak için kullanabiliriz. Liste 21-9, büyük `if` ve `else` bloklarının yerine geçen sonuç kodunu göstermektedir.

**Dosya adı:** src/main.rs

```rust
// --snip--

fn handle_connection(mut stream: TcpStream) {
    // --snip--

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
```

Liste 21-9: `if` ve `else` bloklarını yalnızca iki durum arasındaki farkı içerecek şekilde refaktör etmek

Artık `if` ve `else` blokları yalnızca uygun durum satırı ve dosya adı değerlerini bir **tuple** olarak döndürüyor; ardından, Bölüm 19’da tartışıldığı gibi, bu iki değeri `let` ifadesinde bir desen (pattern) kullanarak `status_line` ve `filename` değişkenlerine atamak için yapısal çözümleme (destructuring) kullanıyoruz.

Önceden yinelenen kod artık `if` ve `else` bloklarının dışında ve `status_line` ile `filename` değişkenlerini kullanıyor. Bu yaklaşım, iki durum arasındaki farkı görmeyi kolaylaştırıyor ve dosya okuma ile yanıt yazma işlemlerini değiştirmek istersek, kodu yalnızca tek bir yerde güncellememiz gerektiği anlamına geliyor. Liste 21-9’daki kodun davranışı, Liste 21-7’dekiyle aynı olacaktır.

Harika! Artık yaklaşık **40 satır Rust koduyla** basit bir web sunucumuz var: `/` isteğine bir içerik sayfası döndürüyor ve diğer tüm isteklere 404 yanıtı veriyor.

Şu anda, sunucumuz tek iş parçacığında (single thread) çalışıyor, yani aynı anda yalnızca bir isteğe hizmet edebiliyor. Yavaş istekleri simüle ederek bunun nasıl bir sorun olabileceğini inceleyelim. Ardından sunucumuzu, aynı anda birden fazla isteği işleyebilecek şekilde düzelteceğiz.
