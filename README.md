# Face Value Calculator

A simple web application to calculate the current value of a debt based on the initial amount, interest rate, and start date.

## Live Demo

Visit the live application at: [https://bugos.github.io/face-value/](https://bugos.github.io/face-value/)

## Features

- Calculate debt value based on initial amount, interest rate, and start date
- URL parameter support for easy sharing and bookmarking
- Responsive design that works on mobile and desktop

## Usage

You can use the calculator by adding parameters to the URL:

```
https://bugos.github.io/face-value/#amount=1000&interest=5&start_date=2023-01-01
```

Parameters:
- `amount`: The initial debt amount (numeric)
- `interest`: Annual interest rate as percentage (numeric)
- `start_date`: Start date in YYYY-MM-DD format

## Local Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Running Locally

1. Clone the repository:
   ```
   git clone https://github.com/bugos/face-value.git
   cd face-value
   ```

2. Run the development server:
   ```
   ./serve.sh
   ```

3. Open your browser to [http://localhost:8081](http://localhost:8081)

## Deployment

The application is automatically deployed to GitHub Pages when changes are pushed to the main branch.

To manually prepare files for deployment:

```
./deploy.sh
```

This will create a `deploy` directory with all the necessary files.

## License

MIT
