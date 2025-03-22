use crate::StorageConfig;
use std::fs;
use std::path::Path;

pub fn store_certificates(
    domain: &str,
    cert_pem: &[u8],
    key_pem: &[u8],
    config: &StorageConfig,
) {
    let domain_path = Path::new(&config.certs_base_path).join(domain);
    fs::create_dir_all(&domain_path).unwrap();

    let cert_path = domain_path.join(&config.cert_file_name);
    let key_path = domain_path.join(&config.key_file_name);

    fs::write(cert_path, cert_pem).unwrap();
    fs::write(key_path, key_pem).unwrap();
}
