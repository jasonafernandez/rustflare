use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct CloudflareConfig {
    pub api_token: String,
    pub zone_id: String,
    pub api_base_url: String,
    pub dns_record_ttl: u32,
}

pub async fn create_dns_challenge(
    domain: &str,
    txt_value: &str,
    cf_cfg: &CloudflareConfig,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let record_name = format!("_acme-challenge.{}", domain);
    let api_url = cf_cfg.api_base_url.replace("{zone_id}", &cf_cfg.zone_id);

    let response = client
        .post(&api_url)
        .header("Authorization", format!("Bearer {}", cf_cfg.api_token))
        .header("Content-Type", "application/json")
        .body(
            json!({
                "type": "TXT",
                "name": record_name,
                "content": txt_value,
                "ttl": cf_cfg.dns_record_ttl
            })
            .to_string(),
        )
        .send()
        .await?;

    if response.status().is_success() {
        println!("[{}] Cloudflare TXT record created", domain);
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "<empty body>".to_string());
        Err(format!("Cloudflare API error ({}): {}", status, body).into())
    }
}
