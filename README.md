# Proxy Security Guard (PSG)

[![Rust](https://img.shields.io/badge/rust-1.75-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Proxy Security Guard (PSG) is a high-performance, security-focused proxy server built in Rust. It acts as a secure gateway for your backend services, providing request integrity verification and digital signing capabilities.

## Features

*   **Request Integrity Verification:** PSG uses HMAC-SHA256 to ensure that incoming requests have not been tampered with.
*   **Digital Signature Notary:** For file uploads, PSG can generate an Ed25519 digital signature of the file, acting as a "copyright seal" to prove the file's authenticity.
*   **High Performance:** Built with Rust and the `tokio` runtime, PSG is designed for high performance and low resource usage.
*   **Easy to Configure:** PSG is easy to configure using environment variables.
*   **Flexible:** PSG can proxy requests to any backend service and supports both `application/json` and `multipart/form-data` content types.

## How it Works

PSG sits between your clients and your backend service. When a client sends a request, it must include a special header, `x-psg-signature`, which contains an HMAC-SHA256 signature of the request body.

PSG intercepts the request, recalculates the HMAC signature, and compares it to the one provided in the header. If the signatures match, PSG forwards the request to the backend service. If the signatures do not match, PSG rejects the request with a `401 Unauthorized` error.

For file uploads (`multipart/form-data`), PSG can also generate an Ed25519 digital signature of the uploaded file. This signature is added to the request headers as `X-PSG-Copyright-Seal` before forwarding it to the backend. This allows the backend to verify the authenticity of the file.

## Getting Started

### Requirements

The pre-compiled binary for Linux requires the following shared libraries to be installed on your system:

- `libgcc_s.so.1`
- `libm.so.6`
- `libc.so.6`

These are standard system libraries and are typically pre-installed on most Linux distributions.

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) (2021 edition or later)

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/ex-jandal/proxy-security-guard.git
    cd proxy-security-guard
    ```

2.  Generate the necessary cryptographic keys:
    ```bash
    cargo run -- --generate-keys > .env
    ```
    This will create a `.env` file with the `HMAC_KEY` and `SIG_KEY` environment variables.

3.  Build the project:
    ```bash
    cargo build --release
    ```

### Running the Proxy

To run the proxy, use the following command:

```bash
cargo run --release
```

By default, the proxy will run on `127.0.0.1:4000`. You can change the port using the `--port` command-line argument:

```bash
cargo run --release -- --port 8080
```

## Configuration

PSG is configured using environment variables. The following variables are required:

*   `HMAC_KEY`: The secret key used for HMAC-SHA256 request integrity verification.
*   `SIG_KEY`: The private key used for Ed25519 digital signing.

These keys are automatically generated when you run the `--generate-keys` command.

## Usage

To use the proxy, clients must include the `x-psg-signature` header in their requests. The value of this header should be the HMAC-SHA256 signature of the request body, encoded as a hexadecimal string.

Here's an example of how to generate the signature in JavaScript:

```javascript
const crypto = require('crypto');

const hmacKey = 'YOUR_HMAC_KEY'; // The HMAC_KEY from your .env file
const requestBody = JSON.stringify({ message: 'Hello, world!' });

const signature = crypto
  .createHmac('sha256', hmacKey)
  .update(requestBody)
  .digest('hex');

// Now, send the request with the signature in the headers
fetch('http://127.0.0.1:4000/your-endpoint', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'x-psg-signature': signature,
  },
  body: requestBody,
});
```

## Command-Line Arguments

*   `--generate-keys`: Generate a new Ed25519 + HMAC private key and save them to a `.env` file.
*   `--port <PORT>`: Set the port for the gateway (default: `4000`).
*   `--debug`: Run in debug mode with verbose logging.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
