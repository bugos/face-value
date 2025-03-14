#!/bin/bash

function test_dev() {
    echo "Testing development build..."
    trunk serve --open
}

function test_prod() {
    echo "Testing production build..."
    # Build the release version
    trunk build --release

    # Create a temporary Python HTTP server to serve the files
    cd dist
    echo "Starting local server at http://localhost:8000"
    echo "Test your app at: http://localhost:8000/index.html"
    echo "Press Ctrl+C to stop the server"
    python3 -m http.server 8000
}

# Check if argument is provided
if [ "$1" == "dev" ]; then
    test_dev
elif [ "$1" == "prod" ]; then
    test_prod
else
    echo "Usage: ./test-local.sh [dev|prod]"
    echo "  dev  - Run development server with hot reload"
    echo "  prod - Build and serve production version"
    exit 1
fi
