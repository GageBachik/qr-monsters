use qrcode::QrCode;
use qrcode::render::svg;

/// Error type for QR encoding
#[derive(Debug)]
pub struct QrEncodeError(pub String);

impl std::fmt::Display for QrEncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QR encode error: {}", self.0)
    }
}

impl std::error::Error for QrEncodeError {}

/// Generate an SVG string for a QR code
///
/// # Arguments
/// * `data` - The string data to encode in the QR code
/// * `size` - The size in pixels for the QR code
///
/// # Returns
/// An SVG string that can be embedded directly in HTML
pub fn qr_svg(data: &str, size: u32) -> Result<String, QrEncodeError> {
    let code = QrCode::new(data.as_bytes())
        .map_err(|e| QrEncodeError(e.to_string()))?;

    let svg = code.render()
        .min_dimensions(size, size)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

    Ok(svg)
}

/// Generate an SVG string with custom colors
pub fn qr_svg_colored(
    data: &str,
    size: u32,
    dark: &str,
    light: &str
) -> Result<String, QrEncodeError> {
    let code = QrCode::new(data.as_bytes())
        .map_err(|e| QrEncodeError(e.to_string()))?;

    let svg = code.render()
        .min_dimensions(size, size)
        .dark_color(svg::Color(dark))
        .light_color(svg::Color(light))
        .build();

    Ok(svg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_svg_generation() {
        let result = qr_svg("QRM1:test", 200);
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }
}
