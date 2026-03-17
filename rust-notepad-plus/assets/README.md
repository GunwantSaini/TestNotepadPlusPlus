# Application Assets

This directory contains graphical assets for Notepad++ Rust Edition.

## Required Assets

### Windows

1. **icon.ico** - Application icon
   - Multi-resolution ICO file (16x16, 32x32, 48x48, 256x256)
   - Used for executable icon and installer
   - Create with: ImageMagick, GIMP, or online converter

2. **header.bmp** - Installer header image
   - Size: 150x57 pixels
   - 24-bit BMP format
   - Displayed at top of NSIS installer pages

3. **wizard.bmp** - Installer wizard image
   - Size: 164x314 pixels
   - 24-bit BMP format
   - Displayed on left side of NSIS installer

### Linux

4. **icon.png** - Application icon
   - Recommended sizes: 16x16, 32x32, 48x48, 64x64, 128x128, 256x256
   - Install to: `/usr/share/icons/hicolor/{size}/apps/notepad-plus-rust.png`
   - SVG version also recommended: `icon.svg`

5. **icon.svg** - Scalable application icon
   - Vector format for any size
   - Install to: `/usr/share/icons/hicolor/scalable/apps/notepad-plus-rust.svg`

### macOS

6. **icon.icns** - macOS application icon
   - Multi-resolution ICNS file
   - Sizes: 16, 32, 64, 128, 256, 512, 1024 (all @1x and @2x)
   - Create with: `iconutil` or `png2icns`

## Creating Icons

### From SVG Source

If you have an SVG source icon (`icon.svg`), generate all formats:

**Windows ICO:**
```bash
# Using ImageMagick
convert icon.svg -define icon:auto-resize=256,128,96,64,48,32,16 icon.ico
```

**Linux PNG (multiple sizes):**
```bash
# Generate all required sizes
for size in 16 32 48 64 128 256; do
  convert icon.svg -resize ${size}x${size} icon-${size}.png
done
```

**macOS ICNS:**
```bash
# Create iconset directory
mkdir icon.iconset

# Generate all sizes
for size in 16 32 64 128 256 512; do
  convert icon.svg -resize ${size}x${size} icon.iconset/icon_${size}x${size}.png
  convert icon.svg -resize $((size*2))x$((size*2)) icon.iconset/icon_${size}x${size}@2x.png
done

# Convert to ICNS
iconutil -c icns icon.iconset -o icon.icns
```

### BMP for NSIS Installer

**Header image (150x57):**
```bash
convert header-source.png -resize 150x57 -background white -gravity center -extent 150x57 header.bmp
```

**Wizard image (164x314):**
```bash
convert wizard-source.png -resize 164x314 -background white -gravity center -extent 164x314 wizard.bmp
```

## Icon Design Guidelines

### Style
- Modern, flat design
- Simple and recognizable at small sizes
- Clear silhouette
- Professional appearance

### Colors
- Primary: Blue/Purple tones (to differentiate from original Notepad++)
- Accent: Rust orange (to represent Rust programming language)
- Background: Transparent or solid color

### Content
- Consider incorporating:
  - Document/page icon base
  - "++", "Rust", or "R" symbol
  - Cursor/editing indicator
  - Gear/cog (for developer focus)

### Platform Consistency
- Follow platform-specific icon guidelines:
  - **Windows**: Microsoft Fluent Design
  - **macOS**: Apple Human Interface Guidelines
  - **Linux**: freedesktop.org icon theme specification

## Placeholder Icon

Until custom icons are created, you can use a text-based placeholder:

```bash
# Create a simple placeholder icon
convert -size 256x256 xc:navy \
  -font Arial -pointsize 72 -fill white \
  -gravity center -annotate +0+0 'N++' \
  -alpha set icon.png

# Convert to ICO
convert icon.png -define icon:auto-resize=256,128,96,64,48,32,16 icon.ico
```

## Installation

### Embedding in Executable (Windows)

Add to `Cargo.toml`:

```toml
[package.metadata.winresource]
OriginalFilename = "notepad-plus.exe"
ProductName = "Notepad++ Rust Edition"
FileDescription = "Advanced Text Editor"
CompanyName = "Notepad++ Rust Contributors"
LegalCopyright = "GPL-3.0"
# Path to icon file
ico = "assets/icon.ico"
```

Then install and use `winresource`:

```bash
cargo install winresource
```

Build script (`build.rs`):

```rust
#[cfg(windows)]
extern crate winresource;

fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().unwrap();
    }
}
```

### Linux Icon Installation

```bash
# Install PNG icons (all sizes)
for size in 16 32 48 64 128 256; do
  sudo install -Dm644 assets/icon-${size}.png \
    /usr/share/icons/hicolor/${size}x${size}/apps/notepad-plus-rust.png
done

# Install SVG icon
sudo install -Dm644 assets/icon.svg \
  /usr/share/icons/hicolor/scalable/apps/notepad-plus-rust.svg

# Update icon cache
sudo gtk-update-icon-cache /usr/share/icons/hicolor/
```

## Resources

- **Icon design tools:**
  - Inkscape (SVG): https://inkscape.org/
  - GIMP (raster): https://www.gimp.org/
  - Figma (design): https://www.figma.com/

- **Icon conversion:**
  - ImageMagick: https://imagemagick.org/
  - iconutil (macOS built-in)
  - png2icons: https://github.com/idesis-gmbh/png2icons

- **Design inspiration:**
  - Material Design Icons: https://materialdesignicons.com/
  - Fluent UI System Icons: https://github.com/microsoft/fluentui-system-icons
  - Feather Icons: https://feathericons.com/
