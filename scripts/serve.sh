#!/bin/bash
set -e

echo "Building WASM module..."
wasm-pack build --target web

echo "Creating dist directory..."
mkdir -p dist

echo "Copying files to dist..."
cp -r pkg dist/
cp index.html dist/
cp favicon.ico dist/ 2>/dev/null || touch dist/favicon.ico

echo "Starting local server on port 8081..."
cd dist
python3 -m http.server 8081 || echo "Failed to start server. Is port 8081 already in use?"
