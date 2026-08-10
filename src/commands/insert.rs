use crate::config::TapeManifest;
use crate::dependencies::check_dependencies;
use crate::{quit, STATE_FILE};
use crate::{run_command, APP_NAME};
use anyhow::{bail, Context, Result};
use std::os::unix::fs::symlink;
use std::{
    fs,
    path::{Path, PathBuf},
};
use yansi::Paint;

pub fn insert(name: Option<String>, path: Option<PathBuf>, dry_run: bool) -> Result<()> {
    let source_path = get_source_path(&name, path)?;

    let local_dir: PathBuf = dirs::data_local_dir()
        .context("Could not locate user local dir")?
        .join(APP_NAME);

    let identifier = name.unwrap_or_else(|| source_path.display().to_string());
    if !source_path.exists() || source_path == local_dir {
        bail!("Tape '{identifier}' does not exist at {:?}", source_path);
    }

    let actual_path = source_path
        .canonicalize()
        .context(format!("Failed to canonicalize {:?}", source_path))?;

    let user_config: PathBuf =
        dirs::config_dir().context("Could not locate user config direcotry")?;
    let home_dir: PathBuf = dirs::home_dir().context("Could not locate home directory")?;


    check_dependecies(&source_path)?;

    if dry_run {
        // LATER: Will do later
        return Ok(());
    }

    let targets = resolve_targets(&actual_path)?;

    for target in &targets {
        let tape_target_source = actual_path.join(".config").join(target);
        if !tape_target_source.exists() {
            let direct_source = actual_path.join(target);
            if !direct_source.exists() {
                continue;
            }
        }

        let actual_source = actual_path.join(target);

        let user_target_dest = user_config.join(target);


        if user_target_dest.exists() {
            let backup_path = get_available_backup_path_local(&user_target_dest);

            fs::rename(&user_target_dest, &backup_path)?;

            println!(
                "{} ~/{} to ~/{}",
                "Backed up: ".bold().blue(),
                user_target_dest
                    .strip_prefix(&home_dir)?
                    .display()
                    .magenta(),
                backup_path.strip_prefix(&home_dir)?.display().magenta()
            );
        }

        if let Some(parent) = user_target_dest.parent() {
            fs::create_dir_all(parent)?;
        }

        symlink(&actual_source, &user_target_dest).context("Failed to symlink target")?;

        println!(
            "{} ~/{} -> ~/{}",
            "Symlinked: ".bold().blue(),
            actual_source.strip_prefix(&home_dir)?.display().magenta(),
            user_target_dest
                .strip_prefix(&home_dir)?
                .display()
                .magenta()
        );
    }

    if source_path.is_dir() {
        let manifest_path = source_path.join("tape.toml");
        if manifest_path.is_file() {
            if let Ok(tape) = TapeManifest::load_from_file(manifest_path) {
                println!("{}", "Running insert hooks...".bold().bright_red());
                run_insert_hooks(&tape)?;

                println!(
                    "\n{}{}\n",
                    "Inserted: ".bold().green(),
                    tape.tape.name.bold()
                );

                store_current_tape(&tape.tape.name.to_string(), &identifier)?;
            }
        }
    }

    Ok(())
}

pub fn locate_local_dir() -> Result<PathBuf> {
    let config_dir = dirs::data_local_dir()
        .context("Could not locate local directory")?
        .join(APP_NAME);

    Ok(config_dir)
}

pub fn get_source_path(name: &Option<String>, path: Option<PathBuf>) -> Result<PathBuf> {
    let source_path = match (name, path) {
        (Some(_name), Some(path)) => path,
        (Some(name), _) => locate_local_dir()?.join(name),
        (None, Some(path)) => path,
        (None, None) => bail!("Must provide either a tape name or --path"),
    };

    Ok(source_path)
}

pub fn get_available_backup_path_local(target_path: &Path) -> PathBuf {
    let file_name = target_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("config");

    let backup_path = target_path.with_file_name(format!(".{file_name}.bak"));

    while std::fs::symlink_metadata(&backup_path).is_ok() {
        quit(&format!(
            "{}",
            "Backup of config already found".bold().red()
        ))
    }

    backup_path
}

pub fn run_insert_hooks(tape: &TapeManifest) -> Result<()> {
    if let Some(hooks) = &tape.hooks {
        if !hooks.insert.is_empty() {
            for hook in &hooks.insert {
                println!(" {} {}", Paint::new("•").red(), hook.red());

                run_command(hook)?;
            }
        }
    }

    Ok(())
}
fn check_dependecies(source_path: &Path) -> Result<()> {
    if source_path.is_dir() {
        let manifest_path = source_path.join("tape.toml");
        if manifest_path.is_file() {
            if let Ok(tape) = TapeManifest::load_from_file(manifest_path) {
                check_dependencies(&tape)?;
            }
        }
    }

    Ok(())
}

pub fn store_current_tape(current_rice_name: &str, filename: &str) -> Result<()> {
    let path = locate_local_dir()?.join(STATE_FILE);

    let current_rice = format!("CURRENT-RICE: {}\nFILENAME: {}", current_rice_name, filename);

    let current_rice = format!(r#"{}
         // If your IQ is not too high, Do not edit this file
         // If anything breaks, Do not blame me
         "#, current_rice);

    fs::write(&path, current_rice).context("Failed to write to file")?;
   
    Ok(())
}

pub fn resolve_targets(tape_path: &Path) -> Result<Vec<PathBuf>> {
    let config_dir = tape_path.join(".config");
    let scan_dir = if config_dir.is_dir() {
        config_dir
    } else {
        tape_path.to_path_buf()
    };

    let mut detected = Vec::new();
    for entry in fs::read_dir(scan_dir)? {
        let entry = entry?;
        let name = entry.file_name();
        if name != "tape.toml" && name != ".git" && name != "README.md" {
            detected.push(PathBuf::from(name))
        }

    }

    Ok(detected)
}
