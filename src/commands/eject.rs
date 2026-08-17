use crate::commands::insert::{locate_local_dir, resolve_targets};
use crate::config::TapeManifest;
use crate::{debug, run_command, STATE_FILE};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use yansi::Paint;

pub fn eject() -> Result<()> {
    let user_config = dirs::config_dir().context("Cannot find user config")?;
    let home_dir: PathBuf = dirs::home_dir().context("Could not locate home directory")?;

    let state_path = locate_local_dir()?.join(STATE_FILE);
    if !state_path.exists() {
        bail!("No active tape nothing to eject");
    }

    let state_content =
        fs::read_to_string(&state_path).context("Failed to read active tape state file")?;

    let tape_name = state_content
        .lines()
        .find_map(|line| line.strip_prefix("FILENAME: "))
        .context("State file is corrupted or is missing CURRENT-RICE entry")?
        .trim();

    let tape_dir = locate_local_dir()?.join(tape_name);
    let manifest_path = tape_dir.join("tape.toml");

    let tape_manifest = if manifest_path.is_file() {
        TapeManifest::load_from_file(&manifest_path).ok()
    } else {
        None
    };

    let targets = resolve_targets(&tape_dir)?;

    for target in targets {
        let user_target = user_config.join(&target);

        if user_target.is_symlink() {
            fs::remove_file(&user_target)?;
            if debug::is_debug() {
                debug::status_ok_kv(
                    "unlink",
                    "target",
                    &user_target.strip_prefix(&home_dir)?.display().to_string(),
                );
            } else {
                println!(
                    "{} ~/{}",
                    "Unlinked target:".bold().yellow(),
                    user_target.strip_prefix(&home_dir)?.display().magenta()
                );
            }
        }

        let backup = get_lastest_backup(&user_target);
        if backup.exists() {
            fs::rename(&backup, &user_target)?;
            if debug::is_debug() {
                debug::status_ok_fields(
                    "restore",
                    &[
                        ("source", &backup.strip_prefix(&home_dir)?.display().to_string()),
                        ("dest", &user_target.strip_prefix(&home_dir)?.display().to_string()),
                    ],
                );
            } else {
                println!(
                    "{} ~/{} from ~/{}",
                    "Restored: ".bold().blue(),
                    user_target.strip_prefix(&home_dir)?.display().magenta(),
                    backup.strip_prefix(&home_dir)?.display().magenta()
                );
            }
        }
    }

    if let Some(tape) = tape_manifest {
        if !debug::is_debug() {
            println!("{}", "Running eject hooks...".bold().bright_red());
        }
        run_eject_hooks(tape)?;
        if debug::is_debug() {
            debug::status_ok_kv("eject", "tape", tape_name);
        } else {
            println!("{} {}", "Ejected:".bold().green(), tape_name.bold());
        }
    }

    delete_state_file()?;

    if debug::is_debug() {
        debug::status_ok("restore_configs");
    } else {
        println!("{}", "Restored user original target configs".bold().green());
    }

    Ok(())
}

fn get_lastest_backup(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("config");

    return path.with_file_name(format!(".{file_name}.bak"));
}

pub fn run_eject_hooks(tape: TapeManifest) -> Result<()> {
    if let Some(hooks) = &tape.hooks {
        if !hooks.insert.is_empty() {
            for hook in &hooks.eject {
                if debug::is_debug() {
                    debug::hook(hook);
                } else {
                    println!(" {} {}", Paint::new("•").red(), hook.red());
                }

                run_command(hook)?;
            }
        }
    }

    Ok(())
}

fn delete_state_file() -> Result<()> {
    let path = locate_local_dir()?.join(STATE_FILE);

    fs::remove_file(path)?;

    Ok(())
}
