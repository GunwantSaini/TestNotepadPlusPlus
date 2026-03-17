# Installer Build Instructions

This directory contains installer scripts and resources for distributing Notepad++ Rust Edition.

## Windows Installer (NSIS)

### Prerequisites

1. Install NSIS (Nullsoft Scriptable Install System):
   - Download from: https://nsis.sourceforge.io/Download
   - Version 3.0 or later required
   - Install the EnVar plugin: https://nsis.sourceforge.io/EnVar_plug-in

2. Build the release binary:
   ```bash
   cargo build --release
   ```

3. Create application assets (optional but recommended):
   - `assets/icon.ico` - Application icon (256x256)
   - `assets/header.bmp` - Installer header image (150x57)
   - `assets/wizard.bmp` - Installer wizard image (164x314)

### Building the Installer

From the project root directory:

```bash
# On Windows with NSIS installed
makensis installer\windows\notepad-plus-rust.nsi

# Or from the installer directory
cd installer\windows
makensis notepad-plus-rust.nsi
```

The installer will be created as `notepad-plus-rust-setup.exe` in the `installer/windows/` directory.

### Installer Features

The Windows installer provides:

- ✅ Installation of main executable
- ✅ Start Menu shortcuts
- ✅ Optional Desktop shortcut
- ✅ Optional PATH registration (command-line access)
- ✅ Optional file associations (.txt, .md, .rs, etc.)
- ✅ Automatic uninstaller
- ✅ Upgrade handling (removes old version)
- ✅ Windows registry integration

### File Associations

The installer can associate the following file types with Notepad++ Rust:
- `.txt` - Plain text files
- `.log` - Log files
- `.md` - Markdown files
- `.rs` - Rust source files
- `.toml` - TOML configuration files
- `.json` - JSON files
- `.xml` - XML files
- `.yml`, `.yaml` - YAML files

Users can choose to enable/disable this during installation.

## Linux Distribution

### Desktop Integration

The `.desktop` file provides Linux desktop environment integration:

```bash
# Install system-wide (requires root)
sudo cp linux/notepad-plus-rust.desktop /usr/share/applications/

# Install for current user only
cp linux/notepad-plus-rust.desktop ~/.local/share/applications/

# Update desktop database
update-desktop-database ~/.local/share/applications/
```

### Package Distribution

Consider creating distribution-specific packages:

**Debian/Ubuntu (.deb):**
```bash
# Using cargo-deb
cargo install cargo-deb
cargo deb
```

**Fedora/RHEL (.rpm):**
```bash
# Using cargo-rpm
cargo install cargo-rpm
cargo rpm build
```

**Arch Linux (PKGBUILD):**
See `linux/PKGBUILD` for Arch User Repository (AUR) package definition.

**AppImage (Universal):**
```bash
# Using cargo-appimage
cargo install cargo-appimage
cargo appimage
```

## macOS Distribution

### App Bundle (.app)

For macOS distribution, create a `.app` bundle:

```bash
# Using cargo-bundle
cargo install cargo-bundle
cargo bundle --release
```

The bundle will be created in `target/release/bundle/osx/`.

### DMG Distribution

Create a disk image for easy distribution:

```bash
# Using create-dmg
brew install create-dmg
create-dmg \
  --volname "Notepad++ Rust" \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --app-drop-link 600 185 \
  notepad-plus-rust.dmg \
  target/release/bundle/osx/
```

## Release Checklist

Before creating installers for distribution:

- [ ] Update version number in `Cargo.toml`
- [ ] Update version in `installer/windows/notepad-plus-rust.nsi`
- [ ] Update `CHANGELOG.md` with release notes
- [ ] Run full test suite: `cargo test --workspace`
- [ ] Build release binary: `cargo build --release`
- [ ] Test installer on clean VM
- [ ] Sign executables (Windows/macOS)
- [ ] Create checksums: `sha256sum notepad-plus-rust-setup.exe`
- [ ] Upload to GitHub Releases
- [ ] Update documentation

## Code Signing

### Windows

Use SignTool from Windows SDK:

```cmd
signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com notepad-plus-rust-setup.exe
```

### macOS

Use codesign:

```bash
codesign --force --deep --sign "Developer ID Application: Your Name" Notepad++\ Rust.app
```

## Automated Builds

The GitHub Actions CI/CD pipeline (`.github/workflows/release.yml`) automatically:
- Builds release binaries for all platforms
- Creates installers
- Uploads artifacts to GitHub Releases
- Generates checksums

Trigger a release build by creating a new tag:
```bash
git tag -a v8.0.0 -m "Release version 8.0.0"
git push origin v8.0.0
```
