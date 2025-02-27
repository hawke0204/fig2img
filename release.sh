#!/bin/bash
set -e

echo "🚀 Running release process..."
cargo make release
echo "✨ Release completed successfully!"
cargo make cleanup