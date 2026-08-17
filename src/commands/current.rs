use crate::{commands::insert::locate_local_dir, debug, STATE_FILE};
use anyhow::Result;
use std::fs;
use yansi::Paint;

pub fn display_currently_inserted_tape() -> Result<()> {
    let path = locate_local_dir()?.join(STATE_FILE);

    if path.exists() {
        let current_tape: String = fs::read_to_string(path)?;

        let title_len = "CURRENT-RICE: ".len();
        let strings = current_tape.split_at(title_len);

        let (title, name) = strings;
        let name: Vec<_> = name.split("\n").collect();
        let name = name[0];

        if debug::is_debug() {
            debug::info("status", "ok");
            debug::info("tape", name);
        } else {
            println!("{}{}", title.bold().green(), name);
        }
    } else {
        if debug::is_debug() {
            debug::info("status", "empty");
        } else {
            println!("{}", "No tapes currently inserted".bold().red());
        }
    }

    Ok(())
}
