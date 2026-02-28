# BitmapFont Creator

[![Crates.io](https://img.shields.io/crates/v/bitmapfont-creator.svg)](https://crates.io/crates/bitmapfont-creator)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

一个用于为 [Phaser](https://phaser.io/) 游戏创建位图字体的命令行工具。

## 功能特性

- 将多个字符图片合并为单一纹理图集（PNG）
- 支持从精灵图（sprite sheet）中提取指定帧
- 支持字符间距（padding）设置，防止纹理渗色
- 生成 Phaser BitmapText 兼容的 XML 字体文件
- 结构化 JSON 输出，便于程序化处理

## 安装

### 从源码编译

```bash
git clone https://github.com/lchung0/bitmaptext-creator.git
cd bitmaptext-creator
cargo build --release
```

编译后的二进制文件位于 `target/release/bitmapfont-creator`。

### 从 Crates.io 安装

```bash
cargo install bitmapfont-creator
```

## 使用方法

### 基本命令

```bash
bitmapfont-creator --config font-config.json --output ./dist
```

### 命令行参数

| 参数 | 简写 | 说明 | 默认值 |
|------|------|------|--------|
| `--config` | `-c` | JSON 配置文件路径 | - |
| `--stdin` | | 从标准输入读取配置 | false |
| `--output` | `-o` | 输出目录 | 当前目录 |
| `--font-name` | `-f` | 字体名称 | "font" |
| `--max-size` | | 图集最大尺寸 | 4096 |
| `--json` | | 输出 JSON 格式结果 | false |
| `--verbose` | `-v` | 详细输出模式 | false |

## 配置文件格式

### 基本格式

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

### 从精灵图提取帧

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

### 配置字段说明

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `path` | string | 是 | - | 图片文件路径 |
| `frame` | object | 否 | - | 精灵图帧定义 |
| `frame.x` | number | 是* | - | 帧在精灵图中的 X 坐标 |
| `frame.y` | number | 是* | - | 帧在精灵图中的 Y 坐标 |
| `frame.w` | number | 是* | - | 帧宽度 |
| `frame.h` | number | 是* | - | 帧高度 |
| `padding` | number | 否 | 4 | 字符间距（防止纹理渗色） |
| `font_name` | string | 否 | "font" | 字体名称 |
| `output_dir` | string | 否 | "." | 输出目录 |

*\* 当使用 `frame` 时必填*

## 输出文件

工具生成两个文件：

- **`{字体名称}.png`** - 包含所有字符的纹理图集
- **`{字体名称}.xml`** - Phaser BitmapText 兼容的字体描述文件

### XML 格式

生成的 XML 文件遵循 AngelCode BMFont 格式：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<font>
  <info face="my-font" size="32" ... padding="4,4,4,4" .../>
  <common lineHeight="30" base="30" scaleW="128" scaleH="64" .../>
  <pages>
    <page id="0" file="my-font.png"/>
  </pages>
  <chars count="2">
    <char id="65" x="4" y="4" width="23" height="30" xoffset="0" yoffset="0" xadvance="23" page="0" chnl="15"/>
  </chars>
</font>
```

### 字符属性说明

| 属性 | 说明 |
|------|------|
| `x`, `y` | 字符在图集中的位置（已包含 padding 偏移） |
| `width`, `height` | 字符原始尺寸（不含 padding） |
| `xoffset`, `yoffset` | 渲染偏移（始终为 0） |
| `xadvance` | 水平步进距离（仅字符宽度，不含 padding） |

### lineHeight 和 base

- **`lineHeight`**: 所有字符的最大高度（基于 frame.h 或图片实际高度）
- **`base`**: 与 lineHeight 相同

## 在 Phaser 中使用

```javascript
// 预加载
function preload() {
  this.load.bitmapFont('myFont', 'assets/my-font.png', 'assets/my-font.xml');
}

// 创建文本
function create() {
  const text = this.add.bitmapText(100, 100, 'myFont', 'Hello World');
  text.setFontSize(32);
}
```

## 从标准输入读取

适用于程序化调用：

```bash
echo '{"A":{"path":"a.png"},"font_name":"dynamic-font"}' | \
  bitmapfont-creator --stdin --json
```

## JSON 输出格式

使用 `--json` 参数获取结构化输出：

### 成功响应

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
      "width": 23,
      "height": 30,
      "xoffset": 0,
      "yoffset": 0,
      "xadvance": 23,
      "padding": 4
    }
  }
}
```

### 错误响应

```json
{
  "success": false,
  "error_type": "ImageLoad",
  "error_message": "无法加载图片: chars/A.png",
  "suggestion": "请检查文件是否存在且为有效的 PNG 格式"
}
```

## 错误类型

| 错误类型 | 说明 |
|----------|------|
| `ConfigParse` | JSON 配置解析失败 |
| `ImageLoad` | 图片加载失败 |
| `Packing` | 图集打包失败（空间不足） |
| `Io` | 文件系统错误 |
| `XmlGeneration` | XML 生成错误 |

## 技术栈

- **Rust** - 系统编程语言
- **image** - 图像处理
- **serde/serde_json** - JSON 序列化
- **clap** - 命令行参数解析
- **rectangle-pack** - 矩形打包算法

## 许可证

本项目基于 [MIT 许可证](LICENSE) 开源。