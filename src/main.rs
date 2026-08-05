mod config;
use config::TapeManifest;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use anyhow::{Context, Result};

fn main() -> anyhow::Result<()> {
    let tape_path = get_tape_config_path("foo")?;


    if tape_path.is_file() {
        let tape = TapeManifest::load_from_file(&tape_path)?;
        get_tape_properties(tape, &tape_path)?;
    } else {
        eprintln!("Error: Tape path not found at {:?}", tape_path);
    }

    Ok(())
}

fn get_tape_properties(tape: TapeManifest, tape_path: &PathBuf) -> anyhow::Result<()> {
    println!(
        "Loaded tape: {} (v{}) from \"{}\"",
        tape.tape.name,
        tape.tape.version,
        tape_path.display()
    );

    println!();

    println!("Description: {}", tape.tape.desc);

    println!();

    let binaries = tape
        .dependencies
        .as_ref()
        .and_then(|dep| dep.binaries.as_ref());

    if let Some(bin) = binaries {
        println!("Dependencies");
        for n in bin.iter() {
            println!(" - {}", n);
        }
    }

    println!();

    if let Some(targets) = &tape.targets {
        for (target_name, target_path) in targets {
            println!("{}: {}", target_name, target_path)
        }
    }

    println!();

    if let Some(hooks) = &tape.hooks {
        println!("{:#?}", hooks)
    }

    Ok(())
}

fn get_tape_config_path(tape_name: &str) -> Result<PathBuf> {

    let config_dir = dirs::config_local_dir().context("Could not locate home/config directory")?;

    Ok(config_dir.join(tape_name).join("tape.toml"))
    
}
