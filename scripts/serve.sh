#!/bin/bash
set -e

echo "Building WASM module..."
wasm-pack build --target web

echo "Creating dist directory..."
mkdir -p dist

echo "Copying static files to dist..."
cp -r static/* dist/
cp -r pkg dist/

echo "Starting local server on port 8081..."
cd dist
python3 -m http.server 8081 || echo "Failed to start server. Is port 8081 already in use?"
