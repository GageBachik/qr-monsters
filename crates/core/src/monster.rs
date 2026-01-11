use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Element {
    Fire,
    Water,
    Earth,
    Air,
    Electric,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Archetype {
    Tank,
    Assassin,
    Mage,
    Beast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub hp: i16,
    pub atk: i16,
    pub def: i16,
    pub spd: i16,
    pub crit: i16, // percent 0..=50
    pub luck: i16, // 0..=20
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
    pub v: u8, // version for forwards-compat
    pub id: Uuid,
    pub name: String,
    pub rarity: Rarity,
    pub element: Element,
    pub archetype: Archetype,
    pub stats: Stats,
}

impl Monster {
    pub fn validate(&self) -> Result<(), String> {
        let s = &self.stats;
        if !(0..=999).contains(&s.hp) {
            return Err("hp out of range".into());
        }
        if !(0..=999).contains(&s.atk) {
            return Err("atk out of range".into());
        }
        if !(0..=999).contains(&s.def) {
            return Err("def out of range".into());
        }
        if !(0..=999).contains(&s.spd) {
            return Err("spd out of range".into());
        }
        if !(0..=50).contains(&s.crit) {
            return Err("crit out of range".into());
        }
        if !(0..=20).contains(&s.luck) {
            return Err("luck out of range".into());
        }
        Ok(())
    }
}
