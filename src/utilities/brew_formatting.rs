use crate::models::BrewPackage;

#[allow(dead_code)]
pub fn format_package_name(value: &BrewPackage) -> String {
    value.name.to_string()
}
