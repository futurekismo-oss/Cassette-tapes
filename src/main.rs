mod commands;
mod config;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    commands: String,
}

pub const APP_NAME: &str = "tapes";

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.commands == "show" {
        commands::show::execute()?;
    }

    Ok(())
}
