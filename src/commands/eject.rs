use crate::config::TapeManifest;
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::{fs, process::Command};
use yansi::Paint;

fn is_tape_symlink(path: &Path) -> bool {
    if !path.is_symlink() {
        return false;
    }

    if let Ok(target) = fs::read_link(path) {
        target.to_string_lossy().contains(".local/share/tapes")
    } else {
        false
    }
}

pub fn eject() -> Result<()> {
    let user_config = dirs::config_dir().context("Cannot find user config")?;

    if !is_tape_symlink(&user_config) {
        bail!(
            "~/.config is not a {} symlink. Nothing to eject",
            "tape".bold().italic().invert(),
        )
    }

    let backup = get_lastest_backup(&user_config);

    if backup.exists() {
        if user_config.is_dir() {
            let manifest_path = user_config.join("tape.toml");
            if manifest_path.is_file() {
                if let Ok(tape) = TapeManifest::load_from_file(&manifest_path) {
                    fs::remove_file(&user_config)?;

                    fs::rename(&backup, &user_config)?;

                    println!("{} {}", "Ejected:".bold().green(), tape.tape.name.bold());

                    println!("{}", "Runnning eject hooks...".bold().bright_red());

                    run_eject_hooks(tape)?;
                }
            }
        }

        let home_dir: PathBuf = dirs::home_dir().context("Could not locate home directory")?;

        println!(
            "{} ~/{} from ~/{}",
            "Restored: ".bold().blue(),
            user_config.strip_prefix(&home_dir)?.display().magenta(),
            backup.strip_prefix(&home_dir)?.display().magenta()
        );

        println!("{}", "Restored user original config".bold().green())
    } else {
        println!("No backup found. ~/.config kept.");
    }

    Ok(())
}

fn get_lastest_backup(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("config");

    return path.with_file_name(format!("{file_name}.bak"));
}

pub fn run_eject_hooks(tape: TapeManifest) -> Result<()> {
    if let Some(hooks) = &tape.hooks {
        if !hooks.insert.is_empty() {
            for hook in &hooks.eject {
                println!(" {} {}", Paint::new("•").red(), hook.red());

                run_command(hook)?;
            }
        }
    }

    Ok(())
}

fn run_command(line: &str) -> Result<()> {
    if line.trim().is_empty() {
        return Ok(());
    }

    let status = Command::new("sh").arg("-c").arg(line).status()?;

    if !status.success() {
        println!("Command returned a non-exit code: {:?}", status.code());
    }

    Ok(())
}
