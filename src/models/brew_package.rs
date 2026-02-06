use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BrewPackage {
    pub name: String,
    pub category: Option<String>,
    #[allow(dead_code)]
    pub url: Option<String>,
    pub cask: Option<bool>,
    #[allow(dead_code)]
    pub version: Option<String>
}
