# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Fixed `xadvance` calculation to not include padding - padding now only affects texture atlas spacing, not character rendering spacing
- Fixed `lineHeight` and `base` to use actual character heights instead of hardcoded values

## [0.1.0] - 2026-02-28

### Added
- Initial release
- CLI tool to create bitmap fonts for Phaser games
- Merge multiple character images into a single texture atlas (PNG)
- Support for extracting frames from sprite sheets
- Character padding support to prevent texture bleeding
- Generate Phaser BitmapText compatible XML font files
- Structured JSON output for programmatic processing
- Command line interface with clap
- Support for reading config from stdin
- Comprehensive error handling with helpful suggestions
