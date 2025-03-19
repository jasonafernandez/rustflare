use reqwest::Client;
use serde_json::json;
use crate::CloudflareConfig;

pub async fn create_dns_challenge(
    domain: &str,
    txt_value: &str,
    cf_cfg: &CloudflareConfig,
) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let record_name = format!("_acme-challenge.{}", domain);

    let api_url = cf_cfg.api_base_url.replace("{zone_id}", &cf_cfg.zone_id);

    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", cf_cfg.api_token))
        .json(&json!({
            "type": "TXT",
            "name": record_name,
            "content": txt_value,
            "ttl": cf_cfg.dns_record_ttl
        }))
        .send()
        .await?;

    response.error_for_status()?;
    println!("[{}] Cloudflare TXT record created", domain);
    Ok(())
}
