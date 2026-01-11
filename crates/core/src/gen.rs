use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

use crate::{Archetype, Element, Monster, Rarity, Stats};

pub fn generate_monster(seed: [u8; 32]) -> Monster {
    let mut rng = ChaCha8Rng::from_seed(seed);

    let rarity = roll_rarity(&mut rng);
    let element = *[
        Element::Fire,
        Element::Water,
        Element::Earth,
        Element::Air,
        Element::Electric,
    ]
    .choose(&mut rng)
    .unwrap();
    let archetype = *[
        Archetype::Tank,
        Archetype::Assassin,
        Archetype::Mage,
        Archetype::Beast,
    ]
    .choose(&mut rng)
    .unwrap();

    let name = format!("{} {}", adj(&mut rng), noun(&mut rng));

    let points = match rarity {
        Rarity::Common => rng.gen_range(28..=40),
        Rarity::Rare => rng.gen_range(38..=52),
        Rarity::Epic => rng.gen_range(48..=64),
        Rarity::Legendary => rng.gen_range(60..=80),
    };

    let mut stats = allocate(points, archetype, &mut rng);

    // element flavor (tiny nudge)
    match element {
        Element::Fire => stats.atk += 2,
        Element::Water => stats.def += 2,
        Element::Earth => stats.hp += 3,
        Element::Air => stats.spd += 2,
        Element::Electric => stats.crit += 1,
    }

    // clamp
    stats.crit = stats.crit.clamp(0, 50);
    stats.luck = stats.luck.clamp(0, 20);

    Monster {
        v: 1,
        id: Uuid::new_v4(),
        name,
        rarity,
        element,
        archetype,
        stats,
    }
}

fn roll_rarity<R: Rng>(rng: &mut R) -> Rarity {
    // weights: 70/20/9/1
    let x = rng.gen_range(0..100);
    match x {
        0..=69 => Rarity::Common,
        70..=89 => Rarity::Rare,
        90..=98 => Rarity::Epic,
        _ => Rarity::Legendary,
    }
}

fn allocate<R: Rng>(points: i16, archetype: Archetype, rng: &mut R) -> Stats {
    // base minimums
    let mut hp = 12;
    let mut atk = 6;
    let mut def = 6;
    let mut spd = 6;
    let mut crit = 3;
    let mut luck = 3;

    let mut remaining = points - (hp + atk + def + spd + crit + luck);

    // archetype bias: choose which stats get more of the remaining
    let bias: &[fn(&mut i16)] = match archetype {
        Archetype::Tank => &[|v| *v += 1, |v| *v += 1, |v| *v += 1, |_v| {}],
        Archetype::Assassin => &[|_v| {}, |v| *v += 1, |_v| {}, |v| *v += 1],
        Archetype::Mage => &[|_v| {}, |v| *v += 1, |_v| {}, |_v| {}],
        Archetype::Beast => &[|v| *v += 1, |v| *v += 1, |_v| {}, |_v| {}],
    };

    while remaining > 0 {
        let roll = rng.gen_range(0..100);
        match roll {
            0..=29 => {
                hp += 1;
            }
            30..=54 => {
                atk += 1;
            }
            55..=74 => {
                def += 1;
            }
            75..=86 => {
                spd += 1;
            }
            87..=94 => {
                crit += 1;
            }
            _ => {
                luck += 1;
            }
        }

        // tiny bias each loop
        let mut knobs = [0i16, 0, 0, 0];
        for f in bias {
            f(&mut knobs[rng.gen_range(0..4)]);
        }
        hp += knobs[0];
        atk += knobs[1];
        def += knobs[2];
        spd += knobs[3];

        remaining -= 1;
    }

    Stats {
        hp,
        atk,
        def,
        spd,
        crit,
        luck,
    }
}

fn adj<R: Rng>(rng: &mut R) -> &'static str {
    *[
        "Blazing", "Mossy", "Glacial", "Vicious", "Tiny", "Ancient", "Stormy", "Cursed", "Shiny",
        "Wicked",
    ]
    .choose(rng)
    .unwrap()
}

fn noun<R: Rng>(rng: &mut R) -> &'static str {
    *[
        "Otter", "Golem", "Wisp", "Crab", "Raptor", "Moth", "Slime", "Wolf", "Sprite", "Toad",
    ]
    .choose(rng)
    .unwrap()
}
