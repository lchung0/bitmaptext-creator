//! CLI module for command-line argument parsing

use clap::Parser;
use std::path::PathBuf;

/// BitmapFont Creator - Generate bitmap fonts for Phaser games
#[derive(Parser, Debug)]
#[command(name = "bitmapfont-creator")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the JSON configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Read configuration from stdin instead of file
    #[arg(long)]
    pub stdin: bool,

    /// Output directory for generated files
    #[arg(short, long, value_name = "DIR")]
    pub output: Option<PathBuf>,

    /// Font name (used for output file names)
    #[arg(short, long, value_name = "NAME")]
    pub font_name: Option<String>,

    /// Maximum atlas size (default: 4096)
    #[arg(long, default_value = "4096")]
    pub max_size: u32,

    /// Output result as JSON (for Claude Skill integration)
    #[arg(long)]
    pub json: bool,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}