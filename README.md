# Tapes

[![Rust](https://img.shields.io/badge/rust-2024_Edition-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Tapes** isan atomic dotfile switcher for ricing. It allows you to instantly switch between complete configuration setups (rices) with a single command, with automatic backups and rollback capability.

## Concept

A **tape** is a self-contained rice directory containing:
- A `.config/` folder with your configuration files
- An optional `tape.toml` manifest defining metadata, dependencies, and lifecycle hooks

When you **insert** a tape, your `~/.config` directory is atomically replaced with a symlink to the tape's `.config`. Your original configuration is automatically backed up. When you **eject**, your original config is restored just as quickly.

## Features

- **Atomic switching**: Your config is never in a half-state during insert/eject
- **Automatic backups**: Original `~/.config` is always preserved
- **Lifecycle hooks**: Run commands before/after insert and eject
- **Dependency checking**: Verify required binaries exist before switching
- **Clean uninstalls**: Generated files in tapes are cleaned up on eject
- **Colorful output**: Clear, colored terminal output
- **Immutable by default**: Tapes are symlinked, preserving the original

## Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/futurekismo/cassette-rices
cd cassette-rices

# Build and install
cargo install --path .
```

### Using Nix

If you have Nix/flakes configured:

```bash
# Clone and enter development shell
 git clone https://github.com/futurekismo/cassette-rices
cd cassette-rices

# Build with nix
nix build
```

## Quick Start

### 1. Create a Tape

Create a directory for your rice with a `.config/` folder:

```bash
mkdir -p ~/my-hypr-rice/.config/hypr
# Copy your hyprland config, waybar config, etc. into the .config folder
```

### 2. Add a Manifest (Optional)

Create a `tape.toml` file in your tape directory:

```toml
[tape]
name = "my-hypr-rice"
version = "0.1.0"
desc = "My Hyprland + Waybar setup"

[dependencies]
binaries = ["hyprland", "waybar", "foot"]

[hooks]
insert = [
  "echo 'Activating Hyprland rice...'",
  "pkill -9 waybar || true"
]
eject = [
  "echo 'Rice deactivated'",
  "killall waybar || true"
]
```

### 3. Try It Out

```bash
# Test the tape without replacing files (dry run)
tape insert --path ~/my-hypr-rice --dry-run

# Actually insert the tape
tape insert --path ~/my-hypr-rice

# Don't like it? Revert instantly
tape eject
```

### 4. Install Permanently (Optional)

Move your tape to the tapes directory for easy access:

```bash
# Install to the tapes library
mkdir -p ~/.local/share/tapes
cp -r ~/my-hypr-rice ~/.local/share/tapes/

# Now you can insert by name
tape insert my-hypr-rice
```

## Usage

### Commands

| Command | Description |
|---------|-------------|
| `tape insert <name>` | Insert a tape from `~/.local/share/tapes/` |
| `tape insert .` | Insert a tape from the current directory |
| `tape insert --path <path>` | Insert a tape from an arbitrary path |
| `tape insert --reinsert` | Eject current tape first, then insert (force replace) |
| `tape insert --dry-run` | Validate and show what would happen without making changes |
| `tape eject` | Remove current tape and restore original config |
| `tape show` | Display details of all installed tapes |
| `tape show <name>` | Display details of a specific tape |
| `tape show --list` | List all known tapes in a compact format |

### Examples

```bash
# Clone someone's rice and try it
git clone https://github.com/user/awesome-rice
cd awesome-rice
tape insert .

# Don't like it? Revert instantly
tape eject

# List all your installed tapes
tape show --list

# Use a tape by name
tape insert my-hypr-rice

# Force reinsert (useful if you've made changes to the tape)
tape insert my-hypr-rice --reinsert

# Test without replacing files
tape insert my-sway-rice --dry-run
```

## Directory Structure

```
~/.local/share/tapes/
├── my-hypr-rice/
│   ├── tape.toml          # Tape manifest
│   └── .config/
│       ├── hypr/
│       │   └── hyprland.conf
│       ├── waybar/
│       │   └── config
│       └── foot/
│           └── foot.ini
└── my-sway-rice/
    ├── tape.toml
    └── .config/
        └── sway/
            └── config
```

## Tape Manifest (`tape.toml`)

The manifest file defines tape metadata and behavior:

```toml
[tape]
name = "my-rice"
version = "0.1.0"
desc = "My awesome configuration setup"

[dependencies]
# List of required binaries - checked before insert
binaries = ["hyprland", "waybar", "foot"]

[hooks]
# Commands run after the tape is activated
insert = [
  "echo 'Activating rice...'",
  "pkill -9 waybar || true"
]

# Commands run after the tape is removed
eject = [
  "echo 'Rice deactivated'",
  "killall waybar || true"
]
```

## How It Works

### Insert Flow

1. Validate the tape exists and has a valid `tape.toml` (if present)
2. Check dependencies (if any binaries are missing, abort)
3. Rename `~/.config` → `~/.config.bak` (creates numbered backups if needed)
4. Create symlink from tape directory → `~/.config`
5. Track original tape file list for cleanup
6. Run insert hooks

### Eject Flow

1. Verify `~/.config` is a tape symlink
2. Clean up any generated files in the tape directory
3. Remove `~/.config` symlink
4. Rename `~/.config.bak` → `~/.config` (restore latest backup)
5. Run eject hooks

## Safety Features

- **Atomic operations**: Insert and eject are atomic — your config is never partially replaced
- **Automatic backups**: Original `~/.config` is always backed up before replacement
- **Numbered backups**: Multiple backups are supported (`~/.config.bak`, `~/.config.bak.1`, etc.)
- **Dependency validation**: Missing binaries are caught before switching
- **No overwrites**: Uses backups, never clobbers existing files directly
- **Clean cleanup**: Generated files in tapes are automatically removed on eject

## Project Structure

```
cassette-rices/
├── Cargo.toml          # Rust project configuration
├── DESIGN.md           # Design document and roadmap
├── VISION.md           # Future ideas and extended concepts
├── flake.nix           # Nix development environment
└── src/
    ├── main.rs         # CLI entry point and argument parsing
    ├── config.rs       # Tape manifest parsing
    └── commands/
        ├── mod.rs       # Module exports
        ├── insert.rs    # Insert command implementation
        ├── eject.rs     # Eject command implementation
        └── show.rs       # Show/list command implementation
```

## Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Configuration

Tapes uses the following conventions:

- **Tape library**: `~/.local/share/tapes/` (can be overridden with `--path`)
- **Config directory**: `~/.config` (standard XDG location)
- **Backup directory**: `~/.config.bak` (with numbered suffixes for multiple backups)
- **Manifest file**: `tape.toml` (in each tape directory)

## Dependencies

- Rust 2024 Edition
- `serde` with derive feature - TOML parsing
- `toml` - TOML parsing library
- `anyhow` - Error handling
- `walkdir` - Directory traversal
- `tempfile` - Temporary file handling
- `dirs` - XDG directory location
- `yansi` - Colored terminal output
- `clap` with derive feature - CLI argument parsing

## Roadmap

See [DESIGN.md](DESIGN.md) for the current design and [VISION.md](VISION.md) for future ideas including:

- Composable tapes (multiple active simultaneously)
- Per-file symlinking instead of directory replacement
- State tracking for clean uninstalls
- Lock files for reproducible tape states
- Remote tape support (`tape run <url>`)
- Conflict detection between multiple tapes
- Rollback capability for individual files

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License.

## Acknowledgments

- Inspired by Nix flakes and other modular configuration management systems
- Built with Rust for safety and performance
- Designed for the ricing community
