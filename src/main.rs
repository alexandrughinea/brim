use std::time::Instant;

use clap::{Arg, Command};
use console::{style, StyledObject};
use dialoguer::MultiSelect;

use models::{BrewPackage, BrewPackageState};
use utilities::{fetch_packages, format_package_name, install_packages, list_installed_packages, remove_packages};

mod constants;
mod models;
mod utilities;

//brim -u="https://raw.githubusercontent.com/alexandrughinea/brew-packages-json/main/list.json"

#[tokio::main]
async fn main() {
    let start_time = Instant::now();

    let matches = Command::new("BRIM")
        .arg(
            Arg::new("url")
                .long("url")
                .help("Your remote file location"),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .help("List preinstalled Homebrew packages."),
        )
        .arg(
            Arg::new("remove")
                .long("remove")
                .help("Remove Homebrew packages (forced)."))
        .get_matches();

    let installed_packages = list_installed_packages();

    if let Some(value) = matches.get_one::<String>("url") {
        match fetch_packages(value).await {
            Ok(packages) => {
                let prompt: String = format!(
                    "BRIM found {} packages to install with Homebrew",
                    packages.len()
                );
                let package_option: Vec<_> = packages
                    .iter()
                    .map(|package| {
                        let is_installed = installed_packages
                            .iter()
                            .any(|p| p.name.to_string().contains(&package.name));

                        let is_cask = package.cask.is_some();
                        let state = if is_installed && is_cask {
                            BrewPackageState::InstalledCask
                        } else if is_installed {
                            BrewPackageState::Installed
                        } else if is_cask {
                            BrewPackageState::Cask
                        } else {
                            BrewPackageState::Default
                        };

                        let formatted_name = format_package_name(&package, Some(state));
                        let style_package_name = style(formatted_name);

                        if is_cask {
                            style_package_name.magenta()
                        } else {
                            style_package_name.green().dim()
                        }
                    })
                    .collect();
                let defaults: Vec<bool> = packages
                    .iter()
                    .map(|package| {
                        !installed_packages
                            .iter()
                            .find(|p| p.name.to_string().contains(&package.name))
                            .is_some()
                    })
                    .collect();
                let package_selections: Vec<usize> = MultiSelect::new()
                    .with_prompt(prompt)
                    .items(&package_option)
                    .defaults(&defaults) // uncomment to preselect options.
                    .interact()
                    .unwrap();

                let mut selected_packages: Vec<BrewPackage> = vec![];

                for index in &package_selections {
                    let package_clone: BrewPackage = packages[*index].clone();
                    selected_packages.push(package_clone);
                }

                if selected_packages.len() > 0 {
                    install_packages(&selected_packages);
                }
            }
            Err(err) => {
                eprintln!("Error fetching packages: {:?}", err);
            }
        }
    }

    if let Some(_value) = matches.get_one::<String>("list") {
        let joined_installed_packages = installed_packages
            .iter()
            .map(|p| p.name.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        eprintln!("Installed packages: {}\n", joined_installed_packages);
    }

    if let Some(_value) = matches.get_one::<String>("remove") {
        let prompt: String = format!(
            "BRIM found {} packages to uninstall",
            installed_packages.len()
        );
        let package_option: Vec<_> = installed_packages
            .iter()
            .map(|package| -> StyledObject<String> {
                style(package.name.to_string()).dim()
            })
            .collect();
        let package_selections: Vec<usize> = MultiSelect::new()
            .with_prompt(prompt)
            .items(&package_option)
            .interact()
            .unwrap();

        let mut selected_packages: Vec<BrewPackage> = vec![];

        for index in &package_selections {
            let package_clone: BrewPackage = installed_packages[*index].clone();
            selected_packages.push(package_clone);
        }

        if selected_packages.len() > 0 {
            remove_packages(&selected_packages);
        }
    }

    eprintln!("Elapsed time: {:?} seconds", start_time.elapsed().as_secs());
}
