//! Texture packing module

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rectangle_pack::{
    GroupedRectsToPlace, RectToInsert, TargetBin,
};

use crate::error::{BitmapFontError, Result};
use crate::image_loader::CharImage;

/// A packed character with its position in the atlas
#[derive(Debug, Clone)]
pub struct PackedChar {
    /// The character identifier
    pub char: String,
    /// X position in the atlas
    pub x: u32,
    /// Y position in the atlas
    pub y: u32,
    /// Width in pixels (including padding)
    pub width: u32,
    /// Height in pixels (including padding)
    pub height: u32,
    /// Original width (without padding)
    pub original_width: u32,
    /// Original height (without padding)
    pub original_height: u32,
    /// Padding applied
    pub padding: u32,
}

/// Result of the packing operation
#[derive(Debug)]
pub struct AtlasResult {
    /// The generated atlas image
    pub image: DynamicImage,
    /// Information about packed characters
    pub chars: Vec<PackedChar>,
    /// Atlas width
    pub width: u32,
    /// Atlas height
    pub height: u32,
}

/// Pack character images into a single atlas
pub fn pack_images(images: Vec<CharImage>, max_size: u32) -> Result<AtlasResult> {
    if images.is_empty() {
        return Err(BitmapFontError::Packing("No images to pack".to_string()));
    }

    // Create rectangles for packing
    let mut rects: GroupedRectsToPlace<String, ()> = GroupedRectsToPlace::new();

    for img in &images {
        let rect = RectToInsert::new(img.width, img.height, 1);
        rects.push_rect(img.char.clone(), None, rect);
    }

    // Create target bins (try different sizes)
    let mut target_bins = std::collections::BTreeMap::new();
    target_bins.insert((), TargetBin::new(max_size, max_size, 1));

    // Perform packing
    let result = rectangle_pack::pack_rects(
        &rects,
        &mut target_bins,
        &rectangle_pack::volume_heuristic,
        &rectangle_pack::contains_smallest_box,
    );

    let placements = result.map_err(|e| BitmapFontError::Packing(
        format!("Failed to pack images: {:?}", e)
    ))?;

    // Calculate actual atlas size needed
    let mut max_width = 0u32;
    let mut max_height = 0u32;

    let mut packed_positions: std::collections::HashMap<String, (u32, u32)> = std::collections::HashMap::new();

    for (char_key, (_, location)) in placements.packed_locations() {
        max_width = max_width.max(location.x() + location.width());
        max_height = max_height.max(location.y() + location.height());
        packed_positions.insert(char_key.clone(), (location.x(), location.y()));
    }

    // Round up to power of 2 for better GPU compatibility
    let atlas_width = next_power_of_two(max_width);
    let atlas_height = next_power_of_two(max_height);

    // Create the atlas image
    let mut atlas: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(atlas_width, atlas_height);

    // Place images on the atlas
    let mut packed_chars = Vec::new();

    for img in images {
        let (x, y) = packed_positions
            .get(&img.char)
            .copied()
            .unwrap_or((0, 0));

        // Copy image to atlas
        for py in 0..img.height {
            for px in 0..img.width {
                let pixel = img.image.get_pixel(px, py);
                atlas.put_pixel(x + px, y + py, pixel);
            }
        }

        packed_chars.push(PackedChar {
            char: img.char,
            x,
            y,
            width: img.width,
            height: img.height,
            original_width: img.original_width,
            original_height: img.original_height,
            padding: img.padding,
        });
    }

    Ok(AtlasResult {
        image: DynamicImage::ImageRgba8(atlas),
        chars: packed_chars,
        width: atlas_width,
        height: atlas_height,
    })
}

/// Calculate the next power of two
fn next_power_of_two(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut power = 1;
    while power < n {
        power *= 2;
    }
    power
}