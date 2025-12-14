#!/bin/bash

# HyperReview Bundle Compression Script
# Uses UPX to compress release binaries for smaller distribution size
# Usage: ./scripts/compress-bundles.sh

set -e

echo "=== HyperReview Bundle Compression ==="
echo ""

# Check if UPX is installed
if ! command -v upx &> /dev/null; then
    echo "⚠️  UPX not found. Installing UPX..."
    echo ""

    # Detect OS and install UPX
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v apt-get &> /dev/null; then
            sudo apt-get update && sudo apt-get install -y upx-ucl
        elif command -v yum &> /dev/null; then
            sudo yum install -y upx
        elif command -v pacman &> /dev/null; then
            sudo pacman -S --noconfirm upx
        else
            echo "❌ Could not detect package manager. Please install UPX manually."
            exit 1
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v brew &> /dev/null; then
            brew install upx
        else
            echo "❌ Homebrew not found. Please install UPX manually from https://upx.github.io/"
            exit 1
        fi
    else
        echo "❌ Unsupported OS. Please install UPX manually from https://upx.github.io/"
        exit 1
    fi
fi

# Verify UPX is now available
if ! command -v upx &> /dev/null; then
    echo "❌ UPX installation failed. Please install UPX manually."
    exit 1
fi

echo "✅ UPX version: $(upx --version | head -n 1)"
echo ""

# Find all executable binaries in the bundle directory
BUNDLE_DIR="src-tauri/target/release/bundle"

if [ ! -d "$BUNDLE_DIR" ]; then
    echo "❌ Bundle directory not found: $BUNDLE_DIR"
    echo "   Please run 'npm run tauri:build' first"
    exit 1
fi

echo "📦 Compressing binaries in $BUNDLE_DIR..."
echo ""

# Compress executables
find "$BUNDLE_DIR" -type f \( -name "hyperreview" -o -name "hyperreview.exe" -o -name "hyperreview.app" \) | while read -r binary; do
    if [ -f "$binary" ]; then
        original_size=$(du -h "$binary" | cut -f1)
        echo "Compressing: $binary (original size: $original_size)"

        # Create backup
        cp "$binary" "${binary}.backup"

        # Compress with best compression
        if upx --best --lzma "$binary"; then
            compressed_size=$(du -h "$binary" | cut -f1)
            echo "  ✅ Compressed to: $compressed_size"

            # Verify the compressed binary
            if upx -t "$binary" > /dev/null 2>&1; then
                echo "  ✅ Integrity check passed"
            else
                echo "  ❌ Integrity check failed, restoring backup"
                mv "${binary}.backup" "$binary"
            fi
        else
            echo "  ⚠️  Compression failed, keeping original"
            mv "${binary}.backup" "$binary"
        fi

        echo ""
    fi
done

echo "=== Compression Complete ==="
echo ""
echo "💡 Tip: To skip compression, use 'npm run tauri:build' instead"
