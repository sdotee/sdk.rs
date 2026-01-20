# S.EE Rust SDK

[![Crates.io](https://img.shields.io/crates/v/see-sdk.svg)](https://crates.io/crates/see-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/sdotee/sdk.rs/actions/workflows/cargo.yml/badge.svg)](https://github.com/sdotee/sdk.rs/actions)

A clean and elegant Rust SDK for content sharing services (URL, Text, File) using the [S.EE API](https://s.ee/docs/developers/).

## Introduction

see-sdk is a Rust client library specifically designed for the S.EE content sharing service. It provides a type-safe and easy-to-use API that allows you to easily integrate URL shortening, text sharing, and file sharing functionality into your Rust applications. Whether you need simple content sharing or advanced features like custom aliases, expiration times, and tag management, this SDK has you covered.

## Key Features

- 🚀 **Clean & Intuitive** - Fluent API design that's easy to understand and use
- 🔒 **Type-Safe** - Complete type checking and error handling
- ⚙️ **Flexible Configuration** - Support for timeout, retry, custom domains, and more
- 📁 **Multi-Format Support** - Unified support for URL shortening, text sharing, and file sharing
- 🏷️ **Feature Complete** - Support for tag management, domain management, content deletion, and more
- 📦 **Lightweight Dependencies** - Minimal dependencies for fast compilation
- ✅ **Well-Tested** - Comprehensive unit tests and documentation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
see-sdk = "1.1.0"
```

And then run `cargo build` to download and compile the crate. For the latest version, check out the [crates.io page](https://crates.io/crates/see-sdk).

## Examples

For comprehensive usage examples covering all features, please refer to the [examples/](examples/) directory in this repository.

Available examples include:
- **[basic.rs](examples/basic.rs)**: Basic URL shortening operations
- **[advanced.rs](examples/advanced.rs)**: Advanced features like custom aliases and expiration
- **[files.rs](examples/files.rs)**: File upload and management operations
- **[text.rs](examples/text.rs)**: Text sharing and pastebin functionality
- **[batch.rs](examples/batch.rs)**: Batch processing capabilities

You can run any example using cargo:

```bash
# Export your API key first
export SEE_API_KEY="your-api-key"

# Run a specific example
cargo run --example basic
```

## Development & Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific tests (requires API Key environment variable)
export SEE_API_KEY="your-api-key"
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
