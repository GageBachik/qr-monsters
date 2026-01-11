//! Procedural monster avatar generation for QR Monsters
//!
//! This crate provides utilities for generating deterministic SVG avatars
//! for monsters based on their attributes.

pub mod palette;
pub mod svg;

pub use palette::{ColorScheme, RarityGlow, rarity_class, rarity_color};
pub use svg::monster_svg;
