use crate::constants::PROGRAM;
use crate::models::BrewPackage;
use std::process::{exit, Command, Stdio};

pub fn install_packages(packages: &Vec<BrewPackage>) {
    for package in packages {
        let mut command: Command = Command::new(PROGRAM);
        let args = command.arg("install").arg(&package.name);

        if package.cask.is_some() {
            args.arg("--cask");
            eprintln!("Package {} has a cask option", package.name);
        }

        let output = args
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    eprintln!("Package {} has been installed.", package.name);
                    exit(0);
                } else {
                    eprintln!(
                        "Failed to install package {}. Error: {}",
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
