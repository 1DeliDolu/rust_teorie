## 🧩 Kapanışlar (closures): Ortamlarını Yakalayan Anonim Fonksiyonlar

Rust’un kapanışları (closures), bir değişkende saklayabileceğiniz veya başka fonksiyonlara argüman olarak geçebileceğiniz anonim fonksiyonlardır. Bir kapanışı bir yerde tanımlayıp başka bir yerde çağırabilir ve farklı bir bağlamda çalıştırabilirsiniz. Fonksiyonlardan farklı olarak, kapanışlar tanımlandıkları kapsamdan (scope) değerler yakalayabilir. Bu özellik, kodun yeniden kullanılmasına ve davranışların özelleştirilmesine olanak tanır.

---

## 🔗 Kapanışlarla Ortamı Yakalama

Önce, kapanışları tanımlandıkları ortamdan değerleri daha sonra kullanmak üzere nasıl yakalayabileceğimizi inceleyelim. Senaryo şu: Tişört şirketimiz zaman zaman promosyon amacıyla posta listemizdeki birine özel, sınırlı sayıda üretilen bir tişört hediye ediyor. Posta listesinde olan kişiler profillerine favori renklerini ekleyebilir. Seçilen kişinin favori rengi varsa, o renkte bir tişört alır. Eğer favori rengi belirtilmemişse, şirketin şu anda en çok sahip olduğu renkte tişört gönderilir.

Bunu uygulamanın birçok yolu var. Bu örnek için, yalnızca iki varyanta sahip bir enum (numaralandırma) `ShirtColor` tanımlayacağız: `Red` ve `Blue` (basitlik için renk sayısını sınırlıyoruz). Şirketin stoklarını, içinde mevcut tişört renklerini temsil eden bir `Vec<ShirtColor>` bulunan `shirts` alanına sahip bir `Inventory` yapısı ile göstereceğiz. `Inventory` üzerinde tanımlanan `giveaway` metodu, ücretsiz tişört kazanan kişinin isteğe bağlı renk tercihini alır ve kişinin alacağı rengi döndürür. Bu yapılandırma 13-1 numaralı listede gösterilmiştir:

**Dosya adı:** `src/main.rs`

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

📌 Bu kod, kullanıcı tercihini ve stok durumunu kullanarak verilecek tişört rengini belirler.

---

`main` fonksiyonunda tanımlanan mağazada (`store`) dağıtım için iki mavi ve bir kırmızı tişört bulunmaktadır. `giveaway` metodu bir kırmızı tercih eden kullanıcı için ve hiç tercih belirtmeyen kullanıcı için çağrılır.

Bu kod birçok şekilde uygulanabilirdi, ancak burada kapanışlara odaklanmak için yalnızca daha önce öğrendiğiniz kavramları kullandık. Tek yeni kısım, `giveaway` metodunun gövdesinde bir kapanış kullanmamızdır. `giveaway` metodunda kullanıcı tercihini `Option<ShirtColor>` tipinde bir parametre olarak alıyoruz ve `user_preference` üzerinde `unwrap_or_else` metodunu çağırıyoruz. `unwrap_or_else`, standart kütüphanede tanımlıdır. Bir argüman alır: herhangi bir parametre almayan ve bir `T` değeri döndüren bir kapanış (`Option<T>`’in `Some` varyantında saklanan tür ile aynı tipte, burada `ShirtColor`). Eğer `Option<T>` `Some` varyantıysa, `unwrap_or_else` içindeki değeri döndürür. Eğer `None` ise, kapanışı çağırır ve kapanışın döndürdüğü değeri verir.

Burada, `unwrap_or_else` metoduna argüman olarak `|| self.most_stocked()` ifadesini verdik. Bu, hiçbir parametre almayan ve gövdesinde `self.most_stocked()` çağıran bir kapanıştır. Kapanışı burada tanımlıyoruz, ancak `unwrap_or_else`’un implementasyonu gerekirse kapanışı daha sonra çalıştırır.

Bu kodu çalıştırdığımızda şu sonucu verir:

