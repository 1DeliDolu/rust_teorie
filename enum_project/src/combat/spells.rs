// 🪄 Spells - Büyü Sistemi
// Farklı büyü türleri ve etkiler

use crate::character::Element;

// Büyü enum'ı - karmaşık veri yapıları
#[derive(Debug, Clone, PartialEq)]
pub enum Spell {
    // Saldırı büyüleri
    Fireball { 
        damage: u32, 
        radius: f32 
    },
    LightningBolt { 
        damage: u32, 
        chain_targets: u32 
    },
    IceSpear { 
        damage: u32, 
        freeze_chance: f32 
    },
    
    // Destek büyüleri
    Heal { 
        amount: u32 
    },
    Shield { 
        defense_bonus: u32, 
        duration: u32 
    },
    Haste { 
        speed_bonus: u32, 
        duration: u32 
    },
    
    // Element büyüleri
    ElementalBlast { 
        element: Element, 
        damage: u32, 
        special_effect: ElementalEffect 
    },
    
    // Area büyüler
    Meteor { 
        damage: u32, 
        area: f32, 
        delay: u32 
    },
    Blizzard { 
        damage_per_second: u32, 
        area: f32, 
        duration: u32 
    },
}

// Element etkiler enum'ı
#[derive(Debug, Clone, PartialEq)]
pub enum ElementalEffect {
    Burn { damage_per_turn: u32, turns: u32 },
    Freeze { slow_percentage: u32, duration: u32 },
    Poison { damage_per_turn: u32, turns: u32 },
    Stun { duration: u32 },
}

impl Spell {
    // Method - Büyü maliyeti
    pub fn mana_cost(&self) -> u32 {
        match self {
            Spell::Fireball { damage, .. } => damage / 2,
            Spell::LightningBolt { damage, chain_targets } => damage / 2 + chain_targets * 5,
            Spell::IceSpear { damage, .. } => damage / 2 + 5,
            
            Spell::Heal { amount } => amount / 3,
            Spell::Shield { defense_bonus, duration } => defense_bonus + duration * 2,
            Spell::Haste { speed_bonus, duration } => speed_bonus + duration * 3,
            
            Spell::ElementalBlast { damage, .. } => damage / 2 + 10,
            
            Spell::Meteor { damage, .. } => damage / 2 + 20,
            Spell::Blizzard { damage_per_second, duration, .. } => {
                damage_per_second * duration + 15
            }
        }
    }
    
    // Method - Büyü türü
    pub fn spell_type(&self) -> SpellType {
        match self {
            Spell::Fireball { .. } | 
            Spell::LightningBolt { .. } | 
            Spell::IceSpear { .. } |
            Spell::ElementalBlast { .. } => SpellType::Offensive,
            
            Spell::Heal { .. } | 
            Spell::Shield { .. } | 
            Spell::Haste { .. } => SpellType::Support,
            
            Spell::Meteor { .. } | 
            Spell::Blizzard { .. } => SpellType::Area,
        }
    }
    
    // Method - Büyü adı
    pub fn name(&self) -> &str {
        match self {
            Spell::Fireball { .. } => "Ateş Topu",
            Spell::LightningBolt { .. } => "Şimşek",
            Spell::IceSpear { .. } => "Buz Mızrağı",
            Spell::Heal { .. } => "İyileştirme",
            Spell::Shield { .. } => "Kalkan",
            Spell::Haste { .. } => "Hızlandırma",
            Spell::ElementalBlast { element, .. } => {
                match element {
                    Element::Fire => "Ateş Patlaması",
                    Element::Water => "Su Patlaması",
                    Element::Earth => "Toprak Patlaması",
                    Element::Air => "Hava Patlaması",
                }
            },
            Spell::Meteor { .. } => "Meteor",
            Spell::Blizzard { .. } => "Kar Fırtınası",
        }
    }
    
    // Method - Etki alanı var mı?
    pub fn has_area_effect(&self) -> bool {
        match self {
            Spell::Fireball { radius, .. } => *radius > 1.0,
            Spell::Meteor { .. } | 
            Spell::Blizzard { .. } => true,
            _ => false,
        }
    }
    
    // Associated function - Başlangıç büyüleri
    pub fn starter_spells() -> Vec<Spell> {
        vec![
            Spell::Fireball { damage: 20, radius: 2.0 },
            Spell::Heal { amount: 25 },
            Spell::Shield { defense_bonus: 5, duration: 3 },
        ]
    }
    
    // Associated function - Gelişmiş büyüler
    pub fn advanced_spells() -> Vec<Spell> {
        vec![
            Spell::LightningBolt { damage: 35, chain_targets: 2 },
            Spell::ElementalBlast { 
                element: Element::Fire, 
                damage: 40, 
                special_effect: ElementalEffect::Burn { 
                    damage_per_turn: 5, 
                    turns: 3 
                }
            },
            Spell::Meteor { damage: 60, area: 5.0, delay: 2 },
        ]
    }
}

// Büyü türü enum'ı
#[derive(Debug, Clone, PartialEq)]
pub enum SpellType {
    Offensive,  // Saldırı
    Support,    // Destek
    Area,       // Alan etkili
}

impl ElementalEffect {
    // Method - Etkinin açıklaması
    pub fn description(&self) -> String {
        match self {
            ElementalEffect::Burn { damage_per_turn, turns } => {
                format!("🔥 Yanma: Tur başına {} hasar, {} tur", damage_per_turn, turns)
            }
            ElementalEffect::Freeze { slow_percentage, duration } => {
                format!("❄️ Donma: %{} yavaşlatma, {} tur", slow_percentage, duration)
            }
            ElementalEffect::Poison { damage_per_turn, turns } => {
                format!("☠️ Zehir: Tur başına {} hasar, {} tur", damage_per_turn, turns)
            }
            ElementalEffect::Stun { duration } => {
                format!("⚡ Sersemletme: {} tur", duration)
            }
        }
    }
}

impl std::fmt::Display for Spell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🪄 {} (Mana: {})", self.name(), self.mana_cost())
    }
}

impl std::fmt::Display for SpellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            SpellType::Offensive => "⚔️ Saldırı",
            SpellType::Support => "🛡️ Destek", 
            SpellType::Area => "💥 Alan",
        };
        write!(f, "{}", name)
    }
}