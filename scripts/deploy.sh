#!/bin/bash
set -e

echo "Building WASM module..."
wasm-pack build --target web

echo "Creating dist directory..."
rm -rf dist
mkdir -p dist

echo "Copying static files to dist directory..."
cp -r static/* dist/
cp -r pkg dist/

echo "✅ Deployment files are ready in the 'dist' directory."
echo "📱 Your app will be available at: https://bugos.github.io/face-value/"
echo "🔍 You can view the deployment status at: https://github.com/bugos/face-value/actions"
