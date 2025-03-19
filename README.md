# 🌟 Rustflare

A lightweight Rust-based service for automating TLS certificate issuance and renewal via ACME DNS-01 using Cloudflare and ZeroSSL with External Account Binding (EAB).

## 🚀 Features
- Rust-based ACME DNS-01 certificate automation
- Fully Dockerized setup
- YAML-based configuration with EAB (ZeroSSL)
- Supports multiple domains and subdomains
- Cloudflare DNS integration

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
