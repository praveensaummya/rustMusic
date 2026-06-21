# 🎵 RustMusic Player

A cross-platform music player built with Rust, featuring a modern GUI, theme support, and persistent preferences.

![Rust](https://img.shields.io/badge/rust-1.96%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)

## Features

- 🎨 **5 Built-in Themes** - Dark, Light, Midnight, Ocean, Forest
- 🔄 **Persistent Preferences** - Volume, theme, and last folder saved automatically
- 📂 **Folder Scanning** - Load entire music directories at once
- 🔍 **Live Search** - Filter songs by title, artist, or album
- 🎵 **Audio Formats** - MP3, WAV, FLAC, OGG, M4A, AAC, WMA
- 🔀 **Shuffle & Repeat** - Randomized playback and loop modes
- ⏯ **Full Controls** - Play, pause, next, previous, seek, volume
- 📊 **Progress Tracking** - Draggable timeline with time display
- 🖱️ **Double-click to Play** - Quick playback from playlist
- 🔄 **Refresh Button** - Rescan folder for new files

## Performance Metrics

| Metric | Value |
|--------|-------|
| Binary Size | 21 MB (release, optimized) |
| Build Time | ~0.35s (incremental) |
| Source Lines | 1,598 LOC |
| Dependencies | 14 direct crates |
| Memory Usage | ~30-50 MB runtime |

## Installation

### Prerequisites

- Rust 1.70+ ([install Rust](https://www.rust-lang.org/tools/install))
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/praveensaummya/rustMusic.git
cd rustMusic

# Build release binary
cargo build --release

# Run the player
./target/release/rustmusic
```

### Quick Install (Linux)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/praveensaummya/rustMusic.git
cd rustMusic
cargo build --release
./target/release/rustmusic
```

## Usage

### Basic Controls

| Action | How |
|--------|-----|
| Open folder | Click **📂 Open Folder** |
| Add files | Click **➕ Add Files** |
| Play song | Double-click in playlist |
| Play/Pause | Click **▶/⏸** button |
| Next/Prev | Click **⏭/⏮** buttons |
| Seek | Drag progress slider |
| Volume | Drag volume slider |
| Search | Type in search bar |
| Refresh | Click **🔄 Refresh** (after opening folder) |
| Settings | Click **⚙️ Settings** |

### Settings Window

Access settings via the **⚙️ Settings** button:

- **Theme Selection** - Choose from 5 color themes
- **Default Volume** - Set startup volume (0-100%)
- **Last Session** - View previously opened folder
- **Save Preferences** - Manually save or auto-saves on exit

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Space | Play/Pause |
| Left Arrow | Previous track |
| Right Arrow | Next track |

## Configuration

Preferences are stored in:

- **Linux**: `~/.config/rustmusic/config.toml`
- **macOS**: `~/Library/Application Support/rustmusic/config.toml`
- **Windows**: `%APPDATA%\rustmusic\config.toml`

### Config File Format

```toml
theme = "Dark"
volume = 0.8
last_folder = "/path/to/music"
window_width = 1000.0
window_height = 700.0
```

## Architecture

```
src/
├── main.rs       # Entry point (23 LOC)
├── audio.rs      # Audio playback engine (310 LOC)
├── playlist.rs   # Playlist management (167 LOC)
├── theme.rs      # Theme definitions (206 LOC)
├── config.rs     # Config persistence (71 LOC)
└── ui.rs         # GUI rendering (821 LOC)
```

### Key Technologies

- **GUI**: [egui](https://github.com/emilk/egui) + [eframe](https://github.com/emilk/egui) - Immediate mode GUI
- **Audio**: [rodio](https://github.com/RustAudio/rodio) - Cross-platform audio playback
- **Dialogs**: [rfd](https://github.com/PolyMeilex/rfd) - Native file dialogs
- **Config**: [serde](https://github.com/serde-rs/serde) + [toml](https://github.com/toml-rs/toml) - Serialization
- **Paths**: [dirs](https://github.com/soc/dirs-rs) - Platform-specific directories

## Themes

| Theme | Description |
|-------|-------------|
| **Dark** | Classic dark theme (default) |
| **Light** | Clean light theme |
| **Midnight** | Deep blue-purple tones |
| **Ocean** | Blue-green aquatic theme |
| **Forest** | Green nature-inspired theme |

## Supported Audio Formats

- MP3 (MPEG Audio)
- WAV (Waveform Audio)
- FLAC (Free Lossless Audio Codec)
- OGG (Vorbis/Opus)
- M4A (AAC in MP4 container)
- AAC (Advanced Audio Coding)
- WMA (Windows Media Audio)

## Building

### Debug Build

```bash
cargo build
```

### Release Build (Optimized)

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Check Code

```bash
cargo check
cargo clippy
cargo fmt -- --check
```

## Troubleshooting

### No Audio Output

- Ensure your system has audio output configured
- Check that the audio file format is supported
- Try running `pavucontrol` (Linux) or equivalent to verify output

### Build Fails

- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build --release`
- Check dependencies: `cargo update`

### Config Not Saving

- Ensure `~/.config/rustmusic/` directory is writable
- Check disk space
- Run with appropriate permissions

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Releases

This project uses GitHub Actions for automated cross-platform releases.

### Creating a Release

1. Update the version in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"  # Update this
   ```

2. Commit and push changes:
   ```bash
   git add .
   git commit -m "Release v0.2.0"
   git push origin main
   ```

3. Create and push a git tag:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. GitHub Actions will automatically:
   - Build binaries for Linux, macOS, and Windows
   - Strip debug symbols for smaller binaries
   - Create a GitHub Release with all binaries attached

### Downloading Releases

Visit the [Releases page](https://github.com/praveensaummya/rustMusic/releases) to download:

- `rustmusic-linux-x64` - Linux binary
- `rustmusic-macos-x64` - macOS binary
- `rustmusic-windows-x64` - Windows executable

No installation required - just download, make executable (Linux/macOS), and run!

### Manual Build

If you prefer to build locally:

```bash
git clone https://github.com/praveensaummya/rustMusic.git
cd rustMusic
cargo build --release
# Binary location: target/release/rustmusic
```

## Roadmap

- [ ] Playlist save/load functionality
- [ ] Album art display
- [ ] Lyrics integration
- [ ] Equalizer
- [ ] Crossfade between tracks
- [ ] Last.fm/Spotify scrobbling
- [ ] Podcast support
- [ ] Internet radio streams
- [ ] System tray integration
- [ ] Global hotkeys

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [RustAudio](https://github.com/RustAudio) for audio libraries
- [egui](https://github.com/emilk/egui) for the amazing GUI framework
- All contributors and testers

---

**Built with ❤️ using Rust**

*RustMusic Player - Fast, reliable, cross-platform music playback*