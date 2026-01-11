//! QR code encoding and decoding for QR Monsters
//!
//! This crate provides utilities for generating QR code SVGs from share codes
//! and decoding QR codes from images.

pub mod encode;
pub mod decode;

pub use encode::{qr_svg, qr_svg_colored, QrEncodeError};
pub use decode::{decode_qr, decode_from_image, QrDecodeError};
