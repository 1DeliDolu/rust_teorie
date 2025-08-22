// 📺 Display Utilities - Görüntüleme Yardımcıları
// Bu modül struct'ları güzel formatlarda görüntülemek için kullanılır

pub struct ShapeDisplay;

impl ShapeDisplay {
    // Associated function - Display rectangle info
    pub fn rectangle_info(rect: &crate::shapes::Rectangle) {
        println!("┌{:─^30}┐", " RECTANGLE INFO ");
        println!("│ Width:     {:>15.2} │", rect.width);
        println!("│ Height:    {:>15.2} │", rect.height);
        println!("│ Area:      {:>15.2} │", rect.area());
        println!("│ Perimeter: {:>15.2} │", rect.perimeter());
        println!("│ Is Square: {:>15} │", if rect.is_square() { "Yes" } else { "No" });
        println!("└{:─^30}┘", "");
    }
    
    // Associated function - Display circle info
    pub fn circle_info(circle: &crate::shapes::Circle) {
        println!("┌{:─^30}┐", " CIRCLE INFO ");
        println!("│ Radius:       {:>12.2} │", circle.radius);
        println!("│ Diameter:     {:>12.2} │", circle.diameter());
        println!("│ Area:         {:>12.2} │", circle.area());
        println!("│ Circumference:{:>12.2} │", circle.perimeter());
        println!("└{:─^30}┘", "");
    }
    
    // Associated function - Display comparison
    pub fn compare_shapes(shape1: &str, area1: f64, shape2: &str, area2: f64) {
        println!("\n🔍 Shape Comparison:");
        println!("   {} Area: {:.2}", shape1, area1);
        println!("   {} Area: {:.2}", shape2, area2);
        
        if area1 > area2 {
            println!("   → {} is larger!", shape1);
        } else if area2 > area1 {
            println!("   → {} is larger!", shape2);
        } else {
            println!("   → Both shapes have equal area!");
        }
    }
}