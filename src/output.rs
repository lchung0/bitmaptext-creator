//! Output handling module

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

use crate::error::{BitmapFontError, Result};
use crate::packer::AtlasResult;
use crate::xml_generator::CharMetrics;

/// Output result containing generated file paths
#[derive(Debug, Serialize, Deserialize)]
pub struct OutputResult {
    /// Whether the operation succeeded
    pub success: bool,
    /// Path to the generated atlas PNG
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atlas_path: Option<String>,
    /// Path to the generated XML file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_path: Option<String>,
    /// Number of characters processed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_count: Option<usize>,
    /// Atlas dimensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atlas_size: Option<AtlasSize>,
    /// Character map with positions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_map: Option<serde_json::Value>,
    /// Error type (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Suggestion for fixing the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

/// Atlas size information
#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasSize {
    pub width: u32,
    pub height: u32,
}

impl OutputResult {
    /// Create a successful result
    pub fn success(
        atlas_path: PathBuf,
        xml_path: PathBuf,
        char_count: usize,
        atlas_width: u32,
        atlas_height: u32,
        chars: &[CharMetrics],
    ) -> Self {
        let char_map = chars.iter().map(|c| {
            (c.char.clone(), serde_json::json!({
                "x": c.x,
                "y": c.y,
                "width": c.width,
                "height": c.height,
                "xoffset": c.xoffset,
                "yoffset": c.yoffset,
                "xadvance": c.xadvance,
                "padding": c.padding
            }))
        }).collect::<serde_json::Map<String, serde_json::Value>>();

        OutputResult {
            success: true,
            atlas_path: Some(atlas_path.to_string_lossy().to_string()),
            xml_path: Some(xml_path.to_string_lossy().to_string()),
            char_count: Some(char_count),
            atlas_size: Some(AtlasSize {
                width: atlas_width,
                height: atlas_height,
            }),
            char_map: Some(serde_json::Value::Object(char_map)),
            error_type: None,
            error_message: None,
            suggestion: None,
        }
    }

    /// Create an error result
    pub fn error(error_type: &str, message: &str, suggestion: &str) -> Self {
        OutputResult {
            success: false,
            atlas_path: None,
            xml_path: None,
            char_count: None,
            atlas_size: None,
            char_map: None,
            error_type: Some(error_type.to_string()),
            error_message: Some(message.to_string()),
            suggestion: Some(suggestion.to_string()),
        }
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| BitmapFontError::XmlGeneration(format!("Failed to serialize JSON: {}", e)))
    }
}

/// Save output files
pub fn save_outputs(
    atlas: &AtlasResult,
    xml_content: &str,
    output_dir: &Path,
    font_name: &str,
) -> Result<(PathBuf, PathBuf)> {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

    // Save atlas PNG
    let atlas_path = output_dir.join(format!("{}.png", font_name));
    atlas.image.save(&atlas_path)
        .map_err(|e| BitmapFontError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to save atlas: {}", e)
        )))?;

    // Save XML file
    let xml_path = output_dir.join(format!("{}.xml", font_name));
    std::fs::write(&xml_path, xml_content)?;

    Ok((atlas_path, xml_path))
}