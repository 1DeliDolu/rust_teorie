// 🗡️ Weapons - Silah Sistemi
// Farklı silah türleri ve saldırı sonuçları

// Silah enum'ı - veri içeren variants
#[derive(Debug, Clone, PartialEq)]
pub enum Weapon {
    Sword { 
        damage: u32, 
        durability: u32 
    },
    Bow { 
        damage: u32, 
        arrows: u32 
    },
    Staff { 
        magic_power: u32, 
        mana_cost: u32 
    },
    Dagger { 
        damage: u32, 
        critical_chance: f32 
    },
    Axe { 
        damage: u32, 
        armor_penetration: u32 
    },
}

impl Weapon {
    // Method - Silah hasarı
    pub fn base_damage(&self) -> u32 {
        match self {
            Weapon::Sword { damage, .. } => *damage,
            Weapon::Bow { damage, .. } => *damage,
            Weapon::Staff { magic_power, .. } => *magic_power,
            Weapon::Dagger { damage, .. } => *damage,
            Weapon::Axe { damage, .. } => *damage,
        }
    }
    
    // Method - Silah türü
    pub fn weapon_type(&self) -> &str {
        match self {
            Weapon::Sword { .. } => "Kılıç",
            Weapon::Bow { .. } => "Yay",
            Weapon::Staff { .. } => "Asa",
            Weapon::Dagger { .. } => "Hançer",
            Weapon::Axe { .. } => "Balta",
        }
    }
    
    // Method - Kullanılabilir mi?
    pub fn is_usable(&self) -> bool {
        match self {
            Weapon::Sword { durability, .. } => *durability > 0,
            Weapon::Bow { arrows, .. } => *arrows > 0,
            Weapon::Staff { .. } => true, // Asa her zaman kullanılabilir
            Weapon::Dagger { .. } => true,
            Weapon::Axe { .. } => true,
        }
    }
    
    // Method - Silah durumunu güncelle
    pub fn use_weapon(&mut self) -> Result<(), String> {
        match self {
            Weapon::Sword { durability, .. } => {
                if *durability > 0 {
                    *durability -= 1;
                    Ok(())
                } else {
                    Err("Kılıç kırık!".to_string())
                }
            }
            Weapon::Bow { arrows, .. } => {
                if *arrows > 0 {
                    *arrows -= 1;
                    Ok(())
                } else {
                    Err("Ok yok!".to_string())
                }
            }
            Weapon::Staff { .. } => Ok(()), // Asa aşınmaz
            Weapon::Dagger { .. } => Ok(()),
            Weapon::Axe { .. } => Ok(()),
        }
    }
    
    // Associated function - Örneklem silahlar
    pub fn legendary_sword() -> Weapon {
        Weapon::Sword { damage: 50, durability: 200 }
    }
    
    pub fn magic_staff() -> Weapon {
        Weapon::Staff { magic_power: 60, mana_cost: 20 }
    }
    
    pub fn elven_bow() -> Weapon {
        Weapon::Bow { damage: 40, arrows: 100 }
    }
}

// Saldırı sonucu enum'ı
#[derive(Debug, Clone, PartialEq)]
pub enum AttackResult {
    Success { 
        damage: u32, 
        critical: bool 
    },
    Miss,
    Blocked { 
        absorbed: u32 
    },
    WeaponBroken,
    OutOfAmmo,
}

impl AttackResult {
    // Method - Saldırı başarılı mı?
    pub fn is_successful(&self) -> bool {
        matches!(self, AttackResult::Success { .. })
    }
    
    // Method - Verilen hasar
    pub fn damage_dealt(&self) -> u32 {
        match self {
            AttackResult::Success { damage, .. } => *damage,
            AttackResult::Blocked { absorbed } => {
                // Bloklandığında bazı hasar geçebilir
                if *absorbed > 5 { 0 } else { 5 - absorbed }
            }
            _ => 0,
        }
    }
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Weapon::Sword { damage, durability } => {
                write!(f, "🗡️ Kılıç (Hasar: {}, Dayanıklılık: {})", damage, durability)
            }
            Weapon::Bow { damage, arrows } => {
                write!(f, "🏹 Yay (Hasar: {}, Ok: {})", damage, arrows)
            }
            Weapon::Staff { magic_power, mana_cost } => {
                write!(f, "🪄 Asa (Büyü Gücü: {}, Mana: {})", magic_power, mana_cost)
            }
            Weapon::Dagger { damage, critical_chance } => {
                write!(f, "🗡️ Hançer (Hasar: {}, Kritik: {:.0}%)", damage, critical_chance * 100.0)
            }
            Weapon::Axe { damage, armor_penetration } => {
                write!(f, "🪓 Balta (Hasar: {}, Zırh Delme: {})", damage, armor_penetration)
            }
        }
    }
}

impl std::fmt::Display for AttackResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttackResult::Success { damage, critical } => {
                if *critical {
                    write!(f, "💥 Kritik Vuruş! {} hasar", damage)
                } else {
                    write!(f, "⚔️ {} hasar", damage)
                }
            }
            AttackResult::Miss => write!(f, "💨 Kaçtı!"),
            AttackResult::Blocked { absorbed } => write!(f, "🛡️ Bloklandı ({} hasar emildi)", absorbed),
            AttackResult::WeaponBroken => write!(f, "💔 Silah kırıldı!"),
            AttackResult::OutOfAmmo => write!(f, "🚫 Mühimmat bitti!"),
        }
    }
}