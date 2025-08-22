## 📍 Desenlerin Kullanılabileceği Tüm Yerler (all the places patterns can be used)

Desenler (patterns) Rust’ta birçok yerde karşımıza çıkar ve aslında farkında olmadan onları sık sık kullanmış olabilirsiniz! Bu bölümde, desenlerin geçerli olduğu tüm yerleri ele alacağız.

---

## 🎯 match Kolları (match arms)

6. bölümde tartıştığımız gibi, desenleri `match` ifadelerinin kollarında kullanırız. Resmî olarak, `match` ifadeleri `match` anahtar sözcüğünden, üzerinde eşleme yapılacak bir değerden ve her biri bir desen ile bu desen eşleşirse çalışacak ifadeden oluşan bir veya daha fazla `match` kolundan meydana gelir:

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

Örneğin, aşağıda `x` değişkeninde bulunan `Option<i32>` değeri üzerinde eşleme yapan 6-5 numaralı listedeki `match` ifadesi yer alıyor:

```rust
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

Bu `match` ifadesindeki desenler, her okun (=>) sol tarafındaki `None` ve `Some(i)`’dir.

`match` ifadeleri için bir gereklilik, **kapsayıcı (exhaustive)** olmalarıdır; yani `match` ifadesindeki değer için tüm olasılıkların hesaba katılması gerekir. Tüm olasılıkları kapsadığınızdan emin olmanın bir yolu, son kolda **yakala-tümü (catchall)** deseni kullanmaktır: örneğin, herhangi bir değeri eşleştiren bir değişken adı hiçbir zaman başarısız olmaz ve kalan tüm durumları kapsar.

Özel bir desen olan `_` her şeyi eşleştirir, ancak hiçbir zaman bir değişkene bağlanmaz. Bu yüzden genellikle son `match` kolunda kullanılır. `_` deseni, özellikle belirtilmeyen herhangi bir değeri göz ardı etmek istediğinizde faydalıdır. `_` desenini “Bir Desende Değerleri Yok Sayma” başlığında daha ayrıntılı inceleyeceğiz.

---

## ⚖️ Koşullu if let İfadeleri (conditional if let expressions)

6. bölümde, `if let` ifadelerini esas olarak yalnızca bir durumu eşleyen `match` ifadesinin kısaltılmış biçimi olarak nasıl kullanabileceğimizi tartıştık. İsteğe bağlı olarak, `if let` bir `else` bloğu da içerebilir; bu blok, eğer `if let` içindeki desen eşleşmezse çalışacak koddur.

19-1 numaralı liste, `if let`, `else if` ve `else if let` ifadelerinin nasıl karıştırılıp eşleştirilebileceğini gösterir. Bu yaklaşım bize yalnızca bir değer üzerinde desenlerle karşılaştırma yapabildiğimiz `match` ifadelerine göre daha fazla esneklik sağlar. Ayrıca Rust, `if let`, `else if` ve `else if let` kollarının birbirleriyle ilişkili olmasını şart koşmaz.

Aşağıdaki kodda, bir dizi koşul kontrol edilerek arka plan renginin ne olacağı belirlenir. Bu örnekte, gerçek bir programın kullanıcı girdisinden alabileceği değerleri sabit değişkenlerle oluşturduk.

### 📂 Dosya Adı: `src/main.rs`

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

👉 Bu kod, kullanıcıdan gelen çeşitli koşullara göre arka plan rengini belirler.

---

Kullanıcı bir **favori renk** belirtirse, arka plan o renk olur. Eğer favori renk yoksa ve bugün **Salı** ise, arka plan rengi **yeşil** olur. Aksi takdirde, kullanıcı yaşını bir string olarak belirtmişse ve bu başarıyla bir sayıya çevrilebiliyorsa, sayı 30’dan büyükse renk **mor**, değilse **turuncu** olur. Hiçbiri geçerli değilse arka plan rengi **mavi** olur.

Bu koşullu yapı, karmaşık gereksinimleri desteklememizi sağlar. Buradaki sabit değerlerle örnek, çıktıda şu satırı basacaktır:

```
Using purple as the background color
```

Ayrıca, `if let` de `match` kollarında olduğu gibi mevcut değişkenleri gölgeleyen (shadow) yeni değişkenler tanıtabilir:

```rust
else if let Ok(age) = age {
    // burada yeni bir `age` değişkeni tanımlanır
}
```

Bu satırda, `Ok` varyantı içindeki değeri alan yeni bir `age` değişkeni tanıtılır ve önceki `age` değişkenini gölgeler. Bu yüzden `if age > 30` koşulunu süslü parantezler içindeki blokta yazmamız gerekir; çünkü `30` ile karşılaştırmak istediğimiz yeni `age`, bu yeni kapsam (scope) başlayana kadar geçerli değildir.

`if let` ifadelerini kullanmanın dezavantajı, derleyicinin **kapsayıcılığı (exhaustiveness)** denetlememesidir; oysa `match` ifadelerinde bu denetim yapılır. Eğer son `else` bloğunu atlamış olsaydık ve bazı durumları işlemeyi unutmuş olsaydık, derleyici bizi bu mantıksal hataya karşı uyarmayacaktı.

## 🔄 while let Koşullu Döngüler (while let conditional loops)

`if let` yapısına benzer şekilde, `while let` koşullu döngüsü bir desen eşleşmeye devam ettiği sürece `while` döngüsünün çalışmasına izin verir. 19-2 numaralı listede, iş parçacıkları (threads) arasında gönderilen mesajları bekleyen bir `while let` döngüsü gösterilmiştir; bu örnekte ise `Option` yerine bir `Result` kontrol edilmektedir.

```rust
let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for val in [1, 2, 3] {
        tx.send(val).unwrap();
    }
});

