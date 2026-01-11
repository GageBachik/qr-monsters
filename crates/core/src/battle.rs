use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

use crate::{Element, Monster};

#[derive(Debug, Clone)]
pub struct BattleLine(pub String);

#[derive(Debug, Clone)]
pub struct BattleResult {
    pub winner: Uuid,
    pub turns: u32,
    pub log: Vec<BattleLine>,
    pub a_remaining_hp: i16,
    pub b_remaining_hp: i16,
}

pub fn battle(a: &Monster, b: &Monster) -> BattleResult {
    // deterministic seed derived from ids only (replayable)
    let seed = battle_seed(a.id, b.id);
    let mut rng = ChaCha8Rng::from_seed(seed);

    let mut a_hp = a.stats.hp;
    let mut b_hp = b.stats.hp;

    let mut log = Vec::new();
    let mut turn: u32 = 0;

    let a_first = a.stats.spd >= b.stats.spd;

    while a_hp > 0 && b_hp > 0 && turn < 50 {
        turn += 1;

        if a_first {
            step(&mut rng, a, b, &mut a_hp, &mut b_hp, &mut log, turn);
            if b_hp <= 0 {
                break;
            }
            step(&mut rng, b, a, &mut b_hp, &mut a_hp, &mut log, turn);
        } else {
            step(&mut rng, b, a, &mut b_hp, &mut a_hp, &mut log, turn);
            if a_hp <= 0 {
                break;
            }
            step(&mut rng, a, b, &mut a_hp, &mut b_hp, &mut log, turn);
        }
    }

    let winner = if a_hp == b_hp {
        // tie-breaker: higher SPD then ATK
        if (a.stats.spd, a.stats.atk) >= (b.stats.spd, b.stats.atk) {
            a.id
        } else {
            b.id
        }
    } else if a_hp > b_hp {
        a.id
    } else {
        b.id
    };

    BattleResult {
        winner,
        turns: turn,
        log,
        a_remaining_hp: a_hp.max(0),
        b_remaining_hp: b_hp.max(0),
    }
}

fn step(
    rng: &mut ChaCha8Rng,
    atk: &Monster,
    def: &Monster,
    atk_hp: &mut i16,
    def_hp: &mut i16,
    log: &mut Vec<BattleLine>,
    turn: u32,
) {
    let adv = element_advantage(atk.element, def.element);
    let hit = d20(rng) + atk.stats.atk + atk.stats.luck / 2 + adv;
    let block = d20(rng) + def.stats.def + def.stats.luck / 2;

    let mut dmg = (hit - block).max(1);
    let crit_roll: i16 = rng.gen_range(0..100);
    let is_crit = crit_roll < atk.stats.crit;

    if is_crit {
        dmg *= 2;
    }

    *def_hp -= dmg;

    log.push(BattleLine(format!(
        "T{turn}: {} hits {} for {dmg}{} ({}→{})",
        atk.name,
        def.name,
        if is_crit { " CRIT" } else { "" },
        (*def_hp + dmg).max(0),
        (*def_hp).max(0),
    )));

    // (atk_hp unused now, but keeps signature symmetric for future)
    let _ = atk_hp;
}

fn d20(rng: &mut ChaCha8Rng) -> i16 {
    rng.gen_range(1..=20)
}

// simple advantage table: Fire>Earth, Earth>Electric, Electric>Water, Water>Fire, Air neutral
fn element_advantage(a: Element, b: Element) -> i16 {
    use Element::*;
    match (a, b) {
        (Fire, Earth) => 2,
        (Earth, Electric) => 2,
        (Electric, Water) => 2,
        (Water, Fire) => 2,
        (_, Air) | (Air, _) => 0,
        _ => 0,
    }
}

/// Generate a battle seed that includes the current date.
/// This makes battles deterministic for the same day but different across days.
fn battle_seed(a: Uuid, b: Uuid) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();

    // Add current day (days since Unix epoch) to make battles change daily
    let today = current_day();
    hasher.update(&today.to_le_bytes());

    // Sort IDs so battle(a,b) == battle(b,a) for same-day consistency
    let (id1, id2) = if a < b { (a, b) } else { (b, a) };
    hasher.update(id1.as_bytes());
    hasher.update(id2.as_bytes());

    *hasher.finalize().as_bytes()
}

/// Get the current day as days since Unix epoch.
/// Works on both wasm and native targets.
fn current_day() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        // In wasm, use js_sys to get current time
        let millis = js_sys::Date::now();
        let secs = (millis / 1000.0) as u64;
        (secs / 86400) as u32
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        (secs / 86400) as u32
    }
}
