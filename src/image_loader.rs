//! Image loading module

use std::path::Path;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

use crate::error::{BitmapFontError, Result};
use crate::config::FontConfig;

/// A loaded character image with its metadata
#[derive(Debug)]
pub struct CharImage {
    /// The character identifier
    pub char: String,
    /// The loaded image (with padding applied)
    pub image: DynamicImage,
    /// Image width in pixels (including padding)
    pub width: u32,
    /// Image height in pixels (including padding)
    pub height: u32,
    /// Original width (without padding)
    pub original_width: u32,
    /// Original height (without padding)
    pub original_height: u32,
    /// Padding applied
    pub padding: u32,
}

/// Load all character images from a font configuration
pub fn load_images(config: &FontConfig, base_path: &Path) -> Result<Vec<CharImage>> {
    let mut images = Vec::new();
    // Cache for loaded sprite sheets
    let mut sheet_cache: std::collections::HashMap<std::path::PathBuf, DynamicImage> = std::collections::HashMap::new();

    for (char_key, char_config) in &config.chars {
        let full_path = if char_config.path.is_absolute() {
            char_config.path.clone()
        } else {
            base_path.join(&char_config.path)
        };

        // Load or get cached sprite sheet
        let sheet = if let Some(cached) = sheet_cache.get(&full_path) {
            cached.clone()
        } else {
            let img = image::open(&full_path)
                .map_err(|e| BitmapFontError::ImageLoad(
                    format!("Failed to load image '{}': {}", full_path.display(), e)
                ))?;
            sheet_cache.insert(full_path.clone(), img.clone());
            img
        };

        // Extract frame or use whole image
        let (original_width, original_height, char_img) = if let Some(frame) = &char_config.frame {
            // Extract specific frame from sprite sheet
            let frame_img = extract_frame(&sheet, frame)?;
            (frame.w, frame.h, frame_img)
        } else {
            // Use whole image
            (sheet.width(), sheet.height(), sheet.clone())
        };

        // Apply padding
        let padding = char_config.padding;
        let padded_img = apply_padding(&char_img, padding);
        let width = original_width + padding * 2;
        let height = original_height + padding * 2;

        images.push(CharImage {
            char: char_key.clone(),
            image: padded_img,
            width,
            height,
            original_width,
            original_height,
            padding,
        });
    }

    Ok(images)
}

/// Extract a frame from a sprite sheet
fn extract_frame(sheet: &DynamicImage, frame: &crate::config::Frame) -> Result<DynamicImage> {
    // Validate frame bounds
    if frame.x + frame.w > sheet.width() || frame.y + frame.h > sheet.height() {
        return Err(BitmapFontError::ImageLoad(
            format!(
                "Frame ({}, {}, {}, {}) is out of bounds for image size ({}, {})",
                frame.x, frame.y, frame.w, frame.h,
                sheet.width(), sheet.height()
            )
        ));
    }

    // Extract the frame region
    let mut frame_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(frame.w, frame.h);

    for y in 0..frame.h {
        for x in 0..frame.w {
            let pixel = sheet.get_pixel(frame.x + x, frame.y + y);
            frame_img.put_pixel(x, y, pixel);
        }
    }

    Ok(DynamicImage::ImageRgba8(frame_img))
}

/// Apply padding around an image
fn apply_padding(img: &DynamicImage, padding: u32) -> DynamicImage {
    if padding == 0 {
        return img.clone();
    }

    let width = img.width() + padding * 2;
    let height = img.height() + padding * 2;

    let mut padded: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Fill with transparent pixels (padding area)
    for pixel in padded.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]);
    }

    // Copy original image to the center
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            padded.put_pixel(x + padding, y + padding, pixel);
        }
    }

    DynamicImage::ImageRgba8(padded)
}