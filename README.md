<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a id="readme-top"></a>


<!-- PROJECT SHIELDS -->
[![Stargazers][stars-shield]][stars-url]
[![License][license-shield]][license-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">
<h1 align="center">Cassette Tapes</h1>

  <img src="media/icon.svg" alt="Logo" width="200">

  <p align="center">
    An atomic dotfile switcher for ricing.
    <br />
    <br />
    <a href="https://github.com/futurekismo-oss/cassette-rices/issues/new">Report Bug</a>
    &middot;
    <a href="https://github.com/futurekismo-oss/cassette-rices/issues/new">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#how-it-works">How It Works</a></li>
    <li><a href="#directory-structure">Directory Structure</a></li>
    <li><a href="#tape-manifest-tapetoml">Tape Manifest</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

**Tapes** is an atomic dotfile switcher for ricing. It allows you to instantly switch between complete configuration setups (rices) with a single command, with automatic backups and rollback capability.

A **tape** is a self-contained rice directory containing a `.config/` folder and an optional `tape.toml` manifest. When you **insert** a tape, conflicting configs in `~/.config` are backed up and the tape's configs are symlinked in. When you **eject**, the symlinks are removed and your original config is restored.

Key features:

* **Atomic switching** — your config is never in a half-state during insert/eject.
* **Automatic backups** — original `~/.config` folders are always preserved.
* **Lifecycle hooks** — run commands before/after insert and eject.
* **Dependency checking** — verify required binaries exist before switching.
* **Clean uninstalls** — generated files in tapes are cleaned up on eject.
* **Immutable by default** — tapes are symlinked, preserving the original.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* [![Rust][Rust]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

Get a local copy up and running with a couple of simple steps.

### Prerequisites

* Rust 2024 Edition

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/futurekismo-oss/cassette-rices.git
   cd cassette-rices
   ```
2. Build and install
   ```sh
   cargo install --path .
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

### Commands

| Command | Description |
|---------|-------------|
| `tape insert <name>` | Insert a tape from `~/.local/share/tapes/` |
| `tape insert .` | Insert a tape from the current directory |
| `tape insert --path <path>` | Insert a tape from an arbitrary path |
| `tape insert --reinsert` | Eject current tape first, then insert (force replace) |
| `tape eject` | Remove current tape and restore original config |
| `tape show` | Display details of all installed tapes |
| `tape show <name>` | Display details of a specific tape |
| `tape show --list` | List all known tapes in a compact format |
| `tape current` | Displays the currently inserted tape if any |

### Examples

```sh
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

# Display the currently inserted tape
tape current
```

### Creating a Tape

1. Create a directory with a `.config/` folder:
   ```sh
   mkdir -p ~/my-hypr-rice/.config/hypr
   ```

2. Add a `tape.toml` manifest (optional):
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

3. Move it to the tapes library for easy access:
   ```sh
   mkdir -p ~/.local/share/tapes
   cp -r ~/my-hypr-rice ~/.local/share/tapes/
   tape insert my-hypr-rice
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- HOW IT WORKS -->
## How It Works

### Insert Flow

1. Validate the tape exists and has a valid `tape.toml` (if present)
2. Check dependencies (abort if any binaries are missing)
3. Rename conflicting configs in `~/.config/` to `.bak`
4. Symlink each config file/dir in the tape directory → `~/.config`
5. Track original tape file list for cleanup
6. Run insert hooks
7. Store the state data in `~/.local/share/tapes/current-tape`

### Eject Flow

1. Check if a tape is inserted via `~/.local/share/tapes/current-tape`
2. Remove symlinks in `~/.config`
3. Restore backed-up configs (`.bak` → original)
4. Run eject hooks

### Safety Features

* **Atomic operations** — insert and eject are atomic, your config is never partially replaced.
* **Automatic backups** — original `~/.config` folders are always backed up before replacement.
* **Dependency validation** — missing binaries are caught before switching.
* **No overwrites** — uses backups, never clobbers existing files directly.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- DIRECTORY STRUCTURE -->
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

### Project Structure

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

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- TAPE MANIFEST -->
## Tape Manifest (`tape.toml`)

The manifest file defines tape metadata and behavior:

```toml
[tape]
name = "my-rice"          # Required
version = "0.1.0"         # Required
desc = "My awesome setup" # Required

[dependencies]
binaries = ["hyprland", "waybar", "foot"]

[hooks]
insert = [
  "echo 'Activating rice...'",
  "pkill -9 waybar || true"
]
eject = [
  "echo 'Rice deactivated'",
  "killall waybar || true"
]
```

### Configuration Conventions

* **Tape library**: `~/.local/share/tapes/` (override with `--path`)
* **State file**: `~/.local/share/tapes/current-tape` — stores the currently inserted tape
* **Manifest file**: `tape.toml` (in each tape directory)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Composable tapes (multiple active simultaneously)
- [ ] State tracking for clean uninstalls
- [ ] Lock files for reproducible tape states
- [ ] Remote tape support (`tape run <url>`)
- [ ] Conflict detection between multiple tapes
- [ ] Rollback capability for individual files
- [ ] TUI

See the [open issues](https://github.com/futurekismo-oss/cassette-rices/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* Inspired by Nix flakes and other modular configuration management systems
* Built with Rust for safety and performance
* Designed for the ricing community
* [Best-README-Template](https://github.com/othneildrew/Best-README-Template)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
[stars-shield]: https://img.shields.io/github/stars/futurekismo-oss/cassette-rices.svg?style=for-the-badge
[stars-url]: https://github.com/futurekismo-oss/cassette-rices/stargazers
[license-shield]: https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge
[license-url]: https://github.com/futurekismo-oss/cassette-rices/blob/master/LICENSE
[Rust]: https://img.shields.io/badge/Rust-2024_Edition-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
