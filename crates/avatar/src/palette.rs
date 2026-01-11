//! Color palettes for monster elements and rarities

use qrmonsters_core::{Element, Rarity};

/// Color scheme for a monster based on its element
#[derive(Debug, Clone, Copy)]
pub struct ColorScheme {
    /// Main body/primary color
    pub primary: &'static str,
    /// Accent/secondary color
    pub secondary: &'static str,
    /// Highlight/glow color
    pub highlight: &'static str,
    /// Dark shade for shadows
    pub shadow: &'static str,
}

impl From<Element> for ColorScheme {
    fn from(element: Element) -> Self {
        match element {
            Element::Fire => ColorScheme {
                primary: "#FF6B35",
                secondary: "#FFD166",
                highlight: "#FF4500",
                shadow: "#8B2500",
            },
            Element::Water => ColorScheme {
                primary: "#4ECDC4",
                secondary: "#45B7D1",
                highlight: "#00CED1",
                shadow: "#1A5F7A",
            },
            Element::Earth => ColorScheme {
                primary: "#8B7355",
                secondary: "#A0522D",
                highlight: "#DEB887",
                shadow: "#3D2914",
            },
            Element::Air => ColorScheme {
                primary: "#E8E8E8",
                secondary: "#B8C5D6",
                highlight: "#FFFFFF",
                shadow: "#6B7B8C",
            },
            Element::Electric => ColorScheme {
                primary: "#FFD700",
                secondary: "#FFA500",
                highlight: "#FFFF00",
                shadow: "#8B6914",
            },
        }
    }
}

/// Get glow effect parameters based on rarity
#[derive(Debug, Clone, Copy)]
pub struct RarityGlow {
    /// Whether to show glow effect
    pub enabled: bool,
    /// Glow color
    pub color: &'static str,
    /// Glow intensity (blur radius)
    pub intensity: u8,
    /// Number of particles/sparkles
    pub particles: u8,
}

impl From<Rarity> for RarityGlow {
    fn from(rarity: Rarity) -> Self {
        match rarity {
            Rarity::Common => RarityGlow {
                enabled: false,
                color: "transparent",
                intensity: 0,
                particles: 0,
            },
            Rarity::Rare => RarityGlow {
                enabled: true,
                color: "#4A90D9",
                intensity: 4,
                particles: 2,
            },
            Rarity::Epic => RarityGlow {
                enabled: true,
                color: "#9B59B6",
                intensity: 6,
                particles: 4,
            },
            Rarity::Legendary => RarityGlow {
                enabled: true,
                color: "#F1C40F",
                intensity: 10,
                particles: 8,
            },
        }
    }
}

/// Get CSS class name for rarity styling
pub fn rarity_class(rarity: Rarity) -> &'static str {
    match rarity {
        Rarity::Common => "rarity-common",
        Rarity::Rare => "rarity-rare",
        Rarity::Epic => "rarity-epic",
        Rarity::Legendary => "rarity-legendary",
    }
}

/// Get display color for rarity badge
pub fn rarity_color(rarity: Rarity) -> &'static str {
    match rarity {
        Rarity::Common => "#9E9E9E",
        Rarity::Rare => "#4A90D9",
        Rarity::Epic => "#9B59B6",
        Rarity::Legendary => "#F1C40F",
    }
}
