use serde::Serialize;
use std::time::Duration;

#[derive(Serialize)]
pub struct WebhookPayload {
    pub status: String,
    pub total: usize,
    pub completed: usize,
    pub failed: usize,
    pub packages: Vec<PackageResult>,
    pub elapsed_seconds: u64,
}

#[derive(Serialize)]
pub struct PackageResult {
    pub name: String,
    pub status: String,
}

const WEBHOOK_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn post_webhook(url: &str, payload: WebhookPayload) -> Result<(), String> {
    let client = reqwest::Client::builder()
        .timeout(WEBHOOK_TIMEOUT)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    client
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to post webhook: {}", e))?;

    Ok(())
}
