//! Procedural SVG monster avatar generation

use qrmonsters_core::{Archetype, Monster};
use crate::palette::{ColorScheme, RarityGlow};

/// Generate a deterministic SVG avatar for a monster
///
/// The avatar is generated based on:
/// - Monster ID (seed for random variations)
/// - Archetype (base shape)
/// - Element (colors)
/// - Rarity (glow effects)
pub fn monster_svg(monster: &Monster, size: u32) -> String {
    let colors = ColorScheme::from(monster.element);
    let glow = RarityGlow::from(monster.rarity);

    // Use monster ID bytes as seed for deterministic variations
    let seed_bytes = monster.id.as_bytes();
    let var1 = seed_bytes[0] as f32 / 255.0; // 0.0-1.0
    let var2 = seed_bytes[1] as f32 / 255.0;
    let var3 = seed_bytes[2] as f32 / 255.0;

    // Generate shape based on archetype
    let body = match monster.archetype {
        Archetype::Tank => generate_tank_body(size, var1, var2, &colors),
        Archetype::Assassin => generate_assassin_body(size, var1, var2, &colors),
        Archetype::Mage => generate_mage_body(size, var1, var2, &colors),
        Archetype::Beast => generate_beast_body(size, var1, var2, &colors),
    };

    // Generate eyes
    let eyes = generate_eyes(size, var1, var3, &colors);

    // Generate optional decorations based on rarity
    let decorations = if glow.enabled {
        generate_particles(size, glow.particles, glow.color, var1, var2)
    } else {
        String::new()
    };

    // Build glow filter if needed
    let filter_def = if glow.enabled {
        format!(
            r#"<defs>
                <filter id="glow" x="-50%" y="-50%" width="200%" height="200%">
                    <feGaussianBlur stdDeviation="{}" result="blur"/>
                    <feMerge>
                        <feMergeNode in="blur"/>
                        <feMergeNode in="SourceGraphic"/>
                    </feMerge>
                </filter>
            </defs>"#,
            glow.intensity
        )
    } else {
        String::new()
    };

    let filter_attr = if glow.enabled {
        r#" filter="url(#glow)""#
    } else {
        ""
    };

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" width="{size}" height="{size}">
            {filter_def}
            <rect width="{size}" height="{size}" fill="transparent"/>
            <g{filter_attr}>
                {body}
                {eyes}
            </g>
            {decorations}
        </svg>"#
    )
}

fn generate_tank_body(size: u32, var1: f32, var2: f32, colors: &ColorScheme) -> String {
    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    let base_r = size as f32 * 0.35;

    // Tank: Bulky rounded rectangle with thick limbs
    let width = base_r * 2.0 * (0.9 + var1 * 0.2);
    let height = base_r * 1.8 * (0.9 + var2 * 0.2);
    let corner = base_r * 0.3;

    format!(
        r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="{corner}"
              fill="{primary}" stroke="{shadow}" stroke-width="2"/>
           <rect x="{x2}" y="{y2}" width="{w2}" height="{h2}" rx="{c2}"
              fill="{secondary}"/>"#,
        x = cx - width / 2.0,
        y = cy - height / 2.0,
        width = width,
        height = height,
        corner = corner,
        primary = colors.primary,
        shadow = colors.shadow,
        secondary = colors.secondary,
        x2 = cx - width / 3.0,
        y2 = cy - height / 4.0,
        w2 = width * 0.66,
        h2 = height * 0.4,
        c2 = corner * 0.5,
    )
}

fn generate_assassin_body(size: u32, var1: f32, var2: f32, colors: &ColorScheme) -> String {
    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    let base_r = size as f32 * 0.3;

    // Assassin: Sleek diamond/angular shape
    let stretch = 1.0 + var1 * 0.3;
    let points = format!(
        "{},{} {},{} {},{} {},{}",
        cx, cy - base_r * stretch,           // top
        cx + base_r * 0.8, cy,               // right
        cx, cy + base_r * (0.7 + var2 * 0.3), // bottom
        cx - base_r * 0.8, cy                // left
    );

    format!(
        r#"<polygon points="{points}" fill="{primary}" stroke="{shadow}" stroke-width="2"/>
           <polygon points="{inner}" fill="{secondary}"/>"#,
        points = points,
        primary = colors.primary,
        shadow = colors.shadow,
        secondary = colors.secondary,
        inner = format!(
            "{},{} {},{} {},{} {},{}",
            cx, cy - base_r * 0.5,
            cx + base_r * 0.4, cy,
            cx, cy + base_r * 0.3,
            cx - base_r * 0.4, cy
        ),
    )
}

