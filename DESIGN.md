# Tapes Design

A lightweight, modular dotfile manager inspired by Nix flakes, built in Rust.

## Concept
A "Tape" is a self-contained directory containing:
- A `.config/` folder mirroring standard configuration structures.
- A `tape.toml` file defining metadata and dependencies.
- A `tape.lock` to ensure reproductibilty

## `tape.toml` (Draft)
```toml
[tape]
name = "example"
version = "0.1.0"

[dependencies]
binaries = ["hyprland", "foot"]


[targets]
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

tape - metadata
dependencies - programs your rice depends on
targets - for configs that dont follow the general ~/.config
hooks - commands needed to setup your rice for other systems


## Core Operations
- Insert (tape insert <path>): Safely symlinks tape contents into ~/.config.
- Eject (tape eject <name>): Removes active symlinks for a given tape.
- List (tape list): Shows currently active tapes.
- Info (tape info <path>): Inspects a tape's manifest before installing.
- Run (tap run <link>): Run external tapes 

## Safety & State
- Tracks installed files in a local state file to allow clean uninstalls.
- Avoids overwriting unmanaged files unless forced.
