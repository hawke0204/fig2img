# Figma image tool

### Features

- Figma image nodes downloading
- PNG -> Webp converting
- PNG -> AVIF converting

### Developer Guide

Create a `config.toml` file in the root directory.

```toml
// config.toml
figma_access_token = "TOKEN"
figma_api_url = "URL"
figma_file_key = "KEY"
```

#### Download image nodes

```bash
cargo run download
cargo run download --download_dir ./downloads
```

#### Convert image

```bash
cargo run convert --format webp
cargo run convert --format avif
```
