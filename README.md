# WIM Parser

[![Crates.io](https://img.shields.io/crates/v/wim-parser.svg)](https://crates.io/crates/wim-parser)
[![Documentation](https://docs.rs/wim-parser/badge.svg)](https://docs.rs/wim-parser)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/junjiangao/wim-parser/workflows/Build/badge.svg)](https://github.com/junjiangao/wim-parser/actions)
[![Release](https://github.com/junjiangao/wim-parser/workflows/Release%20and%20Publish/badge.svg)](https://github.com/junjiangao/wim-parser/actions)

A Rust library for parsing Windows Imaging (WIM) files.

## Features

- 🔍 Parse WIM file headers and metadata
- 📊 Extract detailed image information
- 🏗️ Support for multiple compression formats (XPRESS, LZX)
- 🪟 Windows version detection (Windows 10, 11, Server editions)
- 🏛️ Architecture identification (x86, x64, ARM, ARM64)
- 📝 Comprehensive XML metadata parsing
- 🔧 Optional logging support with `tracing`

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
wim-parser = "0.1"
```

### Basic Usage

```rust
use wim_parser::WimParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = WimParser::new("path/to/install.wim")?;
    parser.parse_full()?;

    // Get basic information
    println!("Image count: {}", parser.get_image_count());
    println!("Compressed: {}", parser.is_compressed());

    // Iterate through images
    for image in parser.get_images() {
        println!("Image: {}", image.name);
        println!("  Files: {}", image.file_count);
        println!("  Directories: {}", image.dir_count);
        println!("  Total size: {} bytes", image.total_bytes);

        if let Some(version) = &image.version {
            println!("  Version: {}", version);
        }

        if let Some(arch) = &image.architecture {
            println!("  Architecture: {}", arch);
        }
    }

    // Get Windows-specific information
    if let Some(windows_info) = parser.get_windows_info() {
        println!("Windows Info: {}", windows_info);
    }

    Ok(())
}
```

### Without Logging

If you don't need logging functionality, you can disable it:

```toml
[dependencies]
wim-parser = { version = "0.1", default-features = false }
```

## API Overview

### Core Types

- `WimParser` - Main parser for WIM files
- `WimHeader` - WIM file header information
- `ImageInfo` - Individual image metadata
- `WindowsInfo` - Windows-specific information summary

### Key Methods

- `WimParser::new()` - Create a new parser
- `parse_full()` - Parse the entire WIM file
- `get_images()` - Get all image information
- `get_windows_info()` - Get Windows-specific summary
- `has_version()` - Check for specific Windows version
- `has_architecture()` - Check for specific architecture

## WIM File Format

WIM (Windows Imaging) files are archive files used by Microsoft for Windows installation media. This library supports:

- **WIM Header**: File signature, metadata, and resource information
- **XML Data**: Detailed image metadata including version and architecture
- **Compression**: XPRESS and LZX compression detection
- **Multiple Images**: Support for WIM files containing multiple Windows editions

## Architecture Detection

The library can identify the following architectures:

- `x86` (32-bit Intel/AMD)
- `x64` (64-bit Intel/AMD)
- `ARM` (32-bit ARM)
- `ARM64` (64-bit ARM)

## Version Detection

Supports detection of:

- Windows 10 (various editions)
- Windows 11 (various editions)
- Windows Server 2019/2022
- Generic Windows versions

## Error Handling

The library uses `anyhow` for error handling, providing detailed error messages for common issues:

- Invalid WIM file signatures
- Corrupted file headers
- Missing or invalid XML data
- I/O errors during file reading

## Examples

See the `examples/` directory for more detailed usage examples.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Release Process

This project uses automated release workflows for publishing to both GitHub Releases and crates.io.

### For Maintainers

1. **Update version in `Cargo.toml`**
2. **Create and push a version tag**:
   ```bash
   git tag v0.1.1
   git push origin v0.1.1
   ```
3. **GitHub Actions will automatically**:
   - Run comprehensive tests and validation
   - Verify version consistency between Git tag and `Cargo.toml`
   - Create a GitHub Release with auto-generated changelog
   - Publish to crates.io (requires manual approval in GitHub Actions)

### Manual Release Trigger

You can also trigger a release manually using GitHub Actions:
1. Go to the [Actions tab](https://github.com/junjiangao/wim-parser/actions)
2. Select "Release and Publish" workflow
3. Click "Run workflow" and specify the version

### Release Requirements

- Version must follow semantic versioning (e.g., `v0.1.0`)
- Git tag version must match `Cargo.toml` version
- All tests must pass
- Code must be properly formatted and pass clippy checks
- Package must build successfully

## Changelog

### 0.1.0

- Initial release
- Basic WIM file parsing
- Image information extraction
- Windows version and architecture detection
- Optional logging support