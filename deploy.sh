#!/bin/bash

# Build the WASM module
wasm-pack build --target web

# Create a deployment directory
rm -rf deploy
mkdir -p deploy

# Copy the necessary files
cp -r pkg deploy/
cp index.html deploy/
cp favicon.ico deploy/

# Create a test page with example links
cat > deploy/test.html << 'EOL'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Face Value - Test Links</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 2rem auto; padding: 0 1rem; }
        .link-group { margin: 2rem 0; }
        a { display: block; margin: 0.5rem 0; color: #0066cc; }
    </style>
</head>
<body>
    <h1>Face Value Calculator - Test Links</h1>
    <div class="link-group">
        <h2>Example Links:</h2>
        <a href="https://bugos.github.io/face-value/#amount=1000&interest=5&start_date=2023-01-01">$1,000 at 5% from 2023-01-01</a>
        <a href="https://bugos.github.io/face-value/#amount=5000&interest=3.5&start_date=2024-01-01">$5,000 at 3.5% from 2024-01-01</a>
        <a href="https://bugos.github.io/face-value/#amount=10000&interest=7&start_date=2022-06-15">$10,000 at 7% from 2022-06-15</a>
    </div>
    <div class="link-group">
        <h2>Instructions:</h2>
        <p>Click any link above to test the calculator with different parameters.</p>
        <p>You can also modify the URL parameters directly:</p>
        <ul>
            <li><code>amount</code>: The initial debt amount (numeric)</li>
            <li><code>interest</code>: Annual interest rate as percentage (numeric)</li>
            <li><code>start_date</code>: Start date in YYYY-MM-DD format</li>
        </ul>
    </div>
    <p class="mt-4">
        <a href="https://github.com/bugos/face-value">View source on GitHub</a>
    </p>
</body>
</html>
EOL

echo "Deployment files are ready in the 'deploy' directory."
echo "Your app will be available at: https://bugos.github.io/face-value/"
echo "You can view the deployment status at: https://github.com/bugos/face-value/actions"
