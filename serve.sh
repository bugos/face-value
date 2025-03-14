#!/bin/bash

# Build the WASM module
wasm-pack build --target web

# Create a dist directory
mkdir -p dist

# Copy the necessary files
cp -r pkg dist/
cp index.html dist/

# Serve the dist directory
cd dist
python3 -m http.server 8081
