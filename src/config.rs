//! Configuration parsing module

use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;

use crate::error::{BitmapFontError, Result};

/// Frame definition for sprite sheet
#[derive(Debug, Deserialize, Clone)]
pub struct Frame {
    /// X position in the sprite sheet
    pub x: u32,
    /// Y position in the sprite sheet
    pub y: u32,
    /// Width of the frame
    pub w: u32,
    /// Height of the frame
    pub h: u32,
}

/// Character configuration entry
#[derive(Debug, Deserialize, Clone)]
pub struct CharConfig {
    /// Path to the sprite sheet image
    pub path: PathBuf,
    /// Frame definition (optional, for sprite sheets)
    #[serde(default)]
    pub frame: Option<Frame>,
    /// Padding around the character (default: 4)
    #[serde(default = "default_padding")]
    pub padding: u32,
}

/// Default padding value
fn default_padding() -> u32 {
    4
}

/// Font configuration from JSON
#[derive(Debug, Deserialize)]
pub struct FontConfig {
    /// Character to config mapping
    #[serde(flatten)]
    pub chars: HashMap<String, CharConfig>,

    /// Optional font name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_name: Option<String>,

    /// Optional output directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_dir: Option<PathBuf>,
}

impl FontConfig {
    /// Parse configuration from a file
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| BitmapFontError::ConfigParse(
                format!("Failed to read config file '{}': {}", path.display(), e)
            ))?;

        Self::from_str(&content)
    }

    /// Parse configuration from a string (JSON)
    pub fn from_str(content: &str) -> Result<Self> {
        serde_json::from_str(content)
            .map_err(|e| BitmapFontError::ConfigParse(
                format!("Invalid JSON format: {}", e)
            ))
    }

    /// Parse configuration from stdin
    pub fn from_stdin() -> Result<Self> {
        use std::io::{self, Read};

        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| BitmapFontError::ConfigParse(
                format!("Failed to read from stdin: {}", e)
            ))?;

        Self::from_str(&buffer)
    }
}