fn generate_mage_body(size: u32, var1: f32, var2: f32, colors: &ColorScheme) -> String {
    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    let base_r = size as f32 * 0.28;

    // Mage: Floating orb with mystical aura
    let orb_r = base_r * (0.8 + var1 * 0.4);
    let aura_r = orb_r * 1.3;

    format!(
        r#"<circle cx="{cx}" cy="{cy}" r="{aura_r}" fill="{highlight}" opacity="0.3"/>
           <circle cx="{cx}" cy="{cy}" r="{orb_r}" fill="{primary}" stroke="{shadow}" stroke-width="2"/>
           <ellipse cx="{cx}" cy="{shine_y}" rx="{shine_rx}" ry="{shine_ry}" fill="{secondary}" opacity="0.6"/>"#,
        cx = cx,
        cy = cy,
        aura_r = aura_r,
        orb_r = orb_r,
        primary = colors.primary,
        secondary = colors.secondary,
        shadow = colors.shadow,
        highlight = colors.highlight,
        shine_y = cy - orb_r * 0.3,
        shine_rx = orb_r * 0.4 * (0.8 + var2 * 0.4),
        shine_ry = orb_r * 0.25,
    )
}

fn generate_beast_body(size: u32, var1: f32, var2: f32, colors: &ColorScheme) -> String {
    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    let base_r = size as f32 * 0.32;

    // Beast: Organic blob with ears/horns
    let body_rx = base_r * (0.9 + var1 * 0.3);
    let body_ry = base_r * (0.8 + var2 * 0.2);

    // Ears/horns positions
    let ear_size = base_r * 0.35;
    let ear_offset = body_rx * 0.6;

    format!(
        r#"<ellipse cx="{ear_lx}" cy="{ear_y}" rx="{ear_size}" ry="{ear_size}" fill="{secondary}"/>
           <ellipse cx="{ear_rx}" cy="{ear_y}" rx="{ear_size}" ry="{ear_size}" fill="{secondary}"/>
           <ellipse cx="{cx}" cy="{cy}" rx="{body_rx}" ry="{body_ry}"
              fill="{primary}" stroke="{shadow}" stroke-width="2"/>"#,
        cx = cx,
        cy = cy,
        body_rx = body_rx,
        body_ry = body_ry,
        ear_lx = cx - ear_offset,
        ear_rx = cx + ear_offset,
        ear_y = cy - body_ry * 0.7,
        ear_size = ear_size,
        primary = colors.primary,
        secondary = colors.secondary,
        shadow = colors.shadow,
    )
}

fn generate_eyes(size: u32, var1: f32, var2: f32, _colors: &ColorScheme) -> String {
    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    let eye_r = size as f32 * 0.06;
    let eye_spacing = size as f32 * 0.15 * (0.8 + var1 * 0.4);
    let eye_y = cy - size as f32 * 0.02 * (1.0 + var2);
    let left_x = cx - eye_spacing;
    let right_x = cx + eye_spacing;
    let pupil_r = eye_r * 0.5;

    format!(
        r##"<circle cx="{left_x}" cy="{eye_y}" r="{eye_r}" fill="#FFFFFF"/>
           <circle cx="{right_x}" cy="{eye_y}" r="{eye_r}" fill="#FFFFFF"/>
           <circle cx="{left_x}" cy="{eye_y}" r="{pupil_r}" fill="#1a1a2e"/>
           <circle cx="{right_x}" cy="{eye_y}" r="{pupil_r}" fill="#1a1a2e"/>"##
    )
}

fn generate_particles(size: u32, count: u8, color: &str, var1: f32, var2: f32) -> String {
    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    let radius = size as f32 * 0.45;

    let mut particles = String::new();

    for i in 0..count {
        let angle = (i as f32 / count as f32) * std::f32::consts::TAU + var1 * 0.5;
        let dist = radius * (0.7 + var2 * 0.3);
        let px = cx + angle.cos() * dist;
        let py = cy + angle.sin() * dist;
        let particle_r = size as f32 * 0.02 * (0.5 + var1 * 0.5);

        particles.push_str(&format!(
            r#"<circle cx="{px}" cy="{py}" r="{particle_r}" fill="{color}" opacity="0.8">
                <animate attributeName="opacity" values="0.8;0.3;0.8" dur="2s" repeatCount="indefinite" begin="{begin}s"/>
            </circle>"#,
            px = px,
            py = py,
            particle_r = particle_r,
            color = color,
            begin = i as f32 * 0.2,
        ));
    }

    particles
}

#[cfg(test)]
mod tests {
    use super::*;
    use qrmonsters_core::generate_monster;

    #[test]
    fn test_monster_svg_generation() {
        let seed = [0u8; 32];
        let monster = generate_monster(seed);
        let svg = monster_svg(&monster, 128);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("circle") || svg.contains("rect") || svg.contains("polygon"));
    }
}
