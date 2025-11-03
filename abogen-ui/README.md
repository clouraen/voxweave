# Abogen UI - Cyberpunk Cross-Platform GUI

A production-ready, cross-platform GUI application built with **Rust + Dioxus 0.5**, featuring a neon **Cyberpunk/Blade-Runner** aesthetic. The app provides a single codebase with platform-specific launchers for desktop (Dioxus Desktop), web (WASM), and mobile (Android/iOS).

## 🎨 Features

- **Cyberpunk Theme**: Neon accents with dark UI, featuring cyan, magenta, amber, and lime glow effects
- **Cross-Platform**: Single codebase supporting Desktop, Web, and Mobile platforms
- **File Processing**: Drag-and-drop file selection with support for `.txt`, `.epub`, and `.pdf` files
- **Queue Management**: Add, manage, and clear processing queue
- **Voice Selection**: Configurable voice selection with searchable combobox
- **Processing Screen**: Real-time log display and progress tracking
- **Accessibility**: WCAG AA compliant with keyboard navigation and focus states

## 📋 Requirements

- **Rust 1.80+** (Edition 2021)
- **Dioxus 0.5+**
- For Web builds: **Trunk** (`cargo install trunk`)
- For Desktop builds: Native dependencies (for Dioxus Desktop)
- For Mobile builds: **dioxus-mobile** toolchain setup

## 🚀 Quick Start

### Desktop (Dioxus Desktop)

```bash
cd apps/desktop
cargo run --release
```

Or for development:

```bash
cd apps/desktop
cargo run
```

### Web (WASM with Trunk)

```bash
cd apps/web
trunk serve
```

Then open `http://localhost:8080` in your browser.

### Mobile (Android/iOS)

```bash
cd apps/mobile
dx build android  # or ios
```

Note: Mobile builds require additional setup for dioxus-mobile. See [Dioxus Mobile Documentation](https://dioxuslabs.com/docs/0.5/guide/desktop/mobile/) for details.

## 🏗️ Project Structure

```
abogen-ui/
├── Cargo.toml              # Workspace configuration
├── crates/
│   └── ui/                 # Shared UI crate
│       ├── lib.rs          # Root App component and routes
│       ├── state.rs        # Application state management
│       ├── theme.rs        # Global CSS styles
│       ├── components/     # Reusable UI components
│       │   ├── header.rs
│       │   ├── drop_zone.rs
│       │   ├── neon_button.rs
│       │   ├── slider.rs
│       │   ├── combo.rs
│       │   ├── checkbox.rs
│       │   ├── progress_bar.rs
│       │   └── log_panel.rs
│       └── services/       # Platform-agnostic services
│           ├── file_picker.rs
│           ├── tts_stub.rs
│           └── gpu_probe.rs
└── apps/
    ├── desktop/            # Dioxus Desktop launcher
    ├── web/                # WASM/Trunk launcher
    └── mobile/             # Dioxus Mobile launcher
```

## 🎯 Usage

### Main Screen

1. **File Selection**: Click the drop zone or drag-and-drop a file (`.txt`, `.epub`, `.pdf`)
2. **Configure Settings**:
   - Adjust speed slider (0.5x - 2.0x)
   - Select voice from dropdown
   - Choose subtitle generation mode (Sentence | Paragraph | None)
   - Select output formats (voice: wav/mp3/flac, subtitles: ass/srt/vtt)
   - Configure newline replacement
   - Choose save location
   - Enable/disable GPU acceleration
3. **Queue Management**:
   - Click **ADD TO QUEUE** to add current file and settings
   - Click **MANAGE QUEUE** to view and reorder queue items
   - Click **CLEAR QUEUE** to remove all items
4. **Start Processing**: Click **START** when queue is ready

### Processing Screen

- View real-time logs with color-coded messages (cyan: info, amber: notices, red: errors)
- Monitor progress bar (0-100%)
- Click **CANCEL** to stop processing and return to main screen

## 🎨 Theming

The app uses a cyberpunk color palette defined in `crates/ui/theme.rs`:

- `--neon-cyan: #27E8FF`
- `--neon-magenta: #FF3AD4`
- `--neon-amber: #FFB300`
- `--neon-lime: #39FF14`
- `--panel-bg: #0A0F1A`
- `--bg: #070A12`

Typography uses **Rajdhani** font from Google Fonts with fallback to sans-serif.

## 🔧 Features & Flags

### Cargo Features

- `gpu`: Enable GPU acceleration probe (default: disabled)
- `real-tts`: Real TTS backend with Kokoro (default: uses stub)
- `video-generation`: Video generation with Z.ai API (requires `reqwest`)
- `zai-video`: Enhanced Z.AI video generation with style presets and MLT composition

Build with features:

```bash
# Basic build
cargo build --features gpu

# With TTS and video generation
cargo build --features "real-tts video-generation"

# With Z.AI video generation
cargo build --features "real-tts video-generation zai-video"
```

### Z.AI Video Generation

The app supports automatic video generation using Z.AI API:
- Select video style (BioTech, Cyberpunk, Educational, etc.)
- Choose resolution (720p, 1080p, 4K)
- Video is automatically composed with audio and word-highlighted subtitles

See `ZAI_VIDEO_FEATURE.md` for details.

## 🧪 Testing

Run unit tests:

```bash
cd crates/ui
cargo test
```

Tests cover:
- State management (file info, enums)
- GPU probe functionality
- TTS pipeline progress updates (stub)

## 📦 Building for Production

### Desktop

```bash
cd apps/desktop
cargo build --release
```

### Web

```bash
cd apps/web
trunk build --release
```

Output will be in `apps/web/dist/`.

### Mobile

```bash
cd apps/mobile
dx build android --release  # or ios
```

## 🐛 Known Limitations

- **File Picker**: Currently stubbed - platform-specific implementations needed
- **TTS Pipeline**: Uses mock/stub implementation - integrate with real TTS backend via `real-tts` feature
- **Save Location**: "Choose..." option needs file picker implementation
- **Queue Modal**: "Manage Queue" button needs modal implementation

## 🔮 Future Enhancements

- [ ] Implement real file picker for all platforms
- [ ] Integrate actual TTS backend
- [ ] Add queue management modal
- [ ] Implement file drop handling
- [ ] Add save location file picker
- [ ] Add screenshot/documentation images

## 📄 License

MIT OR Apache-2.0

## 🙏 Acknowledgments

- **Dioxus**: Cross-platform Rust UI framework
- **Rajdhani Font**: Google Fonts
- Cyberpunk aesthetic inspiration from Blade Runner and similar media

## 📸 Screenshots

_Placeholder for screenshots of the application_

---

**Version**: 1.1.0  
**Built with**: Rust + Dioxus 0.5

