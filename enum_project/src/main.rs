// ⚔️ RPG Oyun Karakter Sistemi - Enum Öğrenme Projesi
// Bu projede Rust'ın enum, pattern matching ve match kavramlarını öğreneceğiz

mod character;
mod combat;
mod items;
mod game;

use character::{CharacterClass, Element};
use combat::{Weapon, Spell, AttackResult};
use items::{Item, Inventory};
use game::GameEngine;

fn main() {
    println!("⚔️ RPG Oyun Karakter Sistemi - Enum Dersleri");
    println!("{}", "=".repeat(50));
    
    // ADIM 1: Temel Enum Tanımları
    step1_basic_enums();
    
    // ADIM 2: Enum with Data - Veri içeren enum'lar
    step2_enums_with_data();
    
    // ADIM 3: Match Control Flow - match yapısı
    step3_match_control_flow();
    
    // ADIM 4: Option ve Result handling
    step4_option_and_result();
    
    // ADIM 5: if let ve let else
    step5_if_let_and_let_else();
    
    // ADIM 6: Advanced Pattern Matching
    step6_advanced_patterns();
    
    // ADIM 7: Real Game Simulation
    step7_game_simulation();
}

// ADIM 1: Temel Enum Kavramları
fn step1_basic_enums() {
    println!("\n🎭 ADIM 1: Temel Enum Tanımları");
    println!("{}", "-".repeat(40));
    
    // Karakter sınıfları
    let warrior = CharacterClass::Warrior;
    let mage = CharacterClass::Mage;
    let archer = CharacterClass::Archer;
    let rogue = CharacterClass::Rogue;
    
    println!("👤 Karakter Sınıfları:");
    print_character_class(&warrior);
    print_character_class(&mage);
    print_character_class(&archer);
    print_character_class(&rogue);
    
    // Element türleri
    let elements = vec![
        Element::Fire,
        Element::Water,
        Element::Earth,
        Element::Air,
    ];
    
    println!("\n🌟 Element Türleri:");
    for element in elements {
        println!("   {} - Güç: {}", get_element_name(&element), get_element_power(&element));
    }
    
    println!("✅ Temel enum işlemleri tamamlandı!");
}

// ADIM 2: Veri İçeren Enum'lar
fn step2_enums_with_data() {
    println!("\n📦 ADIM 2: Veri İçeren Enum'lar");
    println!("{}", "-".repeat(40));
    
    // Silah enum'ı - farklı data türleri
    let sword = Weapon::Sword { damage: 25, durability: 100 };
    let bow = Weapon::Bow { damage: 20, arrows: 50 };
    let staff = Weapon::Staff { magic_power: 30, mana_cost: 10 };
    
    println!("🗡️ Silah Bilgileri:");
    print_weapon_info(&sword);
    print_weapon_info(&bow);
    print_weapon_info(&staff);
    
    // Büyü enum'ı
    let fireball = Spell::Fireball { damage: 40, radius: 5.0 };
    let heal = Spell::Heal { amount: 30 };
    let shield = Spell::Shield { defense_bonus: 15, duration: 10 };
    
    println!("\n🪄 Büyü Bilgileri:");
    print_spell_info(&fireball);
    print_spell_info(&heal);
    print_spell_info(&shield);
    
    println!("✅ Veri içeren enum'lar tamamlandı!");
}

// ADIM 3: Match Control Flow
fn step3_match_control_flow() {
    println!("\n🎯 ADIM 3: Match Control Flow");
    println!("{}", "-".repeat(40));
    
    let warrior_sword = Weapon::Sword { damage: 30, durability: 95 };
    let mage_staff = Weapon::Staff { magic_power: 40, mana_cost: 15 };
    
    // Match ile saldırı hesaplama
    let attack1 = calculate_attack(&CharacterClass::Warrior, &warrior_sword);
    let attack2 = calculate_attack(&CharacterClass::Mage, &mage_staff);
    
    println!("⚔️ Saldırı Sonuçları:");
    match attack1 {
        AttackResult::Success { damage, critical } => {
            println!("   Warrior saldırısı: {} hasar{}", damage, if critical { " (Critical!)" } else { "" });
        }
        AttackResult::Miss => println!("   Warrior saldırısı kaçtı!"),
        AttackResult::Blocked { absorbed } => println!("   Warrior saldırısı bloklandı, {} hasar emildi", absorbed),
        AttackResult::WeaponBroken => println!("   Warrior'ın silahı kırıldı!"),
        AttackResult::OutOfAmmo => println!("   Warrior'ın mühimmatı bitti!"),
    }
    
    match attack2 {
        AttackResult::Success { damage, critical } => {
            println!("   Mage saldırısı: {} hasar{}", damage, if critical { " (Critical!)" } else { "" });
        }
        AttackResult::Miss => println!("   Mage saldırısı kaçtı!"),
        AttackResult::Blocked { absorbed } => println!("   Mage saldırısı bloklandı, {} hasar emildi", absorbed),
        AttackResult::WeaponBroken => println!("   Mage'in silahı kırıldı!"),
        AttackResult::OutOfAmmo => println!("   Mage'in mühimmatı bitti!"),
    }
    
    // Element etkileşimleri
    println!("\n🌪️ Element Etkileşimleri:");
    test_element_interaction(Element::Fire, Element::Water);
    test_element_interaction(Element::Earth, Element::Air);
    
    println!("✅ Match control flow tamamlandı!");
}

