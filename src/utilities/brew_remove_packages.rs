use std::process::{exit, Command, Stdio};
use crate::models::BrewPackage;
use crate::constants::PROGRAM;

pub fn remove_packages(packages: &Vec<BrewPackage>) {
    for package in packages {
        let output = Command::new(PROGRAM)
            .arg("remove")
            .arg("-f") // Use -f flag to force uninstall without confirmation
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg(&package.name)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    eprintln!("Package {} has been removed.\nNow cleaning up dependencies.", package.name);

                    let auto_remove_output = Command::new(PROGRAM)
                        .arg("autoremove")
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .output();

                    match auto_remove_output {
                        Ok(_output) => {
                            eprintln!("Dependencies have been successfully cleaned up.");
                            exit(0);
                        }
                        Err(_error) => {
                            exit(1);
                        }
                    }
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
