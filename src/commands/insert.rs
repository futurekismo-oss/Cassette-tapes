use crate::APP_NAME;
use anyhow::{bail, Context, Result};
use std::os::unix::fs::symlink;
use std::{fmt::format, fs, path::PathBuf};
use yansi::Paint;

pub fn insert(name: Option<String>, path: Option<PathBuf>, dry_run: bool) -> Result<()> {
    let source_path = get_source_path(&name, path)?;

    if !source_path.exists() {
        let identifier = name.unwrap_or_else(|| source_path.display().to_string());
        bail!("Tape '{identifier}' does not exist at {:?}", source_path);
    }

    let actual_path = source_path
        .canonicalize()
        .context(format!("Failed to canonicalize {:?}", source_path))?;

    let user_config: PathBuf =
        dirs::config_dir().context("Could not locate user config direcotry")?;

    let available_backup_path = get_available_backup_path(&user_config);

    if dry_run {
        // LATER: Will do later
        return Ok(());
    }

    fs::rename(&user_config, &available_backup_path)?;

    println!(
        "{} {} to {}",
        "Renamed: ".bold().blue(),
        &user_config.display().magenta(),
        &available_backup_path.display().magenta()
    );

    symlink(&actual_path, &user_config).context("Failed to symlink")?;

    Ok(())
}

pub fn locate_local_dir() -> Result<PathBuf> {
    let config_dir = dirs::data_local_dir()
        .context("Could not locate local directory")?
        .join(APP_NAME);

    Ok(config_dir)
}

pub fn locate_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_local_dir()
        .context("Could not locate config directory")?
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

pub fn get_available_backup_path(target_path: &PathBuf) -> PathBuf {
    let file_name = target_path.file_name().and_then(|n| n.to_str()).unwrap_or("config");

    let mut backup_path = target_path.with_file_name(format!("{file_name}.bak"));
    let mut counter = 1;

    while std::fs::symlink_metadata(&backup_path).is_ok() {
        // backup_path = target_path.with_file_name(format!("{file_name}.bak.{counter}"));
        // counter += 1;
        panic!("Backup of confing already found")
    }

    backup_path
}
