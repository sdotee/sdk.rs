# S.EE SDK for Rust

[![Crates.io](https://img.shields.io/crates/v/see-rust-sdk.svg)](https://crates.io/crates/see-rust-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/sdotee/sdk.rs/actions/workflows/cargo.yml/badge.svg)](https://github.com/sdotee/sdk.rs/actions)

A clean and elegant Rust SDK for URL shortening services using the [s.ee API](https://s.ee/docs/development/api/).

## Introduction

see-rust-sdk is a Rust client library specifically designed for the s.ee URL shortening service. It provides a type-safe and easy-to-use API that allows you to easily integrate URL shortening functionality into your Rust applications. Whether you need simple URL shortening or advanced features like custom aliases, expiration times, and tag management, this SDK has you covered.

## Key Features

- 🚀 **Clean & Intuitive** - Fluent API design that's easy to understand and use
- 🔒 **Type-Safe** - Complete type checking and error handling
- ⚙️ **Flexible Configuration** - Support for timeout, retry, custom domains, and more
- 🏷️ **Feature Complete** - Support for tag management, domain management, URL deletion, and more
- 📦 **Lightweight Dependencies** - Minimal dependencies for fast compilation
- ✅ **Well-Tested** - Comprehensive unit tests and documentation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
see-rust-sdk = "1.0.0"
```

And then run `cargo build` to download and compile the crate. For the latest version, check out the [crates.io page](https://crates.io/crates/see-rust-sdk).

## Quick Start

```rust
use see_rust_sdk::client::Client;
use see_rust_sdk::config::Config;
use see_rust_sdk::url::builder::UrlShortenerRequestBuilder;
use see_rust_sdk::url::ShortenService;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your API key
    let config = Config::default().with_api_key("your-api-key-here");
    let client = Client::new(config)?;
    
    // Shorten a URL
    let request = UrlShortenerRequestBuilder::new("https://example.com/very/long/url")?
        .build();
    
    let response = client.shorten(request)?;
    println!("Shortened URL: {}", response.data.short_url);
    
    Ok(())
}
```

## Examples

The `examples/` directory in this repository contains complete usage examples covering various common scenarios:

- **`basic.rs`** - Basic URL shortening functionality
- **`advanced.rs`** - Advanced features (tag management, domain management, custom options, etc.)
- **`batch.rs`** - Batch processing and error retry logic

> 💡 **Tip**: Make sure to set a valid API key in the code before running the examples.

## Development & Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific tests (requires API Key environment variable)
export URL_SHORTENER_API_KEY="your-api-key"
cargo test -- --nocapture
```

Build examples:

```bash
cargo build --examples
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! If you find bugs or have feature suggestions, please submit an Issue or Pull Request.
