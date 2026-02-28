//! BitmapFont Creator - A CLI tool to create bitmap fonts for Phaser games

use std::path::PathBuf;

use bitmapfont_creator::{
    config::FontConfig,
    error::BitmapFontError,
    image_loader::load_images,
    output::{save_outputs, OutputResult},
    packer::pack_images,
    xml_generator::{generate_xml, CharMetrics},
};
use clap::Parser;

mod cli;

use cli::Args;

fn main() {
    let args = Args::parse();

    match run(args) {
        Ok(result) => {
            if result.success {
                if let Some(atlas) = &result.atlas_path {
                    eprintln!("Atlas saved: {}", atlas);
                }
                if let Some(xml) = &result.xml_path {
                    eprintln!("XML saved: {}", xml);
                }
                if let Some(count) = result.char_count {
                    eprintln!("Characters processed: {}", count);
                }
            }
            println!("{}", result.to_json().unwrap_or_default());
            std::process::exit(if result.success { 0 } else { 1 });
        }
        Err(e) => {
            let result = OutputResult::error(
                error_type(&e),
                &e.to_string(),
                &suggestion(&e),
            );
            println!("{}", result.to_json().unwrap_or_default());
            std::process::exit(1);
        }
    }
}

fn run(args: Args) -> Result<OutputResult, BitmapFontError> {
    // Load configuration
    let config = if args.stdin {
        if args.verbose {
            eprintln!("Reading configuration from stdin...");
        }
        FontConfig::from_stdin()?
    } else if let Some(config_path) = &args.config {
        if args.verbose {
            eprintln!("Loading configuration from: {}", config_path.display());
        }
        FontConfig::from_file(config_path)?
    } else {
        return Err(BitmapFontError::ConfigParse(
            "No configuration provided. Use --config <FILE> or --stdin".to_string(),
        ));
    };

    if args.verbose {
        eprintln!("Found {} character(s) in configuration", config.chars.len());
    }

    // Determine base path for relative image paths
    let base_path = if args.stdin {
        PathBuf::from(".")
    } else if let Some(config_path) = &args.config {
        config_path.parent().unwrap_or(PathBuf::from(".").as_path()).to_path_buf()
    } else {
        PathBuf::from(".")
    };

    // Load images
    if args.verbose {
        eprintln!("Loading character images...");
    }
    let images = load_images(&config, &base_path)?;

    if args.verbose {
        eprintln!("Loaded {} image(s)", images.len());
    }

    // Pack images into atlas
    if args.verbose {
        eprintln!("Packing images into atlas (max size: {}x{})...", args.max_size, args.max_size);
    }
    let atlas = pack_images(images, args.max_size)?;

    if args.verbose {
        eprintln!("Atlas size: {}x{}", atlas.width, atlas.height);
    }

    // Convert to character metrics
    let char_metrics: Vec<CharMetrics> = atlas.chars.iter().map(|c| c.clone().into()).collect();

    // Determine font name
    let font_name = args.font_name
        .or(config.font_name)
        .unwrap_or_else(|| "font".to_string());

    // Generate XML
    if args.verbose {
        eprintln!("Generating XML font file...");
    }
    let xml = generate_xml(&font_name, atlas.width, atlas.height, &char_metrics)?;

    // Determine output directory
    let output_dir = args.output
        .or(config.output_dir)
        .unwrap_or_else(|| PathBuf::from("."));

    // Save outputs
    if args.verbose {
        eprintln!("Saving outputs to: {}", output_dir.display());
    }
    let (atlas_path, xml_path) = save_outputs(&atlas, &xml, &output_dir, &font_name)?;

    // Create result
    let result = OutputResult::success(
        atlas_path,
        xml_path,
        char_metrics.len(),
        atlas.width,
        atlas.height,
        &char_metrics,
    );

    Ok(result)
}

fn error_type(error: &BitmapFontError) -> &'static str {
    match error {
        BitmapFontError::ConfigParse(_) => "ConfigParse",
        BitmapFontError::ImageLoad(_) => "ImageLoad",
        BitmapFontError::Packing(_) => "Packing",
        BitmapFontError::XmlGeneration(_) => "XmlGeneration",
        BitmapFontError::Io(_) => "Io",
        BitmapFontError::Json(_) => "Json",
    }
}

fn suggestion(error: &BitmapFontError) -> String {
    match error {
        BitmapFontError::ConfigParse(_) => "Check JSON syntax and ensure valid key-value pairs".to_string(),
        BitmapFontError::ImageLoad(_) => "Verify image file exists and is a valid PNG format".to_string(),
        BitmapFontError::Packing(_) => "Try increasing --max-size or reducing character count".to_string(),
        BitmapFontError::XmlGeneration(_) => "Check character encoding and font name".to_string(),
        BitmapFontError::Io(_) => "Check file permissions and disk space".to_string(),
        BitmapFontError::Json(_) => "Verify JSON output configuration".to_string(),
    }
}