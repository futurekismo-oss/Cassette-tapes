use crate::config::TapeManifest;
use crate::APP_NAME;
use anyhow::{Context, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn execute() -> Result<()> {
    let config_dir = dirs::config_local_dir()
        .context("Could not locate config direcoty")?
        .join(APP_NAME);

    if !config_dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(&config_dir)?.flatten() {
        let path = entry.path();

        if path.is_dir() {
            let manifest_path = path.join("tape.toml");

            if manifest_path.is_file() {
                if let Ok(tape) = TapeManifest::load_from_file(&manifest_path) {
                    get_tape_properties(&tape, &manifest_path);
                    println!("\n==========================================\n");
                }
            }
        }
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

// NOTE: Unused weird, I'll keep it around thou
// pub fn get_tape_config_path(tape_name: &str) -> Result<PathBuf> {
//     let config_dir = dirs::config_local_dir().context("Could not locate config directory")?;
//     Ok(config_dir.join(APP_NAME).join(tape_name))
// }
