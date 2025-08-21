// 📐 Geometri Hesaplayıcısı - Struct Öğrenme Projesi
// Bu projede Rust'ın struct, method ve associated function kavramlarını öğreneceğiz

mod shapes;
mod geometry;
mod display;

use shapes::{Rectangle, Circle, Triangle};
use geometry::ShapeCalculator;
use display::ShapeDisplay;

fn main() {
    println!("🚀 Geometri Hesaplayıcısı - Struct Dersleri");
    println!("{}", "=".repeat(50));
    
    // ADIM 1: Temel Struct Tanımları ve Instantiation
    step1_basic_structs();
    
    // ADIM 2: Tuple Struct ve Unit Struct
    step2_tuple_and_unit_structs();
    
    // ADIM 3: Method Implementation (&self, &mut self, self)
    step3_methods();
    
    // ADIM 4: Associated Functions (new, constructor'lar)
    step4_associated_functions();
    
    // ADIM 5: Multiple impl Blocks ve Traits
    step5_multiple_impl_blocks();
    
    // ADIM 6: Real-World Example - Şekil Koleksiyonu
    step6_real_world_example();
}

// ADIM 1: Temel Struct Kavramları
fn step1_basic_structs() {
    println!("\n📖 ADIM 1: Temel Struct Tanımları");
    println!("{}", "-".repeat(40));
    
    // Rectangle struct instantiation
    let rect1 = Rectangle {
        width: 30.0,
        height: 50.0,
    };
    
    // Field access
    println!("📐 Dikdörtgen - Genişlik: {}, Yükseklik: {}", rect1.width, rect1.height);
    
    // Circle struct
    let circle1 = Circle {
        radius: 25.0,
    };
    
    println!("🔵 Daire - Yarıçap: {}", circle1.radius);
    
    // Mutable struct
    let mut rect2 = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    
    println!("📏 Değiştirilmeden önce: {}x{}", rect2.width, rect2.height);
    
    // Field modification
    rect2.width = 15.0;
    rect2.height = 25.0;
    
    println!("📏 Değiştirildikten sonra: {}x{}", rect2.width, rect2.height);
    println!("✅ Temel struct işlemleri tamamlandı!");
}

// ADIM 2: Tuple ve Unit Struct'lar
fn step2_tuple_and_unit_structs() {
    println!("\n🎨 ADIM 2: Tuple ve Unit Struct'lar");
    println!("{}", "-".repeat(40));
    
    // Tuple struct - Color
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);
    
    println!("🔴 Kırmızı: RGB({}, {}, {})", red.0, red.1, red.2);
    println!("🟢 Yeşil: RGB({}, {}, {})", green.0, green.1, green.2);
    println!("🔵 Mavi: RGB({}, {}, {})", blue.0, blue.1, blue.2);
    
    // Unit struct
    let _marker = ShapeMarker;
    println!("🏷️ Marker struct oluşturuldu");
    
    println!("✅ Tuple ve unit struct'lar tamamlandı!");
}

// ADIM 3: Method Implementation
fn step3_methods() {
    println!("\n⚙️ ADIM 3: Method Implementation");
    println!("{}", "-".repeat(40));
    
    let rect = Rectangle {
        width: 30.0,
        height: 50.0,
    };
    
    // &self methods - borrowing
    println!("📐 Dikdörtgen Alanı: {}", rect.area());
    println!("📐 Dikdörtgen Çevresi: {}", rect.perimeter());
    
    let circle = Circle { radius: 15.0 };
    
    println!("🔵 Daire Alanı: {:.2}", circle.area());
    println!("🔵 Daire Çevresi: {:.2}", circle.perimeter());
    
    // &mut self method
    let mut rect_mut = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    
    println!("📏 Ölçeklendirmeden önce alan: {}", rect_mut.area());
    rect_mut.scale(2.0);
    println!("📏 2x ölçeklendirme sonrası alan: {}", rect_mut.area());
    
    println!("✅ Method implementation tamamlandı!");
}

// ADIM 4: Associated Functions
fn step4_associated_functions() {
    println!("\n🏗️ ADIM 4: Associated Functions");
    println!("{}", "-".repeat(40));
    
    // Constructor functions
    let rect = Rectangle::new(25.0, 35.0);
    println!("🔨 Constructor ile oluşturulan dikdörtgen: {}x{}", rect.width, rect.height);
    
    let square = Rectangle::square(20.0);
    println!("⬜ Kare (square factory): {}x{}", square.width, square.height);
    
    let circle = Circle::new(10.0);
    println!("🔵 Constructor ile oluşturulan daire: yarıçap {}", circle.radius);
    
    let circle_from_diameter = Circle::from_diameter(30.0);
    println!("🔵 Çaptan oluşturulan daire: yarıçap {}", circle_from_diameter.radius);
    
    println!("✅ Associated functions tamamlandı!");
}

// ADIM 5: Multiple impl Blocks
fn step5_multiple_impl_blocks() {
    println!("\n🔄 ADIM 5: Multiple impl Blocks");
    println!("{}", "-".repeat(40));
    
    let rect = Rectangle::new(15.0, 25.0);
    
    // Debug formatting
    println!("🐛 Debug format: {:?}", rect);
    
    // Display formatting
    println!("📺 Display format: {}", rect);
    
    // Comparison
    let rect2 = Rectangle::new(15.0, 25.0);
    println!("📊 Dikdörtgenler eşit mi? {}", rect.equals(&rect2));
    
    let circle = Circle::new(12.0);
    println!("🔵 Daire bilgileri: {}", circle);
    
    println!("✅ Multiple impl blocks tamamlandı!");
}

// ADIM 6: Real-World Example
fn step6_real_world_example() {
    println!("\n🌍 ADIM 6: Gerçek Dünya Örneği");
    println!("{}", "-".repeat(40));
    
    // Şekil koleksiyonu
    let shapes = vec![
        ShapeType::Rectangle(Rectangle::new(10.0, 15.0)),
        ShapeType::Circle(Circle::new(8.0)),
        ShapeType::Rectangle(Rectangle::square(12.0)),
        ShapeType::Circle(Circle::from_diameter(20.0)),
    ];
    
    println!("📚 Şekil Koleksiyonu Analizi:");
    println!("{}", "-".repeat(25));
    
    let mut total_area = 0.0;
    
    for (i, shape) in shapes.iter().enumerate() {
        let area = match shape {
            ShapeType::Rectangle(rect) => {
                println!("{}. 📐 Dikdörtgen: {}x{} - Alan: {:.2}", i+1, rect.width, rect.height, rect.area());
                rect.area()
            },
            ShapeType::Circle(circle) => {
                println!("{}. 🔵 Daire: r={} - Alan: {:.2}", i+1, circle.radius, circle.area());
                circle.area()
            },
        };
        total_area += area;
    }
    
    println!("\n📊 Toplam Alan: {:.2}", total_area);
    println!("✅ Real-world example tamamlandı!");
}

// Tuple struct for colors
#[derive(Debug)]
struct Color(u8, u8, u8);

// Unit struct for markers
struct ShapeMarker;

// Enum for different shape types
#[derive(Debug)]
enum ShapeType {
    Rectangle(Rectangle),
    Circle(Circle),
}
