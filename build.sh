#!/bin/bash

##
## Build Script for Electromagnetic Field Response Simulator
##
## This script compiles the Rust WASM module and prepares the
## static site for deployment to Cloudflare Pages or other hosts.
##
## Prerequisites:
## - Rust toolchain (rustup)
## - wasm-pack (cargo install wasm-pack)
## - wasm32-unknown-unknown target
##

set -e  # Exit on error

echo "=========================================="
echo "Building EM Field Response Simulator"
echo "=========================================="

# Step 1: Check for required tools
echo ""
echo "Checking prerequisites..."

if ! command -v rustc &> /dev/null; then
    echo "Error: Rust is not installed"
    echo "Please install from: https://rustup.rs/"
    exit 1
fi

if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed"
    echo "Install with: cargo install wasm-pack"
    exit 1
fi

echo "✓ All prerequisites found"

# Step 2: Build WASM module
echo ""
echo "Building WebAssembly module..."
cd wasm

# Build with wasm-pack for web target
# - --target web: Generates ES module output
# - --release: Optimized build
# - --out-dir: Output to frontend/pkg directory
wasm-pack build --target web --release --out-dir ../frontend/pkg

if [ $? -eq 0 ]; then
    echo "✓ WASM module built successfully"
else
    echo "✗ WASM build failed"
    exit 1
fi

cd ..

# Step 3: Copy frontend files to dist directory
echo ""
echo "Preparing distribution files..."

# Create dist directory
rm -rf dist
mkdir -p dist

# Copy frontend files
cp frontend/index.html dist/
cp frontend/styles.css dist/
cp frontend/app.js dist/
cp -r frontend/pkg dist/

echo "✓ Distribution files prepared in ./dist"

# Step 4: Display build summary
echo ""
echo "=========================================="
echo "Build completed successfully!"
echo "=========================================="
echo ""
echo "Output directory: ./dist"
echo "WASM module: ./frontend/pkg"
echo ""
echo "To test locally:"
echo "  1. Serve the dist directory with a static file server"
echo "  2. Example: python3 -m http.server 8000 --directory dist"
echo "  3. Open http://localhost:8000 in your browser"
echo ""
echo "To deploy to Cloudflare Pages:"
echo "  1. Push changes to your Git repository"
echo "  2. Connect repository to Cloudflare Pages"
echo "  3. Set build command: ./build.sh"
echo "  4. Set build output directory: dist"
echo ""
