#!/bin/bash
set -e

echo "Building WASM module..."
wasm-pack build --target web

echo "Creating dist directory..."
mkdir -p dist

echo "Copying static files to dist..."
cp -r static/* dist/
cp -r pkg dist/

# Try different ports if the default one is in use
for PORT in 8081 8082 8083 8084 8085; do
  echo "Attempting to start server on port $PORT..."
  cd dist
  python3 -m http.server $PORT && break || echo "Port $PORT is in use, trying next port..."
  cd ..
done
