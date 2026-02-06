use crate::constants::PROGRAM;
use std::process::Command;


pub fn ensure_brew_available() -> Result<(), String> {
    Command::new(PROGRAM)
        .arg("--version")
        .output()
        .map_err(|e| format!("Homebrew not found: {}. Is brew installed and in PATH?", e))?;
    Ok(())
}
