# Notepad++ Rust Edition - User Manual

Version 8.0.0

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Basic Features](#basic-features)
4. [File Operations](#file-operations)
5. [Editing Features](#editing-features)
6. [Search and Replace](#search-and-replace)
7. [Settings and Configuration](#settings-and-configuration)
8. [Keyboard Shortcuts](#keyboard-shortcuts)
9. [Advanced Features](#advanced-features)
10. [Troubleshooting](#troubleshooting)
11. [FAQ](#faq)

---

## Introduction

### What is Notepad++ Rust Edition?

Notepad++ Rust Edition is a modern, high-performance text and source code editor written in Rust. It provides a faithful port of the classic Notepad++ experience with enhanced performance, safety, and cross-platform capabilities.

### Key Features

- ✅ **Fast and Lightweight** - Written in Rust for maximum performance
- ✅ **Syntax Highlighting** - Support for 80+ programming languages
- ✅ **Multi-Document Interface** - Work with multiple files simultaneously
- ✅ **Powerful Search** - Regular expressions, find and replace
- ✅ **Customizable** - Themes, plugins, and extensive settings
- ✅ **Cross-Platform** - Windows, Linux, and macOS support (planned)
- ✅ **Open Source** - GPL-3.0 licensed

### System Requirements

**Windows:**
- Windows 10 or later (64-bit)
- 50 MB free disk space
- 256 MB RAM (minimum)

**Linux:**
- Modern Linux distribution
- GTK 4.0 or later
- 50 MB free disk space

---

## Getting Started

### Installation

**Windows:**
1. Download `notepad-plus-rust-setup.exe`
2. Run the installer
3. Follow the installation wizard
4. Launch from Start Menu or Desktop shortcut

**Linux (Arch/Manjaro):**
```bash
# Install from AUR
yay -S notepad-plus-rust

# Or manually
git clone https://aur.archlinux.org/notepad-plus-rust.git
cd notepad-plus-rust
makepkg -si
```

**Linux (Other Distributions):**
```bash
# Download and extract binary
tar -xzf notepad-plus-rust-linux-x64.tar.gz
sudo mv notepad-plus /usr/local/bin/
sudo mv notepad-plus-rust.desktop /usr/share/applications/
```

### First Launch

When you first launch Notepad++ Rust Edition:

1. The application opens with an empty document
2. Default settings are loaded (tab size: 4, Windows line endings)
3. You can immediately start typing or open existing files

### Command Line Usage

Launch from the command line:

```bash
# Open a file
notepad-plus myfile.txt

# Open multiple files
notepad-plus file1.txt file2.rs file3.json

# Create a new file
notepad-plus newfile.md
```

---

## Basic Features

### The User Interface

```
┌─────────────────────────────────────────┐
│  File  Edit  Search  View  Help     [_][□][×]
├─────────────────────────────────────────┤
│  New  Open  Save  Undo  Redo  Find ...  │ ← Toolbar
├─────────────────────────────────────────┤
│                                         │
│                                         │
│              Editor Area                │ ← Main editing space
│                                         │
│                                         │
├─────────────────────────────────────────┤
│  Ln: 1  Col: 1  |  UTF-8  |  Windows    │ ← Status bar
└─────────────────────────────────────────┘
```

**Menu Bar:** Access all features and commands
**Toolbar:** Quick access to common operations
**Editor Area:** Main text editing workspace
**Status Bar:** Shows line, column, encoding, and EOL format

### Creating a New File

1. **File → New** or press `Ctrl+N`
2. Start typing in the new document
3. Save with **File → Save** (`Ctrl+S`)

### Opening Files

**Method 1: Menu**
1. **File → Open** or press `Ctrl+O`
2. Browse to your file
3. Click "Open"

**Method 2: Drag and Drop**
- Drag files from Explorer directly into the editor

**Method 3: Recent Files**
- **File → Recent Files** shows last 10 opened files

### Saving Files

**Save:**
- **File → Save** or `Ctrl+S`
- Saves current file

**Save As:**
- **File → Save As** or `Ctrl+Shift+S`
- Save with a new name or location

**Save All:**
- **File → Save All**
- Saves all open documents

---

## File Operations

### File Encoding

Notepad++ Rust Edition supports multiple encodings:

- **ANSI** - Legacy single-byte encoding
- **UTF-8** - Default, no BOM
- **UTF-8 BOM** - UTF-8 with byte order mark
- **UTF-16 LE** - Little-endian 16-bit Unicode
- **UTF-16 BE** - Big-endian 16-bit Unicode

**Change Encoding:**
1. **File → Encoding → [Select encoding]**
2. File will be saved in new encoding

### Line Endings

Three formats supported:

- **Windows (CRLF)** - `\r\n` - Default
- **Unix (LF)** - `\n` - Linux/macOS
- **Mac (CR)** - `\r` - Classic Mac OS

**Change Line Endings:**
1. **Edit → EOL Conversion → [Select format]**
2. Choose Windows, Unix, or Mac
3. Status bar shows current format

### Auto-Detection

- File encoding is automatically detected on open
- Line ending format is detected and preserved
- Mixed line endings trigger a warning

---

## Editing Features

### Basic Editing

**Undo/Redo:**
- `Ctrl+Z` - Undo last action
- `Ctrl+Y` - Redo last undone action
- Unlimited undo history

**Cut/Copy/Paste:**
- `Ctrl+X` - Cut selected text
- `Ctrl+C` - Copy selected text
- `Ctrl+V` - Paste from clipboard

**Select All:**
- `Ctrl+A` - Select entire document

### Text Selection

**Mouse Selection:**
- Click and drag to select text
- Double-click to select word
- Triple-click to select line

**Keyboard Selection:**
- `Shift + Arrow keys` - Extend selection
- `Ctrl+Shift+Left/Right` - Select by word
- `Shift+Home/End` - Select to line start/end

### Indentation

**Tab Settings:**
- Default: 4 spaces (configurable)
- Use tabs or spaces (toggle in settings)

**Indent/Unindent:**
- `Tab` - Indent selected lines
- `Shift+Tab` - Unindent selected lines

**Auto-Indent:**
- Automatically matches indentation of previous line
- Language-aware indentation (planned)

---

## Search and Replace

### Find

**Basic Find:**
1. Press `Ctrl+F` or **Search → Find**
2. Enter search text
3. Click "Find Next" or press `Enter`
4. Click "Find Previous" for reverse search

**Find Options:**
- ☑ **Match case** - Case-sensitive search
- ☑ **Whole word** - Match complete words only
- ☑ **Regular expression** - Use regex patterns
- ☑ **Wrap around** - Continue from document start/end

### Replace

**Find and Replace:**
1. Press `Ctrl+H` or **Search → Replace**
2. Enter find and replace text
3. Options:
   - **Replace** - Replace current match
   - **Replace All** - Replace all occurrences
   - **Replace in Selection** - Limited to selection

### Regular Expressions

Notepad++ Rust Edition supports standard regex syntax:

**Common Patterns:**
- `.` - Any character
- `*` - Zero or more
- `+` - One or more
- `?` - Zero or one
- `\d` - Digit
- `\w` - Word character
- `\s` - Whitespace
- `^` - Line start
- `$` - Line end

**Example Searches:**
```regex
\d{3}-\d{4}        # Phone number (XXX-XXXX)
[A-Z][a-z]+        # Capitalized word
^\s*$              # Empty line
https?://\S+       # URL
```

**Capture Groups:**
```regex
Find:    (\w+)\s+(\w+)
Replace: $2 $1
Result:  Swaps first two words
```

### Find in Files

**Search Multiple Files:**
1. **Search → Find in Files** or `Ctrl+Shift+F`
2. Enter search pattern
3. Select directory
4. Specify file filters (e.g., `*.txt,*.md`)
5. Click "Find All"
6. Results appear in search panel

---

## Settings and Configuration

### General Settings

Access via **File → Preferences**

**Interface:**
- Theme (Light/Dark)
- Font family and size
- Line numbers
- Word wrap
- Whitespace visibility

**Editor:**
- Tab size (default: 4)
- Use tabs or spaces
- Auto-indent
- Line endings (Windows/Unix/Mac)
- Default encoding (UTF-8)

**Backup:**
- Enable/disable auto-backup
- Backup directory location
- Backup on save

**Session:**
- Remember open files
- Restore session on startup
- Multi-instance mode

### Themes

**Built-in Themes:**
- Default (Light)
- Dark
- High Contrast

**Custom Themes:**
- Place theme files in `themes/` directory
- **Settings → Style Configurator**
- Select language and customize colors

### File Associations

**Windows:**
- Automatically configured during installation
- **File → Preferences → File Association**
- Select file types to associate

**Linux:**
- Configured via `.desktop` file
- `~/.local/share/applications/mimeapps.list`
- Set as default with: `xdg-mime default notepad-plus-rust.desktop text/plain`

---

## Keyboard Shortcuts

### File Operations

| Shortcut | Action |
|----------|--------|
| `Ctrl+N` | New file |
| `Ctrl+O` | Open file |
| `Ctrl+S` | Save file |
| `Ctrl+Shift+S` | Save As |
| `Ctrl+W` | Close file |
| `Ctrl+Shift+W` | Close all |
| `Ctrl+Tab` | Next document |
| `Ctrl+Shift+Tab` | Previous document |

### Editing

| Shortcut | Action |
|----------|--------|
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `Ctrl+X` | Cut |
| `Ctrl+C` | Copy |
| `Ctrl+V` | Paste |
| `Ctrl+A` | Select all |
| `Ctrl+D` | Duplicate line |
| `Ctrl+L` | Delete line |
| `Ctrl+T` | Swap line up |

### Search

| Shortcut | Action |
|----------|--------|
| `Ctrl+F` | Find |
| `Ctrl+H` | Replace |
| `F3` | Find next |
| `Shift+F3` | Find previous |
| `Ctrl+Shift+F` | Find in files |
| `Ctrl+G` | Go to line |

### View

| Shortcut | Action |
|----------|--------|
| `F11` | Full screen |
| `Ctrl++` | Zoom in |
| `Ctrl+-` | Zoom out |
| `Ctrl+0` | Reset zoom |

### Customizing Shortcuts

1. **Settings → Shortcut Mapper**
2. Select command category
3. Click on command to modify
4. Press new key combination
5. Click "OK" to save

---

## Advanced Features

### Multi-Document Editing

**Working with Multiple Files:**
- All open files appear in tabs (future feature)
- `Ctrl+Tab` cycles through open documents
- Window title shows active file

**Comparing Files:**
- Open two files
- **Plugins → Compare** (planned)
- Side-by-side diff view

### Bookmarks

**Set Bookmark:**
- `Ctrl+F2` - Toggle bookmark on current line
- Blue marker appears in margin

**Navigate Bookmarks:**
- `F2` - Next bookmark
- `Shift+F2` - Previous bookmark
- `Ctrl+Shift+F2` - Clear all bookmarks

### Macros (Planned)

Record and playback repeated actions:
1. **Macro → Start Recording**
2. Perform actions
3. **Macro → Stop Recording**
4. **Macro → Playback** or `Ctrl+Shift+P`
5. **Macro → Save** to reuse later

### Plugins (Planned)

Extend functionality with plugins:
- **Plugins → Plugin Manager**
- Browse available plugins
- Install with one click
- Manage installed plugins

---

## Troubleshooting

### Common Issues

**Problem: File won't open**
- Check file permissions
- Verify file is not corrupted
- Try opening with different encoding

**Problem: Text appears garbled**
- Wrong encoding detected
- Try **File → Encoding → [different encoding]**
- Common fix: UTF-8 → UTF-8 BOM or ANSI

**Problem: Line endings inconsistent**
- File has mixed line endings
- **Edit → EOL Conversion → [select format]**
- **Edit → EOL Conversion → Convert to Windows/Unix**

**Problem: Application crashes**
- Update to latest version
- Check system requirements
- Disable plugins temporarily
- Reset settings: delete `%APPDATA%\Notepad++ Rust\config.xml`

### Performance Issues

**Large Files:**
- Files over 100MB may load slowly
- Disable syntax highlighting for huge files
- Consider using stream editor for massive files

**Memory Usage:**
- Close unused documents
- Disable plugins you don't need
- Restart application periodically

### Reporting Bugs

Found a bug? Help us improve:

1. Check existing issues: https://github.com/yourusername/notepad-plus-rust/issues
2. Create new issue with:
   - Operating system and version
   - Notepad++ Rust version
   - Steps to reproduce
   - Expected vs actual behavior
   - Screenshots if applicable
3. Include log file from `%APPDATA%\Notepad++ Rust\logs\`

---

## FAQ

**Q: Is this a replacement for the original Notepad++?**
A: Notepad++ Rust Edition is a modern port that aims to provide the same experience with enhanced performance and cross-platform support. It's an alternative, not a replacement.

**Q: Will my original Notepad++ plugins work?**
A: No. Notepad++ Rust Edition uses a different plugin architecture. Plugins need to be ported to the Rust plugin API.

**Q: How do I migrate from original Notepad++?**
A: Settings and themes are not directly compatible. You'll need to reconfigure your preferences manually.

**Q: Is it faster than the original?**
A: Generally yes, especially for large files and complex regex searches, thanks to Rust's performance and safety guarantees.

**Q: Can I use it on Linux?**
A: Yes! Linux support is available via GTK4 backend (in development). Windows is fully supported now.

**Q: Is it open source?**
A: Yes, licensed under GPL-3.0, same as the original Notepad++.

**Q: How do I contribute?**
A: See CONTRIBUTING.md in the source repository. We welcome bug reports, feature requests, and pull requests!

**Q: Where are settings stored?**

**Windows:**
```
%APPDATA%\Notepad++ Rust\
├── config.xml        # Application settings
├── session.xml       # Session file
├── themes/          # Custom themes
└── plugins/         # Installed plugins
```

**Linux:**
```
~/.config/notepad-plus-rust/
├── config.toml      # Application settings
├── session.toml     # Session file
├── themes/          # Custom themes
└── plugins/         # Installed plugins
```

---

## Getting Help

### Resources

- **Documentation:** https://docs.notepad-plus-rust.org
- **Issue Tracker:** https://github.com/yourusername/notepad-plus-rust/issues
- **Discussions:** https://github.com/yourusername/notepad-plus-rust/discussions
- **Discord:** https://discord.gg/notepad-plus-rust

### Support

For support questions:
1. Check this manual
2. Search existing issues and discussions
3. Ask in Discord community
4. Create a GitHub discussion

For bug reports:
1. Use GitHub issue tracker
2. Follow the bug report template
3. Include all requested information

---

## License

Notepad++ Rust Edition is free and open source software licensed under GPL-3.0.

Copyright © 2024 Notepad++ Rust Contributors

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

---

**Last Updated:** 2024-03-17
**Version:** 8.0.0
**Manual Version:** 1.0
