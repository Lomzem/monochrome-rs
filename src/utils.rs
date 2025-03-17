/// Convert HSL color values to RGB
///
/// # Arguments
///
/// * `h` - Hue (0-360)
/// * `s` - Saturation (0-100)
/// * `l` - Lightness (0-100)
///
/// # Returns
///
/// A tuple of (r, g, b) values, each in the range 0-255
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    // Normalize saturation and lightness to 0-1
    let s = s / 100.0;
    let l = l / 100.0;

    // Algorithm from https://www.rapidtables.com/convert/color/hsl-to-rgb.html
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    // Convert to 0-255 range
    ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0)
}
