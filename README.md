# Figma To Image

### Features

- Download Figma image nodes
- Convert PNG to WebP
- Convert PNG to AVIF

### Getting Started

Set up your environment variables:

```bash
# fig2img
export FIGMA_ACCESS_TOKEN="YOUR_ACCESS_TOKEN"
export FIGMA_FILE_KEY="YOUR_FILE_KEY"
# fig2img end
```

### Usage

#### Download Images

```bash
fig2img download --download_dir ./downloads
```

#### Convert Images

```bash
fig2img convert --format webp
fig2img convert --format avif
```
