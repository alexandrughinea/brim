use crate::constants::PROGRAM;
use crate::models::BrewPackage;
use std::process::{Command, Stdio};

pub fn list_installed_packages() -> Result<Vec<BrewPackage>, String> {
    let output = Command::new(PROGRAM)
        .arg("list")
        .stdout(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to run brew: {}. Is Homebrew installed and in PATH?", e))?;

    let stdout = &output.stdout;
    let result: Vec<BrewPackage> = String::from_utf8_lossy(stdout)
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| BrewPackage {
            name: s.to_string(),
            category: None,
            url: None,
            cask: None,
            version: None,
        })
        .collect();

    Ok(result)
}