```bash
$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

Buradaki ilginç nokta, mevcut `Inventory` örneği üzerinde `self.most_stocked()` metodunu çağıran bir kapanışı aktarmış olmamızdır. Standart kütüphane bizim tanımladığımız `Inventory` veya `ShirtColor` tipleri hakkında ya da bu senaryoda kullanmak istediğimiz mantık hakkında hiçbir şey bilmek zorunda değildi. Kapanış, `self` örneğine (Inventory’nin değiştirilemez bir referansına) erişti ve bizim belirttiğimiz kodu `unwrap_or_else` metoduna aktardı. Fonksiyonlar ise ortamlarını bu şekilde yakalayamazlar.


## 🧩 Kapanışlarda (closures) Tür Çıkarımı ve Açık Tür Belirtimi (annotation)

Fonksiyonlar ile kapanışlar arasında başka farklılıklar da vardır. Fonksiyonların (`fn`) aksine, kapanışlarda genellikle parametrelerin veya dönüş değerinin türünü belirtmeniz gerekmez. Fonksiyonlarda tür açıklamaları zorunludur çünkü türler, kullanıcılarınıza açıkça sunulan bir arayüzün parçasıdır. Bu arayüzün katı bir şekilde tanımlanması, herkesin bir fonksiyonun hangi tür değerleri kullandığı ve döndürdüğü konusunda hemfikir olmasını sağlar.

Kapanışlar ise böyle bir dış arayüzde kullanılmaz; genellikle değişkenlerde saklanır, adlandırılmadan kullanılır ve kütüphanemizin kullanıcılarına doğrudan açılmaz.

Kapanışlar tipik olarak kısadır ve rastgele herhangi bir senaryoda değil, dar bir bağlam içinde kullanılır. Bu sınırlı bağlamlarda derleyici, parametrelerin ve dönüş değerinin türlerini, tıpkı çoğu değişkenin türünü çıkarabildiği gibi, çıkarabilir. (Nadir durumlarda derleyicinin kapanış tür açıklamalarına da ihtiyacı olabilir.)

Değişkenlerde olduğu gibi, daha açık ve net olmak için tür açıklamaları ekleyebiliriz. Bu, gereğinden daha fazla sözdizimi kullanma pahasına olabilir. Tür açıklamalarını eklenmiş bir kapanış tanımı 13-2 numaralı listede gösterilmiştir. Bu örnekte, kapanışı bir değişkende saklıyoruz; 13-1’de yaptığımız gibi doğrudan argüman olarak tanımlamıyoruz.

**Dosya adı:** `src/main.rs`

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

📌 Bu örnekte, kapanışın parametre (`u32`) ve dönüş tipi (`u32`) açıkça belirtilmiştir.

---

Kapanışlarda tür açıklamaları eklediğimizde sözdizimi, fonksiyon sözdizimine daha çok benzer. Aşağıda bir fonksiyon ile aynı işi yapan farklı kapanış tanımları karşılaştırmalı olarak verilmiştir:

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

* İlk satır: normal bir fonksiyon tanımı.
* İkinci satır: tüm tür açıklamaları eklenmiş bir kapanış tanımı.
* Üçüncü satır: kapanıştan tür açıklamaları kaldırılmış.
* Dördüncü satır: kapanış gövdesi tek bir ifade olduğu için süslü parantezler de kaldırılmış.

Bunların hepsi geçerli tanımlardır ve çağrıldıklarında aynı davranışı gösterirler. Ancak `add_one_v3` ve `add_one_v4` kapanışlarının derlenebilmesi için önce çağrılmaları gerekir, çünkü türler ancak kullanım yerinden çıkarılabilir. Bu, `let v = Vec::new();` ifadesinin tür çıkarımı için ya açık tür açıklaması ya da içine eleman eklenmesini gerektirmesine benzer.

---

Derleyici, kapanış tanımlarında her parametre ve dönüş değeri için yalnızca **bir somut tür** çıkarır. Örneğin, 13-3 numaralı listede sadece aldığı değeri geri döndüren kısa bir kapanış gösterilmiştir. Bu kapanışın tanımında hiçbir tür açıklaması yoktur. İlk çağrıda bir `String` ile kullanıldığında derleyici, parametre ve dönüş tipini `String` olarak belirler. Daha sonra aynı kapanışı bir tamsayı ile çağırmaya çalıştığımızda hata alırız.

**Dosya adı:** `src/main.rs`
(Bu kod derlenmez!)

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

📌 İlk çağrıdan sonra kapanışın parametre ve dönüş tipi `String` olarak kilitlenir, bu nedenle ikinci çağrı derleyici hatasına yol açar.

---

Derleyicinin verdiği hata çıktısı:

```bash
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |             --------------- ^- help: try using a conversion method: `.to_string()`
  |             |               |
  |             |               expected `String`, found integer
  |             arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `String`
 --> src/main.rs:4:29
  |
4 |     let s = example_closure(String::from("hello"));
  |             --------------- ^^^^^^^^^^^^^^^^^^^^^ expected because this argument is of type `String`
  |             |
  |             in this closure call
note: closure parameter defined here
 --> src/main.rs:2:28
  |
2 |     let example_closure = |x| x;
  |                            ^
