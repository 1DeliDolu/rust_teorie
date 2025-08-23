## 🧪 Test durumu: haritalama-indirge (map-reduce)

Rust, geleneksel olarak böyle bir girişime eşlik eden pek çok zorluk olmadan veri işlemeyi paralelleştirmeyi çok kolay hale getirir.

Standart kütüphane, kutudan çıktığı haliyle mükemmel iş parçacığı (thread) ilkelleri sağlar. Bunlar, Rust’ın sahiplik (Ownership) ve aliasing kuralları ile birleştiğinde, veri yarışlarını (data race) otomatik olarak engeller.

Aliasing kuralları (bir yazılabilir başvuru (reference) XOR birçok okunabilir başvuru) sizi, diğer iş parçacıkları tarafından görülebilen durumu manipüle etmekten otomatik olarak alıkoyar. (Eşzamanlama gerektiğinde, `Mutex` veya `Channel` gibi eşzamanlama ilkelleri vardır.)

Bu örnekte, bir sayı bloğundaki tüm rakamların toplamını hesaplayacağız. Bunu, bloğun parçalarını farklı iş parçacıklarına dağıtarak yapacağız. Her iş parçacığı kendi küçük rakam bloğunu toplayacak ve ardından her iş parçacığının ürettiği ara toplamları toplayacağız.

Dikkat edilmelidir ki, iş parçacığı sınırları boyunca başvurular geçiriyor olsak da, Rust yalnızca salt-okunur (read-only) başvurular geçirdiğimizi anlar, bu nedenle güvensizlik veya veri yarışı olamaz. Ayrıca, geçirdiğimiz başvuruların `'static` yaşam sürelerine sahip olması nedeniyle Rust, bu iş parçacıkları hâlâ çalışırken verimizin yok edilmeyeceğini anlar. (İş parçacıkları arasında statik olmayan veriyi paylaşmanız gerektiğinde, veriyi canlı tutmak ve statik olmayan yaşam sürelerinden kaçınmak için `Arc` gibi akıllı işaretçiler (smart pointer) kullanabilirsiniz.)

```rust
use std::thread;

// Bu `main` iş parçacığıdır
fn main() {

    // Bu işlenecek verimizdir.
    // Tüm rakamların toplamını iş parçacıklı bir map-reduce algoritmasıyla hesaplayacağız.
    // Her boşlukla ayrılmış parça farklı bir iş parçacığında işlenecektir.
    //
    // TODO: Çıktıya boşluklar eklerseniz ne olduğuna bakın!
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Başlatacağımız alt iş parçacıklarını tutmak için bir vektör oluştur.
    let mut children = vec![];

    /*************************************************************************
     * "Map" aşaması
     *
     * Verimizi parçalara ayır ve ilk işlemleri uygula
     ************************************************************************/

    // Verimizi bireysel hesaplama için parçalara ayır
    // her parça, gerçek veriye bir başvuru (&str) olacaktır
    let chunked_data = data.split_whitespace();

    // Veri parçaları üzerinde yineleme yap.
    // .enumerate() mevcut döngü indeksini yineleme ile ekler
    // ortaya çıkan "(indeks, eleman)" ikilisi hemen
    // iki değişkene, "i" ve "data_segment"e "ayrıştırılarak" atanır
    for (i, data_segment) in chunked_data.enumerate() {
        println!("veri parçası {}: \"{}\"", i, data_segment);

        // Her veri parçasını ayrı bir iş parçacığında işle
        //
        // spawn() yeni iş parçacığına bir tanıtıcı döndürür,
        // döndürülen değere erişmek için bunu MUTLAKA saklamalıyız
        //
        // 'move || -> u32' bir kapanış için söz dizimidir:
        // * hiçbir argüman almaz ('||')
        // * yakalanan değişkenlerin sahipliğini alır ('move') ve
        // * işaretsiz 32 bitlik bir tamsayı döndürür ('-> u32')
        //
        // Rust, kapanıştan '-> u32' türünü çıkarsayabilir,
        // bu yüzden aslında bunu yazmasak da olurdu.
        //
        // TODO: 'move' ifadesini kaldırmayı deneyin ve ne olduğunu görün
        children.push(thread::spawn(move || -> u32 {
            // Bu parçanın ara toplamını hesapla:
            let result = data_segment
                        // parçamızdaki karakterler üzerinde yinele..
                        .chars()
                        // .. metin-karakterlerini sayısal değerine dönüştür..
                        .map(|c| c.to_digit(10).expect("rakam olmalı"))
                        // .. ve ortaya çıkan sayı yineleyicisini topla
                        .sum();

            // println! stdout’u kilitler, böylece metin çakışması olmaz
            println!("işlenen parça {}, sonuç={}", i, result);

            // "return" gerekli değil, çünkü Rust bir "ifade dili"dir,
            // her bloğun son ifadesi otomatik olarak değeri olur.
            result

        }));
    }


    /*************************************************************************
     * "Reduce" aşaması
     *
     * Ara sonuçlarımızı topla ve bunları nihai bir sonuçta birleştir
     ************************************************************************/

    // Her iş parçacığının ara sonuçlarını tek bir nihai toplamda birleştir.
    //
    // sum() fonksiyonuna tür ipucu vermek için "turbofish" ::<> kullanıyoruz.
    //
    // TODO: turbofish olmadan dene, bunun yerine final_result değişkeninin
    // türünü açıkça belirle
    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Nihai toplam sonucu: {}", final_result);
}
```

### 📋 Ödevler

Kullanıcı tarafından girilen verilere bağlı olarak iş parçacığı sayımızı belirlemek akıllıca değildir. Peki ya kullanıcı çok fazla boşluk eklerse? Gerçekten 2.000 iş parçacığı başlatmak ister miyiz? Programı, verilerin her zaman programın başında tanımlanmış statik bir sabit tarafından belirlenen sınırlı sayıda parçaya bölünecek şekilde değiştirin.

### 🔗 Ayrıca bakınız

* İş parçacıkları (Threads)
* Vektörler (vectors) ve yineleyiciler (iterators)
* Kapanışlar (closures), move semantiği ve move kapanışları
* Ayrıştırmalı atamalar (destructuring assignments)
* Tür çıkarımına yardımcı olmak için turbofish gösterimi (turbofish notation)
* `unwrap` vs. `expect`
* `enumerate`
