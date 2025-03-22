use reqwest::Response;
use reqwest::header;
use reqwest::Client;
use serde_json::json;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, serde::Deserialize)]
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
        Err(format!("Cloudflare API error (create): {} - {}", status, body).into())
    }
}

pub async fn delete_dns_challenge(
    domain: &str,
    txt_value: &str,
    cf_cfg: &CloudflareConfig,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let record_name = format!("_acme-challenge.{}", domain);

    let list_url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?type=TXT&name={}",
        cf_cfg.zone_id, record_name
    );

    let response = client
        .get(&list_url)
        .header("Authorization", format!("Bearer {}", cf_cfg.api_token))
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let Some(records) = json.get("result").and_then(|r| r.as_array()) else {
        return Err("Could not parse DNS records from Cloudflare".into());
    };

    for record in records {
        if record["content"] == txt_value {
            if let Some(id) = record["id"].as_str() {
                let delete_url = format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                    cf_cfg.zone_id, id
                );

                let del_resp = client
                    .delete(&delete_url)
                    .header("Authorization", format!("Bearer {}", cf_cfg.api_token))
                    .send()
                    .await?;

                if del_resp.status().is_success() {
                    println!("[{}] Cloudflare TXT record deleted", domain);
                } else {
                    let body = del_resp.text().await.unwrap_or_default();
                    println!("[{}] Failed to delete TXT record: {}", domain, body);
                }
            }
        }
    }

    Ok(())
}
