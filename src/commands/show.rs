use crate::config::TapeManifest;
use crate::APP_NAME;
use anyhow::{bail, Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};
use yansi::{Color, Paint};

pub fn load_all_tapes() -> Result<Vec<(TapeManifest, PathBuf)>> {
    let config_dir = dirs::data_local_dir()
        .context("Could not locate local directory")?
        .join(APP_NAME);

    if !config_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut tapes = Vec::new();
    for entry in fs::read_dir(&config_dir)?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let manifest_path = path.join("tape.toml");
            if manifest_path.is_file() {
                if let Ok(tape) = TapeManifest::load_from_file(&manifest_path) {
                    tapes.push((tape, path));
                }
            }
        }
    }
    Ok(tapes)
}

pub fn show_all_tapes() -> Result<()> {
    for (tape, manifest_path) in load_all_tapes()? {
        display_tape_properties(&tape, &manifest_path.parent().unwrap());
    }
    Ok(())
}

pub fn show_specific_tape(name: &str) -> Result<()> {
    let specificed_tape_path = crate::commands::insert::locate_local_dir()?.join(name);

    let local_dir: PathBuf = dirs::data_local_dir()
        .context("Could not locate user local dir")?
        .join(APP_NAME);

    if !specificed_tape_path.exists() || specificed_tape_path == local_dir {
        let home_dir: PathBuf = dirs::home_dir().context("Could not locate home directory")?;

        let path_identifier = specificed_tape_path
            .strip_prefix(home_dir)?
            .parent()
            .unwrap();

        bail!(
            "Tape '{name}' does not exist at ~/{}",
            path_identifier.to_string_lossy()
        );
    }

    if specificed_tape_path.is_dir() {
        let manifest_path = specificed_tape_path.join("tape.toml");
        if manifest_path.is_file() {
            if let Ok(tape) = TapeManifest::load_from_file(manifest_path) {
                display_tape_properties(&tape, &specificed_tape_path);
                // return Ok(());
            }
        }
    }

    Ok(())
}

pub fn list_all_known_tapes() -> Result<()> {
    for (tape, _) in load_all_tapes()? {
        let version = "v".to_string() + &tape.tape.version;
        println!(
            " - {} ({}): {}",
            tape.tape.name.bold().blue(),
            version.magenta(),
            tape.tape.desc.italic()
        );
    }
    Ok(())
}

pub fn display_tape_properties(tape: &TapeManifest, tape_path: &Path) {
    let display_path = dirs::home_dir()
        .and_then(|home| tape_path.strip_prefix(home).ok())
        .unwrap_or(tape_path);

    println!(
        "\n{}\n",
        Paint::new("────────────────────────────────────────────────────────────").dim()
    );

    // --- Header ---
    println!(
        "{} {} ({}) from {}",
        Paint::new("Loaded tape:").bold().fg(Color::Yellow),
        Paint::new(&tape.tape.name).bold().fg(Color::Cyan),
        Paint::new(&tape.tape.version).fg(Color::White),
        Paint::new(display_path.display()).dim()
    );

    // --- Description ---
    println!(
        "{} {}",
        Paint::new("Description:").bold().fg(Color::Yellow),
        Paint::new(&tape.tape.desc).fg(Color::White)
    );

    // --- Dependencies ---
    if let Some(deps) = &tape.tape.dependencies {
        println!("\n{}", Paint::new("Dependencies").bold().fg(Color::Yellow));
        for dep in deps {
            println!(
                "  {} {}",
                Paint::new("•").fg(Color::Green),
                Paint::new(dep).fg(Color::White)
            );
        }
    }

    // --- hooks ---
    if let Some(hooks) = &tape.hooks {
        println!("\n{}", Paint::new("Hooks").bold().fg(Color::Magenta));

        // --- Insert Hooks ---
        if !hooks.insert.is_empty() {
            println!("  {}", Paint::new("insert:").fg(Color::Green));
            for hook in &hooks.insert {
                println!(
                    "    {} {}",
                    Paint::new("→").fg(Color::Blue),
                    Paint::new(hook).fg(Color::Cyan)
                );
            }
        } else {
            println!("  {}", Paint::new("No insert hooks found").fg(Color::Red));
        }

        // --- Eject Hooks ---
        if !hooks.eject.is_empty() {
            println!("  {}", Paint::new("eject:").fg(Color::Red));
            for hook in &hooks.eject {
                println!(
                    "    {} {}",
                    Paint::new("→").fg(Color::Blue),
                    Paint::new(hook).fg(Color::Cyan)
                );
            }
        } else {
            println!("  {}", Paint::new("No eject hooks found\n").fg(Color::Red));
        }
    }

    // --- Files ---
    println!("\n{}", Paint::new("Files").bold().fg(Color::Yellow));
    for entry in walkdir::WalkDir::new(tape_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .flatten()
    {
        if entry.file_type().is_file() {
            continue;
        }

        if let Ok(file) = entry.path().strip_prefix(tape_path) {
            println!(
                "  {} {}",
                Paint::new("•").fg(Color::Green),
                Paint::new(file.display()).fg(Color::White)
            );
        }
    }

    println!(
        "\n{}\n",
        Paint::new("────────────────────────────────────────────────────────────").dim()
    );
}