while let Ok(value) = rx.recv() {
    println!("{value}");
}
```

👉 Bu kod, `rx.recv()` `Ok` döndürdüğü sürece değerleri yazdırır.

Bu örnek 1, 2 ve ardından 3’ü yazdırır. `recv` metodu, kanalın alıcı tarafındaki ilk mesajı alır ve `Ok(value)` döndürür. 16. bölümde `recv` metodunu gördüğümüzde, hatayı doğrudan `unwrap` ile açmıştık veya onu bir `for` döngüsünde bir yineleyici olarak kullanmıştık. Ancak 19-2 numaralı listede görüldüğü gibi, `recv` her mesaj geldiğinde `Ok` döndürdüğü için `while let` de kullanılabilir. Gönderici taraf kapandığında ise `Err` döndürür.

---

## 🔁 for Döngüleri (for loops)

Bir `for` döngüsünde, `for` anahtar sözcüğünü doğrudan takip eden değer aslında bir desendir. Örneğin, `for x in y` ifadesindeki `x` bir desendir. 19-3 numaralı liste, bir `for` döngüsünde bir demeti (tuple) açmak için bir desenin nasıl kullanılacağını göstermektedir.

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{value} is at index {index}");
}
```

👉 Bu kod, `enumerate` metodu sayesinde hem değeri hem de indeksini bir demet halinde üretir ve deseni kullanarak parçalar. Çıktı şu şekildedir:

```
a is at index 0
b is at index 1
c is at index 2
```

---

## 📥 let İfadeleri (let statements)

Bu bölüme kadar desenleri yalnızca `match` ve `if let` ile kullanmayı açıkça tartıştık. Ancak aslında desenleri başka yerlerde de kullanıyoruz; örneğin `let` ifadelerinde.

```rust
let x = 5;
```

👉 Aslında her `let` ifadesinde bir desen kullanıyorsunuz! Daha resmi olarak, bir `let` ifadesi şu biçimdedir:

```rust
let PATTERN = EXPRESSION;
```

Örneğin `let x = 5;` ifadesinde `x` bir desendir. Rust, ifadeyi desenle karşılaştırır ve bulduğu isimleri bağlar. Burada `x`, “buradaki değeri `x` değişkenine bağla” anlamına gelir.

Desenlerin `let` ile nasıl çalıştığını daha net görmek için 19-4 numaralı listeye bakalım:

```rust
let (x, y, z) = (1, 2, 3);
```

👉 Bu desen, üç değişkeni aynı anda oluşturur: `x = 1`, `y = 2`, `z = 3`.

Eğer desendeki eleman sayısı ile demetteki eleman sayısı uyuşmazsa, derleyici hata verir. 19-5 numaralı listede bu hatalı durum gösterilmektedir:

```rust
let (x, y) = (1, 2, 3);
```

👉 Derleme hatası: `mismatched types` – 2 eleman bekleniyor, ama 3 elemanlı bir demet veriliyor.

Bu hatayı çözmek için, `_` veya `..` kullanarak bazı değerleri yok sayabiliriz.

---

## 🏷️ Fonksiyon Parametreleri (function parameters)

Fonksiyon parametreleri de desen olabilir. 19-6 numaralı listede, `foo` adında tek parametreli bir fonksiyon görülmektedir:

```rust
fn foo(x: i32) {
    // code goes here
}
```

👉 Buradaki `x` de bir desendir.

Desenleri fonksiyon parametrelerinde demetleri açmak için de kullanabiliriz. 19-7 numaralı listede bunun bir örneği gösterilmektedir:

### 📂 Dosya Adı: `src/main.rs`

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

👉 Çıktı:

```
Current location: (3, 5)
```

Burada `&(3, 5)` değeri, `&(x, y)` desenine eşleşir ve `x = 3`, `y = 5` olur.

Ayrıca, fonksiyon parametrelerinde olduğu gibi **kapanışların (closures)** parametre listelerinde de desenler kullanılabilir. Çünkü kapanışlar fonksiyonlara çok benzer (13. bölümde tartışıldığı gibi).

---

Bu noktaya kadar desenleri kullanmanın birçok yolunu gördük. Ancak desenler her yerde aynı şekilde çalışmaz: bazı yerlerde desenlerin **reddedilemez (irrefutable)** olması gerekir, bazı yerlerde ise **reddedilebilir (refutable)** olabilir. Bir sonraki bölümde bu iki kavramı ele alacağız.

