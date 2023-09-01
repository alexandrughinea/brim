use crate::models::BrewPackage;

pub async fn fetch_packages(url: &String) -> Result<Vec<BrewPackage>, reqwest::Error> {
    let response = reqwest::get(url)
        .await?
        .json::<Vec<BrewPackage>>()
        .await?;

    Ok(response)
}
