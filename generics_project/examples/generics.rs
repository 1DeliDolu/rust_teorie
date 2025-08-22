use generics_project::{Pair, largest, first_word, Printable};

fn main() {
    println!("=== Generic Types Örneği ===");
    
    // Pair<T> örneği - farklı türlerle
    println!("\n1. Pair<i32> örneği:");
    let mut p = Pair::new(10, 20);
    println!("   Orijinal: {}", p.printable());
    p.swap();
    println!("   Swap sonrası: {}", p.printable());
    assert_eq!(p.a, 20);
    assert_eq!(p.b, 10);
    
    println!("\n2. Pair<String> örneği:");
    let mut p_str = Pair::new("alpha".to_string(), "beta".to_string());
    println!("   Orijinal: {}", p_str.printable());
    p_str.swap();
    println!("   Swap sonrası: {}", p_str.printable());

    // largest fonksiyonu - farklı türlerle
    println!("\n3. largest<T> fonksiyonu:");
    let nums = [3, 42, 7, 21];
    println!("   Sayı dizisi: {:?}", nums);
    if let Some(max) = largest(&nums) {
        println!("   En büyük: {}", max);
        assert_eq!(max, 42);
    }
    
    let chars = ['a', 'z', 'c', 'm'];
    println!("   Karakter dizisi: {:?}", chars);
    if let Some(max_char) = largest(&chars) {
        println!("   En büyük karakter: {}", max_char);
        assert_eq!(max_char, 'z');
    }

    // first_word örneği
    println!("\n4. first_word fonksiyonu:");
    let s = String::from("merhaba dünya örnek");
    println!("   Metin: '{}'", s);
    let first = first_word(&s);
    println!("   İlk kelime: '{}'", first);
    assert_eq!(first, "merhaba");
    
    println!("\n✅ Tüm generic type örnekleri başarıyla çalıştı!");
}