```

Özetle: Bir kapanış ilk çağrıldığında kullanılan türler belirlenir ve sabitlenir. Daha sonra farklı bir tür ile çağrıldığında tür uyumsuzluğu hatası alınır.


## 🔗 Referansları Yakalama veya Sahipliği Taşıma

Kapanışlar (closures), ortamlarındaki değerleri üç şekilde yakalayabilir. Bu, fonksiyonların parametreleri üç farklı şekilde almasına doğrudan karşılık gelir:

1. **Değiştirilemez ödünç alma (immutable borrow)**
2. **Değiştirilebilir ödünç alma (mutable borrow)**
3. **Sahipliği alma (ownership move)**

Kapanış, gövdesinde yakaladığı değerlerle ne yaptığına bağlı olarak bu yöntemlerden hangisini kullanacağını kendi belirler.

---

## 📌 Değiştirilemez Referans Yakalama

13-4 numaralı listede, yalnızca değeri yazdırmak için bir vektöre (`list`) değiştirilemez referans yakalayan bir kapanış tanımlıyoruz:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```

📌 Bu kapanış, `list` üzerinde değişiklik yapmadığı için yalnızca değiştirilemez bir referans yakalar.

Çıktı:

```bash
$ cargo run
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]
```

Bu örnek aynı zamanda, bir kapanış tanımını bir değişkene bağlayabileceğimizi ve daha sonra o değişken adını fonksiyon gibi parantezlerle çağırabileceğimizi göstermektedir.

---

## 📌 Değiştirilebilir Referans Yakalama

13-5 numaralı listede kapanış gövdesini değiştirip, listeye yeni bir eleman eklemesini sağlıyoruz. Bu durumda kapanış, değiştirilebilir bir referans yakalar:

**Dosya adı:** `src/main.rs`

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}
```

📌 Bu kez kapanış, `list` üzerinde değişiklik yaptığı için değiştirilebilir (`mutable`) referans yakalar.

Çıktı:

```bash
$ cargo run
Before defining closure: [1, 2, 3]
After calling closure: [1, 2, 3, 7]
```

Dikkat ederseniz, kapanış tanımı ile çağrısı arasında `println!` yok. Çünkü kapanış tanımlandığında `list`’i değiştirilebilir olarak ödünç almıştır. Rust’ta değiştirilebilir bir ödünç varken aynı anda başka bir ödünç (değiştirilebilir veya değiştirilemez) yapılamaz. Bu nedenle, kapanış çağrılmadan önce `println!` eklemeyi denerseniz derleyici hata verecektir.

---

## 📌 Sahipliği Taşımak (move anahtar sözcüğü)

Kapanış gövdesi aslında sahipliği gerektirmese bile, ortamındaki değerlerin sahipliğini kapanışa taşımaya zorlayabilirsiniz. Bunun için parametre listesinden önce `move` anahtar sözcüğünü kullanırız.

Bu teknik özellikle yeni bir iş parçacığı (thread) oluştururken faydalıdır. Yeni iş parçacığına kapanış geçirildiğinde, verilerin o iş parçacığına taşınması gerekir. Aksi halde, ana iş parçacığı veriyi bırakırsa (drop) diğer iş parçacığındaki referans geçersiz hale gelir.

13-6 numaralı liste, 13-4’ün değiştirilmiş halidir. Bu kez vektörü ana iş parçacığında değil, yeni bir iş parçacığında yazdırıyoruz:

**Dosya adı:** `src/main.rs`

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```

📌 Burada kapanışın başına `move` yazıyoruz. Böylece `list`’in sahipliği yeni iş parçacığına taşınıyor.

Bunu yapmazsak derleyici hata verir çünkü ana iş parçacığı, alt iş parçacığı bitmeden önce `list`’i bırakabilir. Bu durumda, yeni iş parçacığında geçersiz bir referans oluşur. Bu yüzden Rust, `list`’in sahipliğini kapanışa taşımamızı zorunlu kılar.

👉 `move` anahtar sözcüğünü kaldırmayı ya da kapanış tanımından sonra `list`’i kullanmayı deneyin; derleyicinin vereceği hata mesajlarını göreceksiniz!


## 🔗 Kapanışlardan (closures) Yakalanan Değerleri Taşımak ve Fn Özellikleri (traits)

Bir kapanış, tanımlandığı ortamdan ya bir referans ya da sahipliği alarak değer yakaladığında (closure capture), kapanışın gövdesinde yazılan kod, bu değerlerin kapanış çalıştırıldığında nasıl kullanılacağını belirler. Yani kapanış gövdesi:

* Yakalanan değeri dışarı taşıyabilir (move),
* Yakalanan değeri değiştirebilir (mutate),
* Değeri ne taşıyabilir ne de değiştirebilir,
* Ya da ortamdan hiçbir şey yakalamayabilir.