// ADIM 4: Option ve Result
fn step4_option_and_result() {
    println!("\n📋 ADIM 4: Option ve Result Handling");
    println!("{}", "-".repeat(40));
    
    let mut inventory = Inventory::new();
    
    // Item ekleme (Result döndürür)
    match inventory.add_item(Item::Weapon(Weapon::Sword { damage: 25, durability: 100 })) {
        Ok(()) => println!("✅ Silah envatere eklendi"),
        Err(msg) => println!("❌ Hata: {}", msg),
    }
    
    match inventory.add_item(Item::Potion { health: 50 }) {
        Ok(()) => println!("✅ İksir envatere eklendi"),
        Err(msg) => println!("❌ Hata: {}", msg),
    }
    
    // Item arama (Option döndürür)
    match inventory.find_weapon() {
        Some(weapon) => {
            println!("🗡️ Silah bulundu:");
            print_weapon_info(weapon);
        }
        None => println!("🔍 Silah bulunamadı"),
    }
    
    // Inventory durumu
    println!("\n📦 Envanter Durumu:");
    println!("   Toplam item: {}", inventory.item_count());
    println!("   Kapasite: {}/{}", inventory.item_count(), 10);
    
    println!("✅ Option ve Result handling tamamlandı!");
}

// ADIM 5: if let ve let else
fn step5_if_let_and_let_else() {
    println!("\n🔀 ADIM 5: if let ve let else");
    println!("{}", "-".repeat(40));
    
    let inventory = create_sample_inventory();
    
    // if let ile kısa pattern matching
    if let Some(weapon) = inventory.find_weapon() {
        println!("🗡️ Silah bulundu (if let):");
        print_weapon_info(weapon);
    } else {
        println!("🔍 Silah bulunamadı");
    }
    
    // let else pattern (modern Rust)
    let weapon = inventory.find_weapon().unwrap_or(&Weapon::Sword { damage: 10, durability: 50 });
    println!("\n⚔️ Kullanılacak silah:");
    print_weapon_info(weapon);
    
    // Multiple if let chains
    let spell = Spell::Fireball { damage: 35, radius: 4.0 };
    
    if let Spell::Fireball { damage, radius } = spell {
        if damage > 30 {
            println!("🔥 Güçlü ateş büyüsü! Hasar: {}, Alan: {}", damage, radius);
        } else {
            println!("🔥 Normal ateş büyüsü");
        }
    }
    
    // Daha fazla büyü türü testi
    let heal_spell = Spell::Heal { amount: 50 };
    if let Spell::Heal { amount } = heal_spell {
        println!("💚 İyileştirme büyüsü: +{} can", amount);
    }
    
    let lightning = Spell::LightningBolt { damage: 45, chain_targets: 3 };
    if let Spell::LightningBolt { damage, chain_targets } = lightning {
        println!("⚡ Şimşek büyüsü: {} hasar, {} hedefe zıplar", damage, chain_targets);
    }
    
    println!("✅ if let ve let else tamamlandı!");
}

