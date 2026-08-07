use crate::config::TapeManifest;
use crate::APP_NAME;
use anyhow::{bail, Context, Result};
use std::{fs, path::Path};
use walkdir::WalkDir;
use yansi::{Color, Paint};

pub fn load_all_tapes() -> Result<Vec<(TapeManifest, std::path::PathBuf)>> {
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
                    tapes.push((tape, manifest_path));
                }
            }
        }
    }
    Ok(tapes)
}

pub fn show_all_tapes() -> Result<()> {
    for (tape, manifest_path) in load_all_tapes()? {
        display_tape_properties(&tape, &manifest_path);
    }
    Ok(())
}

pub fn show_specific_tape(name: &str) -> Result<()> {
    for (tape, manifest_path) in load_all_tapes()? {
        if tape.tape.name == name {
            display_tape_properties(&tape, &manifest_path);
            return Ok(());
        }
    }
    bail!("Tape '{}' was not found", name);
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
        "{}: {}",
        Paint::new("Description:").bold().fg(Color::Yellow),
        Paint::new(&tape.tape.desc).fg(Color::White)
    );

    // --- Dependencies ---
    if let Some(binaries) = tape
        .dependencies
        .as_ref()
        .and_then(|dep| dep.binaries.as_ref())
    {
        println!("\n{}", Paint::new("Dependencies").bold().fg(Color::Yellow));
        for binary in binaries {
            println!(
                "  {} {}",
                Paint::new("•").fg(Color::Green),
                Paint::new(binary).fg(Color::White)
            );
        }
    }

    // if let Some(targets) = &tape.targets {
    //     println!("\n{}", Paint::new("Targets").bold().fg(Color::Yellow));
    //     for (target_name, target_path) in targets {
    //         println!(
    //             "  {} {}: {}",
    //             Paint::new("•").fg(Color::Green),
    //             Paint::new(target_name).fg(Color::Magenta),
    //             Paint::new(target_path).fg(Color::White)
    //         );
    //     }
    //     println!();
    // }

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
            println!("  {}", Paint::new("No insert hooks found\n").fg(Color::Red));
        }
    }

    // --- Files ---
    if let Some(parent_dir) = tape_path.parent() {
        println!("\n{}", Paint::new("Files").bold().fg(Color::Yellow));
        for entry in WalkDir::new(parent_dir).min_depth(1).into_iter().flatten() {
            if !entry.file_type().is_file() {
                continue;
            }
            if let Ok(file) = entry.path().strip_prefix(parent_dir) {
                println!(
                    "  {} {}",
                    Paint::new("•").fg(Color::Green),
                    Paint::new(file.display()).fg(Color::White)
                );
            }
        }
    }

    println!(
        "\n{}\n",
        Paint::new("────────────────────────────────────────────────────────────").dim()
    );
}
