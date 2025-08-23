## 📜 read\_lines

### 🚧 Basit bir yaklaşım

Bu, dosyadan satır okuma için bir acemi tarafından yapılacak makul bir ilk deneme olabilir.

```rust
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
```

Metot `lines()`, dosyadaki satırlar üzerinde bir yineleyici (iterator) döndürdüğünden, ayrıca satırları doğrudan `map` ile dönüştürüp toplayabiliriz. Böylece daha özlü ve akıcı bir ifade elde edilir:

```rust
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // olası dosya okuma hatalarında panik et
        .lines()  // string’i satır dilimlerine böler
        .map(String::from)  // her dilimi string’e dönüştür
        .collect()  // hepsini bir vektörde topla
}
```

Yukarıdaki her iki örnekte de, `lines()` metodunun döndürdüğü `&str` başvurusunu, sahipli `String` türüne dönüştürmek zorundayız (`.to_string()` veya `String::from` ile).

---

### ⚡ Daha verimli bir yaklaşım

Burada, açılmış `File` nesnesinin sahipliğini `BufReader` yapısına geçiriyoruz. `BufReader`, ara belleklemeleri azaltmak için dahili bir tampon (buffer) kullanır.

Ayrıca `read_lines` fonksiyonunu, her satır için yeni `String` nesnesi tahsis etmek yerine, doğrudan bir yineleyici döndürecek şekilde güncelliyoruz.

```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // hosts.txt dosyası mevcut dizinde bulunmalıdır
    if let Ok(lines) = read_lines("./hosts.txt") {
        // Yineleyiciyi tüketir, isteğe bağlı (Optional) String döner
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}

// Çıktı bir Result içine sarılır, böylece hatalara göre eşleştirme yapılabilir.
// Dosyanın satırlarının Reader’ına bir Iterator döndürür.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```

### ✅ Örnek çıktı

```bash
$ echo -e "127.0.0.1\n192.168.0.1\n" > hosts.txt
$ rustc read_lines.rs && ./read_lines
127.0.0.1
192.168.0.1
```

(Dikkat: `File::open`, argüman olarak genel bir `AsRef<Path>` beklediğinden, kendi genel `read_lines()` fonksiyonumuzu da `where` anahtar kelimesini kullanarak aynı genel kısıtlama ile tanımlıyoruz.)

---

Bu yöntem, tüm dosya içeriğini bellekte bir `String` içine yüklemeye kıyasla daha verimlidir. Özellikle büyük dosyalarla çalışırken, bellekte büyük bir `String` tahsis etmek performans sorunlarına yol açabilir.
