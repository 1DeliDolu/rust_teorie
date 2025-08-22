// ⚡ Character Abilities - Karakter Yetenekleri
// Karakter yetenekleri ve özel saldırıları

use super::types::{CharacterClass, Element};

// Karakter yetenekleri enum'ı
#[derive(Debug, Clone)]
pub enum CharacterAbility {
    // Warrior abilities
    PowerStrike { damage_multiplier: f32 },
    DefensiveStance { defense_bonus: u32, duration: u32 },
    Charge { damage: u32, distance: f32 },
    
    // Mage abilities  
    ElementalSpell { element: Element, damage: u32, area: f32 },
    Teleport { distance: f32, mana_cost: u32 },
    ManaShield { absorption: u32, duration: u32 },
    
    // Archer abilities
    PiercingShot { damage: u32, targets: u32 },
    RapidFire { shots: u32, damage_per_shot: u32 },
    TrapShot { damage: u32, slow_duration: u32 },
    
    // Rogue abilities
    Stealth { duration: u32, damage_bonus: u32 },
    PoisonBlade { poison_damage: u32, turns: u32 },
    BackStab { critical_chance: f32, damage_multiplier: f32 },
    
    // Universal abilities
    Rest { health_restored: u32, mana_restored: u32 },
    Block { damage_reduced: u32 },
}

impl CharacterAbility {
    // Method - Yetenek kullanım maliyeti
    pub fn cost(&self) -> u32 {
        match self {
            CharacterAbility::PowerStrike { .. } => 15,
            CharacterAbility::DefensiveStance { .. } => 20,
            CharacterAbility::Charge { .. } => 25,
            
            CharacterAbility::ElementalSpell { .. } => 30,
            CharacterAbility::Teleport { mana_cost, .. } => *mana_cost,
            CharacterAbility::ManaShield { .. } => 40,
            
            CharacterAbility::PiercingShot { .. } => 20,
            CharacterAbility::RapidFire { .. } => 25,
            CharacterAbility::TrapShot { .. } => 15,
            
            CharacterAbility::Stealth { .. } => 20,
            CharacterAbility::PoisonBlade { .. } => 15,
            CharacterAbility::BackStab { .. } => 30,
            
            CharacterAbility::Rest { .. } => 0,
            CharacterAbility::Block { .. } => 10,
        }
    }
    
    // Method - Yetenek adı
    pub fn name(&self) -> &str {
        match self {
            CharacterAbility::PowerStrike { .. } => "Güçlü Darbe",
            CharacterAbility::DefensiveStance { .. } => "Savunma Duruşu",
            CharacterAbility::Charge { .. } => "Hücum",
            
            CharacterAbility::ElementalSpell { element, .. } => {
                match element {
                    Element::Fire => "Ateş Büyüsü",
                    Element::Water => "Su Büyüsü",
                    Element::Earth => "Toprak Büyüsü", 
                    Element::Air => "Hava Büyüsü",
                }
            },
            CharacterAbility::Teleport { .. } => "Işınlanma",
            CharacterAbility::ManaShield { .. } => "Mana Kalkanı",
            
            CharacterAbility::PiercingShot { .. } => "Delici Atış",
            CharacterAbility::RapidFire { .. } => "Hızlı Atış",
            CharacterAbility::TrapShot { .. } => "Tuzak Atışı",
            
            CharacterAbility::Stealth { .. } => "Gizlenme",
            CharacterAbility::PoisonBlade { .. } => "Zehirli Bıçak",
            CharacterAbility::BackStab { .. } => "Arkadan Saldırı",
            
            CharacterAbility::Rest { .. } => "Dinlenme",
            CharacterAbility::Block { .. } => "Blok",
        }
    }
    
    // Associated function - Sınıfa göre varsayılan yetenekler
    pub fn default_abilities_for_class(class: &CharacterClass) -> Vec<CharacterAbility> {
        match class {
            CharacterClass::Warrior => vec![
                CharacterAbility::PowerStrike { damage_multiplier: 1.5 },
                CharacterAbility::DefensiveStance { defense_bonus: 10, duration: 3 },
                CharacterAbility::Charge { damage: 35, distance: 5.0 },
            ],
            CharacterClass::Mage => vec![
                CharacterAbility::ElementalSpell { 
                    element: Element::Fire, 
                    damage: 40, 
                    area: 3.0 
                },
                CharacterAbility::Teleport { distance: 10.0, mana_cost: 25 },
                CharacterAbility::ManaShield { absorption: 50, duration: 5 },
            ],
            CharacterClass::Archer => vec![
                CharacterAbility::PiercingShot { damage: 30, targets: 2 },
                CharacterAbility::RapidFire { shots: 3, damage_per_shot: 15 },
                CharacterAbility::TrapShot { damage: 25, slow_duration: 2 },
            ],
            CharacterClass::Rogue => vec![
                CharacterAbility::Stealth { duration: 3, damage_bonus: 20 },
                CharacterAbility::PoisonBlade { poison_damage: 5, turns: 3 },
                CharacterAbility::BackStab { critical_chance: 0.75, damage_multiplier: 2.0 },
            ],
        }
    }
}

impl std::fmt::Display for CharacterAbility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Maliyet: {})", self.name(), self.cost())
    }
}