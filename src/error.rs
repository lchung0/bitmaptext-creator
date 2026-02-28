//! Error types for bitmapfont-creator

use std::io;
use thiserror::Error;

/// Main error type for the bitmap font creator
#[derive(Debug, Error)]
pub enum BitmapFontError {
    /// Configuration parsing error
    #[error("Config parse error: {0}")]
    ConfigParse(String),

    /// Image loading error
    #[error("Image load error: {0}")]
    ImageLoad(String),

    /// Texture packing error
    #[error("Packing error: {0}")]
    Packing(String),

    /// XML generation error
    #[error("XML generation error: {0}")]
    XmlGeneration(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type alias for bitmap font creator operations
pub type Result<T> = std::result::Result<T, BitmapFontError>;