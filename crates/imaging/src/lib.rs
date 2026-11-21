use palette::{IntoColor, Oklch, Srgb};
use std::path::Path;

pub const FALLBACK_ACCENT_HEX: &str = "#B0532E";

/// Clamps lightness and chroma according to Section 12.3:
/// - Light theme: L in 0.55..=0.72, C at most 0.12, keep hue
/// - Dark theme: L in 0.70..=0.82, C at most 0.12, keep hue
pub fn clamp_accent_color(oklch: Oklch, is_dark_theme: bool) -> Oklch {
    let (min_l, max_l) = if is_dark_theme {
        (0.70, 0.82)
    } else {
        (0.55, 0.72)
    };

    let l = oklch.l.clamp(min_l, max_l);
    let c = oklch.chroma.min(0.12);
    let h = oklch.hue;

    Oklch::new(l, c, h)
}

pub fn oklch_to_hex(oklch: Oklch) -> String {
    let srgb: Srgb = oklch.into_color();
    let r = (srgb.red.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (srgb.green.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (srgb.blue.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

pub fn generate_blurhash(image_path: &Path) -> anyhow::Result<String> {
    let img = image::open(image_path)?.to_rgba8();
    let (width, height) = img.dimensions();
    let blurhash = blurhash::encode(4, 3, width, height, img.as_raw())?;
    Ok(blurhash)
}

pub fn create_thumbnail(
    image_path: &Path,
    thumb_path: &Path,
    target_width: u32,
) -> anyhow::Result<()> {
    let img = image::open(image_path)?;
    let thumb = img.resize(
        target_width,
        target_width * 2,
        image::imageops::FilterType::Lanczos3,
    );
    if let Some(parent) = thumb_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    thumb.save(thumb_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::OklabHue;

    #[test]
    fn test_clamp_accent_light() {
        let input = Oklch::new(0.3f32, 0.25f32, OklabHue::from_degrees(45.0f32));
        let clamped = clamp_accent_color(input, false);
        assert!((clamped.l - 0.55).abs() < 1e-4);
        assert!((clamped.chroma - 0.12).abs() < 1e-4);
    }

    #[test]
    fn test_clamp_accent_dark() {
        let input = Oklch::new(0.9f32, 0.25f32, OklabHue::from_degrees(45.0f32));
        let clamped = clamp_accent_color(input, true);
        assert!((clamped.l - 0.82).abs() < 1e-4);
        assert!((clamped.chroma - 0.12).abs() < 1e-4);
    }
}
