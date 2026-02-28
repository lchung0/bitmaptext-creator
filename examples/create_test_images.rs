use image::{ImageBuffer, Rgba, ImageFormat};
use std::fs;

fn main() {
    fs::create_dir_all("examples/chars").ok();

    // Create test images for digits 0-9
    for i in 0u8..=9 {
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(23, 30);

        // Fill with transparent background
        for pixel in img.pixels_mut() {
            *pixel = Rgba([0u8, 0u8, 0u8, 0u8]);
        }

        // Draw the digit as white text
        draw_digit(&mut img, i);

        let path = format!("examples/chars/digit-{}.png", i);
        img.save_with_format(&path, ImageFormat::Png).unwrap();
        println!("Created: {}", path);
    }

    // Create comma image
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(10, 30);
    for pixel in img.pixels_mut() {
        *pixel = Rgba([0u8, 0u8, 0u8, 0u8]);
    }
    draw_comma(&mut img);
    img.save_with_format("examples/chars/comma.png", ImageFormat::Png).unwrap();
    println!("Created: examples/chars/comma.png");
}

fn draw_digit(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, digit: u8) {
    // Simple 7-segment style digit patterns (5 wide x 7 tall)
    let patterns: [[[bool; 5]; 7]; 10] = [
        // 0
        [[true, true, true, true, true],
         [true, false, false, false, true],
         [true, false, false, false, true],
         [true, false, false, false, true],
         [true, false, false, false, true],
         [true, false, false, false, true],
         [true, true, true, true, true]],
        // 1
        [[false, false, false, true, false],
         [false, false, true, true, false],
         [false, false, false, true, false],
         [false, false, false, true, false],
         [false, false, false, true, false],
         [false, false, false, true, false],
         [false, true, true, true, true]],
        // 2
        [[true, true, true, true, true],
         [false, false, false, false, true],
         [false, false, false, false, true],
         [true, true, true, true, true],
         [true, false, false, false, false],
         [true, false, false, false, false],
         [true, true, true, true, true]],
        // 3
        [[true, true, true, true, true],
         [false, false, false, false, true],
         [false, false, false, false, true],
         [true, true, true, true, true],
         [false, false, false, false, true],
         [false, false, false, false, true],
         [true, true, true, true, true]],
        // 4
        [[true, false, false, false, true],
         [true, false, false, false, true],
         [true, false, false, false, true],
         [true, true, true, true, true],
         [false, false, false, false, true],
         [false, false, false, false, true],
         [false, false, false, false, true]],
        // 5
        [[true, true, true, true, true],
         [true, false, false, false, false],
         [true, false, false, false, false],
         [true, true, true, true, true],
         [false, false, false, false, true],
         [false, false, false, false, true],
         [true, true, true, true, true]],
        // 6
        [[true, true, true, true, true],
         [true, false, false, false, false],
         [true, false, false, false, false],
         [true, true, true, true, true],
         [true, false, false, false, true],
         [true, false, false, false, true],
         [true, true, true, true, true]],
        // 7
        [[true, true, true, true, true],
         [false, false, false, false, true],
         [false, false, false, true, false],
         [false, false, true, false, false],
         [false, false, true, false, false],
         [false, false, true, false, false],
         [false, false, true, false, false]],
        // 8
        [[true, true, true, true, true],
         [true, false, false, false, true],
         [true, false, false, false, true],
         [true, true, true, true, true],
         [true, false, false, false, true],
         [true, false, false, false, true],
         [true, true, true, true, true]],
        // 9
        [[true, true, true, true, true],
         [true, false, false, false, true],
         [true, false, false, false, true],
         [true, true, true, true, true],
         [false, false, false, false, true],
         [false, false, false, false, true],
         [true, true, true, true, true]],
    ];

    let pattern = &patterns[digit as usize];
    let offset_x = 6;
    let offset_y = 8;

    for (py, row) in pattern.iter().enumerate() {
        for (px, &is_on) in row.iter().enumerate() {
            if is_on {
                for dy in 0..2 {
                    for dx in 0..2 {
                        img.put_pixel(
                            offset_x + (px as u32 * 2) + dx,
                            offset_y + (py as u32 * 2) + dy,
                            Rgba([255u8, 255u8, 255u8, 255u8]),
                        );
                    }
                }
            }
        }
    }
}

fn draw_comma(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    // Draw a simple comma (2x4 pattern)
    let pattern = [
        [false, true],
        [false, true],
        [false, true],
        [true, true],
    ];

    let offset_x = 3;
    let offset_y = 20;

    for (py, row) in pattern.iter().enumerate() {
        for (px, &is_on) in row.iter().enumerate() {
            if is_on {
                for dy in 0..2 {
                    for dx in 0..2 {
                        img.put_pixel(
                            offset_x + (px as u32 * 2) + dx,
                            offset_y + (py as u32 * 2) + dy,
                            Rgba([255u8, 255u8, 255u8, 255u8]),
                        );
                    }
                }
            }
        }
    }
}