Bir kapanışın ortamındaki değerleri nasıl yakaladığı ve işlediği, hangi `Fn` özelliğini (trait) uyguladığını belirler. Fonksiyonlar ve yapılar (structs), hangi tür kapanışları kullanabileceklerini bu `Fn` özellikleri aracılığıyla ifade ederler.

Kapanışlar, gövdelerinin değerleri nasıl ele aldığına bağlı olarak şu üç `Fn` özelliğinden birini veya birkaçını otomatik olarak uygularlar:

* **`FnOnce`**: Kapanış en az bir kez çağrılabilir. Tüm kapanışlar en azından bu özelliği uygular. Eğer kapanış gövdesi, yakalanan değerleri dışarı taşıyorsa (`move`), sadece `FnOnce` uygulanır çünkü kapanış yalnızca bir kez çağrılabilir.
* **`FnMut`**: Kapanış, değerleri dışarı taşımaz ama yakalanan değerleri değiştirebilir. Bu tür kapanışlar birden çok kez çağrılabilir.
* **`Fn`**: Kapanış, yakalanan değerleri ne dışarı taşır ne de değiştirir ya da ortamdan hiç değer yakalamaz. Bu tür kapanışlar birden fazla kez ve eşzamanlı olarak çağrılabilir.

---

## 📌 `unwrap_or_else` Örneği

Daha önce kullandığımız `Option<T>` türündeki `unwrap_or_else` metodunun tanımı:

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Burada:

* `T`, `Option` içindeki `Some` varyantında saklanan değerin türünü temsil eder.
* `unwrap_or_else`, kapanış `f`’yi yalnızca **en fazla bir kez** çağıracağı için, tür bağı `F: FnOnce() -> T` şeklindedir.

📌 Yani, `unwrap_or_else` tüm kapanış türlerini (`Fn`, `FnMut`, `FnOnce`) kabul edebilir çünkü hepsi en az `FnOnce` uygular.

Not: Eğer ortamdan değer yakalamamız gerekmiyorsa, kapanış yerine fonksiyon adı da kullanılabilir. Örneğin:

```rust
let v: Option<Vec<i32>> = None;
let result = v.unwrap_or_else(Vec::new);
```

Bu durumda `None` ise boş bir vektör döner.

---

## 📌 `sort_by_key` Örneği

Standart kütüphanedeki `sort_by_key` metodu farklıdır çünkü kapanışı **birden çok kez** çağırır. Bu yüzden `FnMut` ister.

**Dosya adı:** `src/main.rs`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}
```

📌 Bu kodda kapanış hiçbir şeyi taşımıyor veya değiştirmiyor, sadece `width` alanını okuyor. Bu nedenle `FnMut` koşulunu sağlıyor.

Çıktı:

```bash
[
    Rectangle { width: 3, height: 5 },
    Rectangle { width: 7, height: 12 },
    Rectangle { width: 10, height: 1 },
]
```

---

## 📌 Sadece `FnOnce` Uygulayan Bir Kapanış (Derlenmez Kod)

Aşağıdaki örnekte, kapanış ortamdan bir `String` alıyor ve onu `sort_operations` vektörüne taşıyor. Bu yüzden sadece `FnOnce` uygulanır. Ancak `sort_by_key`, kapanışı birden çok kez çağırır, bu yüzden derlenmez:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{list:#?}");
}
```

📌 Hata: `value` taşındıktan sonra ikinci çağrıda artık mevcut olmadığı için kapanış sadece bir kez çağrılabilir.

Derleyici hatası:

```bash
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
```

---

## 📌 `FnMut` ile Doğru Kullanım

Aynı senaryoda, ortamdan `String` taşımak yerine bir sayaç tutarsak, kapanış değiştirilebilir referans (`&mut`) yakalar ve birden çok kez çağrılabilir:

**Dosya adı:** `src/main.rs`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
```

Çıktı:

```bash
[
    Rectangle { width: 3, height: 5 },
    Rectangle { width: 7, height: 12 },
    Rectangle { width: 10, height: 1 },
], sorted in 3 operations
```

---

## 📋 Özet

* **`FnOnce`**: değer taşır → sadece bir kez çağrılabilir.
* **`FnMut`**: değerleri değiştirebilir → birden çok kez çağrılabilir.
* **`Fn`**: değerleri ne taşır ne değiştirir → birden çok kez güvenle çağrılabilir.

Bu `Fn` özellikleri, kapanışlarla çalışan fonksiyonların veya türlerin nasıl tanımlandığını anlamak için kritik öneme sahiptir.

👉 Bir sonraki bölümde yineleyiciler (iterators) konusunu inceleyeceğiz; birçok yineleyici metodu kapanış argümanları alır, bu yüzden burada öğrendiklerimizi akılda tutmalıyız.
