# 🌟 Rustflare

A lightweight Rust-based service for automating TLS certificate issuance and renewal via ACME DNS-01 using Cloudflare.

## 🚀 Features
- Rust-based ACME DNS-01 certificate automation
- Fully Dockerized setup
- YAML-based configuration
- Supports multiple domains and subdomains
- ZeroSSL and Let's Encrypt compatible

## 📁 Project Structure

See [docker/compose.yaml](docker/compose.yaml) and [docker/config.example.yaml](docker/config.example.yaml) to get started quickly.

## ⚙️ Setup Instructions

Clone repository:

```bash
git clone https://github.com/yourusername/rustflare.git
cd rustflare/docker
cp config.example.yaml config.yaml
# Edit config.yaml with your personal settings
vim config.yaml
