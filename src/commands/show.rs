use crate::config::TapeManifest;
use crate::APP_NAME;
use anyhow::{bail, Context, Result};
use std::{
    fs,
    path::Path,
};
use walkdir::WalkDir;

fn load_all_tapes() -> Result<Vec<(TapeManifest, std::path::PathBuf)>> {
    let config_dir = dirs::config_local_dir()
        .context("Could not locate config directory")?
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
        get_tape_properties(&tape, &manifest_path);
        println!("\n==========================================\n");
    }
    Ok(())
}

pub fn show_specific_tape(name: &str) -> Result<()> {
    for (tape, manifest_path) in load_all_tapes()? {
        if tape.tape.name == name {
            get_tape_properties(&tape, &manifest_path);
            return Ok(());
        }
    }
    bail!("Tape '{}' was not found", name);
}

pub fn list_all_known_tapes() -> Result<()> {
    for (tape, _) in load_all_tapes()? {
        println!(" - {}: {}", tape.tape.name, tape.tape.desc);
    }
    Ok(())
}

pub fn get_tape_properties(tape: &TapeManifest, tape_path: &Path) {
    let display_path = dirs::home_dir()
        .and_then(|home| tape_path.strip_prefix(home).ok())
        .unwrap_or(tape_path);

    println!(
        "Loaded tape: {} (v{}) from \"{}\"\n",
        tape.tape.name,
        tape.tape.version,
        display_path.display()
    );

    println!("Description: {}\n", tape.tape.desc);

    if let Some(binaries) = tape
        .dependencies
        .as_ref()
        .and_then(|dep| dep.binaries.as_ref())
    {
        println!("Dependencies");
        for binary in binaries {
            println!(" - {}", binary);
        }
        println!();
    }

    if let Some(targets) = &tape.targets {
        println!("Targets");
        for (target_name, target_path) in targets {
            println!(" - {}: {}", target_name, target_path);
        }
        println!();
    }

    if let Some(hooks) = &tape.hooks {
        println!("Hooks");
        println!("{:#?}", hooks);
    }

    if let Some(parent_dir) = tape_path.parent() {
        println!("\nFiles");
        for entry in WalkDir::new(parent_dir).min_depth(1).into_iter().flatten() {
            if !entry.file_type().is_file() {
                continue;
            }
            if let Ok(rel_path) = entry.path().strip_prefix(parent_dir) {
                println!(" - {}", rel_path.display());
            }
        }
    }
}

