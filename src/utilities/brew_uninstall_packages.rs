use std::process::{exit, Command};
use crate::models::BrewPackage;
use crate::constants::PROGRAM;

#[allow(dead_code)]
pub fn uninstall_packages(packages: &Vec<BrewPackage>) {
    for package in packages {
        let output = Command::new(PROGRAM)
            .arg("uninstall")
            .arg("-f") // Use -f flag to force uninstall without confirmation
            .arg(&package.name)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    eprintln!("Package {} has been removed.", package.name);
                } else {
                    eprintln!(
                        "Failed to uninstall package {}. Error: {}",
                        package.name,
                        String::from_utf8_lossy(&output.stderr)
                    );
                    exit(1);
                }
            }
            Err(error) => {
                eprintln!("Failed to execute command: {}", error);
                exit(1);
            }
        }
    }
}
