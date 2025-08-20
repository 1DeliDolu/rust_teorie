// Alternatif basit saat fonksiyonu
mod bedingung_if_else;
mod guessing_game;

fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);
    
    // Basit selamlama çağır (chrono kullanmadan)
    simple_greeting();
}

fn simple_greeting() {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // Basit saat hesaplama
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let hour = ((timestamp / 3600) + 3) % 24; // UTC+3 (Türkiye saati)
    
    let greeting = if hour < 9 {
        "Günaydın"
    } else if hour < 18 {
        "İyi günler" 
    } else if hour < 22 {
        "İyi akşamlar"
    } else {
        "İyi geceler"
    };
    
    println!("Saat: {}:xx - {}", hour, greeting);


    // guessing game
    guessing_game::guessing_game();
}