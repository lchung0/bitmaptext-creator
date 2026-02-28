# BitmapFont Creator

[![Crates.io](https://img.shields.io/crates/v/bitmapfont-creator.svg)](https://crates.io/crates/bitmapfont-creator)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A command-line tool to create bitmap fonts for [Phaser](https://phaser.io/) games.

## Features

- Merge multiple character images into a single texture atlas (PNG)
- Support for extracting frames from sprite sheets
- Character padding support to prevent texture bleeding
- Generate Phaser BitmapText compatible XML font files
- Structured JSON output for programmatic processing

## Installation

### From Source

```bash
git clone https://github.com/lchung0/bitmaptext-creator.git
cd bitmaptext-creator
cargo build --release
```

The compiled binary will be at `target/release/bitmapfont-creator`.

### From Crates.io

```bash
cargo install bitmapfont-creator
```

## Usage

### Basic Command

```bash
bitmapfont-creator --config font-config.json --output ./dist
```

### Command Line Arguments

| Argument | Short | Description | Default |
|----------|-------|-------------|---------|
| `--config` | `-c` | JSON configuration file path | - |
| `--stdin` | | Read configuration from stdin | false |
| `--output` | `-o` | Output directory | Current directory |
| `--font-name` | `-f` | Font name | "font" |
| `--max-size` | | Maximum atlas size | 4096 |
| `--json` | | Output result in JSON format | false |
| `--verbose` | `-v` | Verbose output mode | false |

## Configuration File Format

### Basic Format

```json
{
  "font_name": "my-font",
  "output_dir": "./output",
  "A": {
    "path": "chars/A.png",
    "padding": 4
  },
  "B": {
    "path": "chars/B.png",
    "padding": 4
  }
}
```

### Extracting Frames from Sprite Sheet

```json
{
  "font_name": "game-font",
  "A": {
    "path": "spritesheet.png",
    "frame": {"x": 0, "y": 0, "w": 32, "h": 32},
    "padding": 4
  },
  "B": {
    "path": "spritesheet.png",
    "frame": {"x": 32, "y": 0, "w": 32, "h": 32},
    "padding": 4
  }
}
```

### Configuration Fields

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `path` | string | Yes | - | Image file path |
| `frame` | object | No | - | Sprite sheet frame definition |
| `frame.x` | number | Yes* | - | Frame X coordinate in sprite sheet |
| `frame.y` | number | Yes* | - | Frame Y coordinate in sprite sheet |
| `frame.w` | number | Yes* | - | Frame width |
| `frame.h` | number | Yes* | - | Frame height |
| `padding` | number | No | 4 | Character spacing (prevents texture bleeding) |
| `font_name` | string | No | "font" | Font name |
| `output_dir` | string | No | "." | Output directory |

*\* Required when using `frame`*

## Output Files

The tool generates two files:

- **`{font-name}.png`** - Texture atlas containing all characters
- **`{font-name}.xml`** - Phaser BitmapText compatible font description file

### XML Format

The generated XML file follows the AngelCode BMFont format:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<font>
  <info face="my-font" size="32" ... padding="4,4,4,4" .../>
  <common lineHeight="32" base="26" scaleW="128" scaleH="64" .../>
  <pages>
    <page id="0" file="my-font.png"/>
  </pages>
  <chars count="2">
    <char id="65" x="4" y="4" width="24" height="28" xoffset="0" yoffset="0" xadvance="28" page="0" chnl="15"/>
  </chars>
</font>
```

### Character Attributes

| Attribute | Description |
|-----------|-------------|
| `x`, `y` | Character position in atlas |
| `width`, `height` | Character original dimensions |
| `xoffset`, `yoffset` | Rendering offset (always 0) |
| `xadvance` | Horizontal advance distance (character width only, no padding) |

## Using with Phaser

```javascript
// Preload
function preload() {
  this.load.bitmapFont('myFont', 'assets/my-font.png', 'assets/my-font.xml');
}

// Create text
function create() {
  const text = this.add.bitmapText(100, 100, 'myFont', 'Hello World');
  text.setFontSize(32);
}
```

## Reading from Stdin

Useful for programmatic invocation:

```bash
echo '{"A":{"path":"a.png"},"font_name":"dynamic-font"}' | \
  bitmapfont-creator --stdin --json
```

## JSON Output Format

Use `--json` flag for structured output:

### Success Response

```json
{
  "success": true,
  "atlas_path": "./output/my-font.png",
  "xml_path": "./output/my-font.xml",
  "char_count": 2,
  "atlas_size": {"width": 128, "height": 64},
  "char_map": {
    "A": {
      "x": 4,
      "y": 4,
      "width": 24,
      "height": 28,
      "xoffset": 0,
      "yoffset": 0,
      "xadvance": 28,
      "padding": 4
    }
  }
}
```

### Error Response

```json
{
  "success": false,
  "error_type": "ImageLoad",
  "error_message": "Failed to load image: chars/A.png",
  "suggestion": "Verify image file exists and is a valid PNG format"
}
```

## Error Types

| Error Type | Description |
|------------|-------------|
| `ConfigParse` | JSON configuration parsing failed |
| `ImageLoad` | Image loading failed |
| `Packing` | Atlas packing failed (insufficient space) |
| `Io` | Filesystem error |
| `XmlGeneration` | XML generation error |

## Tech Stack

- **Rust** - Systems programming language
- **image** - Image processing
- **serde/serde_json** - JSON serialization
- **clap** - Command line argument parsing
- **rectangle-pack** - Rectangle packing algorithm

## License

This project is licensed under the [MIT License](LICENSE).