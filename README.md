# Face Value Calculator

A debt value calculator built with Rust and WebAssembly.

## Live Demo

[https://bugos.github.io/face-value/](https://bugos.github.io/face-value/)

## Features

- Calculate debt value based on initial amount, interest rate, and start date
- Share calculations via URL parameters (query or hash format)
- Pre-configured examples for quick testing

## Usage

Add parameters to the URL:

```
https://bugos.github.io/face-value/#amount=1000&interest=5&start_date=2023-01-01
```

### Parameters

- `amount`: Initial debt amount
- `interest`: Annual interest rate percentage
- `start_date`: Start date (YYYY-MM-DD format)

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Python 3

### Setup

```bash
git clone https://github.com/bugos/face-value.git
cd face-value
rustup target add wasm32-unknown-unknown
```

### Local Development

```bash
./scripts/serve.sh
```

Then open [http://localhost:8081](http://localhost:8081)

### Production Build

```bash
./scripts/deploy.sh
```

## Deployment

The app is automatically deployed to GitHub Pages when changes are pushed to the main branch.

### Manual Deployment

1. Run `./scripts/deploy.sh`
2. Commit and push to main branch
3. GitHub Actions will handle the deployment

## Project Structure

- `src/` - Rust source code
- `static/` - Static assets
  - `index.html` - HTML template
  - `favicon.ico` - Favicon
- `scripts/` - Build and deployment scripts
  - `serve.sh` - Local development script
  - `deploy.sh` - Production build script
- `.github/workflows/` - GitHub Actions workflows

## License

MIT
