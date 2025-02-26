# Figma Image Processing

### Features

- Download Figma image nodes
- Convert PNG to WebP
- Convert PNG to AVIF

### Getting Started

Create a `config.toml` file in the root directory:

```toml
figma_access_token = "TOKEN"
figma_file_key = "KEY"
```

### Usage

#### Download Images

```bash
cargo run download
cargo run download --download_dir ./downloads
```

#### Convert Images

```bash
cargo run convert --format webp
cargo run convert --format avif
```
