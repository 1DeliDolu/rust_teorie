// 🎭 Character Types - Karakter Türleri
// Temel enum tanımları

// Karakter sınıfları enum'ı
#[derive(Debug, Clone, PartialEq)]
pub enum CharacterClass {
    Warrior,   // Savaşçı
    Mage,      // Büyücü  
    Archer,    // Okçu
    Rogue,     // Hırsız
}

impl CharacterClass {
    // Associated function - Karakter stats'ları
    pub fn get_base_stats(&self) -> (u32, u32, u32) { // (strength, agility, intelligence)
        match self {
            CharacterClass::Warrior => (8, 4, 3),
            CharacterClass::Mage => (3, 4, 8),
            CharacterClass::Archer => (5, 8, 4),
            CharacterClass::Rogue => (4, 8, 5),
        }
    }
    
    // Method - Karakter adı
    pub fn name(&self) -> &str {
        match self {
            CharacterClass::Warrior => "Savaşçı",
            CharacterClass::Mage => "Büyücü",
            CharacterClass::Archer => "Okçu", 
            CharacterClass::Rogue => "Hırsız",
        }
    }
    
    // Method - Tercih edilen silah türü
    pub fn preferred_weapon(&self) -> &str {
        match self {
            CharacterClass::Warrior => "Kılıç",
            CharacterClass::Mage => "Asa",
            CharacterClass::Archer => "Yay",
            CharacterClass::Rogue => "Hançer",
        }
    }
}

// Element türleri enum'ı
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Fire,   // Ateş
    Water,  // Su
    Earth,  // Toprak
    Air,    // Hava
}

impl Element {
    // Method - Element gücü
    pub fn power(&self) -> u32 {
        match self {
            Element::Fire => 25,
            Element::Water => 20,
            Element::Earth => 30,
            Element::Air => 15,
        }
    }
    
    // Method - Element zayıflığı
    pub fn weakness(&self) -> Element {
        match self {
            Element::Fire => Element::Water,
            Element::Water => Element::Earth,
            Element::Earth => Element::Air,
            Element::Air => Element::Fire,
        }
    }
    
    // Method - Element güçlülüğü
    pub fn strength(&self) -> Element {
        match self {
            Element::Fire => Element::Air,
            Element::Water => Element::Fire,
            Element::Earth => Element::Water,
            Element::Air => Element::Earth,
        }
    }
}

// Display trait implementations
impl std::fmt::Display for CharacterClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Element::Fire => "🔥 Ateş",
            Element::Water => "💧 Su", 
            Element::Earth => "🌍 Toprak",
            Element::Air => "💨 Hava",
        };
        write!(f, "{}", name)
    }
}