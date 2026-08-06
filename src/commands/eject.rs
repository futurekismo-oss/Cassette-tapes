use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

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
        bail!("~/.config is not a tape symlink. Nothing to eject, {:?}", &user_config)
    }


    let backup = get_lastest_backup(&user_config);

    if backup.exists() {

        fs::remove_file(&user_config)?;

        fs::rename(&backup, &user_config)?;

        println!(
            "Restored {} from {}",
            user_config.display(),
            backup.display()
        );
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

    let backup_path = path.with_file_name(format!("{file_name}.bak"));

    backup_path
}
