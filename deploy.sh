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

echo "Creating test page with example links..."
cat > dist/test.html << 'EOL'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Face Value - Test Links</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 2rem auto;
            padding: 0 1rem;
            background-color: #f5f5f5;
        }
        .link-group {
            margin: 2rem 0;
            background-color: white;
            border-radius: 8px;
            padding: 1.5rem;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        }
        .examples {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
            gap: 1rem;
        }
        a.example-link {
            display: block;
            margin: 0.5rem 0;
            color: #0066cc;
            padding: 0.75rem;
            background-color: #f0f7ff;
            border-radius: 4px;
            text-decoration: none;
            transition: background-color 0.2s;
        }
        a.example-link:hover {
            background-color: #e0f0ff;
            text-decoration: underline;
        }
        h1, h2 {
            color: #333;
        }
        .footer {
            margin-top: 2rem;
            text-align: center;
        }
    </style>
</head>
<body>
    <h1>Face Value Calculator - Test Links</h1>
    <div class="link-group">
        <h2>Example Links:</h2>
        <div class="examples">
            <a class="example-link" href="https://bugos.github.io/face-value/#amount=1000&interest=5&start_date=2023-01-01">$1,000 at 5% from 2023-01-01</a>
            <a class="example-link" href="https://bugos.github.io/face-value/#amount=5000&interest=3.5&start_date=2024-01-01">$5,000 at 3.5% from 2024-01-01</a>
            <a class="example-link" href="https://bugos.github.io/face-value/#amount=10000&interest=7&start_date=2022-06-15">$10,000 at 7% from 2022-06-15</a>
            <a class="example-link" href="https://bugos.github.io/face-value/#amount=25000&interest=4.25&start_date=2020-03-01">$25,000 at 4.25% from 2020-03-01</a>
        </div>
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
    <div class="footer">
        <a href="https://github.com/bugos/face-value">View source on GitHub</a>
    </div>
</body>
</html>
EOL

# Also create a copy of test.html in the root directory for GitHub Actions
cp dist/test.html ./test.html

echo "✅ Deployment files are ready in the 'dist' directory."
echo "📱 Your app will be available at: https://bugos.github.io/face-value/"
echo "🔍 You can view the deployment status at: https://github.com/bugos/face-value/actions"
