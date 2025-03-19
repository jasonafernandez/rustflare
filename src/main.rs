#[derive(Debug, Deserialize, Clone)]
struct Config {
    acme: AcmeConfig,
    cloudflare: CloudflareConfig,
    storage: StorageConfig,
}

#[derive(Debug, Deserialize, Clone)]
struct AcmeConfig {
    ca_url: String,
    eab: EabConfig,
    domains: Vec<String>,
    renewal_interval_days: u64,
}

#[derive(Debug, Deserialize, Clone)]
struct EabConfig {
    key_id: String,
    mac_key: String,
}

#[derive(Debug, Deserialize, Clone)]
struct CloudflareConfig {
    api_token: String,
    zone_id: String,
    api_base_url: String,
    dns_record_ttl: u32,
}

#[derive(Debug, Deserialize, Clone)]
struct StorageConfig {
    certs_base_path: String,
    cert_file_name: String,
    key_file_name: String,
}
