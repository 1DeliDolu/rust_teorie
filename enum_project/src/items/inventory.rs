// 🎒 Inventory System - Envanter Sistemi
// Option ve Result kullanımı için eşya sistemi

use crate::combat::Weapon;

// Eşya enum'ı
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Weapon(Weapon),
    Potion { health: u32 },
    ManaPotion { mana: u32 },
    Armor { defense: u32, durability: u32 },
    Key { id: String },
    Gold { amount: u32 },
    Gem { value: u32, rarity: GemRarity },
}

// Gem nadir türleri
#[derive(Debug, Clone, PartialEq)]
pub enum GemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Item {
    // Method - Eşya değeri
    pub fn value(&self) -> u32 {
        match self {
            Item::Weapon(weapon) => weapon.base_damage() * 2,
            Item::Potion { health } => *health,
            Item::ManaPotion { mana } => *mana,
            Item::Armor { defense, .. } => *defense * 3,
            Item::Key { .. } => 0, // Anahtar satılmaz
            Item::Gold { amount } => *amount,
            Item::Gem { value, rarity } => {
                let rarity_multiplier = match rarity {
                    GemRarity::Common => 1,
                    GemRarity::Uncommon => 2,
                    GemRarity::Rare => 5,
                    GemRarity::Epic => 10,
                    GemRarity::Legendary => 25,
                };
                value * rarity_multiplier
            }
        }
    }
    
    // Method - Eşya adı
    pub fn name(&self) -> String {
        match self {
            Item::Weapon(weapon) => format!("Silah: {}", weapon.weapon_type()),
            Item::Potion { health } => format!("Sağlık İksiri (+{})", health),
            Item::ManaPotion { mana } => format!("Mana İksiri (+{})", mana),
            Item::Armor { defense, .. } => format!("Zırh (Savunma: {})", defense),
            Item::Key { id } => format!("Anahtar: {}", id),
            Item::Gold { amount } => format!("{} Altın", amount),
            Item::Gem { rarity, .. } => {
                let rarity_name = match rarity {
                    GemRarity::Common => "Sıradan",
                    GemRarity::Uncommon => "Nadir",
                    GemRarity::Rare => "Ender",
                    GemRarity::Epic => "Destansı", 
                    GemRarity::Legendary => "Efsanevi",
                };
                format!("{} Mücevher", rarity_name)
            }
        }
    }
    
    // Method - Kullanılabilir mi?
    pub fn is_consumable(&self) -> bool {
        matches!(self, Item::Potion { .. } | Item::ManaPotion { .. })
    }
}

// Envanter sistemi
#[derive(Debug)]
pub struct Inventory {
    items: Vec<Item>,
    max_capacity: usize,
    gold: u32,
}

impl Inventory {
    // Associated function - Yeni envanter
    pub fn new() -> Inventory {
        Inventory {
            items: Vec::new(),
            max_capacity: 10,
            gold: 0,
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Inventory {
        Inventory {
            items: Vec::new(),
            max_capacity: capacity,
            gold: 0,
        }
    }
    
    // Method - Eşya ekleme (Result döndürür)
    pub fn add_item(&mut self, item: Item) -> Result<(), String> {
        if self.items.len() >= self.max_capacity {
            return Err("Envanter dolu!".to_string());
        }
        
        // Altın otomatik olarak birleşir
        if let Item::Gold { amount } = item {
            self.gold += amount;
            Ok(())
        } else {
            self.items.push(item);
            Ok(())
        }
    }
    
    // Method - Silah arama (Option döndürür)
    pub fn find_weapon(&self) -> Option<&Weapon> {
        for item in &self.items {
            if let Item::Weapon(weapon) = item {
                return Some(weapon);
            }
        }
        None
    }
    
    // Method - Belirli türde eşya arama
    pub fn find_potion(&self) -> Option<&Item> {
        self.items.iter().find(|item| {
            matches!(item, Item::Potion { .. })
        })
    }
    
    // Method - Eşya kullanma/çıkarma
    pub fn use_item(&mut self, item_name: &str) -> Result<Item, String> {
        if let Some(pos) = self.items.iter().position(|item| {
            item.name().contains(item_name)
        }) {
            Ok(self.items.remove(pos))
        } else {
            Err(format!("'{}' bulunamadı", item_name))
        }
    }
    
    // Method - Eşya sayısı
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    
    // Method - Envanter dolu mu?
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.max_capacity
    }
    
    // Method - Toplam değer
    pub fn total_value(&self) -> u32 {
        self.items.iter().map(|item| item.value()).sum::<u32>() + self.gold
    }
    
    // Method - Tür bazında eşya sayısı
    pub fn count_by_type(&self, item_type: &str) -> usize {
        self.items.iter().filter(|item| {
            match (item_type, item) {
                ("weapon", Item::Weapon(_)) => true,
                ("potion", Item::Potion { .. }) => true,
                ("armor", Item::Armor { .. }) => true,
                ("gem", Item::Gem { .. }) => true,
                _ => false,
            }
        }).count()
    }
    
    // Method - Envanter listesi
    pub fn list_items(&self) {
        println!("📦 Envanter İçeriği:");
        println!("💰 Altın: {}", self.gold);
        println!("📋 Eşyalar ({}/{}):", self.items.len(), self.max_capacity);
        
        if self.items.is_empty() {
            println!("   (Boş)");
        } else {
            for (i, item) in self.items.iter().enumerate() {
                println!("   {}. {} (Değer: {})", i + 1, item.name(), item.value());
            }
        }
    }
}

impl std::fmt::Display for GemRarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, symbol) = match self {
            GemRarity::Common => ("Sıradan", "⚪"),
            GemRarity::Uncommon => ("Nadir", "🟢"),
            GemRarity::Rare => ("Ender", "🔵"),
            GemRarity::Epic => ("Destansı", "🟣"),
            GemRarity::Legendary => ("Efsanevi", "🟠"),
        };
        write!(f, "{} {}", symbol, name)
    }
}