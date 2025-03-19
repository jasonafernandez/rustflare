use std::fs;
use crate::StorageConfig;

pub fn store_certificates(domain: &str, cert: String, key: String, storage_cfg: &StorageConfig) {
    let path = format!("{}/{}", storage_cfg.certs_base_path, domain);
    fs::create_dir_all(&path).expect("Failed to create cert directory");
    fs::write(format!("{}/{}", path, storage_cfg.cert_file_name), cert)
        .expect("Failed to write certificate");
    fs::write(format!("{}/{}", path, storage_cfg.key_file_name), key)
        .expect("Failed to write key");
}
