use crate::config::TapeManifest;
use anyhow::Result;
use std::process::Command;
use yansi::Paint;
use crate::quit;

pub fn check_dependencies(tape: &TapeManifest) -> Result<()> {
    if let Some(dependencies) = &tape.tape.dependencies {
        for deps in dependencies {
            let line = format!("command -v {}", deps);
            let result = Command::new("sh").arg("-c").arg(&line).output()?;

            if result.stdout.is_empty() {
                quit(&format!("Dependency `{}` not found", deps.bold()))

            }
        }
    }

    Ok(())
}
