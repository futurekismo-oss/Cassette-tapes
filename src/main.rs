mod commands;
mod config;
mod debug;
mod dependencies;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::os::unix::fs::symlink;
use std::{
    fs,
    path::PathBuf,
    process::{exit, Command},
};
use yansi::Paint;

use crate::commands::insert::locate_local_dir;

#[derive(Parser, Debug)]
#[command(name = "tape")]
#[command(about = "A tool for managing rice installations", long_about = None)]
struct Cli {
    /// Debug for pure cli
    #[arg(short, long, global = true)]
    debug: bool,

    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// show installed & found paths in ~/.local/tapes
    Show {
        /// Name of tape to show, prints all the found tapes if not given
        name: Option<String>,

        /// List all the found tapes
        #[arg(long)]
        list: bool,
    },

    /// Inserts the tape
    Insert {
        /// Name of tape to insert
        name: Option<String>,

        /// Path of the tape to insert
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// First eject the current tape if any before inserting
        #[arg(short, long)]
        reinsert: bool,
    },

    /// Eject the current tape if you used any
    Eject,

    /// Displays the currently inserted rice
    Current,
}

pub const APP_NAME: &str = "tapes";
pub const STATE_FILE: &str = "current-tape";

fn main() -> Result<()> {
    let cli = Cli::parse();

    debug::set_debug(cli.debug);

    create_tapes_symlink()?;

    match cli.commands {
        Commands::Show { name, list } => match name {
            Some(name) => {
                commands::show::show_specific_tape(&name)?;
            }
            None => {
                if !list {
                    commands::show::show_all_tapes()?
                } else {
                    if !debug::is_debug() {
                        println!("{}", "Known tapes: ".bold().italic().green());
                    }
                    commands::show::list_all_known_tapes()?
                }
            }
        },

        Commands::Insert {
            name,
            path,
            reinsert,
        } => {
            if !reinsert {
                commands::insert::insert(name, path)?;
            } else {
                commands::eject::eject()?;
                if !debug::is_debug() {
                    println!(
                        "\n{}\n",
                        Paint::new("────────────────────────────────────────────────────────────")
                            .dim()
                    );
                }
                commands::insert::insert(name, path)?;
            }
        }

        Commands::Eject => commands::eject::eject()?,

        Commands::Current => commands::current::display_currently_inserted_tape()?,
    }

    Ok(())
}

fn create_tapes_symlink() -> Result<()> {
    let tapes_path: PathBuf = dirs::home_dir()
        .context("Could not find home directory")?
        .join(".tapes");

    if tapes_path.is_symlink() {
        if let Ok(target) = fs::read_link(&tapes_path) {
            if target.to_string_lossy().contains(".local/share/tapes") {
                return Ok(());
            }
        }
    }

    let actual_path: PathBuf = locate_local_dir()?;

    symlink(&actual_path, &tapes_path).context("Failed to symlink")?;

    Ok(())
}

pub fn run_command(line: &str) -> Result<()> {
    if line.trim().is_empty() {
        return Ok(());
    }

    let status = Command::new("sh").arg("-c").arg(line).status()?;

    if !status.success() {
        if debug::is_debug() {
            debug::status_error(&format!(
                "action=command_exit code={}",
                status.code().unwrap_or(-1)
            ));
        } else {
            println!("Command returned a non-exit code: {:?}", status.code());
        }
    }

    Ok(())
}

pub fn quit(message: &str) -> ! {
    if debug::is_debug() {
        debug::status_error(message);
    } else {
        eprintln!("{}", message);
    }
    exit(1);
}
