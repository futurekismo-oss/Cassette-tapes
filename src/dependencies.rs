use anyhow::{Result};
use yansi::Paint;
use crate::config::TapeManifest;
use std::process::Command;

pub fn check_dependencies(tape: &TapeManifest) -> Result<()> {
   let deps = tape.dependencies.as_ref().unwrap();

   let binaries = deps.binaries.as_ref().unwrap();

   for bin in binaries {
      let line = format!("command -v {}", bin);
      let result = Command::new("sh").arg("-c").arg(&line).output()?;


      if result.stdout.is_empty() {
         panic!("Dependency `{}` not found", bin.bold())
      }
      
      // println!("Result for {}: {:?}", bin, result.stderr)      

      
   
      
   }

   

   
   Ok(())
} 
