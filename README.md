# Face Value Calculator

A simple web application to calculate the current value of a debt based on the initial amount, interest rate, and start date. Built with Rust and WebAssembly using the Dioxus framework.

## Live Demo

Visit the live application at: [https://bugos.github.io/face-value/](https://bugos.github.io/face-value/)

## Features

- **Debt Value Calculation**: Calculate the current value of a debt based on:
  - Initial amount
  - Annual interest rate
  - Start date
- **URL Parameter Support**: Easily share calculations by adding parameters to the URL
  - Works with both query parameters (`?amount=1000`) and hash parameters (`#amount=1000`)
  - Bookmarkable for future reference
- **Interactive Examples**: Click on pre-configured examples to see different scenarios
- **Responsive Design**: Works on mobile and desktop devices
- **Offline Capable**: Once loaded, calculations can be performed without an internet connection
- **Fast Performance**: Built with Rust and compiled to WebAssembly for native-like speed
- **Minimal Dependencies**: Small download size for quick loading

## Usage

You can use the calculator by adding parameters to the URL:

```
https://bugos.github.io/face-value/#amount=1000&interest=5&start_date=2023-01-01
```

### Parameters

- `amount`: The initial debt amount (numeric)
- `interest`: Annual interest rate as percentage (numeric)
- `start_date`: Start date in YYYY-MM-DD format

### Examples

The application includes several pre-configured examples:
- $1,000 at 5% from 2023-01-01
- $5,000 at 3.5% from 2024-01-01
- $10,000 at 7% from 2022-06-15
- $25,000 at 4.25% from 2020-03-01

## Local Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for WebAssembly compilation)
- Python 3 (for the local development server)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/bugos/face-value.git
   cd face-value
   ```

2. Install the WebAssembly target for Rust:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

### Running Locally

Run the development server:
```bash
./serve.sh
```

This will:
1. Build the WebAssembly module
2. Create a `dist` directory with all necessary files
3. Start a local server on port 8081

Open your browser to [http://localhost:8081](http://localhost:8081)

### Building for Production

To build the application for production:
```bash
./deploy.sh
```

This will:
1. Build the WebAssembly module
2. Create a `dist` directory with all necessary files
3. Generate a test page with example links

The production-ready files will be in the `dist` directory.

## Deployment to GitHub Pages

### Automatic Deployment

The application is automatically deployed to GitHub Pages when changes are pushed to the main branch using GitHub Actions.

### Manual Deployment

1. Build the application:
   ```bash
   ./deploy.sh
   ```

2. Commit and push your changes to the main branch:
   ```bash
   git add .
   git commit -m "Your commit message"
   git push origin main
   ```

3. GitHub Actions will automatically deploy the application to GitHub Pages.

### Checking Deployment Status

You can check the status of your deployment at:
[https://github.com/bugos/face-value/actions](https://github.com/bugos/face-value/actions)

## Project Structure

- `src/` - Rust source code
  - `app.rs` - Main application logic
  - `lib.rs` - WebAssembly entry point
- `index.html` - HTML template
- `serve.sh` - Script for local development
- `deploy.sh` - Script for production builds
- `.github/workflows/deploy.yml` - GitHub Actions workflow for deployment

## License

MIT
