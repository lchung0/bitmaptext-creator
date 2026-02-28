//! BitmapFont Creator - A CLI tool to create bitmap fonts for Phaser games
//!
//! This library provides functionality:
//! - Parse JSON configuration files for character images
//! - Load and process character images (with frame extraction and padding)
//! - Pack images into a texture atlas
//! - Generate Phaser-compatible XML font files

pub mod error;
pub mod config;
pub mod image_loader;
pub mod packer;
pub mod xml_generator;
pub mod output;

pub use error::{BitmapFontError, Result};
pub use config::{FontConfig, CharConfig, Frame};
pub use output::OutputResult;