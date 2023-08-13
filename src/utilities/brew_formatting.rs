use crate::models::{BrewPackage, BrewPackageState};

pub fn format_package_name(value: &BrewPackage, template: Option<BrewPackageState>) -> String {
    let case = match template {
        Some(BrewPackageState::InstalledCask) => "cask installed",
        Some(BrewPackageState::Installed) => "installed",
        Some(BrewPackageState::Cask) => "cask",
        _ => "",
    };

    if !template.is_some() {
        return format!("{}", value.name);
    }

    format!("{} - [{}]", value.name, case)
}
