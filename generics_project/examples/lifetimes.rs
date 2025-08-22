use generics_project::{ImportantExcerpt, first_word};

// Lifetime'ı açık şekilde gösteren fonksiyon
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("=== Lifetimes Örneği ===");
    
    println!("\n1. ImportantExcerpt struct lifetime örneği:");
    let excerpt_text = "önemli bir alıntı";
    let excerpt = ImportantExcerpt { part: excerpt_text };
    let result = excerpt.announce_and_return_part("Bilgilendirme");
    println!("   Sonuç: '{}'", result);
    assert_eq!(result, excerpt_text);
    
    println!("\n2. first_word fonksiyonu ile birlikte:");
    let sentence = String::from("ilk ikinci üçüncü dördüncü");
    println!("   Cümle: '{}'", sentence);
    let first = first_word(&sentence);
    println!("   İlk kelime: '{}'", first);
    
    let excerpt2 = ImportantExcerpt { part: first };
    let result2 = excerpt2.announce_and_return_part("Parsing sonucu");
    println!("   Excerpt sonucu: '{}'", result2);
    assert_eq!(result2, "ilk");
    
    println!("\n3. Custom lifetime fonksiyonu:");
    let str1 = "kısa";
    let str2 = "daha uzun metin";
    let longer = longest(str1, str2);
    println!("   '{}' vs '{}' -> En uzun: '{}'", str1, str2, longer);
    assert_eq!(longer, str2);
    
    println!("\n4. Lifetime scope örneği:");
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result_in_scope = longest(&string1, &string2);
        println!("   Scope içinde: {}", result_in_scope);
        // result_in_scope burada string1'i işaret ediyor (daha uzun olduğu için)
        assert_eq!(result_in_scope, "long string is long");
    }
    // string2 artık scope dışında - bu yüzden result3'ü scope dışında kullanamazdık
    println!("   String1 hâlâ geçerli: {}", string1);
    
    println!("\n✅ Tüm lifetime örnekleri başarıyla çalıştı!");
}
