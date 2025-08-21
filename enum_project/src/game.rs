// 🎮 Game Engine - Oyun Motoru
// Ana oyun mantığını ve karakter yönetimini içerir

use crate::character::CharacterClass;
use crate::combat::{Weapon, AttackResult};
use crate::items::Inventory;
use std::collections::HashMap;

// Oyuncu karakteri
#[derive(Debug)]
pub struct Player {
    name: String,
    class: CharacterClass,
    level: u32,
    health: u32,
    max_health: u32,
    mana: u32,
    max_mana: u32,
    weapon: Option<Weapon>,
    inventory: Inventory,
}

impl Player {
    // Associated function - Yeni karakter
    pub fn new(name: String, class: CharacterClass) -> Player {
        let (health, mana) = match class {
            CharacterClass::Warrior => (120, 30),
            CharacterClass::Mage => (70, 150),
            CharacterClass::Archer => (90, 60),
            CharacterClass::Rogue => (85, 80),
        };
        
        Player {
            name,
            class,
            level: 1,
            health,
            max_health: health,
            mana,
            max_mana: mana,
            weapon: None,
            inventory: Inventory::new(),
        }
    }
    
    // Method - Silah kuşanma
    pub fn equip_weapon(&mut self, weapon: Weapon) -> Result<(), String> {
        if weapon.is_usable() {
            self.weapon = Some(weapon);
            Ok(())
        } else {
            Err("Silah kullanılamaz durumda!".to_string())
        }
    }
    
    // Method - Saldırı
    pub fn attack(&self) -> AttackResult {
        match &self.weapon {
            Some(weapon) => {
                let base_damage = weapon.base_damage();
                let class_bonus = match self.class {
                    CharacterClass::Warrior => 5,
                    CharacterClass::Mage => 3,
                    CharacterClass::Archer => 4,
                    CharacterClass::Rogue => 6,
                };
                
                AttackResult::Success {
                    damage: base_damage + class_bonus,
                    critical: false,
                }
            }
            None => AttackResult::Miss,
        }
    }
    
    // Method - Hasar alma
    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }
    
    // Method - İyileşme
    pub fn heal(&mut self, amount: u32) {
        self.health = (self.health + amount).min(self.max_health);
    }
    
    // Method - Yaşıyor mu?
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}

// Oyun motoru
pub struct GameEngine {
    players: HashMap<String, Player>,
    current_turn: u32,
}

impl GameEngine {
    // Associated function - Yeni oyun
    pub fn new() -> GameEngine {
        GameEngine {
            players: HashMap::new(),
            current_turn: 0,
        }
    }
    
    // Method - Karakter oluştur
    pub fn create_character(&mut self, name: String, class: CharacterClass) {
        let player = Player::new(name.clone(), class);
        self.players.insert(name, player);
    }
    
    // Method - Silah kuşandır
    pub fn equip_weapon(&mut self, player_name: &str, weapon: Weapon) -> Result<(), String> {
        match self.players.get_mut(player_name) {
            Some(player) => player.equip_weapon(weapon),
            None => Err(format!("Oyuncu '{}' bulunamadı", player_name)),
        }
    }
    
    // Method - Savaş simülasyonu
    pub fn battle(&mut self, player1_name: &str, player2_name: &str) -> Result<String, String> {
        // Oyuncuları kontrol et
        if !self.players.contains_key(player1_name) {
            return Err(format!("Oyuncu '{}' bulunamadı", player1_name));
        }
        if !self.players.contains_key(player2_name) {
            return Err(format!("Oyuncu '{}' bulunamadı", player2_name));
        }
        
        // Basit savaş simülasyonu
        let mut round = 1;
        
        loop {
            println!("🥊 Round {}", round);
            
            // Player 1 saldırısı
            let attack1 = {
                let player1 = self.players.get(player1_name).unwrap();
                player1.attack()
            };
            
            match attack1 {
                AttackResult::Success { damage, critical } => {
                    println!("   {} saldırdı: {} hasar{}", 
                        player1_name, damage, 
                        if critical { " (Kritik!)" } else { "" });
                    
                    let player2 = self.players.get_mut(player2_name).unwrap();
                    player2.take_damage(damage);
                    
                    if !player2.is_alive() {
                        return Ok(player1_name.to_string());
                    }
                }
                AttackResult::Miss => {
                    println!("   {} saldırısı kaçtı!", player1_name);
                }
                _ => {}
            }
            
            // Player 2 saldırısı
            let attack2 = {
                let player2 = self.players.get(player2_name).unwrap();
                player2.attack()
            };
            
            match attack2 {
                AttackResult::Success { damage, critical } => {
                    println!("   {} saldırdı: {} hasar{}", 
                        player2_name, damage,
                        if critical { " (Kritik!)" } else { "" });
                    
                    let player1 = self.players.get_mut(player1_name).unwrap();
                    player1.take_damage(damage);
                    
                    if !player1.is_alive() {
                        return Ok(player2_name.to_string());
                    }
                }
                AttackResult::Miss => {
                    println!("   {} saldırısı kaçtı!", player2_name);
                }
                _ => {}
            }
            
            // Durum kontrolü
            {
                let player1 = self.players.get(player1_name).unwrap();
                let player2 = self.players.get(player2_name).unwrap();
                println!("   💚 {} Can: {}/{}", player1_name, player1.health, player1.max_health);
                println!("   💚 {} Can: {}/{}", player2_name, player2.health, player2.max_health);
            }
            
            round += 1;
            if round > 10 { // Maksimum 10 round
                return Ok("Berabere!".to_string());
            }
            
            println!(); // Boş satır
        }
    }
    
    // Method - Oyuncu bilgisi
    pub fn get_player_info(&self, player_name: &str) -> Option<String> {
        self.players.get(player_name).map(|player| {
            format!(
                "👤 {} ({})\n   Level: {}\n   Can: {}/{}\n   Mana: {}/{}\n   Silah: {}",
                player.name,
                player.class.name(),
                player.level,
                player.health,
                player.max_health,
                player.mana,
                player.max_mana,
                match &player.weapon {
                    Some(weapon) => weapon.weapon_type(),
                    None => "Yok",
                }
            )
        })
    }
}