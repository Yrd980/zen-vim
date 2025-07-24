# 🧘 Zen-Vim

> **A minimalist Vim-like terminal editor written in Rust**, inspired by Neovim + Snacks.nvim philosophy.

Zen-Vim is a from-scratch terminal text editor that brings the zen of minimal UI with the power of modal editing. Built for speed, clarity, and hackability.

---

## ✨ Features

- 🚀 **Modal Editing**: Full Vim-like Normal/Insert/Visual modes
- 🔎 **Smart Pickers**: File finder, live grep, buffer switcher
- 💾 **Session Management**: Auto-save and restore your workspace
- ✍️ **File Operations**: Rename, save, manage multiple buffers
- 🎯 **Minimal UI**: Clean terminal interface, no bloat
- 🧩 **Extensible**: Built with modularity in mind
- ⚡ **Fast**: Rust performance with async file operations

---

## 🏗️ Architecture

```
zen-vim/
├── src/
│   ├── main.rs          # Entry point & CLI
│   ├── app.rs           # Main application loop
│   ├── config.rs        # TOML configuration
│   ├── modes/           # Modal editing system
│   ├── core/            # Buffer & cursor management
│   │   ├── buffer.rs    # Text editing operations
│   │   ├── cursor.rs    # Position tracking
│   │   └── session.rs   # Save/restore state
│   ├── ui/              # Terminal rendering
│   │   ├── dashboard.rs # Zen startup screen
│   │   └── mod.rs       # Editor UI
│   └── picker.rs        # File/grep/buffer pickers
├── Cargo.toml           # Rust dependencies
└── README.md
```

---

## 🛠️ Installation

### Prerequisites
- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs/)
- **ripgrep** (optional): For faster text search

### Build from Source

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/zen-vim
cd zen-vim

# Build release binary
cargo build --release

# Install to system (optional)
cargo install --path .

# Or run directly
cargo run
```

---

## 🧪 Usage

### Launch the Editor

```bash
# Start with dashboard
zen-vim

# Open specific files
zen-vim file1.txt file2.rs

# Show dashboard even with files
zen-vim --dashboard file.txt

# Enable debug logging
zen-vim --debug
```

### Keybindings

**Normal Mode** (Default):
| Key | Action |
|-----|--------|
| `h/j/k/l` | Move cursor left/down/up/right |
| `w/b` | Move word forward/backward |
| `0/$` | Move to line start/end |
| `g/G` | Move to file start/end |
| `i/a` | Enter insert mode (before/after cursor) |
| `I/A` | Enter insert mode (line start/end) |
| `o/O` | New line below/above and insert |
| `v` | Enter visual mode |
| `x` | Delete character |
| `d` | Delete line |
| `u` | Undo |
| `Ctrl+r` | Redo |

**Leader Key Commands** (`<space>`):
| Combination | Action |
|-------------|--------|
| `<space>pf` | Find Files |
| `<space>pt` | Grep Text |
| `<space>pb` | Buffer List |
| `<space>pr` | Resume Session |
| `<space>rn` | Rename File |
| `<space>d` | Show Dashboard |
| `<space>q` | Quit |

**Insert Mode**:
- `Esc` - Return to Normal mode
- Regular typing, Enter, Backspace, etc.

---

## ⚙️ Configuration

Zen-vim creates a config file at `~/.config/zen-vim/config.toml`:

```toml
[ui]
theme = "zen"
show_line_numbers = false
show_status_line = false
tab_width = 2
wrap_lines = false

[keymaps]
leader = " "
timeout_ms = 1000

[picker]
file_ignore_patterns = [".git", "node_modules", "target", "*.pyc"]
max_results = 100
preview_enabled = true

[dashboard]
show_recent_files = true
max_recent_files = 5
custom_header = ""
```

---

## 🧠 Philosophy

Zen-Vim follows the **"Less is More"** principle:

- **Minimal by Design**: No unnecessary UI elements
- **Modal Efficiency**: Vim-style editing for speed
- **Focused Experience**: Distraction-free text editing
- **Extensible Core**: Build features without bloat

Instead of feature creep, we prioritize:
- ⚡ **Performance**: Fast startup and operations
- 🎯 **Clarity**: Clean, readable interface
- 🔧 **Stability**: Reliable core functionality

---

## 🏗️ Development

### Running Tests
```bash
cargo test
```

### Code Structure
- **Modal System**: Vim-like modes in `src/modes/`
- **Buffer Management**: Text operations in `src/core/buffer.rs`
- **UI Rendering**: Terminal UI with Ratatui in `src/ui/`
- **Picker System**: File/grep functionality in `src/picker.rs`

### Adding Features
1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests
5. Submit a pull request

---

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Built with ❤️ in Rust** | Inspired by Vim, Neovim, and the zen of simplicity