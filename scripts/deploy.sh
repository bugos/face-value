#!/bin/bash
set -e

echo "Building WASM module..."
wasm-pack build --target web

echo "Creating dist directory..."
rm -rf dist
mkdir -p dist

echo "Copying files to dist directory..."
cp -r pkg dist/
cp index.html dist/
cp favicon.ico dist/ 2>/dev/null || touch dist/favicon.ico

echo "✅ Deployment files are ready in the 'dist' directory."
echo "📱 Your app will be available at: https://bugos.github.io/face-value/"
echo "🔍 You can view the deployment status at: https://github.com/bugos/face-value/actions"
