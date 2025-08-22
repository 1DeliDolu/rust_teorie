// Collections Project - Pratik Örnekler
// Bu dosyada 8. bölümde öğrenilen konuların pratik örneklerini bulacaksınız

use std::collections::HashMap;

fn main() {
    println!("🦀 RUST COLLECTIONS - PRATİK ÖRNEKLER");
    println!("=====================================\n");

    // 1. Vector Örnekleri
    vector_examples();
    
    // 2. String Örnekleri  
    string_examples();
    
    // 3. HashMap Örnekleri
    hashmap_examples();
    
    // 4. Kombinasyon Örnekleri
    combination_examples();
}

fn vector_examples() {
    println!("📋 VECTOR ÖRNEKLERİ");
    println!("==================");
    
    // Yeni vector oluşturma
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("✅ Vector oluşturuldu: {:?}", numbers);
    
    // vec! makrosu ile oluşturma
    let fruits = vec!["elma", "armut", "muz"];
    println!("✅ Meyveler: {:?}", fruits);
    
    // Vector elemanlarına erişim
    let first = &fruits[0];
    println!("✅ İlk meyve: {}", first);
    
    // Güvenli erişim
    match fruits.get(1) {
        Some(fruit) => println!("✅ İkinci meyve: {}", fruit),
        None => println!("❌ Meyve bulunamadı"),
    }
    
    // Vector üzerinde döngü
    println!("✅ Tüm meyveler:");
    for fruit in &fruits {
        println!("  🍎 {}", fruit);
    }
    
    // Functional programming
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even_numbers: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();
    println!("✅ Çift sayılar (x2): {:?}", even_numbers);
    
    println!();
}

fn string_examples() {
    println!("🔤 STRING ÖRNEKLERİ");
    println!("==================");
    
    // Yeni String oluşturma
    let mut greeting = String::new();
    greeting.push_str("Merhaba");
    greeting.push(' ');
    greeting.push_str("Rust!");
    println!("✅ Selamlama: {}", greeting);
    
    // String::from ile oluşturma
    let message = String::from("Rust öğreniyorum!");
    println!("✅ Mesaj: {}", message);
    
    // to_string() metodu
    let data = "İlk içerik";
    let s = data.to_string();
    println!("✅ String'e dönüştürülmüş: {}", s);
    
    // String birleştirme
    let s1 = String::from("Merhaba, ");
    let s2 = String::from("dünya!");
    let result = s1 + &s2; // s1 artık kullanılamaz
    println!("✅ Birleştirilmiş: {}", result);
    
    // format! makrosu
    let name = "Ahmet";
    let age = 25;
    let formatted = format!("Benim adım {} ve {} yaşındayım", name, age);
    println!("✅ Formatlanmış: {}", formatted);
    
    // UTF-8 desteği
    let hello_turkish = String::from("Merhaba 🦀 Rust");
    let hello_arabic = String::from("مرحبا");
    let hello_japanese = String::from("こんにちは");
    println!("✅ Türkçe: {}", hello_turkish);
    println!("✅ Arapça: {}", hello_arabic);
    println!("✅ Japonca: {}", hello_japanese);
    
    // String metodları
    let text = "  Rust Programlama Dili  ";
    println!("✅ Orijinal: '{}'", text);
    println!("✅ Trimmed: '{}'", text.trim());
    println!("✅ Büyük harf: '{}'", text.trim().to_uppercase());
    println!("✅ Küçük harf: '{}'", text.trim().to_lowercase());
    println!("✅ 'Rust' içeriyor mu? {}", text.contains("Rust"));
    
    println!();
}

fn hashmap_examples() {
    println!("📦 HASHMAP ÖRNEKLERİ");
    println!("===================");
    
    // Yeni HashMap oluşturma
    let mut scores = HashMap::new();
    scores.insert(String::from("Mavi"), 10);
    scores.insert(String::from("Sarı"), 50);
    println!("✅ Skorlar: {:?}", scores);
    
    // Değer alma
    let team_name = String::from("Mavi");
    match scores.get(&team_name) {
        Some(score) => println!("✅ {} takımının skoru: {}", team_name, score),
        None => println!("❌ Takım bulunamadı"),
    }
    
    // Güvenli değer alma
    let score = scores.get(&String::from("Mavi")).copied().unwrap_or(0);
    println!("✅ Mavi takımının skoru (varsayılan 0): {}", score);
    
    // HashMap üzerinde döngü
    println!("✅ Tüm takımlar:");
    for (key, value) in &scores {
        println!("  🏆 {}: {}", key, value);
    }
    
    // Değer güncelleme - üzerine yazma
    scores.insert(String::from("Mavi"), 25);
    println!("✅ Mavi takım skoru güncellendi: {:?}", scores);
    
    // Değer güncelleme - sadece yoksa ekle
    scores.entry(String::from("Yeşil")).or_insert(30);
    scores.entry(String::from("Mavi")).or_insert(100); // Eklenmeyecek
    println!("✅ Entry API kullanımı: {:?}", scores);
    
    // Değer güncelleme - mevcut değeri baz alarak
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("✅ Kelime sayıları:");
    for (word, count) in &word_count {
        println!("  📝 '{}': {} kez", word, count);
    }
    
    println!();
}

fn combination_examples() {
    println!("🔄 KOMBİNASYON ÖRNEKLERİ");
    println!("======================");
    
    // Öğrenci notları sistemi
    #[derive(Debug)]
    struct Student {
        name: String,
        grades: Vec<i32>,
    }
    
    let mut students: HashMap<String, Student> = HashMap::new();
    
    // Öğrenci ekleme
    let student1 = Student {
        name: String::from("Ahmet Yılmaz"),
        grades: vec![85, 92, 78, 90],
    };
    
    let student2 = Student {
        name: String::from("Zeynep Kaya"),
        grades: vec![95, 87, 92, 89],
    };
    
    students.insert(String::from("001"), student1);
    students.insert(String::from("002"), student2);
    
    // Öğrenci bilgilerini gösterme
    for (id, student) in &students {
        let average: f64 = student.grades.iter().sum::<i32>() as f64 / student.grades.len() as f64;
        let grades_str: Vec<String> = student.grades.iter().map(|&x| x.to_string()).collect();
        
        println!("✅ ID: {}", id);
        println!("   👤 Ad: {}", student.name);
        println!("   📊 Notlar: [{}]", grades_str.join(", "));
        println!("   📈 Ortalama: {:.2}", average);
        println!();
    }
    
    // Kelime indeksi oluşturma
    let documents = vec![
        "Rust güvenli bir programlama dilidir",
        "Python kolay öğrenilebilir bir dildir",
        "Rust performans odaklı bir dildir",
    ];
    
    let mut word_index: HashMap<String, Vec<usize>> = HashMap::new();
    
    for (doc_id, document) in documents.iter().enumerate() {
        for word in document.split_whitespace() {
            let word_lower = word.to_lowercase();
            word_index.entry(word_lower).or_insert_with(Vec::new).push(doc_id);
        }
    }
    
    println!("📚 Kelime İndeksi:");
    for (word, doc_ids) in &word_index {
        let ids: Vec<String> = doc_ids.iter().map(|&x| x.to_string()).collect();
        println!("  🔍 '{}' -> Dökümanlarda: [{}]", word, ids.join(", "));
    }
    
    println!();
    println!("🎉 Tüm örnekler tamamlandı!");
    println!("📖 Artık Vector, String ve HashMap'leri rahatlıkla kullanabilirsiniz!");
}
