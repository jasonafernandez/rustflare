use crate::{cloudflare, storage, AcmeConfig, CloudflareConfig, StorageConfig};
use acme_lib::{Account, Directory, create_p384_key};
use tokio::time::{sleep, interval, Duration};
use acme_lib::persist::MemoryPersist;

pub async fn request_certificate(
    domain: &str,
    acme_cfg: &AcmeConfig,
    cf_cfg: &CloudflareConfig,
    storage_cfg: &StorageConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let persist = MemoryPersist::new();
    let dir = Directory::from_url(&acme_cfg.ca_url).await?;

    // EAB credentials integration
    let account = dir.account_registration()
        .eab(&acme_cfg.eab.key_id, &acme_cfg.eab.mac_key)
        .register().await?;

    let order = account.new_order(domain, &[]).await?;
    let auth = order.authorizations().await?.remove(0);

    if let Some(challenge) = auth.dns_challenge() {
        let token = challenge.dns_proof();
        cloudflare::create_dns_challenge(domain, &token, cf_cfg).await?;

        println!("[{}] Waiting for DNS propagation...", domain);
        sleep(Duration::from_secs(30)).await;

        challenge.validate().await?;
        println!("[{}] Validated. Issuing certificate...", domain);

        let pkey = order.private_key();
        let cert = order.finalize().await?;

        storage::store_certificates(
            domain,
            cert.certificate().to_pem(),
            pkey.private_key().to_pem(),
            storage_cfg,
        );

        println!("[{}] Certificate stored.", domain);
        Ok(())
    } else {
        Err("DNS-01 challenge not found".into())
    }
}

pub async fn auto_renew(
    domain: String,
    acme_cfg: AcmeConfig,
    cf_cfg: CloudflareConfig,
    storage_cfg: StorageConfig,
) {
    let mut interval = interval(Duration::from_secs(
        60 * 60 * 24 * acme_cfg.renewal_interval_days,
    ));

    loop {
        interval.tick().await;
        if let Err(e) = request_certificate(&domain, &acme_cfg, &cf_cfg, &storage_cfg).await {
            eprintln!("[{}] Renewal failed: {}", domain, e);
        }
    }
}