// ADIM 6: Advanced Pattern Matching
fn step6_advanced_patterns() {
    println!("\n🎯 ADIM 6: Advanced Pattern Matching");
    println!("{}", "-".repeat(40));
    
    let weapons = vec![
        Weapon::Sword { damage: 25, durability: 100 },
        Weapon::Bow { damage: 20, arrows: 0 },
        Weapon::Staff { magic_power: 35, mana_cost: 12 },
        Weapon::Sword { damage: 40, durability: 25 },
        Weapon::Dagger { damage: 18, critical_chance: 0.3 },
        Weapon::Axe { damage: 35, armor_penetration: 15 },
    ];
    
    println!("🔍 Silah Analizi:");
    for (i, weapon) in weapons.iter().enumerate() {
        print!("{}. ", i + 1);
        
        // Advanced pattern matching with guards
        match weapon {
            Weapon::Sword { damage, durability } if *durability < 30 => {
                println!("⚠️ Hasar görmüş kılıç (hasar: {}, dayanıklılık: {})", damage, durability);
            }
            Weapon::Sword { damage, durability } if *damage > 35 => {
                println!("💎 Efsanevi kılıç (hasar: {}, dayanıklılık: {})", damage, durability);
            }
            Weapon::Sword { damage, durability } => {
                println!("🗡️ Normal kılıç (hasar: {}, dayanıklılık: {})", damage, durability);
            }
            Weapon::Bow { damage: _, arrows: 0 } => {
                println!("🏹 Ok bitti! Yay kullanılamaz");
            }
            Weapon::Bow { damage, arrows } => {
                println!("🏹 Yay (hasar: {}, ok: {})", damage, arrows);
            }
            Weapon::Staff { magic_power, mana_cost } if *magic_power > 30 => {
                println!("✨ Güçlü asa (büyü gücü: {}, mana: {})", magic_power, mana_cost);
            }
            Weapon::Staff { magic_power, mana_cost } => {
                println!("🪄 Asa (büyü gücü: {}, mana: {})", magic_power, mana_cost);
            }
            Weapon::Dagger { damage, critical_chance } => {
                println!("🗡️ Hançer (hasar: {}, kritik: {:.0}%)", damage, critical_chance * 100.0);
            }
            Weapon::Axe { damage, armor_penetration } => {
                println!("🪓 Balta (hasar: {}, zırh delme: {})", damage, armor_penetration);
            }
        }
    }
    
    println!("✅ Advanced pattern matching tamamlandı!");
}

// ADIM 7: Gerçek Oyun Simülasyonu
fn step7_game_simulation() {
    println!("\n🎮 ADIM 7: Oyun Simülasyonu");
    println!("{}", "-".repeat(40));
    
    let mut game = GameEngine::new();
    
    // Karakterler oluştur
    game.create_character("Aragorn".to_string(), CharacterClass::Warrior);
    game.create_character("Gandalf".to_string(), CharacterClass::Mage);
    
    // Ekipman ver
    let _ = game.equip_weapon("Aragorn", Weapon::Sword { damage: 35, durability: 100 });
    let _ = game.equip_weapon("Gandalf", Weapon::Staff { magic_power: 45, mana_cost: 15 });
    
    // Savaş simülasyonu
    println!("⚔️ SAVAŞ BAŞLIYOR!");
    println!("{}", "-".repeat(20));
    
    match game.battle("Aragorn", "Gandalf") {
        Ok(winner) => println!("🏆 Kazanan: {}", winner),
        Err(error) => println!("❌ Savaş hatası: {}", error),
    }
    
    println!("✅ Oyun simülasyonu tamamlandı!");
}

// Yardımcı fonksiyonlar
fn print_character_class(class: &CharacterClass) {
    let (name, stats) = match class {
        CharacterClass::Warrior => ("Savaşçı", "Güç: 8, Çeviklik: 4, Zeka: 3"),
        CharacterClass::Mage => ("Büyücü", "Güç: 3, Çeviklik: 4, Zeka: 8"),
        CharacterClass::Archer => ("Okçu", "Güç: 5, Çeviklik: 8, Zeka: 4"),
        CharacterClass::Rogue => ("Hırsız", "Güç: 4, Çeviklik: 8, Zeka: 5"),
    };
    println!("   {} - {}", name, stats);
}

fn get_element_name(element: &Element) -> &str {
    match element {
        Element::Fire => "🔥 Ateş",
        Element::Water => "💧 Su",
        Element::Earth => "🌍 Toprak",
        Element::Air => "💨 Hava",
    }
}

fn get_element_power(element: &Element) -> u32 {
    match element {
        Element::Fire => 25,
        Element::Water => 20,
        Element::Earth => 30,
        Element::Air => 15,
    }
}

