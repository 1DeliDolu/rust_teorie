use std::time::{SystemTime, UNIX_EPOCH};

pub fn greeting_by_time() {
    // Basit saat hesaplama (chrono olmadan)
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let hour = ((timestamp / 3600) + 3) % 24; // UTC+3 (Türkiye saati)

    let greeting = if hour < 9 {
        "Günaydın"
    } else if hour < 18 {
        "İyi günler"
    } else if hour < 22 {
        "İyi akşamlar"
    } else if hour < 24 || hour < 4 {
        "İyi geceler"
    } else {
        "Merhaba"
    };

    println!("Saat: {}:xx - {}", hour, greeting);
}