# 🌟 Rustflare

A lightweight Rust-based service for automating TLS certificate issuance and renewal via ACME DNS-01 using Cloudflare and ZeroSSL with External Account Binding (EAB).

## 🚀 Features
- Rust-based ACME DNS-01 certificate automation
- Fully Dockerized setup
- YAML-based configuration with EAB (ZeroSSL)
- Supports multiple domains and subdomains
- Cloudflare DNS integration

## 📌 Compatibility

Rustflare integrates seamlessly with the [rust-rpxy](https://github.com/junkurihara/rust-rpxy) reverse proxy:

- Rustflare automatically manages TLS certificates and stores them in a shared directory.
- Configure rust-rpxy to directly read certificates from this shared location, enabling TLS termination effortlessly.

## 📁 Project Structure

See [docker/compose.yaml](docker/compose.yaml) and [docker/config.example.yaml](docker/config.example.yaml) to get started quickly.

## ⚙️ Setup Instructions

Clone repository:

```bash
git clone https://github.com/jasonafernandez/rustflare.git
cd rustflare/docker
cp config.example.yaml config.yaml
# Edit config.yaml with your personal settings
nano config.yaml
