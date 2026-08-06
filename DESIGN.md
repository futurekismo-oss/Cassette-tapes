# Tape: Atomic Rice Switcher

A simple, atomic dotfile switcher for ricing. Switch between complete configuration setups with one command.

## Concept

A **tape** is a self-contained rice directory. When you insert a tape, your entire `~/.config` is replaced atomically with the tape's contents. When you eject, your original config is restored.

This is designed for ricers who:
- Want to test complete setups quickly
- Need a safety net to revert changes
- Switch between different window manager or shell configurations

## Structure

```
~/.local/share/tapes/
├── my-hypr-rice/
│   ├── tape.toml          # Tape metadata
│   └── .config/
│       ├── hypr/
│       ├── waybar/
│       └── ...
└── my-sway-rice/
    ├── tape.toml
    └── .config/
        └── ...
```

## `tape.toml`

Each tape contains a manifest that defines metadata, dependencies, and lifecycle hooks.

```toml
[tape]
name = "my-hypr-rice"
version = "0.1.0"
desc = "My Hyprland + Waybar setup"

[dependencies]
binaries = ["hyprland", "waybar", "foot"]

[hooks]
# Commands to run before the tape is activated
insert = [
  "echo 'Activating Hyprland rice...'",
  "pkill -9 waybar || true"
]

# Commands to run after the tape is removed
eject = [
  "echo 'Rice deactivated'",
  "killall waybar || true"
]
```

- **`name`**: Tape identifier
- **`version`**: Tape version
- **`desc`**: Human-readable description
- **`dependencies.binaries`**: Required programs (checked before insert)
- **`hooks.insert`**: Commands run after the tape is symlinked into place
- **`hooks.eject`**: Commands run after the tape is removed

## Core Operations

| Command | Description |
|---------|-------------|
| `tape insert <name>` | Replace `~/.config` with tape from `~/.local/share/tapes/` |
| `tape insert .` | Insert tape from current directory |
| `tape insert --path <path>` | Insert tape from arbitrary path |
| `tape eject` | Restore original `~/.config` from backup |
| `tape show` | List all discovered tapes |
| `tape show <name>` | Show details for a specific tape |

## How It Works

### Insert Flow
1. Validate the tape exists and has a valid `tape.toml`
2. Check dependencies (if any binaries are missing, abort)
3. Rename `~/.config` → `~/.config.bak` (or `~/.config.bak.1`, etc.)
4. Symlink tape directory → `~/.config`
5. Run insert hooks

### Eject Flow  
1. Remove `~/.config` symlink
2. Rename `~/.config.bak` → `~/.config` (restore latest backup)
3. Run eject hooks

## Safety

- **Atomic**: Insert and eject are atomic operations — your config is never in a half-state
- **Backup**: Original `~/.config` is always backed up before replacement
- **Dependency checks**: Missing binaries are caught before switching
- **No overwrites**: Uses backups, never clobbers existing files

## Usage Examples

```bash
# Clone someone's rice
git clone https://github.com/user/awesome-rice
cd awesome-rice

# Try it out
tape insert .

# Don't like it? Revert instantly
tape eject

# List your installed tapes
tape show --list

# Use a tape by name (from ~/.local/share/tapes/)
tape insert my-hypr-rice
```
