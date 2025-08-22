use generics_project::{Pair, Printable};

// Generic fonksiyon trait bound ile
fn print_anything<T: Printable>(item: &T) {
    println!("   Printable trait ile: {}", item.printable());
}

// Trait bound'ı where clause ile yazmak
fn display_item<T>(item: &T) 
where 
    T: Printable 
{
    println!("   Where clause ile: {}", item.printable());
}

fn main() {
    println!("=== Traits Örneği ===");
    
    println!("\n1. Trait implementation - Pair<String>:");
    let p_str = Pair::new("hello".to_string(), "world".to_string());
    println!("   Doğrudan çağrı: {}", p_str.printable());
    print_anything(&p_str);
    display_item(&p_str);
    
    println!("\n2. Trait implementation - Pair<i32>:");
    let p_num = Pair::new(100, 200);
    println!("   Doğrudan çağrı: {}", p_num.printable());
    print_anything(&p_num);
    display_item(&p_num);
    
    // Assertion'lar
    assert!(p_str.printable().contains("hello"));
    assert!(p_num.printable().contains("100"));
    
    println!("\n✅ Tüm trait örnekleri başarıyla çalıştı!");
}
