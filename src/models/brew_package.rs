use serde::Deserialize;

// Struct to deserialize options from the JSON response
#[derive(Deserialize, Clone)]
pub struct BrewPackage {
    pub name: String,
    pub category: Option<String>,
    pub url: Option<String>,
    pub cask: Option<bool>,
}

pub enum BrewPackageState {
    InstalledCask,
    Installed,
    Cask,
}
