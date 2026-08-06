# Vision & Future Ideas

This document captures the original, more ambitious vision for Tape — ideas that may be implemented in the future.

## Original Concept (Modular Dotfile Manager)

A lightweight, modular dotfile manager inspired by Nix flakes, built in Rust.

A **tape** could be a self-contained directory containing:
- A `.config/` folder mirroring standard configuration structures
- A `tape.toml` file defining metadata and dependencies
- A `tape.lock` to ensure reproducibility

## Extended `tape.toml` (Future)

```toml
[tape]
name = "example"
version = "0.1.0"

[dependencies]
binaries = ["hyprland", "foot"]

[targets]
# For configs that don't follow standard XDG paths
zshrc = "~/.zshrc"
"starship.toml" = '~/.config/starship.toml'

[hooks]
# Runs before symlinks are created
pre_insert = ["echo 'Preparing Hyprland setup...'"]

# Runs after symlinks are successfully created
post_insert = [
  "hyprctl reload",
  "fc-cache -fv"
]

# Runs before active symlinks are removed
pre_eject = []

# Runs after active symlinks are removed
post_eject = ["killall waybar"]
```

## Advanced Operations (Future)

- **Composable tapes**: Multiple tapes active simultaneously, with conflict resolution
- **Per-file symlinking**: Instead of replacing entire directories, symlink individual files
- **State tracking**: Track exactly which files were installed by which tape for clean uninstalls
- **Lock files**: Ensure reproducible tape states across systems
- **Run external tapes**: `tape run <url>` to temporarily test a remote tape without installing

## Safety & State (Future)

- Track installed files in a local state file to allow clean uninstalls
- Avoid overwriting unmanaged files unless forced
- Conflict detection between multiple active tapes
- Rollback capability for individual files
