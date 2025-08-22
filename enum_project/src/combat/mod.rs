// ⚔️ Combat Module - Savaş Sistemi
// Bu modül silahlar, büyüler ve savaş mekanikleri içerir

pub mod weapons;
pub mod spells;

pub use weapons::{Weapon, AttackResult};
pub use spells::Spell;