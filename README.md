# Face Value - Debt Calculator

A static web application built with Rust and Dioxus that calculates the current value of debt based on URL parameters. The app can run directly from your file system without a web server!

## Features

- Calculate debt value based on initial amount, interest rate, and start date
- Parameters stored in URL or hash for easy sharing
- Real-time calculation based on days passed
- Clean, responsive UI
- Works directly from file system - no web server needed!

## Usage

You can use the calculator in two ways:

1. Using URL hash parameters (works from file system):
   ```
   file:///path/to/index.html#amount=1000&interest=5&start_date=2023-01-01
   ```

2. Using URL query parameters (when served from a web server):
   ```
   http://localhost:8080/?amount=1000&interest=5&start_date=2023-01-01
   ```

Parameters:
- `amount`: Initial debt amount (numeric)
- `interest`: Annual interest rate as a percentage (numeric)
- `start_date`: Start date in YYYY-MM-DD format

## Building and Running

1. Install Rust and Cargo:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Trunk (for building):
   ```bash
   cargo install trunk
   ```

3. Add the WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

4. Build the app:
   ```bash
   trunk build --release
   ```

5. The built files will be in the `dist` directory. You can:
   - Open `dist/index.html` directly in your browser
   - Host the `dist` directory on any static file server
   - Share the files with others to run locally

## Development

For development, you can use:
```bash
trunk serve
```

This will start a development server with hot reloading at `http://localhost:8080` 