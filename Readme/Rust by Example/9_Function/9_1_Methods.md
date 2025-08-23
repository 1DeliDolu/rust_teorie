## 🛠️ İlişkili Fonksiyonlar (associated functions) ve Metotlar (methods)

Bazı fonksiyonlar belirli bir türe bağlıdır. Bunlar iki biçimde gelir: ilişkili fonksiyonlar (associated functions) ve metotlar (methods). İlişkili fonksiyonlar, bir türe genel olarak tanımlanan fonksiyonlardır; metotlar ise bir türün belirli bir örneği (instance) üzerinde çağrılan ilişkili fonksiyonlardır.

```rust
struct Point {
    x: f64,
    y: f64,
}

// Uygulama bloğu, tüm `Point` ilişkili fonksiyonları ve metotları buraya girer
impl Point {
    // Bu bir "ilişkili fonksiyon" çünkü bu fonksiyon belirli bir türe,
    // yani Point türüne bağlıdır.
    //
    // İlişkili fonksiyonların bir örnek ile çağrılması gerekmez.
    // Bu fonksiyonlar genellikle kurucu (constructor) gibi kullanılır.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // İki argüman alan başka bir ilişkili fonksiyon:
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}
```

```rust
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // Bu bir metottur
    // `&self`, `self: &Self` ifadesinin kısaltmasıdır.
    // Burada `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self`, nokta operatörü ile struct alanlarına erişim sağlar
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs`, çağıranın mutlak değerini döndüren bir `f64` metodudur
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // Bu metodun çağrıldığı nesnenin değiştirilebilir (mutable) olması gerekir
    // `&mut self`, `self: &mut Self` ifadesinin kısaltmasıdır
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}
```

```rust
// `Pair`, kaynaklara sahiptir: heap üzerinde ayrılmış iki tamsayı
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // Bu metod, çağıran nesnenin kaynaklarını "tüketir"
    // `self`, `self: Self` ifadesinin kısaltmasıdır
    fn destroy(self) {
        // `self` yapıbozuma uğratılır
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` ve `second` kapsam dışına çıkar ve serbest bırakılır
    }
}
```

```rust
fn main() {
    let rectangle = Rectangle {
        // İlişkili fonksiyonlar çift iki nokta ile çağrılır
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Metotlar nokta operatörü ile çağrılır
    // İlk argüman olan `&self` örtük (implicit) olarak geçirilir
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Hata! `rectangle` değiştirilemez (immutable),
    // fakat bu metod değiştirilebilir (mutable) bir nesne ister
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Bu satırı açmayı deneyin

    // Tamam! Değiştirilebilir nesneler değiştirilebilir metodları çağırabilir
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Hata! Önceki `destroy` çağrısı `pair` değişkenini "tüketti"
    //pair.destroy();
    // TODO ^ Bu satırı açmayı deneyin
}
```