fn print_weapon_info(weapon: &Weapon) {
    match weapon {
        Weapon::Sword { damage, durability } => {
            println!("   🗡️ Kılıç - Hasar: {}, Dayanıklılık: {}", damage, durability);
        }
        Weapon::Bow { damage, arrows } => {
            println!("   🏹 Yay - Hasar: {}, Ok Sayısı: {}", damage, arrows);
        }
        Weapon::Staff { magic_power, mana_cost } => {
            println!("   🪄 Asa - Büyü Gücü: {}, Mana Maliyeti: {}", magic_power, mana_cost);
        }
        Weapon::Dagger { damage, critical_chance } => {
            println!("   🗡️ Hançer - Hasar: {}, Kritik: {:.0}%", damage, critical_chance * 100.0);
        }
        Weapon::Axe { damage, armor_penetration } => {
            println!("   🪓 Balta - Hasar: {}, Zırh Delme: {}", damage, armor_penetration);
        }
    }
}

fn print_spell_info(spell: &Spell) {
    match spell {
        Spell::Fireball { damage, radius } => {
            println!("   🔥 Ateş Topu - Hasar: {}, Alan: {}", damage, radius);
        }
        Spell::LightningBolt { damage, chain_targets } => {
            println!("   ⚡ Şimşek - Hasar: {}, Zincir: {} hedef", damage, chain_targets);
        }
        Spell::IceSpear { damage, freeze_chance } => {
            println!("   ❄️ Buz Mızrağı - Hasar: {}, Donma: {:.0}%", damage, freeze_chance * 100.0);
        }
        Spell::Heal { amount } => {
            println!("   💚 İyileştirme - Miktar: {}", amount);
        }
        Spell::Shield { defense_bonus, duration } => {
            println!("   🛡️ Kalkan - Savunma: {}, Süre: {}", defense_bonus, duration);
        }
        Spell::Haste { speed_bonus, duration } => {
            println!("   💨 Hızlandırma - Hız: {}, Süre: {}", speed_bonus, duration);
        }
        Spell::ElementalBlast { element, damage, .. } => {
            println!("   💥 Element Patlaması ({}) - Hasar: {}", get_element_name(element), damage);
        }
        Spell::Meteor { damage, area, delay } => {
            println!("   ☄️ Meteor - Hasar: {}, Alan: {}, Gecikme: {}", damage, area, delay);
        }
        Spell::Blizzard { damage_per_second, area, duration } => {
            println!("   🌨️ Kar Fırtınası - {}/sn hasar, Alan: {}, Süre: {}", 
                damage_per_second, area, duration);
        }
    }
}

fn calculate_attack(class: &CharacterClass, weapon: &Weapon) -> AttackResult {
    let base_damage = match weapon {
        Weapon::Sword { damage, .. } => *damage,
        Weapon::Bow { damage, .. } => *damage,
        Weapon::Staff { magic_power, .. } => *magic_power,
        Weapon::Dagger { damage, .. } => *damage,
        Weapon::Axe { damage, .. } => *damage,
    };
    
    let class_bonus = match class {
        CharacterClass::Warrior => 5,
        CharacterClass::Mage => 3,
        CharacterClass::Archer => 4,
        CharacterClass::Rogue => 6,
    };
    
    // Basit random olmadan sabit sonuç
    let total_damage = base_damage + class_bonus;
    
    if total_damage > 40 {
        AttackResult::Success {
            damage: total_damage + 10,
            critical: true,
        }
    } else {
        AttackResult::Success {
            damage: total_damage,
            critical: false,
        }
    }
}

fn test_element_interaction(elem1: Element, elem2: Element) {
    let result = match (elem1, elem2) {
        (Element::Fire, Element::Water) | (Element::Water, Element::Fire) => "Su ateşi söndürür",
        (Element::Earth, Element::Air) | (Element::Air, Element::Earth) => "Rüzgar toprağı dağıtır",
        (Element::Fire, Element::Earth) => "Ateş toprağı eritir",
        (Element::Water, Element::Air) => "Su havayı nemlendirir",
        _ => "Nötr etkileşim",
    };
    
    println!("   {} vs {} → {}", get_element_name(&elem1), get_element_name(&elem2), result);
}

fn create_sample_inventory() -> Inventory {
    let mut inventory = Inventory::new();
    let _ = inventory.add_item(Item::Weapon(Weapon::Sword { damage: 30, durability: 80 }));
    let _ = inventory.add_item(Item::Potion { health: 75 });
    inventory
}
