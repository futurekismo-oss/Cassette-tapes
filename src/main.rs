mod commands;
mod config;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use yansi::Paint;

#[derive(Parser, Debug)]
#[command(name = "tape")]
#[command(about = "A tool for managing rice installations", long_about = None)]
struct Cli {
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

        /// Run but don't replace any files
        #[arg(long)]
        dry_run: bool,

        /// First eject the current tape if any before inserting
        #[arg(short, long)]
        reinsert: bool,

        // /// Whether to allow write access to the tape
        // #[arg(short, long)]
        // write: bool,
    },

    /// Eject the current tape if you used any
    Eject,
}

pub const APP_NAME: &str = "tapes";

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Show { name, list } => match name {
            Some(name) => {
                commands::show::show_specific_tape(&name)?;
            }
            None => {
                if !list {
                    commands::show::show_all_tapes()?
                } else {
                    println!("{}", "Known tapes: ".bold().italic().green());
                    commands::show::list_all_known_tapes()?
                }
            }
        },

        Commands::Insert {
            name,
            path,
            dry_run,
            reinsert
        } => {
            if !reinsert {
                commands::insert::insert(name, path, dry_run)?;
            } else {
                commands::eject::eject()?;
                commands::insert::insert(name, path, dry_run)?;
            }
        }

        Commands::Eject => commands::eject::eject()?,
    }

    Ok(())
}
