# mpesa-rust

A strongly-typed, async Rust SDK for the Safaricom M-Pesa API.

This project is a from-scratch implementation focused on correctness, safety, and proper Rust architecture. It mirrors the behavior of common Node.js M-Pesa SDKs, but with Rust’s type system, async model, and explicit error handling.

---

## Features Implemented

### Core Architecture

* Modular crate layout (`config`, `error`, `mpesa`)
* Strongly typed configuration
* Environment-aware base URL handling (Sandbox vs Production)
* Central client struct (`Mpesa`)
* Reusable async HTTP client (`reqwest::Client`)
* Structured error handling (`MpesaError`)
* Async-first API design

### OAuth Authentication

* Fully implemented OAuth token retrieval
* Uses HTTP Basic Auth
* JSON response deserialization
* Real network call verified against Safaricom sandbox
* Tested using async `tokio` tests

### Error Handling

* No panics, no unwraps in library code
* Explicit error variants:

  * MissingConsumerKey
  * MissingConsumerSecret
  * NotAllowedInProduction
  * RequestFailed
  * JsonParseFailed

### Async Infrastructure

* Built on `tokio`
* Uses `reqwest` for HTTP
* Uses `serde` for JSON parsing

---

## Project Structure

```
src/
  lib.rs        # Public API exports
  config.rs     # Config + Environment
  error.rs      # MpesaError
  mpesa.rs      # Mpesa client + methods
```

---

## Usage

### 1. Set Environment Variables

Never hardcode secrets.

```bash
export MPESA_CONSUMER_KEY="your_key_here"
export MPESA_SECRET_KEY="your_secret_here"
```

---

### 2. Create Client

```rust
use mpesa_rust::{Config, Environment, Mpesa};

let config = Config::new(
    std::env::var("MPESA_CONSUMER_KEY").unwrap(),
    std::env::var("MPESA_SECRET_KEY").unwrap(),
    Environment::Sandbox,
);

let mpesa = Mpesa::new(config)?;
```

---

### 3. Get OAuth Token

```rust
let token = mpesa.oauth().await?;
println!("{:?}", token);
```

---

## Testing

OAuth is tested using an async test:

```bash
cargo test -- --nocapture
```

This performs a real network request to Safaricom’s sandbox and prints the token.

---

## What’s Next

Planned features:

* Authenticated request helper (Bearer token)
* Token caching
* Account Balance endpoint
* B2C
* B2B
* STK Push
* Transaction Status
* Reversal
* C2B Register / Simulate
* Request/response structs
* Better error mapping
* Logging hooks
* Timeouts & retries
