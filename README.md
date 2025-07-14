**Rust Image Compress** is a simple command-line utility written in Rust to convert and compress images (PNG, JPG, JPEG, WebP) into the high-efficiency AVIF format.

## Why ?
Because I needed a batch compression tool for my wallpapers and the GIMP plugin I was using stopped working.

## Features

- Fully offline, no need to be ashamed of your waifu wallpapers
- Supports multiple input formats: PNG, JPG, JPEG, and WebP
- Configurable compression via quality and speed
- Batch processing of all images in a directory
- Rustacean speed with multi-thread support
- Tested on Linux and Windows

## Requirements

- Rust toolchain

## Building

1. Clone the repository:
   ```bash
   git clone https://github.com/JuanPardos/rust-image-compress
   cd rust-image-compress
   ```

2. Build in release mode:
   ```bash
   cargo build --release
   ```

3. The executable will be located at:
   ```bash
   target/release/rust-image-compress
   ```

### Binaries for Linux and Windows are also available in the Releases section

## Usage

Run the program and follow instructions:

```bash
./rust-image-compress
```

You will be prompted for:

1. **Path to the images folder**: directory containing your images.
2. **Quality (1-100)**: sets the compression level. Higher values produce higher quality and larger output files. Default: 85.
3. **Speed (1-10)**: determines the encoding speed. Higher values increase speed but may negatively affect quality and file size. Default: 8.

Images are saved in the `output` folder.

## Notes

Be aware that the AVIF format is not supported everywhere. However, you can always recover the original format using other tools.

