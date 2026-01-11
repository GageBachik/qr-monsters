use image::DynamicImage;
use rqrr::PreparedImage;

/// Error type for QR decoding
#[derive(Debug)]
pub struct QrDecodeError(pub String);

impl std::fmt::Display for QrDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QR decode error: {}", self.0)
    }
}

impl std::error::Error for QrDecodeError {}

/// Decode QR code from image bytes (PNG/JPEG)
///
/// # Arguments
/// * `image_bytes` - Raw bytes of a PNG or JPEG image
///
/// # Returns
/// The decoded string content from the QR code
pub fn decode_qr(image_bytes: &[u8]) -> Result<String, QrDecodeError> {
    // Load image from bytes
    let img = image::load_from_memory(image_bytes)
        .map_err(|e| QrDecodeError(format!("Failed to load image: {}", e)))?;

    decode_from_image(img)
}

/// Decode QR code from a DynamicImage
pub fn decode_from_image(img: DynamicImage) -> Result<String, QrDecodeError> {
    // Convert to grayscale for QR detection
    let gray = img.to_luma8();

    // Prepare image for QR detection
    let mut prepared = PreparedImage::prepare(gray);

    // Find and decode QR codes
    let grids = prepared.detect_grids();

    if grids.is_empty() {
        return Err(QrDecodeError("No QR code found in image".into()));
    }

    // Decode the first QR code found
    let (_meta, content) = grids[0]
        .decode()
        .map_err(|e| QrDecodeError(format!("Failed to decode QR: {:?}", e)))?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::qr_svg;

    // Note: Full round-trip test would require SVG rendering to image,
    // which is complex. In practice, test with actual image files.
    #[test]
    fn test_decode_error_on_invalid_image() {
        let result = decode_qr(b"not an image");
        assert!(result.is_err());
    }
}
