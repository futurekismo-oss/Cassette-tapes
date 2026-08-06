mod commands;
mod config;
use clap::{Parser, Subcommand};
use yansi::{Paint};

#[derive(Parser, Debug)]
#[command(name = "tape")]
#[command(about = "A tool for managing rice installations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Show {
        name: Option<String>,

        #[arg(long)]
        list: bool,
    },
}

pub const APP_NAME: &str = "tapes";

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Show { name, list } => match name {
            Some(name) => {
                commands::show::show_specific_tape(name)?;
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
    }

    Ok(())
}
