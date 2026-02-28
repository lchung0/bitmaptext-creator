//! XML generation module for Phaser BitmapText format

use crate::error::Result;
use crate::packer::PackedChar;

/// Character metrics for XML output
#[derive(Debug, Clone)]
pub struct CharMetrics {
    /// The character
    pub char: String,
    /// X position in atlas
    pub x: u32,
    /// Y position in atlas
    pub y: u32,
    /// Width (original, without padding)
    pub width: u32,
    /// Height (original, without padding)
    pub height: u32,
    /// X offset
    pub xoffset: i32,
    /// Y offset
    pub yoffset: i32,
    /// X advance (horizontal step)
    pub xadvance: u32,
    /// Padding
    pub padding: u32,
}

impl From<PackedChar> for CharMetrics {
    fn from(packed: PackedChar) -> Self {
        // x, y point directly to the character position in atlas (with padding offset)
        // Padding is only used for texture atlas spacing, not for rendering
        CharMetrics {
            char: packed.char,
            x: packed.x + packed.padding,  // Point to actual char position (skip padding)
            y: packed.y + packed.padding,
            width: packed.original_width,
            height: packed.original_height,
            xoffset: 0,  // No offset needed
            yoffset: 0,  // No offset needed
            xadvance: packed.original_width,  // Only character width, no padding
            padding: packed.padding,
        }
    }
}

/// Generate Phaser-compatible XML font file
pub fn generate_xml(
    font_name: &str,
    atlas_width: u32,
    atlas_height: u32,
    chars: &[CharMetrics],
) -> Result<String> {
    let mut xml = String::new();

    // XML header
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");

    // Font element
    xml.push_str(&format!(
        "<font>\n"
    ));

    // Info element - use first char's padding for the font info
    let padding = chars.first().map(|c| c.padding).unwrap_or(4);
    xml.push_str(&format!(
        "  <info face=\"{}\" size=\"32\" bold=\"0\" italic=\"0\" charset=\"\" unicode=\"1\" stretchH=\"100\" smooth=\"1\" aa=\"1\" padding=\"{},{},{},{}\" spacing=\"1,1\" outline=\"0\"/>\n",
        escape_xml(font_name),
        padding, padding, padding, padding
    ));

    // Calculate lineHeight and base from actual character heights
    let max_height = chars.iter().map(|c| c.height).max().unwrap_or(32);
    let line_height = max_height;
    let base = max_height;  // Base is typically the same as line height for bitmap fonts

    // Common element
    xml.push_str(&format!(
        "  <common lineHeight=\"{}\" base=\"{}\" scaleW=\"{}\" scaleH=\"{}\" pages=\"1\" packed=\"0\" alphaChnl=\"0\" redChnl=\"4\" greenChnl=\"4\" blueChnl=\"4\"/>\n",
        line_height, base, atlas_width, atlas_height
    ));

    // Pages element
    xml.push_str("  <pages>\n");
    xml.push_str(&format!("    <page id=\"0\" file=\"{}.png\"/>\n", escape_xml(font_name)));
    xml.push_str("  </pages>\n");

    // Chars element
    xml.push_str(&format!("  <chars count=\"{}\">\n", chars.len()));

    for char_metrics in chars {
        // Get character code (use first char of string)
        let char_code = char_metrics.char.chars().next()
            .map(|c| c as u32)
            .unwrap_or(0);

        xml.push_str(&format!(
            "    <char id=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" xoffset=\"{}\" yoffset=\"{}\" xadvance=\"{}\" page=\"0\" chnl=\"15\"/>\n",
            char_code,
            char_metrics.x,
            char_metrics.y,
            char_metrics.width,
            char_metrics.height,
            char_metrics.xoffset,
            char_metrics.yoffset,
            char_metrics.xadvance
        ));
    }

    xml.push_str("  </chars>\n");
    xml.push_str("</font>\n");

    Ok(xml)
}

/// Escape special XML characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}