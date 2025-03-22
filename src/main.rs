use acme2_eab::{AccountBuilder, Csr, DirectoryBuilder, OrderBuilder};
use acme2_eab::ChallengeStatus;
use openssl::hash::{hash, MessageDigest};
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use serde::Deserialize;
use std::fs;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

mod cloudflare;
mod storage;

use storage::store_certificates;

#[derive(Debug, Deserialize)]
struct Config {
    acme: AcmeConfig,
    cloudflare: cloudflare::CloudflareConfig,
    storage: StorageConfig,
}

#[derive(Debug, Deserialize)]
struct AcmeConfig {
    ca_url: String,
    email: String,
    eab: EabConfig,
    domains: Vec<String>,
    renewal_interval_days: u64,
}

#[derive(Debug, Deserialize)]
struct EabConfig {
    key_id: String,
    mac_key: String,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    certs_base_path: String,
    cert_file_name: String,
    key_file_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = {
        let contents = fs::read_to_string("/app/config.yaml")?;
        serde_yaml::from_str(&contents)?
    };

    for domain in &config.acme.domains {
        println!("Processing certificate for domain: {}", domain);
        request_certificate(domain, &config).await?;
    }

    Ok(())
}

async fn request_certificate(domain: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let directory = DirectoryBuilder::new(config.acme.ca_url.clone()).build().await?;

    let mac_key_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(&config.acme.eab.mac_key)
        .map_err(|e| format!("Failed to decode base64 MAC key: {}", e))?;
    let mac_key_pkey = PKey::hmac(&mac_key_bytes)?;

    let account = Arc::new(
        AccountBuilder::new(directory)
            .contact(vec![format!("mailto:{}", config.acme.email)])
            .terms_of_service_agreed(true)
            .external_account_binding(config.acme.eab.key_id.clone(), mac_key_pkey)
            .build()
            .await?,
    );

    let mut order_builder = OrderBuilder::new(Arc::clone(&account));
    order_builder.add_dns_identifier(domain.to_string());
    let order = order_builder.build().await?;

    let auths = order.authorizations().await?;
    let chall = auths[0]
        .get_challenge("dns-01")
        .ok_or("dns-01 challenge not found")?;

    let key_auth = chall
        .key_authorization()?
        .ok_or("key_authorization was None")?;

    let digest = hash(MessageDigest::sha256(), key_auth.as_bytes())?;
    let dns_proof = URL_SAFE_NO_PAD.encode(digest);

    cloudflare::create_dns_challenge(domain, &dns_proof, &config.cloudflare).await?;

    // Wait for DNS propagation
    sleep(Duration::from_secs(30)).await;

    chall.validate().await?;

    let mut attempts = 0;
    loop {
        let updated_auths = order.authorizations().await?;
        let updated_chall = updated_auths[0]
            .get_challenge("dns-01")
            .ok_or("dns-01 challenge not found during polling")?;

        match updated_chall.status {
            ChallengeStatus::Valid => break,
            ChallengeStatus::Invalid => return Err("DNS-01 challenge validation failed".into()),
            _ => {
                if attempts >= 10 {
                    return Err("DNS-01 challenge did not become valid after waiting".into());
                }
                sleep(Duration::from_secs(5)).await;
                attempts += 1;
            }
        }
    }

    let rsa = Rsa::generate(2048)?;
    let pkey = PKey::from_rsa(rsa)?;
    let csr = Csr::Automatic(pkey.clone());

    let finalized_order = order.finalize(csr).await?;
    let certs = finalized_order
        .certificate()
        .await?
        .ok_or("No certificate returned")?;

    let cert_pem = certs
        .iter()
        .map(|cert| {
            let pem = cert.to_pem()?; 
            String::from_utf8(pem).map_err(|e| Box::<dyn std::error::Error>::from(e))
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?
        .join("\n");

    let priv_key_pem = String::from_utf8(pkey.private_key_to_pem_pkcs8()?)?;

    store_certificates(
        domain,
        cert_pem.as_bytes(),
        priv_key_pem.as_bytes(),
        &config.storage,
    );

    cloudflare::delete_dns_challenge(domain, &dns_proof, &config.cloudflare).await?;

    println!("Certificate stored for domain: {}", domain);

    Ok(())
}
